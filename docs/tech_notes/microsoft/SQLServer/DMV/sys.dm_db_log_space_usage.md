# sys.dm_db_log_space_usage

この DMV は、データベース単位のトランザクション ログ領域使用量を返します。複数のログ ファイルがあっても、結果はデータベース単位に集約されるため、「このデータベースのログがどれだけ逼迫しているか」を最短で知るのに向いています。[official]

更新系ワークロードや RCSI 評価では、ロック待機や tempdb だけでなく、ログ使用量が増えていないかも確認する必要があります。そのときに最初に当たるべき DMV がこれです。

## 1. 主な使いどころ

この DMV は、次のような場面で有効です。

- ログ使用率が高くなっているデータベースを見つけたいとき。
- tempdb のログ空き領域を確認したいとき。
- ログ バックアップ以降の増分量を把握したいとき。

## 2. 列の整理

列数は少なく、どれもそのまま使いやすいです。

| 列 | 意味 | 実務での見方 |
| --- | --- | --- |
| `database_id` | データベース ID | どの DB のログかを識別します。 |
| `total_log_size_in_bytes` | ログ総サイズ | ログ ファイル全体のサイズです。 |
| `used_log_space_in_bytes` | 使用中ログ領域 | 現在どれだけログが埋まっているかを見ます。 |
| `used_log_space_in_percent` | 使用率 | 逼迫度を直感的に判断する主列です。 |
| `log_space_in_bytes_since_last_backup` | 前回ログ バックアップ以降に使用された領域 | ログ バックアップ間隔や更新量の把握に使います。 |

## 3. 読み方のコツ

まず `used_log_space_in_percent` を見て、高いデータベースを候補として拾います。その上で `total_log_size_in_bytes` と `used_log_space_in_bytes` を見れば、単純にログが小さいのか、更新が多すぎるのか、あるいはログ切り捨てが進んでいないのかの当たりがつきます。

`log_space_in_bytes_since_last_backup` は、ログ バックアップを前提とした運用でとくに有効です。前回バックアップ以降にどれだけログが積み上がったかを見ることで、バックアップ間隔や更新量の変化を読み取れます。[official]

## 4. 注意点

この DMV はログ ファイル単位ではなく、データベース単位に全ログ ファイルを合算して返します。どのログ ファイルが問題かまで掘りたい場合は、`sys.dm_db_log_info` や `sys.dm_db_log_stats` を併用した方が適切です。[official]

権限は、SQL Server 2022 以降と Azure SQL Managed Instance では `VIEW SERVER PERFORMANCE STATE`、SQL Server 2019 以前では `VIEW SERVER STATE` が基本です。Azure SQL Database ではサービス階層ごとの条件があります。[official]

[official]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-db-log-space-usage-transact-sql?view=sql-server-ver17
