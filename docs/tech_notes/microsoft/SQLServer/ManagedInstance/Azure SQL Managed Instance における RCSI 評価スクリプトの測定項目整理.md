# Azure SQL Managed Instance における RCSI 評価スクリプトの測定項目整理

> 位置づけ: この文書は、Azure SQL Managed Instance で RCSI を評価するための計測スクリプトについて、「各列が何を表すか」「なぜその列を見るのか」「どう読むべきか」を整理する測定項目リファレンスです。評価スクリプトの実装方針そのものよりも、取得している値の意味と読み方に責務を絞ります。
>
> 読み分け: Managed Instance 固有の前提は [[Azure SQL Managed Instance における RCSI 評価の前提整理]]、観測設計の全体方針と参照スクリプトは [[Azure SQL Managed Instance における RCSI 評価スクリプト設計ガイド]]、`sys.server_resource_stats` 単体の詳説は [[sys.server_resource_stats 詳細解説]]、lock / blocking / version store の概念整理は [[../Concurrency/SQL Server の Lock と Blocking と Version Store の整理]]、`pvs_size_mb` と in-row / off-row の関係は [[../Concurrency/SQL Server で ADR ON のとき PVS は更新のたびに必ず増えるのか]] を参照してください。

## 1. 文脈と目的

この計測スクリプトが答えたい中心的な問いは、単に「RCSI を ON にしたら速くなるか」ではありません。より具体的には、長時間更新トランザクションがある状況でも reader-writer blocking が減るか、その代わりに version store、I/O、ログ、ストレージ、メモリへどの程度コストが移るかを確認することです。

Azure SQL Managed Instance では ADR が常時有効であり、行バージョンの主戦場は `tempdb` ではなく PVS です。そのため、従来の RCSI 評価のように `tempdb` の version store だけを見ても、Managed Instance での実際のコストは捉えきれません。このスクリプトはその前提を織り込み、lock / blocking の改善、PVS の増加、PVS cleanup 停滞、ログとストレージの圧迫までを一つの表へ集約しています。[locking-row-versioning][adr-concepts][adr-troubleshoot][pvs-stats][server-resource-stats]

## 2. この調査が答える問い

このスクリプトは、主に次の四つの問いに答えるために設計されています。

1. `READ_COMMITTED_SNAPSHOT = ON` にすると、長時間更新中でも照会はロック待機しにくくなるか。
2. その効果の裏で、CPU、メモリ、DB I/O、ログ、version store にどんな副作用が出るか。
3. ADR が有効な Managed Instance で、行バージョンの増加は `tempdb` ではなく PVS に現れているか。
4. PVS 増加がインスタンス全体のストレージ上限や対象 DB のデータ / ログ使用量へ波及していないか。

重要なのは、このスクリプトが「RCSI が効いたか」を単一の列で判定しようとしていない点です。lock wait、blocking、PVS、cleanup 停滞、ログ、ストレージを別断面として並べ、原因と副作用を分けて読めるようにしています。

## 3. 先に分けて読むべき値の種類

このスクリプトの列は、性質の異なる値が混在しています。これを混ぜて読むと誤解しやすくなります。

| 種類 | 代表列 | 性質 | 読み方 |
| --- | --- | --- | --- |
| 前提確認の瞬間値 | `rcsi_on`, `adr_on`, `row_version_store_location_desc` | その時点の設定状態 | 「この試験の前提は何か」を固定するために使います。 |
| 瞬間スナップショット | `blocking_count`, `pvs_size_mb`, `log_used_percent`, `mi_storage_used_mb` | その瞬間の残量や件数 | 短時間の変動を見ますが、瞬間値なので取りこぼしがあります。 |
| 累積差分 | `lock_wait_ms`, `database_io_total_ms`, `pvs_filegroup_io_total_ms` | 累積カウンターの差分 | 区間比較で読みます。サンプリング間隔に依存します。 |
| 遅延を伴うサービスメーター | `cpu_usage_ratio`, `mi_reserved_storage_mb`, `mi_storage_used_mb` | Managed Instance 側の集計値 | 15 秒区間の値ですが、ビュー更新には遅延があります。短い試験の即時 CPU とみなさない方が安全です。 |
| スクリプト派生列 | `effective_version_store_mb`, `lock_wait_ratio`, `mi_storage_used_ratio` | DMV の生列ではなく、スクリプト側で加工 | 算出式を理解してから使う必要があります。 |

## 4. このスクリプトの読み順

最初から全部の列を見るより、次の順で読む方が解釈が安定します。

1. 前提を確認する。
   `rcsi_on`、`adr_on`、`current_session_isolation_level_desc`、`row_version_store_location_desc` を見て、何を比較している試験なのかを固定します。
2. RCSI の効果を見る。
   `lock_wait_ms`、`lock_wait_ratio`、`blocking_count` を見て、reader-writer blocking が減ったかを確認します。
3. versioning コストの置き場所を見る。
   `effective_version_store_mb`、`pvs_size_mb`、`tempdb_version_store_mb_reference`、`pvs_filegroup_io_*` を見て、コストがどこへ出たかを確認します。
4. cleanup 停滞の原因を見る。
   `active_snapshot_tx_count`、`snapshot_elapsed_max_sec`、`pvs_oldest_active_transaction_id`、`pvs_skip_oldest_active_pages`、`offrow_version_cleaner_running` を見ます。
5. 全体負荷と容量リスクを見る。
   `database_io_*`、`log_used_percent`、`mi_storage_used_ratio`、`db_rows_used_mb`、`db_log_used_mb` を見て、副作用が許容範囲かを判断します。

この順番にしておくと、「lock は改善したが PVS cleanup が止まり、結果としてログとストレージが苦しくなった」のような因果を追いやすくなります。

## 5. 測定項目一覧

### 5.1 実験条件と前提確認

| 列 | 主な取得元 | 何を見る列か | なぜ必要か |
| --- | --- | --- | --- |
| `sample_time`, `database_name` | `SYSDATETIME()`, `DB_NAME()` | いつ、どの DB を採ったサンプルか | 複数試験や前後比較で時系列を揃える基礎になります。 |
| `rcsi_on` | `sys.databases.is_read_committed_snapshot_on` | 対象 DB で RCSI が有効か | 効果判定の前提そのものです。 |
| `snapshot_isolation_state_desc` | `sys.databases.snapshot_isolation_state_desc` | `ALLOW_SNAPSHOT_ISOLATION` の状態 | `SNAPSHOT` の有効化状態を RCSI と切り分けるために見ます。 |
| `adr_on` | `sys.databases.is_accelerated_database_recovery_on` | ADR が有効か | row version の主な格納先が `tempdb` か PVS かを判断する前提です。Managed Instance では通常 `1` です。[adr-management][locking-row-versioning] |
| `compatibility_level` | `sys.databases.compatibility_level` | DB の互換レベル | optimizer や実行計画面の前提差をメモするためです。RCSI 自体の ON/OFF とは別軸です。 |
| `current_session_isolation_level_desc` | `sys.dm_exec_sessions` | この計測セッション自身の分離レベル | ワークロード側ではなく、あくまで採取セッションの状態確認です。`DBCC USEROPTIONS` の補助的な置き換えとして見ます。 |
| `row_version_store_location_desc` | スクリプト派生 | `ADR ON => PVS(user database)`、それ以外は `tempdb` とした説明列 | この試験で version store をどこ中心に見るべきかを即座に分かるようにするためです。 |
| `pvs_filegroup_id`, `pvs_filegroup_name`, `pvs_filegroup_scope_desc` | `sys.dm_tran_persistent_version_store_stats`, `sys.filegroups` | PVS がどの filegroup にあるか | PVS が `PRIMARY` 共有なのか、専用 filegroup なのかで、I/O や容量の解釈が変わるためです。[pvs-stats][adr-management] |

ここで特に誤解しやすいのは、`snapshot_isolation_state_desc` と `current_session_isolation_level_desc` は同じ意味ではないという点です。前者は DB 設定、後者は計測セッションの現在値です。

### 5.2 RCSI の効果を直接見る列

| 列 | 主な取得元 | 何を見る列か | なぜ必要か |
| --- | --- | --- | --- |
| `lock_wait_ms` | `sys.dm_os_wait_stats` の `LCK%` 差分 | 観測区間で新たに積み上がった lock wait 時間 | RCSI の直接的な狙いは reader-writer の lock wait を減らすことなので、最初に見る中心指標です。[dm-os-wait-stats] |
| `lock_wait_ratio` | `LCK%` 差分 / 全 wait 差分 | その区間で lock wait が全体 wait に占める割合 | 単純な待機時間だけではなく、「全 wait の中で lock がどれだけ支配的か」を見るためです。 |
| `blocking_count` | `sys.dm_exec_requests` | その瞬間に対象 DB 内で `blocking_session_id > 0` の要求数 | `lock_wait_ms` が累積差分なのに対し、こちらは瞬間ブロッキングです。いま詰まっているかを補完します。[dm-exec-requests] |

`lock_wait_ms` と `blocking_count` は同じものではありません。前者は区間累積、後者は瞬間件数です。短いブロッキングは `blocking_count` で見逃すことがあり、逆に長い区間では `lock_wait_ms` の方が方向性をつかみやすくなります。

### 5.3 Version Store と PVS を見る列

| 列 | 主な取得元 | 何を見る列か | なぜ必要か |
| --- | --- | --- | --- |
| `effective_version_store_mb` | スクリプト派生 | `ADR ON` なら PVS サイズ、そうでなければ `tempdb` version store サイズ | この環境で実際に主指標として見るべき version store 容量を 1 列に抽象化したものです。 |
| `tempdb_version_store_mb_reference` | `sys.dm_tran_version_store_space_usage` | `tempdb` 側の version store 使用量 | Managed Instance では主指標ではありませんが、参照値として残すことで「tempdb にはどの程度出ているか」を確認できます。[dm-tran-version-store-space-usage] |
| `pvs_size_mb` | `sys.dm_tran_persistent_version_store_stats.persistent_version_store_size_kb` | PVS の off-row version サイズ | ADR 環境での versioning コスト本体です。`persistent_version_store_size_kb` は off-row 分のみで、in-row version は含みません。[pvs-stats] |
| `pvs_filegroup_io_read_ms`, `pvs_filegroup_io_write_ms`, `pvs_filegroup_io_total_ms` | `sys.dm_io_virtual_file_stats` + `sys.database_files` の差分 | PVS を置いている filegroup の I/O stall 差分 | version が増えるだけでなく、その filegroup の待機時間まで悪化していないかを確認するためです。[dm-io-virtual-file-stats] |
| `pvs_filegroup_bytes_read_mb`, `pvs_filegroup_bytes_written_mb` | 同上 | PVS filegroup の読み書き量差分 | 待機時間だけでなく、実際にどの程度の I/O 量が発生したかを見ます。stall と量を切り分けるためです。 |
| `active_snapshot_tx_count` | `sys.dm_tran_active_snapshot_database_transactions` | row version を生成または参照しうるアクティブトランザクション数 | 明示的な `SNAPSHOT` だけでなく、RCSI や version 生成側も含めて、cleanup を止めうる活動量を見ます。[active-snapshot-dmv] |
| `snapshot_elapsed_max_sec` | 同上の `elapsed_time_seconds` 最大値 | 最も長く生存している snapshot / row-versioning 関連トランザクションの経過時間 | 長時間トランザクションが version cleanup を遅らせていないかを見るためです。[adr-troubleshoot][active-snapshot-dmv] |
| `pvs_oldest_active_transaction_id` | `sys.dm_tran_persistent_version_store_stats.oldest_active_transaction_id` | PVS cleanup の下限を決める最古のアクティブトランザクション ID | 「誰かが古い version を必要としていて掃除できない」状態を把握する手掛かりです。[pvs-stats] |
| `pvs_skip_oldest_active_pages` | `sys.dm_tran_persistent_version_store_stats.pvs_off_row_page_skipped_oldest_active_xdesid` | cleanup が oldest active transaction のために飛ばしたページ数 | 値が増えるなら、最古アクティブトランザクションが cleanup を阻害している可能性が高いことを示します。[pvs-stats] |
| `offrow_version_cleaner_running` | `offrow_version_cleaner_start_time` / `end_time` から派生 | off-row version cleaner が現在走っているか | cleaner が動作中かを把握する補助列です。`1` だから健全、`0` だから異常という単純判定には使いません。[pvs-stats] |

`effective_version_store_mb` と `pvs_size_mb` は、ADR が有効な Managed Instance ではほぼ同じ方向を向きます。ただし前者は抽象化列、後者は PVS の生観測列なので、説明責務が異なります。

### 5.4 CPU、メモリ、ストレージ、DB 容量を見る列

| 列 | 主な取得元 | 何を見る列か | なぜ必要か |
| --- | --- | --- | --- |
| `cpu_usage_ratio` | `sys.server_resource_stats.avg_cpu_percent`、失敗時は `sys.dm_os_ring_buffers` | Managed Instance の CPU 利用率参考値 | RCSI によって lock は減っても CPU へ負荷が移っていないかを確認するためです。ただし主経路はサービスメーターであり、更新遅延があります。[server-resource-stats][mi-monitoring-dmvs] |
| `sql_memory_ratio` | `sys.dm_os_sys_info` | `committed_kb / committed_target_kb` | SQL Server が目標メモリに対してどの程度コミットしているかを見ます。SQL エンジン側の余裕感を把握する補助指標です。[dm-os-sys-info] |
| `os_memory_ratio` | `sys.dm_os_sys_memory` | `available_physical_memory_kb / total_physical_memory_kb` | OS 側の物理メモリ空き率です。名前だけ見ると使用率に見えますが、実際は free ratio です。[dm-os-sys-memory] |
| `mi_reserved_storage_mb`, `mi_storage_used_mb`, `mi_storage_used_ratio` | `sys.server_resource_stats` | インスタンス全体の予約済みストレージ、使用済みストレージ、その比率 | PVS 増加が Managed Instance 全体のストレージ上限へ近づくリスクを確認するためです。対象 DB だけの値ではありません。[server-resource-stats] |
| `db_rows_allocated_mb`, `db_rows_used_mb`, `db_rows_free_mb` | `sys.database_files` + `FILEPROPERTY` | 対象 DB の ROWS ファイル割当量、使用量、空き量 | PVS とユーザーデータが同じ filegroup / data file を共有する場合、どこまで DB 内部で余裕があるかを見るためです。 |
| `db_log_allocated_mb`, `db_log_used_mb` | `sys.database_files`, `sys.dm_db_log_space_usage` | 対象 DB のログ割当量と使用量 | versioning に伴ってログ消費が増え、ログが逼迫していないかを見るためです。[dm-db-log-space-usage] |
| `db_total_allocated_mb`, `db_total_used_mb` | スクリプト派生 | 対象 DB の全割当量、使用総量 | DB 単位の容量増加を、ROWS と LOG を合わせて把握するためです。 |

ここで `mi_storage_used_mb` はインスタンス全体、`db_total_used_mb` は対象 DB 単位です。スコープが違うため、同じ比率感で並べて読むと誤解しやすくなります。

### 5.5 DB I/O とログ圧力を見る列

| 列 | 主な取得元 | 何を見る列か | なぜ必要か |
| --- | --- | --- | --- |
| `database_io_read_ms`, `database_io_write_ms`, `database_io_total_ms` | `sys.dm_io_virtual_file_stats(@dbid, NULL)` 差分 | 対象 DB 全ファイルの I/O stall 差分 | RCSI で lock は減っても、データ / ログ I/O 待機が増えていないかを見るためです。[dm-io-virtual-file-stats] |
| `log_used_percent`, `log_used_mb` | `sys.dm_db_log_space_usage` | その瞬間のログ使用率と使用量 | 長時間トランザクションや version cleanup 停滞がログ切り捨てへ影響していないかを見るためです。[dm-db-log-space-usage] |

`db_log_used_mb` と `log_used_mb` は、どちらも `sys.dm_db_log_space_usage.used_log_space_in_bytes` を元にしています。前者は DB 容量一覧の一部、後者はログ圧力の専用列として重複して保持していると理解すると分かりやすくなります。

## 6. このスクリプトで重要な派生列

このスクリプトには、生の DMV 列ではなく、判断しやすいように組み替えた列がいくつかあります。

| 列 | 算出方法 | 意味 |
| --- | --- | --- |
| `effective_version_store_mb` | `ADR ON => PVS`, `ADR OFF => tempdb version store` | その環境で主として見るべき version store 容量を 1 列で表します。 |
| `lock_wait_ratio` | `delta(LCK wait) / delta(total wait) * 100` | lock wait が全 wait に占める比率です。 |
| `mi_storage_used_ratio` | `mi_storage_used_mb / mi_reserved_storage_mb * 100` | MI 全体ストレージの逼迫度です。 |
| `db_total_used_mb` | `ROWS 使用量 + LOG 使用量` | 対象 DB の実使用量を粗く合算した列です。 |
| `offrow_version_cleaner_running` | `start_time IS NOT NULL AND end_time IS NULL` | PVS off-row cleaner が現在進行中かどうかの目安です。 |
| `row_version_store_location_desc` | `ADR ON => PVS(user database)`、それ以外は `tempdb` | version store の主観測先を説明するための列です。 |
| `pvs_filegroup_scope_desc` | `PRIMARY(shared with user data)` / `Dedicated filegroup` / `PVS not active` | PVS の配置形態を人間向けに読みやすくした説明列です。 |

## 7. 誤解しやすい点

### 7.1 `cpu_usage_ratio` は常に同じ意味ではない

スクリプトはまず `sys.server_resource_stats.avg_cpu_percent` を読み、失敗した場合は `sys.dm_os_ring_buffers` の `ProcessUtilization` へフォールバックします。前者は Managed Instance のサービスティア上限比、後者はリングバッファ由来の CPU 参考値であり、完全に同義ではありません。列名だけ見て同じ意味だと決めつけない方が安全です。[server-resource-stats]

### 7.2 `tempdb_version_store_mb_reference` は Managed Instance の主指標ではない

ADR が有効な Managed Instance では、RCSI/SNAPSHOT の行バージョンは主として PVS に置かれます。したがって `tempdb_version_store_mb_reference` は「ゼロでないかもしれない補助観測」であって、本体指標ではありません。[locking-row-versioning][pvs-stats]

### 7.3 `pvs_size_mb` は PVS の全容量を完全に表すわけではない

`persistent_version_store_size_kb` は off-row version のサイズであり、in-row version は含みません。そのため、PVS の全影響を 1 列で完全に表しているわけではなく、主として off-row version の増減を見る列だと理解するのが適切です。[pvs-stats]

この論点だけを独立して整理したい場合は、[[../Concurrency/SQL Server で ADR ON のとき PVS は更新のたびに必ず増えるのか]] を参照してください。ADR ON での versioning 自体と、`pvs_size_mb` が見ている off-row 観測値を分けて説明しています。

### 7.4 `lock_wait_ratio` が低いから lock 問題がない、とは言い切れない

分母は全 wait 差分なので、別の wait が大きければ lock 比率は見かけ上小さくなります。`lock_wait_ms` の絶対差分、`blocking_count`、実際の待機種別と併せて読む必要があります。[dm-os-wait-stats]

### 7.5 `blocking_count` は瞬間値なので、短い blocking を見逃す

3 秒間隔サンプリングでは、瞬間的に起きてすぐ解消した blocking は取りこぼします。長く詰まる blocking には強い一方、短い競合は `lock_wait_ms` 差分の方が拾いやすい場合があります。[dm-exec-requests]

## 8. まとめ

この計測スクリプトは、RCSI の効果を lock wait と blocking で見つつ、その副作用を PVS、cleanup 停滞、DB I/O、ログ、ストレージへ広げて追うための観測表です。Managed Instance では ADR と PVS が前提にあるため、`tempdb` を主戦場だと思って読むと外しやすく、`effective_version_store_mb` と `pvs_size_mb` を中心に据える方が実態に合います。

判断の順番としては、まず `rcsi_on` と `adr_on` で前提を固定し、次に `lock_wait_ms` と `blocking_count` で効果を見ます。そのうえで `pvs_size_mb`、`snapshot_elapsed_max_sec`、`pvs_skip_oldest_active_pages` で version cleanup 停滞を確認し、最後に `log_used_percent`、`database_io_total_ms`、`mi_storage_used_ratio` で副作用が許容範囲かを判断する、という流れが最もぶれにくくなります。

## 参考

- [SET TRANSACTION ISOLATION LEVEL][set-transaction-isolation]
- [Transaction locking and row versioning guide][locking-row-versioning]
- [Accelerated database recovery][adr-concepts]
- [Manage accelerated database recovery][adr-management]
- [Monitor and troubleshoot accelerated database recovery][adr-troubleshoot]
- [sys.dm_tran_persistent_version_store_stats][pvs-stats]
- [sys.dm_tran_version_store_space_usage][dm-tran-version-store-space-usage]
- [sys.dm_tran_active_snapshot_database_transactions][active-snapshot-dmv]
- [sys.dm_os_wait_stats][dm-os-wait-stats]
- [sys.dm_exec_requests][dm-exec-requests]
- [sys.dm_io_virtual_file_stats][dm-io-virtual-file-stats]
- [sys.dm_os_sys_info][dm-os-sys-info]
- [sys.dm_os_sys_memory][dm-os-sys-memory]
- [sys.dm_db_log_space_usage][dm-db-log-space-usage]
- [sys.server_resource_stats (Azure SQL Managed Instance)][server-resource-stats]
- [Monitoring Azure SQL Managed Instance performance using dynamic management views][mi-monitoring-dmvs]

[set-transaction-isolation]: https://learn.microsoft.com/sql/t-sql/statements/set-transaction-isolation-level-transact-sql?view=sql-server-ver17
[locking-row-versioning]: https://learn.microsoft.com/sql/relational-databases/sql-server-transaction-locking-and-row-versioning-guide?view=sql-server-ver17
[adr-concepts]: https://learn.microsoft.com/sql/relational-databases/accelerated-database-recovery-concepts?view=sql-server-ver17
[adr-management]: https://learn.microsoft.com/sql/relational-databases/accelerated-database-recovery-management?view=sql-server-ver17
[adr-troubleshoot]: https://learn.microsoft.com/sql/relational-databases/accelerated-database-recovery-troubleshoot?view=sql-server-ver17
[pvs-stats]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-tran-persistent-version-store-stats?view=sql-server-ver17
[dm-tran-version-store-space-usage]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-tran-version-store-space-usage?view=sql-server-ver17
[active-snapshot-dmv]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-tran-active-snapshot-database-transactions-transact-sql?view=sql-server-ver17
[dm-os-wait-stats]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-os-wait-stats-transact-sql?view=sql-server-ver17
[dm-exec-requests]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-exec-requests-transact-sql?view=sql-server-ver17
[dm-io-virtual-file-stats]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-io-virtual-file-stats-transact-sql?view=sql-server-ver17
[dm-os-sys-info]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-os-sys-info-transact-sql?view=sql-server-ver17
[dm-os-sys-memory]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-os-sys-memory-transact-sql?view=sql-server-ver17
[dm-db-log-space-usage]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-db-log-space-usage-transact-sql?view=sql-server-ver17
[server-resource-stats]: https://learn.microsoft.com/sql/relational-databases/system-catalog-views/sys-server-resource-stats-azure-sql-database?view=azuresqldb-current
[mi-monitoring-dmvs]: https://learn.microsoft.com/azure/azure-sql/managed-instance/monitoring-with-dmvs?view=azuresql
