# sys.dm_os_waiting_tasks

この DMV は、何らかのリソースを待機しているタスクの待機キューを返します。`sys.dm_os_wait_stats` が累積結果を見るための DMV だとすれば、こちらは「いま実際に何が待っているか」を観測するための瞬間観測 DMV です。[official]

ブロッキング調査や、並列クエリのどのタスクがどこで詰まっているかを掘るときには、この DMV が最も直接的です。要求単位よりも細かいタスク単位で見えるので、`sys.dm_exec_requests` より深いところまで追えます。

## 1. 主な使いどころ

この DMV は、次のような場面で強みがあります。

- 現在ブロックされているタスクを特定したいとき。
- 並列クエリの一部タスクだけが止まっているかを確認したいとき。
- `wait_resource` より詳しいリソース記述を見たいとき。

## 2. 列の整理

実務でまず見るべき列は、待機種別、待機時間、ブロッカー、リソース説明の四本柱です。

| 列 | 意味 | 実務での見方 |
| --- | --- | --- |
| `waiting_task_address` | 待機タスクのアドレス | タスクを一意に追うときの識別子です。 |
| `session_id` | タスクが属するセッション ID | セッション単位の主体を追う入口です。 |
| `exec_context_id` | 実行コンテキスト ID | 並列実行時に、同一要求内のどのタスクかを区別できます。 |
| `wait_duration_ms` | 現在の待機継続時間 | いまの待機がどれだけ長いかを示します。 |
| `wait_type` | 現在の待機種別 | ロック、ラッチ、I/O、スレッドプールなど、原因の方向を見ます。 |
| `resource_address` | 待機対象リソースのアドレス | より低レベルの結合や内部解析で使います。 |
| `blocking_task_address` | そのリソースを保持しているタスクのアドレス | タスク単位でブロッカーを追うときに使います。 |
| `blocking_session_id` | ブロック元セッション ID | セッション単位で犯人を探すときの主列です。 |
| `blocking_exec_context_id` | ブロック元タスクの実行コンテキスト ID | 並列実行時のブロッカー特定に使います。 |
| `resource_description` | 待機中リソースの詳細説明 | この DMV の核です。ロック、ラッチ、並列 Exchange、threadpool などの形式で詳細が出ます。 |
| `pdw_node_id` | 分散ノード識別子 | Azure Synapse Analytics / APS のみで使います。 |

## 3. `resource_description` の見方

この列は、単なる補足ではなく、待機の実体を読むための本体です。たとえばロック待機なら `objectlock`、`pagelock`、`keylock` などの形式で出ますし、並列クエリでは `exchangeEvent`、スレッド枯渇では `threadpool`、ラッチではページやラッチ クラスが埋め込まれます。[official]

そのため、`wait_type` が大分類、`resource_description` が具体的な現場情報という関係で読むと整理しやすくなります。たとえば `PAGEIOLATCH_SH` だけでは I/O 待ちとしか分かりませんが、`resource_description` から対象ページやラッチの文脈を追える場合があります。

## 4. 注意点

この DMV は瞬間観測なので、短い待機は取りこぼします。継続的な傾向を見るには `sys.dm_os_wait_stats`、今の詰まりを掘るには `sys.dm_os_waiting_tasks`、という使い分けが基本です。[official]

`blocking_session_id` の特殊値にも注意が必要です。`-2` は孤立分散トランザクション、`-3` は遅延復旧トランザクション、`-4` は内部ラッチ遷移などで所有者が判定できないケースを表します。[official]

権限は、SQL Server 2022 以降では `VIEW SERVER PERFORMANCE STATE`、それ以前では `VIEW SERVER STATE` が基本です。Azure SQL Database ではサービス階層ごとの条件があります。[official]

[official]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-os-waiting-tasks-transact-sql?view=sql-server-ver17
