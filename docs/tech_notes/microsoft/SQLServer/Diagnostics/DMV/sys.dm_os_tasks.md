# sys.dm_os_tasks

この DMV は、SQL Server インスタンス内でアクティブなタスクごとに 1 行を返します。タスクは SQL Server 実行の基本単位であり、1 つの要求の内部で複数タスクが動くことがあります。とくに並列クエリでは、要求単位では見えない複数タスクの状態を見るために重要です。[official]

`sys.dm_exec_requests` が要求単位の見取り図だとすると、`sys.dm_os_tasks` はその要求を構成する下位の作業単位を見るための DMV です。並列度、タスク状態、I/O ペンディング量を見たいときに役立ちます。

## 1. 主な使いどころ

この DMV は、次のような場面に向いています。

- 並列クエリが何本のタスクで動いているかを見たいとき。
- 同一要求の中で、どのタスクが `RUNNABLE`、`SUSPENDED` になっているかを確認したいとき。
- Windows スレッドやワーカーとの対応付けをしたいとき。

## 2. 列の整理

タスク単位で見るときに重要な列は次のとおりです。

| 列 | 意味 | 実務での見方 |
| --- | --- | --- |
| `task_address` | タスクのメモリ アドレス | タスクの識別子として使います。 |
| `task_state` | タスク状態 | `PENDING`、`RUNNABLE`、`RUNNING`、`SUSPENDED`、`DONE`、`SPINLOOP` などが返ります。 |
| `context_switches_count` | コンテキスト スイッチ回数 | タスクがどれだけスケジューラ切替を受けたかを見る補助列です。 |
| `pending_io_count` | このタスクが実行している物理 I/O 数 | I/O を多く抱えているタスクかどうかを見ます。 |
| `pending_io_byte_count` | 実行中 I/O の総バイト数 | I/O 規模の当たりをつけます。 |
| `pending_io_byte_average` | 実行中 I/O の平均バイト数 | I/O サイズの傾向を補足します。 |
| `scheduler_id` | 親スケジューラ ID | どのスケジューラ配下のタスクかを見ます。 |
| `session_id` | タスクが属するセッション ID | セッション単位で追う入口です。 |
| `exec_context_id` | 実行コンテキスト ID | 並列要求内の各タスクを区別します。 |
| `request_id` | タスクが属する要求 ID | `session_id` と組み合わせて要求へ戻ります。 |
| `worker_address` | タスクを実行しているワーカーのアドレス | `sys.dm_os_workers` や `sys.dm_os_threads` と結合するときに使います。 |
| `host_address` | ホストのメモリ アドレス | このタスクを作成したホストの追跡に使います。 |
| `parent_task_address` | 親タスクのアドレス | タスク階層を追うときの補助列です。 |

## 3. 読み方のコツ

並列要求では、同じ `session_id` と `request_id` に対して複数行が返ります。したがって、1 つの要求が遅いときは、要求 1 行だけで判断せず、その下にぶら下がるタスク群の `task_state` や `pending_io_count` を見ると、CPU 待ちなのか、I/O を抱えているのか、イベント待ちなのかを細かく切り分けられます。[official]

`worker_address` が `NULL` の場合は、ワーカーの実行待ちであるか、タスクが完了直後である可能性があります。タスクの存在とワーカーの存在は常に 1 対 1 ではないことを意識して読む方が安全です。[official]

## 4. 注意点

この DMV はアクティブなタスクだけを返します。スリープ中のセッション情報は返らないため、セッションの全体像は `sys.dm_exec_sessions` や `sys.dm_exec_requests` と併用するのが前提です。[official]

権限は、SQL Server 2022 以降では `VIEW SERVER PERFORMANCE STATE`、それ以前では `VIEW SERVER STATE` が基本です。Azure SQL Database ではサービス階層に応じた条件があります。[official]

[official]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-os-tasks-transact-sql?view=sql-server-ver17
