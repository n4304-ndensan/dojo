# SQL Server の Lock と Blocking と Version Store の整理

> 位置づけ: この文書は、RCSI や SNAPSHOT、ADR、SQL Server 2022 の Ledger を調べるときに混同しやすい lock、blocking、version store の責務を切り分けるための補助資料です。主教材の再掲ではなく、誤解しやすい差分と観測ポイントの整理に責務を絞ります。
>
> 読み分け: SQL Server と RCSI をゼロから理解したい場合は [[SQL Server 入門から始める RCSI 性能評価研修]]、分離レベルの比較は [[SQL Server Snapshot Isolation と RCSI と Read Committed の違い]]、フォルダ全体の入口は [[SQL Server ドキュメントガイド]] を参照してください。

## 1. 文脈と目的

SQL Server の並行実行制御を追っていると、lock、blocking、wait、version store が一つの塊に見えやすくなります。特に RCSI や SNAPSHOT を調べていると、「lock が減った」「blocking が減った」「version store が増えた」という観測が同時に出てくるため、仕組みと症状と格納先が混ざりやすくなります。

さらに SQL Server 2022 では、ADR の PVS、Ledger の履歴テーブル、In-Memory OLTP の内蔵 MVCC など、古い行や履歴が置かれる場所が一つではありません。そのため、「version が見えない」「tempdb が増えない」「PVS が増えない」という現象を見たとき、単純に RCSI の有無だけでは説明しきれないことがあります。

この資料の目的は、次の三つを自力で切り分けられるようにすることです。

- lock は何で、blocking は何か
- version store は何のためにあり、どこに置かれるのか
- version が見えないときに、tempdb、PVS、履歴テーブル、メモリ最適化テーブルのどこを疑うべきか

## 2. まず結論

最初に結論だけまとめると、lock は仕組み、blocking は現象、version store は保持領域です。lock は同時更新の整合性を守るための制御機構であり、blocking はその lock や他資源の競合によってセッションが待たされている観測結果です。version store は、RCSI や SNAPSHOT のような行バージョンベース機能が過去の行イメージを参照できるようにするための保存先です。[locking-row-versioning] [dm-exec-requests]

重要なのは、これら三者は同じものではないという点です。lock が存在しても blocking が起きないことはありますし、blocking が起きても原因が lock とは限りません。また、version store の増減は行バージョン保持のコストを示しますが、それ自体が blocking の有無を直接意味するわけではありません。

SQL Server 2022 で version の置き場所を整理すると、概ね次のようになります。

```text
古い行や履歴の主な置き場所
├── ディスクベース表 + ADR OFF
│   └── tempdb の common version store
├── ディスクベース表 + ADR ON
│   └── ユーザーデータベース内の PVS
├── Updatable Ledger Table
│   └── 履歴テーブル（History Table）
└── Memory-Optimized Table
    └── メモリ最適化データ構造に内蔵された行バージョン
```

したがって、「version store が見えない」という話をするときは、まず何を version store と呼んでいるのかを明確にする必要があります。RCSI/SNAPSHOT の行バージョンを見たいのか、Ledger の履歴行を見たいのか、In-Memory OLTP の行バージョンを見たいのかで、観測先が変わります。[locking-row-versioning] [ledger-updatable] [memory-optimized-transactions]

## 3. Lock と Blocking と Wait の違い

この章では、仕組みと観測結果を分離します。ここが曖昧なままだと、RCSI の効果測定でも「lock が減ったのか」「blocking が減ったのか」「待機時間が減ったのか」が混ざります。

lock は、Database Engine が整合性を守るために取得する制御情報です。共有ロック、更新ロック、排他ロック、スキーマロックなどがあり、同じデータへ矛盾したアクセスが起きないようにします。lock は取得されていても、競合相手がいなければそのまま処理は進むので、lock があること自体は異常ではありません。[locking-row-versioning]

blocking は、ある要求が他のセッションの保持する lock や資源のために進めない状態です。つまり blocking は「待たされている」という観測結果であり、lock そのものではありません。`sys.dm_exec_requests.blocking_session_id` は、今まさに別セッションに止められている要求を見つける入口になります。[dm-exec-requests]

wait はさらに広い概念です。SQL Server のワーカーは、lock 以外にも I/O、latch、memory、network などさまざまな資源を待ちます。そのため、すべての blocking は wait の一種ですが、すべての wait が blocking ではありません。たとえばディスク I/O 待機は wait ですが、通常は blocking_session_id では表れません。

実務上は、次のように読むと混乱しにくくなります。

- lock: 整合性のための制御機構
- blocking: 他セッションに止められている瞬間状態
- wait: SQL Server が何かの資源を待っている全体像

この切り分けを持っておくと、「RCSI の狙いは読取系の shared lock 由来 blocking を減らすことであり、すべての wait を消すことではない」と説明しやすくなります。[locking-row-versioning]

## 4. Version Store とは何か

RCSI や SNAPSHOT は、更新前の行イメージをどこかに残しておくことで、読取側が一貫した過去の状態を参照できるようにします。この「更新前の行イメージを保持する枠組み」が SQL Server の row versioning であり、その保存先が version store です。[locking-row-versioning]

ここで重要なのは、row versioning が RCSI と SNAPSHOT だけのためにあるわけではないことです。Microsoft Learn の row versioning ガイドでは、trigger、MARS、online index operation でも同じ row versioning framework が使われると説明されています。つまり、version store の観測値が増えているとき、それは RCSI だけの副作用とは限りません。[locking-row-versioning]

もう一つ重要なのは、「アクティブな row versioning 読取が存在しないなら、更新前の行は version store に書かれない」という理解は、少なくとも一次資料と整合しないという点です。公式ガイドは、row versioning-based isolation level を有効にすると、そのデータベースのすべてのデータ変更は versioned され、row versioning を使うアクティブトランザクションがなくても更新前コピーは version store に保存されると説明しています。[locking-row-versioning]

ただし、保持期間は別問題です。row version は、必要とする最古のアクティブトランザクションが参照し終わるまで残ります。長時間トランザクションがいると cleanup が進まず、tempdb や PVS が膨らみやすくなります。逆に短時間トランザクションだけなら、生成されてもすぐ掃除され、瞬間観測では目立たないことがあります。[locking-row-versioning]

## 5. SQL Server 2022 で version はどこに格納されるか

SQL Server 2022 を含む現行 SQL Server では、ディスクベース表の row version が常に tempdb に置かれるわけではありません。Microsoft Learn の row versioning ガイドは、ADR が有効なデータベースでは version store はそのデータベース内に作られ、ADR が無効な場合は tempdb に作られると説明しています。[locking-row-versioning]

この差は観測方法に直結します。`sys.dm_tran_version_store_space_usage` や `sys.dm_tran_version_store` など tempdb 側を前提にした DMV は、PVS 側の増加を直接表しません。逆に ADR が有効な環境では、`sys.dm_tran_persistent_version_store_stats` を見ないと、row versioning コストの本体を取り逃がしやすくなります。[locking-row-versioning] [pvs-stats] [dm-tran-version-store-space-usage]

さらに SQL Server 2022 では Ledger が入るため、履歴の保持先がもう一段増えます。updatable ledger table では、更新や削除のたびに以前の行バージョンが自動的に history table へ保存されます。これは tempdb の version store や ADR の PVS とは別の、ユーザーデータベース内の通常テーブルです。したがって、Ledger の履歴が増えていても、tempdb や PVS のサイズだけを見ていると見落とします。[ledger-updatable] [ledger-overview]

In-Memory OLTP も別系統です。memory-optimized table は独自の楽観的同時実行制御を持ち、row versioning がデータ構造に内蔵されています。公式ドキュメントは、disk-based table の row versioning が tempdb ベースであるのに対し、memory-optimized data structure は row versioning built in だと説明しています。つまり、memory-optimized table の version は tempdb/PVS の観測だけでは捉えられません。[memory-optimized-transactions] [memory-optimized-intro]

## 6. 提示された仮説の読み替え

ここでは、調査でよく出る説明を「何を説明できるのか」という観点で整理します。特に、ADR の有効無効、ロックによる待機、クリーニング効率、Ledger、In-Memory OLTP、互換性レベル 160 を同じ土俵に置かないことが重要です。

まず ADR の有効無効は、version の格納先を直接変える一次要因です。ADR が OFF なら tempdb、ADR が ON なら PVS です。したがって、tempdb だけを見て version がないと判断する前に、`sys.databases.is_accelerated_database_recovery_on` を確認する必要があります。[locking-row-versioning] [sys-databases]

次に lock による待機、つまり blocking は、version の格納先ではなく観測される症状の話です。RCSI は shared lock 由来の読取 blocking を減らしやすいですが、更新同士の競合、schema lock、他資源待機は残ります。したがって、blocking が減らないことはあり得ますが、それだけで version store の場所を説明することはできません。[locking-row-versioning] [dm-exec-requests]

クリーニングの効率化も同様です。cleanup の速さは、version がどこへ作られるかではなく、どれだけ長く残るかに効きます。長時間トランザクションが残れば tempdb でも PVS でも cleanup は遅れますし、短時間で終わるトランザクションなら生成されてもすぐ減ります。これは格納先の問題ではなく保持期間の問題です。[locking-row-versioning]

ここまでの三つで説明できない場合に、初めて「そもそも見ている保管先が違うのではないか」を強く疑うべきです。その代表が Ledger と In-Memory OLTP です。Ledger は history table、memory-optimized table は内蔵 row versioning なので、tempdb/PVS を見ても期待どおりに増えないことがあります。[ledger-updatable] [memory-optimized-transactions]

一方で、互換性レベル 160 は注意して扱うべき論点です。SQL Server 2022 の `COMPATIBILITY_LEVEL = 160` は既定値ですが、公式の compatibility level ドキュメントが列挙している差分は Parameter Sensitive Plan optimization や CE Feedback など query processing の挙動です。少なくとも Microsoft Learn の一次説明では、compatibility level 160 が row version の格納先を変えるとは書かれていません。したがって、compatibility level 160 は環境情報として記録すべきですが、version store が見えない理由の第一候補に置くのは根拠が弱いです。[compat-level]

## 7. まず確認するクエリ

観測を誤らないためには、データベース設定、テーブル特性、観測先の三段に分けて確認するのが安全です。この章では、SQL Server 2022 で最初に打つべき確認クエリをまとめます。

### 7.1 データベース設定を確認する

まずは対象 DB が RCSI、SNAPSHOT、ADR、互換性レベル 160 のどれを使っているかを確認します。

```sql
SELECT
    name,
    compatibility_level,
    is_read_committed_snapshot_on,
    snapshot_isolation_state_desc,
    is_accelerated_database_recovery_on,
    is_memory_optimized_elevate_to_snapshot_on
FROM sys.databases
WHERE name = DB_NAME();
```

この結果で見たいのは三点です。`is_read_committed_snapshot_on = 1` なら既定の `READ COMMITTED` が row versioning ベースです。`is_accelerated_database_recovery_on = 1` なら PVS を観測対象に含める必要があります。`compatibility_level = 160` は SQL Server 2022 の既定ですが、これ自体を格納先の説明に使わないことも重要です。[sys-databases] [compat-level]

### 7.2 テーブルが Ledger や Memory-Optimized ではないかを確認する

次に、対象テーブル自体の性質を確認します。ここで updatable ledger table か memory-optimized table かが分かると、見るべき保管先が変わります。

```sql
SELECT
    s.name AS schema_name,
    t.name AS table_name,
    t.is_memory_optimized,
    t.durability_desc,
    t.temporal_type_desc,
    t.ledger_type_desc,
    hs.name AS history_schema_name,
    ht.name AS history_table_name
FROM sys.tables AS t
JOIN sys.schemas AS s
  ON s.schema_id = t.schema_id
LEFT JOIN sys.tables AS ht
  ON ht.object_id = t.history_table_id
LEFT JOIN sys.schemas AS hs
  ON hs.schema_id = ht.schema_id
WHERE t.name = N'YourTableName';
```

`ledger_type_desc = UPDATABLE_LEDGER_TABLE` なら履歴は history table に流れます。`is_memory_optimized = 1` なら In-Memory OLTP の内蔵 row versioning を前提に読むべきです。`history_table_name` が見えていれば、履歴を tempdb ではなくその物理テーブルで追うべきだと判断できます。[sys-tables] [ledger-updatable]

### 7.3 blocking と wait を確認する

lock と blocking を混同しないために、いま誰が止められているかを見ます。

```sql
SELECT
    session_id,
    status,
    command,
    wait_type,
    wait_time,
    blocking_session_id,
    database_id
FROM sys.dm_exec_requests
WHERE blocking_session_id <> 0
   OR wait_type LIKE N'LCK%';
```

このクエリは「いま誰が止められているか」を見るものであり、version の格納先を示すものではありません。blocking が出ていなくても row versioning は動いていることがありますし、逆に blocking が出ていても原因は row versioning ではなく更新競合や schema lock かもしれません。[dm-exec-requests]

### 7.4 tempdb と PVS のどちらを見るべきかを確認する

ADR が OFF なら tempdb 側、ADR が ON なら PVS 側を見るのが基本です。両方を並べると観測漏れを減らせます。

```sql
SELECT
    DB_NAME(database_id) AS database_name,
    reserved_space_kb / 1024.0 AS tempdb_version_store_mb
FROM sys.dm_tran_version_store_space_usage
WHERE database_id = DB_ID();

SELECT
    DB_NAME(database_id) AS database_name,
    persistent_version_store_size_kb / 1024.0 AS pvs_size_mb,
    current_aborted_transaction_count,
    oldest_active_transaction_id
FROM sys.dm_tran_persistent_version_store_stats
WHERE database_id = DB_ID();

SELECT
    SUM(version_store_reserved_page_count) * 8.0 / 1024.0 AS tempdb_file_level_version_store_mb
FROM tempdb.sys.dm_db_file_space_usage;
```

最初のクエリは tempdb 側を DB 単位で見ます。二つ目は ADR 有効時の PVS を見ます。三つ目は tempdb 全体のファイルレベル消費です。SQL Server 2022 で versioning を追うときは、tempdb だけか PVS だけかに決め打ちしない方が安全です。[dm-tran-version-store-space-usage] [pvs-stats] [dm-db-file-space-usage]

## 8. SQL Server ドキュメントの読み分け

このフォルダ内の SQL Server 文書は、それぞれ責務が違います。lock、blocking、version store を整理したいときは、この文書を起点にしつつ、次の読み順にすると重複なく理解しやすくなります。

- 全体像をゼロからつかむ: [[SQL Server 入門から始める RCSI 性能評価研修]]
- Lock と Blocking と Version Store の違いを素早く整理する: この文書
- RCSI と SNAPSHOT と READ COMMITTED の設計差を比較する: [[SQL Server Snapshot Isolation と RCSI と Read Committed の違い]]
- Isolation Level 全般を体系化する: [[SQL Server の Isolation Level（分離レベル）詳細解説]]
- DMV の用途を一覧で引く: [[SQL Server DMV一覧]]
- `tempdb.sys.dm_db_file_space_usage` の読み方を個別に確認する: [[../Diagnostics/DMV/sys.dm_db_file_space_usage]]

公式ドキュメントも、見るべきページを分けると迷いにくくなります。

- row versioning と RCSI/SNAPSHOT の原則: Microsoft Learn の row versioning guide
- Ledger の履歴テーブルと ledger view: Microsoft Learn の updatable ledger table / ledger overview
- In-Memory OLTP の row versioning: Microsoft Learn の transactions with memory-optimized tables
- 互換性レベル 160 の意味: Microsoft Learn の compatibility level

## 9. まとめ

lock は整合性のための仕組みであり、blocking はその競合で要求が止まる現象です。version store は、RCSI や SNAPSHOT などが過去の行を読むための保持領域であり、lock や blocking とは別の責務です。

SQL Server 2022 で「version が見えない」ときは、まず tempdb と PVS のどちらを見ているかを確認し、その次に Ledger の history table と In-Memory OLTP の内蔵 row versioning を疑うべきです。compatibility level 160 は環境情報として重要ですが、少なくとも一次資料の範囲では version の格納先を説明する主因ではありません。

また、RCSI の statement-level という性質は、読取の一貫性の粒度を説明するものです。公式資料が明示しているとおり、row versioning-based isolation を有効にするとデータ変更は versioned されるため、「読取がいないから version store に書かれない」という説明は採りにくいです。観測値が動かない場合は、むしろ見ている保管先と観測間隔を疑う方が筋が通ります。[locking-row-versioning]

## 参考

- [Transaction locking and row versioning guide][locking-row-versioning]
- [Updatable ledger tables][ledger-updatable]
- [Ledger overview][ledger-overview]
- [Transactions with memory-optimized tables][memory-optimized-transactions]
- [Introduction to memory-optimized tables][memory-optimized-intro]
- [ALTER DATABASE compatibility level][compat-level]
- [sys.databases][sys-databases]
- [sys.tables][sys-tables]
- [sys.dm_exec_requests][dm-exec-requests]
- [sys.dm_tran_version_store_space_usage][dm-tran-version-store-space-usage]
- [sys.dm_tran_persistent_version_store_stats][pvs-stats]
- [sys.dm_db_file_space_usage][dm-db-file-space-usage]

[locking-row-versioning]: https://learn.microsoft.com/sql/relational-databases/sql-server-transaction-locking-and-row-versioning-guide?view=sql-server-ver17
[ledger-updatable]: https://learn.microsoft.com/sql/relational-databases/security/ledger/ledger-updatable-ledger-tables?view=sql-server-ver17
[ledger-overview]: https://learn.microsoft.com/sql/relational-databases/security/ledger/ledger-overview?view=sql-server-ver17
[memory-optimized-transactions]: https://learn.microsoft.com/sql/relational-databases/in-memory-oltp/transactions-with-memory-optimized-tables?view=sql-server-ver17
[memory-optimized-intro]: https://learn.microsoft.com/sql/relational-databases/in-memory-oltp/introduction-to-memory-optimized-tables?view=sql-server-ver17
[compat-level]: https://learn.microsoft.com/sql/t-sql/statements/alter-database-transact-sql-compatibility-level?view=sql-server-ver17
[sys-databases]: https://learn.microsoft.com/sql/relational-databases/system-catalog-views/sys-databases-transact-sql?view=sql-server-ver17
[sys-tables]: https://learn.microsoft.com/sql/relational-databases/system-catalog-views/sys-tables-transact-sql?view=sql-server-ver17
[dm-exec-requests]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-exec-requests-transact-sql?view=sql-server-ver17
[dm-tran-version-store-space-usage]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-tran-version-store-space-usage?view=sql-server-ver17
[pvs-stats]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-tran-persistent-version-store-stats?view=sql-server-ver17
[dm-db-file-space-usage]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-db-file-space-usage-transact-sql?view=sql-server-ver17
