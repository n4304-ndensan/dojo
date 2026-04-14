# SQL Server DMV一覧

> 位置づけ: この文書は SQLServer フォルダの監視オブジェクト索引です。何を判断するためにどの DMV / DMF / 関連ビューを使うかを整理し、列の詳細は `DMV/` 配下へ委譲します。
>
> 読み分け: RCSI 評価の流れは [[SQL Server 入門から始める RCSI 性能評価研修]]、Azure SQL Managed Instance 固有の観測観点は [[ManagedInstance/Azure SQL Managed Instance における RCSI 評価の前提整理]]、フォルダ全体の案内は [[SQL Server ドキュメントガイド]] を参照してください。

この文書は、[[SQL Server 入門から始める RCSI 性能評価研修]] と [[SQL Server の全体像 詳細解説]] で登場する監視オブジェクトを、一覧として整理するための補足資料です。SQL Server の性能分析では `sys.dm_` で始まるオブジェクトがよく登場しますが、実務ではそれ以外の Query Store 関連ビューや Azure SQL Managed Instance 固有のシステムビューも併用します。そのため、この一覧は単なる名前の列挙ではなく、「何が DMV で、何が DMF で、何が関連ビューなのか」を区別しながら読める形にしています。[system-dmvs][system-dmvs] [query-store-monitor][query-store-monitor]

重要なのは、監視オブジェクトを一つずつ暗記することではありません。現在の瞬間状態を見たいのか、一定区間の差分を見たいのか、それともテスト後に履歴を集計したいのかで、使うべきオブジェクトは変わります。本資料では、RCSI 性能評価の文脈で最低限押さえるべき対象に絞って、用途ごとに整理します。

## 0. 個別ドキュメント

DMV / DMF ごとの個別メモを分けて参照できるように、主要オブジェクトについては別ファイル化しました。列の意味も各ファイル側で整理しています。

- [sys.dm_exec_requests](DMV/sys.dm_exec_requests.md)
- [sys.dm_os_wait_stats](DMV/sys.dm_os_wait_stats.md)
- [sys.dm_os_waiting_tasks](DMV/sys.dm_os_waiting_tasks.md)
- [sys.dm_os_sys_info](DMV/sys.dm_os_sys_info.md)
- [sys.dm_os_sys_memory](DMV/sys.dm_os_sys_memory.md)
- [sys.dm_os_tasks](DMV/sys.dm_os_tasks.md)
- [sys.dm_io_virtual_file_stats](DMV/sys.dm_io_virtual_file_stats.md)
- [sys.dm_db_file_space_usage](DMV/sys.dm_db_file_space_usage.md)
- [sys.dm_db_session_space_usage](DMV/sys.dm_db_session_space_usage.md)
- [sys.dm_db_task_space_usage](DMV/sys.dm_db_task_space_usage.md)
- [sys.dm_db_log_space_usage](DMV/sys.dm_db_log_space_usage.md)

なお、Query Store 関連ビューは DMV / DMF ではないため、この個別ドキュメント群には含めていません。`sys.server_resource_stats` については DMV / DMF ではないものの、Azure SQL Managed Instance での利用頻度が高いため、`ManagedInstance/` 配下の個別文書として [[ManagedInstance/sys.server_resource_stats 詳細解説]] を用意しました。

## 1. この一覧の位置づけ

Microsoft Learn では、Dynamic Management Views と Dynamic Management Functions を総称して「SQL Server の内部状態を返す監視オブジェクト」として整理しています。さらに、その中にはサーバースコープのものとデータベーススコープのものがあります。一方で、Query Store の情報は DMV ではなくカタログビュー群に保持され、Azure SQL Managed Instance で使う `sys.server_resource_stats` も DMV ではありません。[system-dmvs][system-dmvs] [db-related-dmvs][db-related-dmvs] [server-related-dmvs][server-related-dmvs]

そのため、この一覧では次の三種類を分けて扱います。

- Dynamic Management View。例として `sys.dm_os_wait_stats` や `sys.dm_exec_requests` があり、現在状態や累積統計を見る中心になります。
- Dynamic Management Function。例として `sys.dm_io_virtual_file_stats` があり、引数を取ってファイル I/O の累積統計を返します。
- 関連ビュー。Query Store のカタログビューや `sys.server_resource_stats` のように、性能分析では重要でも DMV ではないものです。

## 2. 先に押さえる区別

最初に用語の境界を揃えておくと、後の分析がぶれにくくなります。SQL Server の公式総覧では、DMV と DMF はともに `sys` スキーマ配下にあり、サーバー状態情報を返す仕組みとして説明されています。ただし、DMV は通常のビューとして参照できる一方、DMF は関数として呼び出します。たとえば `sys.dm_os_wait_stats` は DMV で、`sys.dm_io_virtual_file_stats` は DMF です。[system-dmvs][system-dmvs] [io-related-dmvs][io-related-dmvs]

ここで注意したいのは、性能分析に頻出するオブジェクトがすべて DMV ではないことです。Query Store の `sys.query_store_runtime_stats` や `sys.query_store_wait_stats` はカタログビューであり、履歴集計のための保存領域です。さらに `sys.server_resource_stats` は Azure SQL Managed Instance で使うシステムビューで、CPU や I/O のサービス側統計を見るための補助情報です。名前が似ていても、内部的な責務は同じではありません。[query-store-monitor][query-store-monitor] [query-store-wait-stats][query-store-wait-stats] [mi-server-resource-stats][mi-server-resource-stats]

## 3. サーバースコープの主要 DMV / DMF

RCSI の評価で最初に見ることが多いのは、サーバー全体の待機、現在実行中の要求、メモリの概況、ファイル I/O の累積値です。これらは主にサーバースコープの DMV / DMF で取得します。サーバースコープのオブジェクトは、SQL Server 2022 未満では通常 `VIEW SERVER STATE`、SQL Server 2022 以降では多くの場合 `VIEW SERVER PERFORMANCE STATE` が必要です。[system-dmvs][system-dmvs]

| オブジェクト | 種別 | 主な用途 | 研修での見方 |
| --- | --- | --- | --- |
| `sys.dm_exec_requests` | DMV | 現在実行中の要求、`wait_type`、`wait_time`、`blocking_session_id` を確認する | 瞬間的なブロッキングの有無や、いま何待ちかを確認する入口として使う。[dm-exec-requests][dm-exec-requests] |
| `sys.dm_os_wait_stats` | DMV | インスタンス全体の待機統計の累積を見る | `LCK%`、`PAGEIOLATCH_%`、`WRITE_LOG` などを差分で比較し、ボトルネックの方向を判断する。[dm-os-wait-stats][dm-os-wait-stats] |
| `sys.dm_os_waiting_tasks` | DMV | 現在待機中のタスク行列を見る | `sys.dm_exec_requests` だけでは見えにくい現在進行形の待機を掘るときに補助的に使う。[dm-os-waiting-tasks][dm-os-waiting-tasks] |
| `sys.dm_os_sys_info` | DMV | CPU 数、コミット済みメモリ、目標メモリなどインスタンス基礎情報を見る | `committed_kb` と `committed_target_kb` を比較し、SQL Server がどの程度メモリ余力を持つかを把握する。[dm-os-sys-info][dm-os-sys-info] |
| `sys.dm_os_sys_memory` | DMV | OS 側の物理メモリ状態を見る | OS レベルで空きメモリが逼迫していないかを確認し、SQL Server 以外を含む圧迫の有無を補助的に見る。[dm-os-sys-memory][dm-os-sys-memory] |
| `sys.dm_os_tasks` | DMV | 要求の背後にあるタスク情報を見る | 並列実行時に、1 要求の下で複数タスクがどう動いているかを確認するときに使う。[dm-os-tasks][dm-os-tasks] |
| `sys.dm_io_virtual_file_stats` | DMF | データファイルとログファイルごとの I/O 累積統計を見る | `num_of_reads`、`num_of_writes`、`io_stall_read_ms`、`io_stall_write_ms` を差分で見て、データ I/O とログ I/O を切り分ける。[dm-io-virtual-file-stats][dm-io-virtual-file-stats] |

このカテゴリで特に重要なのは、`sys.dm_exec_requests` が瞬間状態を返し、`sys.dm_os_wait_stats` と `sys.dm_io_virtual_file_stats` が累積値を返す点です。同じ「性能を見る」オブジェクトでも、前者はその場の状況把握、後者は前後差分の比較に向きます。ここを混同すると、数値の増減を誤読しやすくなります。

## 4. データベーススコープの主要 DMV

RCSI の副作用を追うには、サーバー全体の待機だけでは足りません。行バージョンの保持や tempdb 消費を確認するには、データベーススコープの DMV が必要です。特に tempdb や version store を見る指標は、RCSI 導入前後の比較で中心になります。[db-related-dmvs][db-related-dmvs] [dm-db-file-space-usage][dm-db-file-space-usage]

| オブジェクト | 種別 | 主な用途 | 研修での見方 |
| --- | --- | --- | --- |
| `sys.dm_db_file_space_usage` | DMV | tempdb のファイル使用量、version store 使用量、内部オブジェクト使用量を確認する | `version_store_reserved_page_count` を中心に見て、RCSI の副作用が tempdb に出ていないかを判断する。[dm-db-file-space-usage][dm-db-file-space-usage] |
| `sys.dm_db_session_space_usage` | DMV | セッション単位の tempdb 利用量を見る | どの接続が tempdb を多く使っているかを調べるときの補助として使う。[dm-db-session-space-usage][dm-db-session-space-usage] |
| `sys.dm_db_task_space_usage` | DMV | タスク単位の tempdb 利用量を見る | 並列処理や内部作業がどのタスクで tempdb を消費しているかを掘るときに使う。[dm-db-task-space-usage][dm-db-task-space-usage] |
| `sys.dm_db_log_space_usage` | DMV | ログ領域使用量を見る | 更新負荷が高い検証で、ログ逼迫やログ書き込み側の影響を補助的に確認する。[dm-db-log-space-usage][dm-db-log-space-usage] |

このカテゴリは、RCSI の効果確認よりも「副作用確認」に寄っています。ロック待機が減っても tempdb やログが苦しくなれば、システム全体としては別の場所に負荷が移っただけです。そのため、ロック系の指標だけで結論を出さず、tempdb とログも必ず見る必要があります。

## 5. Query Store の関連ビュー

Query Store は、現在の瞬間状態を見るための仕組みではなく、一定時間窓で実行履歴と統計を保持するための仕組みです。このため、ここで登場するオブジェクトは DMV ではなくカタログビューですが、性能評価の総括には不可欠です。特に平均応答時間、P95、待機カテゴリ、時間帯別の変化を追いたいときに使います。[query-store-monitor][query-store-monitor] [query-store-collects][query-store-collects]

| オブジェクト | 種別 | 主な用途 | 研修での見方 |
| --- | --- | --- | --- |
| `sys.query_store_runtime_stats` | カタログビュー | 実行回数、平均時間、CPU、論理読み取りなどの集計値を保持する | `avg_duration` と `count_executions` を使い、テスト区間の平均応答を集計する。[query-store-runtime-stats][query-store-runtime-stats] |
| `sys.query_store_runtime_stats_interval` | カタログビュー | 集計区間の開始時刻と終了時刻を管理する | どの時間窓の統計かを切り出すために `runtime_stats_interval_id` と結合する。[query-store-runtime-stats-interval][query-store-runtime-stats-interval] |
| `sys.query_store_wait_stats` | カタログビュー | クエリ単位の待機カテゴリ集計を保持する | インスタンス全体の待機ではなく、どのクエリがどの種類の待機を多く持つかを見る。[query-store-wait-stats][query-store-wait-stats] |
| `sys.database_query_store_options` | カタログビュー | Query Store の設定状態や保持容量を確認する | `actual_state_desc`、`interval_length_minutes`、`current_storage_size_mb` を見て、履歴の信頼性を確認する。[database-query-store-options][database-query-store-options] |

実務では、これらに加えて `sys.query_store_query` や `sys.query_store_plan` を結合し、どの SQL 文とどの実行計画に統計がぶら下がっているかを確認することが多くあります。ただし、RCSI 性能評価の入口としては、まず runtime stats、wait stats、interval、options の四つを押さえるだけでも十分に効果があります。[query-store-monitor][query-store-monitor]

## 6. Azure SQL Managed Instance で補助的に使う関連ビュー

今回の研修資料では CPU 指標として `sys.server_resource_stats` に触れていますが、これは DMV ではありません。ただし、Azure SQL Managed Instance のサービス側資源利用を把握する補助情報としては有用です。オンプレミス SQL Server の OS CPU 使用率と同一視してはいけませんが、サービスティア上限に対する利用率として読むことで、CPU や I/O の全体傾向を確認できます。[mi-server-resource-stats][mi-server-resource-stats]

このビューは更新粒度と報告粒度を混同しやすいため、使い方の詳細は [[ManagedInstance/sys.server_resource_stats 詳細解説]] に分けています。

| オブジェクト | 種別 | 主な用途 | 研修での見方 |
| --- | --- | --- | --- |
| `sys.server_resource_stats` | システムビュー | Azure SQL Managed Instance の CPU、データ I/O、ログ書き込み利用率を見る | `avg_cpu_percent` などを使い、サービス上限に対する消費率を把握する。ただし更新遅延があるため短時間テストの秒単位観測には向かない。[mi-server-resource-stats][mi-server-resource-stats] |

## 7. RCSI 性能評価での使い分け

一覧として名前を覚えるよりも、どの問いに対して何を使うかを覚えた方が実務では強くなります。RCSI 評価の流れに沿って並べると、使い分けは次のようになります。

1. いま本当に止まっている処理があるかを見るときは `sys.dm_exec_requests` と必要に応じて `sys.dm_os_waiting_tasks` を使います。
2. 区間全体で何待ちが増えたかを見るときは `sys.dm_os_wait_stats` を差分で見ます。
3. RCSI の副作用が tempdb に出ていないかを見るときは `sys.dm_db_file_space_usage` を中心に、必要なら session / task 単位の space usage を掘ります。
4. データファイル I/O とログファイル I/O を分けたいときは `sys.dm_io_virtual_file_stats` を差分で見ます。
5. SQL Server 側と OS 側のメモリ感を併せて見たいときは `sys.dm_os_sys_info` と `sys.dm_os_sys_memory` を組み合わせます。
6. テスト後に平均応答時間、P95、待機カテゴリを比較するときは Query Store 関連ビューを使います。

## 8. 利用時の注意点

DMV / DMF / 関連ビューを使うときには、いくつかの前提を外さないことが重要です。第一に、権限です。サーバースコープは `VIEW SERVER STATE` 系、データベーススコープは `VIEW DATABASE STATE` 系の権限が必要です。第二に、累積値と瞬間値を混ぜないことです。`sys.dm_os_wait_stats` や `sys.dm_io_virtual_file_stats` は差分で見るべきですが、`sys.dm_exec_requests` はその場の瞬間値です。第三に、Query Store は履歴集計の仕組みであり、秒単位の生ログではありません。[system-dmvs][system-dmvs] [query-store-collects][query-store-collects]

さらに、Microsoft Learn は DMV / DMF が内部実装に近い情報を返し、将来のリリースで列が追加される可能性があるため、運用コードで `SELECT *` を使わないことを推奨しています。この一覧は分析の入口として使い、実際の収集 SQL では必要列を明示する方が安全です。[system-dmvs][system-dmvs]

## 9. 結論

SQL Server の性能分析でよく使う監視オブジェクトは、見た目が似ていても役割が同じではありません。現在の要求を見る DMV、累積待機を見る DMV、ファイル I/O を返す DMF、履歴を蓄積する Query Store カタログビュー、そして環境依存の補助ビューがあります。RCSI の評価で重要なのは、それらを一つの処理経路の別断面として使い分けることです。

この資料では、研修で実際に使う主要オブジェクトへ絞って整理しました。より広い製品全体の一覧が必要な場合は、Microsoft Learn の総覧ページを起点にカテゴリ別ドキュメントへ進むのが最も安全です。[system-dmvs][system-dmvs] [server-related-dmvs][server-related-dmvs] [db-related-dmvs][db-related-dmvs] [os-related-dmvs][os-related-dmvs] [io-related-dmvs][io-related-dmvs]

[system-dmvs]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/system-dynamic-management-views?view=sql-server-ver17
[server-related-dmvs]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/server-related-dynamic-management-views-and-functions-transact-sql?view=azuresqldb-current
[db-related-dmvs]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/database-related-dynamic-management-views-transact-sql?view=azuresqldb-current
[os-related-dmvs]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sql-server-operating-system-related-dynamic-management-views-transact-sql?view=azuresqldb-current
[io-related-dmvs]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/i-o-related-dynamic-management-views-and-functions-transact-sql?view=azuresqldb-current
[dm-exec-requests]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-exec-requests-transact-sql?view=sql-server-ver17
[dm-os-wait-stats]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-os-wait-stats-transact-sql?view=sql-server-ver17
[dm-os-waiting-tasks]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-os-waiting-tasks-transact-sql?view=sql-server-ver17
[dm-os-sys-info]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-os-sys-info-transact-sql?view=sql-server-ver17
[dm-os-sys-memory]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-os-sys-memory-transact-sql?view=sql-server-ver17
[dm-os-tasks]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-os-tasks-transact-sql?view=sql-server-ver17
[dm-io-virtual-file-stats]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-io-virtual-file-stats-transact-sql?view=sql-server-ver17
[dm-db-file-space-usage]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-db-file-space-usage-transact-sql?view=sql-server-ver17
[dm-db-session-space-usage]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-db-session-space-usage-transact-sql?view=sql-server-ver17
[dm-db-task-space-usage]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-db-task-space-usage-transact-sql?view=sql-server-ver17
[dm-db-log-space-usage]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-db-log-space-usage-transact-sql?view=sql-server-ver17
[query-store-monitor]: https://learn.microsoft.com/sql/relational-databases/performance/monitoring-performance-by-using-the-query-store?view=sql-server-ver17
[query-store-collects]: https://learn.microsoft.com/sql/relational-databases/performance/how-query-store-collects-data?view=sql-server-ver17
[query-store-runtime-stats]: https://learn.microsoft.com/sql/relational-databases/system-catalog-views/sys-query-store-runtime-stats-transact-sql?view=sql-server-ver17
[query-store-runtime-stats-interval]: https://learn.microsoft.com/sql/relational-databases/system-catalog-views/sys-query-store-runtime-stats-interval-transact-sql?view=sql-server-ver17
[query-store-wait-stats]: https://learn.microsoft.com/sql/relational-databases/system-catalog-views/sys-query-store-wait-stats-transact-sql?view=sql-server-ver17
[database-query-store-options]: https://learn.microsoft.com/sql/relational-databases/system-catalog-views/sys-database-query-store-options-transact-sql?view=sql-server-ver17
[mi-server-resource-stats]: https://learn.microsoft.com/azure/azure-sql/managed-instance/monitoring-sql-managed-instance-azure-performance-reference-tables?view=azuresql#sysserver_resource_stats
