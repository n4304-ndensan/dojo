# Azure SQL Managed Instance における RCSI 評価の前提整理

> 位置づけ: この文書は、Azure SQL Managed Instance で RCSI を評価するときに先に固定しておく前提を整理する導入資料です。個別の調査記録や一時的な修正履歴ではなく、Managed Instance で誤読しやすい論点を先回りで整理することを目的にしています。
>
> 読み分け: 評価スクリプトの設計原則と参照スクリプトは [[Azure SQL Managed Instance における RCSI 評価スクリプト設計ガイド]]、`sys.server_resource_stats` の詳細は [[sys.server_resource_stats 詳細解説]]、分離レベルの総合解説は [[SQL Server の Isolation Level（分離レベル）詳細解説]]、RCSI と SNAPSHOT の比較は [[SQL Server Snapshot Isolation と RCSI と Read Committed の違い]]、フォルダ全体の案内は [[ManagedInstance ドキュメントガイド]] と [[SQL Server ドキュメントガイド]] を参照してください。

## 1. 文脈と目的

RCSI の評価では、lock wait、blocking、tempdb の version store、CPU を見ようとすることが多くあります。方向としては自然ですが、Azure SQL Managed Instance を前提にすると、同じ列名や DMV を見ていても意味が少し変わります。特に、ADR 常時有効、PVS の存在、サービスメーターとしての `sys.server_resource_stats` の更新遅延は、値の読み方をぶらしやすいポイントです。[locking-row-versioning][pvs-stats][server-resource-stats][mi-monitoring-dmvs]

この文書の役割は、個別のスクリプトを添削することではありません。Managed Instance で RCSI 評価を設計したり結果を読んだりするときに、先に揃えておくべき前提をまとめ、どの指標を主に見て、どの指標を補助として扱うべきかを整理します。

## 2. Managed Instance で先に固定すべき前提

Managed Instance では、少なくとも次の四点を先に押さえておく必要があります。ここを曖昧にしたままスクリプトを書き始めると、構文は正しくても意味の取り方でずれやすくなります。

### 2.1 ADR が常に有効で、version store の見方が変わる

Managed Instance では ADR が常時有効です。そのため、RCSI の副作用を tempdb だけで捉える考え方は不十分になりやすく、PVS を中心に見た方が実態に近くなります。`sys.dm_tran_persistent_version_store_stats` を見ずに tempdb だけで結論を出すと、versioning コストの主戦場を取り違える可能性があります。[locking-row-versioning][pvs-stats]

### 2.2 `sys.server_resource_stats` は即時メーターではない

`sys.server_resource_stats` は Managed Instance で有効な重要ビューですが、15 秒区間のデータを 5 分から 10 分程度の遅延を伴って更新するサービスメーターです。そのため、10 秒ループの即時 CPU 比較には向きません。短い試験で値が動かなくても、負荷がなかったとは限らず、単にまだ反映されていないだけということが起こります。[server-resource-stats][mi-monitoring-dmvs]

### 2.3 RCSI と `SNAPSHOT` は別の評価対象である

RCSI は `READ COMMITTED` の読み取り実装を行バージョン方式へ変える仕組みであり、`SNAPSHOT` は独立した分離レベルです。RCSI の評価をしたいときに `SET TRANSACTION ISOLATION LEVEL SNAPSHOT` を混ぜると、何を比較しているのかが崩れます。[set-transaction-isolation][locking-row-versioning]

### 2.4 インスタンス全体の値と対象 DB の値を分けて読む必要がある

`sys.dm_os_wait_stats` や `sys.server_resource_stats` はインスタンス全体の色が強く、`sys.dm_exec_requests` や `sys.dm_db_log_space_usage` は使い方次第で対象 DB に寄せられます。Managed Instance の評価では、「この値は対象 DB 固有なのか、インスタンス全体なのか」を列名やコメントで明示しておく方が安全です。[dm-os-wait-stats][dm-exec-requests][dm-db-log-space-usage]

## 3. 評価で最初に置くべき主指標

RCSI の効果は、大きく分けると「reader-writer ブロッキング低減」と「行バージョン管理コスト増加」です。Managed Instance で評価を安定させるなら、この二系統を主指標にします。

第一に、ブロッキングの改善です。`sys.dm_exec_requests` の `blocking_session_id` を対象 DB で絞って見ると、RCSI が直接狙っている reader-writer 競合の減少を追いやすくなります。`sys.dm_os_wait_stats` の `LCK%` 差分も方向性確認には有効ですが、こちらはインスタンス全体の累積である点を外さない方が安全です。[dm-exec-requests][dm-os-wait-stats]

第二に、versioning コストです。Managed Instance では `sys.dm_tran_persistent_version_store_stats` を主に見て、必要に応じて `sys.dm_tran_version_store_space_usage` で tempdb 側の version store も補助的に見ます。さらに `sys.dm_tran_active_snapshot_database_transactions` で長時間トランザクションの滞留を見て、`sys.dm_db_log_space_usage` でログ使用率を追うと、副作用の読み筋が安定します。[pvs-stats][dm-tran-version-store-space-usage][active-snapshot-dmv][dm-db-log-space-usage]

## 4. 補助指標の位置づけ

Managed Instance 向けの評価で、補助指標として有用でも主指標ではない値もあります。`tempdb.sys.dm_db_file_space_usage` の使用率、`sys.dm_io_virtual_file_stats` の stall 差分、`sys.dm_os_sys_info` と `sys.dm_os_sys_memory` のメモリ比率は、その典型です。これらは周辺症状や別ボトルネックの兆候を見るのには使えますが、RCSI の効き目を直接示すものではありません。[dm-db-file-space-usage][dm-io-virtual-file-stats][dm-os-sys-info][dm-os-sys-memory]

そのため、補助指標は「RCSI が効かなかった理由を探る」「副作用が別の資源へ波及していないかを見る」ために置くのが適切です。主指標より先に補助指標へ飛び込むと、tempdb や CPU の揺れを RCSI そのものの成否と誤読しやすくなります。

## 5. 次に読むべき文書

Managed Instance 向けの前提が見えたら、次は評価スクリプトの設計原則へ進むのが自然です。どの観測項目をどういう責務で置くべきか、参照スクリプトをどのように読めばよいかは [[Azure SQL Managed Instance における RCSI 評価スクリプト設計ガイド]] にまとめています。

あわせて、CPU 指標の読み方をより正確にしたいときは [[sys.server_resource_stats 詳細解説]] を参照してください。`avg_cpu_percent` の意味、更新遅延、I/O バイト数との違いを切り分けておくと、短時間試験の読み違いが減ります。

## 6. まとめ

Managed Instance で RCSI 評価をするときに重要なのは、SQL Server 全般の知識に加えて、「Managed Instance では何が別ルールになるか」を先に揃えることです。ADR 常時有効、PVS、`sys.server_resource_stats` の更新遅延、インスタンス全体メーターと対象 DB 指標の混在。この四点を先に押さえるだけで、評価スクリプトの設計と読み方はかなり安定します。

この文書は入口として役割を絞っています。具体的な評価設計と参照スクリプトは、次の文書で詳細に確認してください。

## 参考

- [SET TRANSACTION ISOLATION LEVEL][set-transaction-isolation]
- [Transaction locking and row versioning guide][locking-row-versioning]
- [sys.dm_tran_persistent_version_store_stats][pvs-stats]
- [sys.dm_tran_version_store_space_usage][dm-tran-version-store-space-usage]
- [sys.dm_tran_active_snapshot_database_transactions][active-snapshot-dmv]
- [sys.dm_os_wait_stats][dm-os-wait-stats]
- [sys.dm_exec_requests][dm-exec-requests]
- [sys.dm_db_log_space_usage][dm-db-log-space-usage]
- [sys.server_resource_stats (Azure SQL Managed Instance)][server-resource-stats]
- [Monitoring Azure SQL Managed Instance performance using dynamic management views][mi-monitoring-dmvs]

[set-transaction-isolation]: https://learn.microsoft.com/sql/t-sql/statements/set-transaction-isolation-level-transact-sql?view=sql-server-ver17
[locking-row-versioning]: https://learn.microsoft.com/sql/relational-databases/sql-server-transaction-locking-and-row-versioning-guide?view=sql-server-ver17
[pvs-stats]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-tran-persistent-version-store-stats?view=sql-server-ver17
[dm-tran-version-store-space-usage]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-tran-version-store-space-usage?view=sql-server-ver17
[active-snapshot-dmv]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-tran-active-snapshot-database-transactions-transact-sql?view=sql-server-ver17
[dm-os-wait-stats]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-os-wait-stats-transact-sql?view=sql-server-ver17
[dm-exec-requests]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-exec-requests-transact-sql?view=sql-server-ver17
[dm-db-log-space-usage]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-db-log-space-usage-transact-sql?view=sql-server-ver17
[dm-io-virtual-file-stats]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-io-virtual-file-stats-transact-sql?view=sql-server-ver17
[dm-os-sys-memory]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-os-sys-memory-transact-sql?view=sql-server-ver17
[server-resource-stats]: https://learn.microsoft.com/sql/relational-databases/system-catalog-views/sys-server-resource-stats-azure-sql-database?view=azuresqldb-current
[mi-monitoring-dmvs]: https://learn.microsoft.com/azure/azure-sql/managed-instance/monitoring-with-dmvs?view=azuresql
