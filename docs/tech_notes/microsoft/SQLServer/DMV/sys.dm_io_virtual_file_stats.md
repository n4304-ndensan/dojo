# sys.dm_io_virtual_file_stats

このオブジェクトは DMV ではなく DMF です。`database_id` と `file_id` を引数に取り、データ ファイルとログ ファイルの I/O 統計を返します。ファイル単位で読み取り回数、書き込み回数、待機時間、読み書きバイト数を見られるため、データ I/O とログ I/O を切り分けるときの中心的な DMF です。[official]

`sys.dm_exec_requests` や `sys.dm_os_wait_stats` が要求や待機を見るのに対し、この DMF はファイル I/O の累積実績を見る役割を持ちます。ストレージの遅さを疑うときに、どのファイルがどれだけ遅いかを数量化できます。

## 1. 引数

この DMF は関数なので、引数の意味を先に押さえておく必要があります。

| 引数 | 意味 | 実務での使い方 |
| --- | --- | --- |
| `database_id` | 対象データベース ID。`NULL` なら全データベース | インスタンス全体を俯瞰するときは `NULL`、特定 DB だけ見たいときは `DB_ID('db_name')` を使います。 |
| `file_id` | 対象ファイル ID。`NULL` ならその DB 内の全ファイル | ログファイルだけ、あるいは特定データファイルだけ見たいときに使います。 |

## 2. 列の整理

返される列は累積カウンター中心です。差分を取ることで平均待機時間やスループットを計算できます。

| 列 | 意味 | 実務での見方 |
| --- | --- | --- |
| `database_name` | データベース名 | 結果をそのまま読めるようにする補助列です。 |
| `database_id` / `file_id` | DB ID とファイル ID | `sys.master_files` や `sys.database_files` と結合するときの主キーです。 |
| `sample_ms` | OS 起動後の経過ミリ秒 | 2 回採取した結果の差分区間を作る基準です。 |
| `num_of_reads` | 読み取り回数 | 読み取り要求件数の累積です。 |
| `num_of_bytes_read` | 読み取りバイト総量 | 読み取りスループットの把握に使います。 |
| `io_stall_read_ms` | 読み取り完了待機時間の累積 | 読み取りレイテンシの源泉です。差分を `num_of_reads` 差分で割ると平均待機時間を求められます。 |
| `num_of_writes` | 書き込み回数 | 書き込み要求件数の累積です。 |
| `num_of_bytes_written` | 書き込みバイト総量 | 書き込みスループットの把握に使います。 |
| `io_stall_write_ms` | 書き込み完了待機時間の累積 | ログ I/O やデータファイル書き込みの遅さを見る列です。 |
| `io_stall` | 読み書き合算の I/O 待機時間累積 | 全体傾向を見るときの簡易指標です。 |
| `size_on_disk_bytes` | ディスク上で消費しているファイル サイズ | スパース ファイルやスナップショットでも実消費量を確認できます。 |
| `io_stall_queued_read_ms` | I/O Resource Governor による読み取りキュー待ち時間 | Resource Governor 配下の I/O 制御影響を見る列です。 |
| `io_stall_queued_write_ms` | I/O Resource Governor による書き込みキュー待ち時間 | 書き込み側の制御待ち時間です。 |
| `file_handle` | Windows ファイル ハンドル | 深い OS 連携分析時の補助列です。 |

## 3. 読み方のコツ

この DMF は累積統計なので、絶対値ではなく差分で読むのが基本です。たとえば 2 回採取して、`delta_io_stall_read_ms / delta_num_of_reads` を取れば、その区間の平均読み取り待機時間を計算できます。同様に `delta_io_stall_write_ms / delta_num_of_writes` で平均書き込み待機時間を見られます。[official]

また、`sys.master_files` や `sys.database_files` と結合すれば、`ROWS` ファイルなのか `LOG` ファイルなのかを区別して読めます。これにより、`WRITELOG` 系の問題なのか、データ読み取り系の問題なのかを切り分けやすくなります。[official]

## 4. 注意点

カウンターは SQL Server サービス開始時に初期化されます。したがって、再起動をまたぐ比較や、起動直後の小さな値と長期間稼働後の大きな値をそのまま比較するのは危険です。[official]

`sample_ms` は SQL Server 2014 より前では `int` で、約 25 日で 0 に戻る点にも注意が必要です。現行バージョンでは `bigint` です。[official]

権限は、SQL Server 2022 以降では `VIEW SERVER PERFORMANCE STATE`、それ以前では `VIEW SERVER STATE` が基本です。[official]

[official]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-io-virtual-file-stats-transact-sql?view=sql-server-ver17
