# Azure SQL Managed Instance における RCSI 評価スクリプト設計ガイド

> 位置づけ: この文書は、Azure SQL Managed Instance で RCSI を評価するときに、どの観測項目をどういう責務で置くべきかを整理する設計ガイドです。個別の調査で直した点を時系列に並べるのではなく、誤解しやすい論点と推奨される観測設計を、再利用可能な形でまとめています。
>
> 読み分け: Managed Instance 前提の入口説明は [[Azure SQL Managed Instance における RCSI 評価の前提整理]]、`sys.server_resource_stats` 単体の詳説は [[sys.server_resource_stats 詳細解説]]、分離レベルの全体像は [[SQL Server の Isolation Level（分離レベル）詳細解説]] を参照してください。

## 1. 文脈と目的

RCSI の評価で本当に答えたい問いは、「reader-writer ブロッキングが減ったか」と「その代わりにどのコストが増えたか」です。Managed Instance では、これを lock wait や tempdb だけで判断すると、ADR、PVS、サービスメーターの更新遅延といった前提を見落としやすくなります。[locking-row-versioning][set-transaction-isolation][pvs-stats][server-resource-stats]

そのため、評価スクリプトは単に取れる値を並べるのではなく、何を効果指標として置き、何を副作用指標として置き、何を参考値として扱うかを先に決めて設計した方がぶれません。この文書では、その考え方を先に整理したうえで、参照スクリプトを示します。

## 2. 評価で答えるべき問い

RCSI 評価の中心は、次の三つの問いです。

1. 対象 DB で reader-writer ブロッキングは減ったか。
2. 行バージョン管理のコストはどこに現れ、どの程度まで増えたか。
3. ログ、I/O、メモリ、サービス資源に副作用が波及していないか。

この三つを区別しておくと、blocking の改善と tempdb の増加を同じ列の重みで読んでしまうような混乱を避けやすくなります。

## 3. 設計原則

### 3.1 RCSI 評価に `SNAPSHOT` を混ぜない

RCSI は `READ COMMITTED` の実装を行バージョン方式へ切り替える仕組みであり、`SNAPSHOT` は独立した分離レベルです。RCSI 評価スクリプトでは `SET TRANSACTION ISOLATION LEVEL SNAPSHOT` を混ぜず、何を比較しているかを崩さない方が安全です。[set-transaction-isolation][locking-row-versioning]

### 3.2 対象 DB の値とインスタンス全体の値を分けて命名する

`sys.dm_exec_requests` の blocking 件数は対象 DB に寄せられますが、`sys.dm_os_wait_stats` や `sys.server_resource_stats` はインスタンス全体の値です。同じ表に入れるなら、列名やコメントでスコープを明示した方が誤読を防ぎやすくなります。[dm-exec-requests][dm-os-wait-stats][server-resource-stats]

### 3.3 効果指標と副作用指標を分ける

RCSI の効果を直接表すのは、対象 DB に絞った blocking 件数や lock wait の減少です。一方で、PVS、tempdb、ログ、I/O、メモリは副作用や周辺影響を見る指標です。両者を同じ重みで読むのではなく、まず効果を見て、そのうえで副作用が許容範囲かを判断する構図にしておくと解釈が安定します。

### 3.4 瞬間値、累積値、遅延値を混ぜて解釈しない

`sys.dm_exec_requests` はその場の状態、`sys.dm_os_wait_stats` と `sys.dm_io_virtual_file_stats` は累積値、`sys.server_resource_stats` は遅延を伴うサービスメーターです。採取元の性質が違う以上、比較方法もそろえる必要があります。累積値は差分で、遅延値は長めの区間比較で読む方が安全です。[dm-os-wait-stats][dm-io-virtual-file-stats][server-resource-stats]

### 3.5 Managed Instance では PVS を主指標に置く

Managed Instance では ADR が常時有効であるため、行バージョン管理のコストは tempdb だけでは捉えきれません。`sys.dm_tran_persistent_version_store_stats` を主に見て、`sys.dm_tran_version_store_space_usage` は補助として使う方が、Managed Instance の実態に近い読み方になります。[pvs-stats][dm-tran-version-store-space-usage]

## 4. 観測項目の置き方

### 4.1 RCSI 状態確認

RCSI の ON/OFF は `sys.databases.is_read_committed_snapshot_on` を一次情報として確認します。`DBCC USEROPTIONS` は接続状態を補助的に見るのには使えますが、RCSI 状態の主確認には向きません。[dbcc-useroptions][alter-database-set-options]

### 4.2 効果側の中心指標

最も直接的なのは、対象 DB に絞った `blocking_session_id > 0` の件数です。これに加えて、`sys.dm_os_wait_stats` の `LCK%` 差分を `instance_lock_wait_ms` として置けば、インスタンス全体で lock wait が増減した方向も補助的に見られます。[dm-exec-requests][dm-os-wait-stats]

### 4.3 versioning コストの中心指標

Managed Instance では `sys.dm_tran_persistent_version_store_stats` を主に見て、`pvs_size_mb`、`current_aborted_transaction_count`、`oldest_active_transaction_id` を取るのが基本です。これに加えて `sys.dm_tran_active_snapshot_database_transactions` で長時間トランザクションの滞留を見て、`snapshot_elapsed_max_sec` を押さえると cleanup 停滞を読みやすくなります。[pvs-stats][active-snapshot-dmv]

### 4.4 補助的に置く指標

`sys.dm_tran_version_store_space_usage` による tempdb 側の version store、`tempdb.sys.dm_db_file_space_usage` の使用率、`sys.dm_io_virtual_file_stats` の stall 差分、`sys.dm_os_sys_info` と `sys.dm_os_sys_memory` のメモリ比率は、周辺症状を見る補助指標として有効です。ただし、これらだけで RCSI の成否を判断しない方が安全です。[dm-tran-version-store-space-usage][dm-db-file-space-usage][dm-io-virtual-file-stats][dm-os-sys-info][dm-os-sys-memory]

### 4.5 参考値として置く指標

`master.sys.server_resource_stats` の `avg_cpu_percent` は、短周期の即時 CPU ではなく、Managed Instance のサービスティア上限に対する区間平均です。短時間ループの直接比較に使うのではなく、試験区間どうしの傾向比較に寄せて読む方が適切です。[server-resource-stats][mi-monitoring-dmvs]

## 5. よくある誤解と避け方

### 5.1 `DBCC USEROPTIONS` だけで RCSI 状態が分かると思う

誤解しやすい点です。`DBCC USEROPTIONS` は接続に有効な SET オプションを返しますが、RCSI の主確認は `sys.databases.is_read_committed_snapshot_on` を使う方が安全です。[dbcc-useroptions][alter-database-set-options]

### 5.2 tempdb だけ見れば versioning コストが分かると思う

Managed Instance では不十分です。ADR と PVS が前提にあるため、PVS を見ないと versioning コストを過小評価しやすくなります。[locking-row-versioning][pvs-stats]

### 5.3 `sys.server_resource_stats` をリアルタイム CPU とみなす

誤りです。これは更新遅延を伴うサービスメーターであり、秒単位の即時負荷を見る用途には向きません。[server-resource-stats][mi-monitoring-dmvs]

### 5.4 累積値をそのまま瞬間比較する

`sys.dm_os_wait_stats` や `sys.dm_io_virtual_file_stats` は累積値なので、区間比較では差分で読む必要があります。値そのものの大小だけでは、いつ増えたかが分かりません。[dm-os-wait-stats][dm-io-virtual-file-stats]

### 5.5 メモリ空き率を使用率だと思う

`sys.dm_os_sys_memory` の `available_physical_memory_kb / total_physical_memory_kb` は空き率です。使用率として列名を付けると、その後の読み手が誤解しやすくなります。[dm-os-sys-memory]

## 6. 参照スクリプトの考え方

参照スクリプトでは、個別案件に依存した DB 名や終了状態を固定しない方が再利用しやすくなります。そのため、対象 DB は変数で受け、開始前の RCSI 状態を保存し、終了時には元の状態へ戻すようにしています。

また、計測結果の保管先は対象 DB の恒久テーブルではなく、一時テーブル `#perf_metrics` にしています。これにより、評価そのものと収集用オブジェクト管理を切り離しやすくなります。恒久表に残したい場合は、このテンプレートを起点に対象環境向けへ調整するとよいです。

## 7. 参照スクリプト

以下のスクリプトは、RCSI ON/OFF 比較のための参照実装です。`START` を実行してループを開始し、停止ボタンで止めたあと、`RESULT` と `RESTORE` を順に実行します。`GO` は使っていません。

```sql
DECLARE @target_db_name sysname = N'SQL20900';
DECLARE @target_dbid int = DB_ID(@target_db_name);
DECLARE @original_rcsi_on bit;
DECLARE @sql nvarchar(max);

IF @target_dbid IS NULL
    THROW 50000, 'Target database was not found.', 1;

SELECT
    @original_rcsi_on = is_read_committed_snapshot_on
FROM sys.databases
WHERE database_id = @target_dbid;

SELECT
    name,
    is_read_committed_snapshot_on
FROM sys.databases
WHERE database_id = @target_dbid;

-- RCSI ON
IF @original_rcsi_on = 0
BEGIN
    SET @sql = N'ALTER DATABASE ' + QUOTENAME(@target_db_name)
        + N' SET READ_COMMITTED_SNAPSHOT ON WITH ROLLBACK IMMEDIATE;';
    EXEC (@sql);
END;

SELECT
    name,
    is_read_committed_snapshot_on
FROM sys.databases
WHERE database_id = @target_dbid;

-- INIT
IF OBJECT_ID(N'tempdb..#perf_metrics', N'U') IS NOT NULL
    DROP TABLE #perf_metrics;

CREATE TABLE #perf_metrics
(
    id bigint IDENTITY(1,1) PRIMARY KEY,
    sample_time datetime2 NOT NULL,

    -- tempdb 側の version store と tempdb 全体の使用率
    tempdb_version_store_mb float NULL,
    tempdb_usage_ratio float NULL,

    tempdb_io_read_ms bigint NULL,
    tempdb_io_write_ms bigint NULL,
    tempdb_io_total_ms bigint NULL,

    -- インスタンス全体の lock wait
    instance_lock_wait_ms bigint NULL,
    instance_lock_wait_ratio float NULL,

    -- 対象 DB に絞った blocking
    db_blocking_count int NULL,

    -- Managed Instance のサービスメーター
    mi_avg_cpu_percent float NULL,

    -- メモリ
    sql_memory_commit_ratio float NULL,
    os_memory_free_ratio float NULL,

    -- 対象 DB の I/O stall 差分
    db_io_read_ms bigint NULL,
    db_io_write_ms bigint NULL,
    db_io_total_ms bigint NULL,

    -- PVS
    pvs_size_mb float NULL,
    pvs_aborted_tx_count bigint NULL,
    pvs_oldest_active_transaction_id bigint NULL,

    -- row versioning 活動
    active_snapshot_tx_count int NULL,
    snapshot_elapsed_max_sec bigint NULL,

    -- log
    log_used_percent float NULL,
    log_used_mb float NULL
);

-- wait stats 初期化は可能なら実施し、権限や運用方針で不可なら継続
BEGIN TRY
    DBCC SQLPERF("sys.dm_os_wait_stats", CLEAR);
END TRY
BEGIN CATCH
    PRINT 'DBCC SQLPERF wait_stats CLEAR skipped (permission or policy).';
END CATCH;

-- START
DECLARE
    @prev_db_io_read bigint,
    @prev_db_io_write bigint,
    @prev_tempdb_io_read bigint,
    @prev_tempdb_io_write bigint,
    @prev_lock_wait bigint,
    @prev_total_wait bigint;

SELECT
    @prev_db_io_read = ISNULL(SUM(io_stall_read_ms), 0),
    @prev_db_io_write = ISNULL(SUM(io_stall_write_ms), 0)
FROM sys.dm_io_virtual_file_stats(@target_dbid, NULL);

SELECT
    @prev_tempdb_io_read = ISNULL(SUM(io_stall_read_ms), 0),
    @prev_tempdb_io_write = ISNULL(SUM(io_stall_write_ms), 0)
FROM sys.dm_io_virtual_file_stats(DB_ID(N'tempdb'), NULL);

SELECT
    @prev_lock_wait = ISNULL(SUM(wait_time_ms), 0)
FROM sys.dm_os_wait_stats
WHERE wait_type LIKE N'LCK%';

SELECT
    @prev_total_wait = ISNULL(SUM(wait_time_ms), 0)
FROM sys.dm_os_wait_stats;

WHILE (1 = 1)
BEGIN
    DECLARE
        @curr_db_io_read bigint,
        @curr_db_io_write bigint,
        @curr_tempdb_io_read bigint,
        @curr_tempdb_io_write bigint,
        @curr_lock_wait bigint,
        @curr_total_wait bigint,
        @cpu decimal(5,2),
        @log_used_percent float,
        @log_used_mb float;

    SELECT
        @curr_db_io_read = ISNULL(SUM(io_stall_read_ms), 0),
        @curr_db_io_write = ISNULL(SUM(io_stall_write_ms), 0)
    FROM sys.dm_io_virtual_file_stats(@target_dbid, NULL);

    SELECT
        @curr_tempdb_io_read = ISNULL(SUM(io_stall_read_ms), 0),
        @curr_tempdb_io_write = ISNULL(SUM(io_stall_write_ms), 0)
    FROM sys.dm_io_virtual_file_stats(DB_ID(N'tempdb'), NULL);

    SELECT
        @curr_lock_wait = ISNULL(SUM(wait_time_ms), 0)
    FROM sys.dm_os_wait_stats
    WHERE wait_type LIKE N'LCK%';

    SELECT
        @curr_total_wait = ISNULL(SUM(wait_time_ms), 0)
    FROM sys.dm_os_wait_stats;

    SELECT TOP (1)
        @cpu = CAST(avg_cpu_percent AS decimal(5,2))
    FROM master.sys.server_resource_stats
    ORDER BY end_time DESC;

    SET @sql = N'USE ' + QUOTENAME(@target_db_name) + N';
        SELECT
            @log_used_percent_out = CAST(used_log_space_in_percent AS float),
            @log_used_mb_out = CAST(used_log_space_in_bytes / 1048576.0 AS float)
        FROM sys.dm_db_log_space_usage;';

    EXEC sys.sp_executesql
        @sql,
        N'@log_used_percent_out float OUTPUT, @log_used_mb_out float OUTPUT',
        @log_used_percent OUTPUT,
        @log_used_mb OUTPUT;

    INSERT INTO #perf_metrics
    SELECT
        SYSDATETIME(),

        -- tempdb 側の version store（DB 別）
        (SELECT ISNULL(MAX(reserved_space_kb), 0) / 1024.0
         FROM sys.dm_tran_version_store_space_usage
         WHERE database_id = @target_dbid),

        -- tempdb 使用率（全体）
        (SELECT
            (SUM(total_page_count) - SUM(unallocated_extent_page_count)) * 1.0
            / NULLIF(SUM(total_page_count), 0) * 100
         FROM tempdb.sys.dm_db_file_space_usage),

        -- tempdb I/O 差分
        (@curr_tempdb_io_read - @prev_tempdb_io_read),
        (@curr_tempdb_io_write - @prev_tempdb_io_write),
        (@curr_tempdb_io_read - @prev_tempdb_io_read) + (@curr_tempdb_io_write - @prev_tempdb_io_write),

        -- インスタンス全体の lock wait 差分
        (@curr_lock_wait - @prev_lock_wait),

        -- 同一観測区間で増えた LCK 待機の割合
        ((@curr_lock_wait - @prev_lock_wait) * 1.0
         / NULLIF((@curr_total_wait - @prev_total_wait), 0) * 100),

        -- 対象 DB に絞った blocking
        (SELECT COUNT(*)
         FROM sys.dm_exec_requests
         WHERE database_id = @target_dbid
           AND blocking_session_id > 0),

        -- Managed Instance の CPU 参考値
        @cpu,

        -- SQL メモリのコミット比率
        (SELECT committed_kb * 1.0 / NULLIF(committed_target_kb, 0) * 100
         FROM sys.dm_os_sys_info),

        -- OS メモリ空き率
        (SELECT available_physical_memory_kb * 1.0
              / NULLIF(total_physical_memory_kb, 0) * 100
         FROM sys.dm_os_sys_memory),

        -- 対象 DB の I/O 差分
        (@curr_db_io_read - @prev_db_io_read),
        (@curr_db_io_write - @prev_db_io_write),
        (@curr_db_io_read - @prev_db_io_read) + (@curr_db_io_write - @prev_db_io_write),

        -- PVS
        (SELECT CAST(ISNULL(persistent_version_store_size_kb, 0) / 1024.0 AS float)
         FROM sys.dm_tran_persistent_version_store_stats
         WHERE database_id = @target_dbid),

        (SELECT current_aborted_transaction_count
         FROM sys.dm_tran_persistent_version_store_stats
         WHERE database_id = @target_dbid),

        (SELECT oldest_active_transaction_id
         FROM sys.dm_tran_persistent_version_store_stats
         WHERE database_id = @target_dbid),

        -- database_id 列がないため、ここではインスタンス全体の行バージョニング活動を見る
        (SELECT COUNT(*)
         FROM sys.dm_tran_active_snapshot_database_transactions),

        (SELECT ISNULL(MAX(elapsed_time_seconds), 0)
         FROM sys.dm_tran_active_snapshot_database_transactions),

        -- ログ使用率
        @log_used_percent,
        @log_used_mb;

    SET @prev_db_io_read = @curr_db_io_read;
    SET @prev_db_io_write = @curr_db_io_write;
    SET @prev_tempdb_io_read = @curr_tempdb_io_read;
    SET @prev_tempdb_io_write = @curr_tempdb_io_write;
    SET @prev_lock_wait = @curr_lock_wait;
    SET @prev_total_wait = @curr_total_wait;

    WAITFOR DELAY '00:00:10';
END;

-- STOP
-- 停止ボタンでループを止める

-- RESULT
SELECT *
FROM #perf_metrics
ORDER BY sample_time;

-- RESTORE
IF @original_rcsi_on = 0
BEGIN
    SET @sql = N'ALTER DATABASE ' + QUOTENAME(@target_db_name)
        + N' SET READ_COMMITTED_SNAPSHOT OFF WITH ROLLBACK IMMEDIATE;';
    EXEC (@sql);
END;

SELECT
    name,
    is_read_committed_snapshot_on
FROM sys.databases
WHERE database_id = @target_dbid;

DROP TABLE IF EXISTS #perf_metrics;
```

## 8. 結果の読み方

このスクリプトで重要なのは、「どの列が効果で、どの列が副作用か」を分けて解釈することです。

### 8.1 効果側の中心指標

最も直接的なのは `db_blocking_count` です。reader-writer 競合が支配的なワークロードなら、RCSI ON 後に平均や最大が下がる方向が期待されます。`instance_lock_wait_ms` と `instance_lock_wait_ratio` はインスタンス全体の値ですが、同時間帯の比較では方向性の補助指標になります。[dm-exec-requests][dm-os-wait-stats]

### 8.2 副作用側の中心指標

Managed Instance でまず見るべきは `pvs_size_mb` です。これが増えていても、`snapshot_elapsed_max_sec` が短く、`log_used_percent` が安定しているなら、コストは許容範囲かもしれません。逆に `pvs_size_mb` と `snapshot_elapsed_max_sec` が両方伸びるなら、長時間トランザクションによる cleanup 停滞を疑った方がよいです。[pvs-stats][active-snapshot-dmv][dm-db-log-space-usage]

### 8.3 参考値として見るもの

`mi_avg_cpu_percent` は短周期の即時比較には向きませんが、長めの試験区間どうしを比べるには有効です。`tempdb_usage_ratio` や `tempdb_io_total_ms` は周辺症状の把握には役立ちますが、Managed Instance では PVS より優先順位を上げすぎない方が安全です。[server-resource-stats][dm-db-file-space-usage]

## 9. 権限と運用上の注意

このスクリプトで使うオブジェクトは、権限要件が完全に同じではありません。`sys.dm_os_wait_stats`、`sys.dm_exec_requests`、`sys.dm_tran_persistent_version_store_stats` などは通常 `VIEW SERVER STATE` 系、`sys.dm_db_log_space_usage` は Managed Instance で `VIEW SERVER PERFORMANCE STATE` が必要とされます。さらに wait stats のリセットには `ALTER SERVER STATE` が必要です。スクリプトが途中で失敗した場合、最初に確認すべきは構文より権限です。[mi-monitoring-dmvs][dm-db-log-space-usage][dbcc-sqlperf]

また、`DBCC SQLPERF("sys.dm_os_wait_stats", CLEAR)` はインスタンス全体へ影響するため、本番や共有監視環境では特に注意が必要です。クリアできなくてもスクリプト自体は継続しますが、その場合は開始時点と終了時点の差分をより丁寧に読む必要があります。[dbcc-sqlperf]

## 10. 使うべき場面と、使うべきでない場面

この設計が向いているのは、同一ワークロード条件で RCSI OFF と RCSI ON を比較し、効果と副作用を区間単位で見たい場面です。逆に、単一クエリの即時ボトルネック調査や、秒単位のリアルタイム CPU 監視には向きません。その場合は `sys.dm_exec_requests`、待機 DMV、Query Store、別の監視系を優先した方が適切です。[dm-exec-requests][dm-os-wait-stats]

## 11. まとめ

Azure SQL Managed Instance での RCSI 評価は、単に取れる列を増やせばよいわけではありません。何を効果指標とし、何を副作用指標とし、どの値を参考値に留めるかを先に設計することで、結果の解釈が安定します。

この文書では、個別調査の修正履歴ではなく、Managed Instance で再利用できる観測設計の骨格を示しました。RCSI 評価を別の案件へ持ち込むときも、この骨格を維持したまま対象 DB 名や保存先だけ調整する方が、持続可能な資料になりやすくなります。

## 参考

- [DBCC USEROPTIONS][dbcc-useroptions]
- [DBCC SQLPERF][dbcc-sqlperf]
- [ALTER DATABASE SET options][alter-database-set-options]
- [SET TRANSACTION ISOLATION LEVEL][set-transaction-isolation]
- [Transaction locking and row versioning guide][locking-row-versioning]
- [sys.dm_os_wait_stats][dm-os-wait-stats]
- [sys.dm_exec_requests][dm-exec-requests]
- [sys.dm_tran_version_store_space_usage][dm-tran-version-store-space-usage]
- [sys.dm_tran_persistent_version_store_stats][pvs-stats]
- [sys.dm_tran_active_snapshot_database_transactions][active-snapshot-dmv]
- [sys.dm_db_log_space_usage][dm-db-log-space-usage]
- [sys.dm_db_file_space_usage][dm-db-file-space-usage]
- [sys.dm_io_virtual_file_stats][dm-io-virtual-file-stats]
- [sys.dm_os_sys_info][dm-os-sys-info]
- [sys.dm_os_sys_memory][dm-os-sys-memory]
- [sys.server_resource_stats (Azure SQL Managed Instance)][server-resource-stats]
- [Monitoring Azure SQL Managed Instance performance using dynamic management views][mi-monitoring-dmvs]

[dbcc-useroptions]: https://learn.microsoft.com/sql/t-sql/database-console-commands/dbcc-useroptions-transact-sql?view=sql-server-ver17
[dbcc-sqlperf]: https://learn.microsoft.com/sql/t-sql/database-console-commands/dbcc-sqlperf-transact-sql?view=sql-server-ver17
[alter-database-set-options]: https://learn.microsoft.com/sql/t-sql/statements/alter-database-transact-sql-set-options?view=sql-server-ver17
[set-transaction-isolation]: https://learn.microsoft.com/sql/t-sql/statements/set-transaction-isolation-level-transact-sql?view=sql-server-ver17
[locking-row-versioning]: https://learn.microsoft.com/sql/relational-databases/sql-server-transaction-locking-and-row-versioning-guide?view=sql-server-ver17
[dm-os-wait-stats]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-os-wait-stats-transact-sql?view=sql-server-ver17
[dm-exec-requests]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-exec-requests-transact-sql?view=sql-server-ver17
[dm-tran-version-store-space-usage]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-tran-version-store-space-usage?view=sql-server-ver17
[pvs-stats]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-tran-persistent-version-store-stats?view=sql-server-ver17
[active-snapshot-dmv]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-tran-active-snapshot-database-transactions-transact-sql?view=sql-server-ver17
[dm-db-log-space-usage]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-db-log-space-usage-transact-sql?view=sql-server-ver17
[dm-db-file-space-usage]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-db-file-space-usage-transact-sql?view=sql-server-ver17
[dm-io-virtual-file-stats]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-io-virtual-file-stats-transact-sql?view=sql-server-ver17
[dm-os-sys-info]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-os-sys-info-transact-sql?view=sql-server-ver17
[dm-os-sys-memory]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-os-sys-memory-transact-sql?view=sql-server-ver17
[server-resource-stats]: https://learn.microsoft.com/sql/relational-databases/system-catalog-views/sys-server-resource-stats-azure-sql-database?view=azuresqldb-current
[mi-monitoring-dmvs]: https://learn.microsoft.com/azure/azure-sql/managed-instance/monitoring-with-dmvs?view=azuresql
