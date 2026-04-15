# sys.dm_db_task_space_usage

この DMV は、tempdb に対するタスクごとのページ割り当てと割り当て解除の情報を返します。セッション単位ではなくタスク単位で見えるため、並列クエリや複数要求が混在するセッションで、どの実行単位が tempdb を消費しているかを掘るときに有効です。[official]

`sys.dm_db_session_space_usage` がセッション累積を見るのに対し、こちらは要求・タスクの粒度まで降りて見られます。tempdb の犯人捜しを細かくやるときはこちらが有利です。

## 1. 主な使いどころ

この DMV は、次のような場面に向いています。

- 並列クエリのどのタスクが tempdb を消費しているか見たいとき。
- 同じセッションの中で、どの要求が tempdb を押し上げたかを知りたいとき。
- セッション単位では粗すぎる tempdb 利用調査をしたいとき。

## 2. 列の整理

列は少なく、タスク粒度の観測に必要な最小構成です。

| 列 | 意味 | 実務での見方 |
| --- | --- | --- |
| `session_id` | セッション ID | 所有者セッションを識別します。 |
| `request_id` | セッション内の要求 ID | 同一セッション内の複数要求を区別します。 |
| `exec_context_id` | タスクの実行コンテキスト ID | 並列実行時の各タスクを区別します。 |
| `database_id` | データベース ID | tempdb 監視で使います。 |
| `user_objects_alloc_page_count` | ユーザー オブジェクトに割り当てたページ数 | タスク単位での一時テーブル等の消費量です。 |
| `user_objects_dealloc_page_count` | ユーザー オブジェクトから解放したページ数 | タスク終了時の解放動向を補足します。 |
| `internal_objects_alloc_page_count` | 内部オブジェクトに割り当てたページ数 | ソート、ハッシュ、スプールなど内部作業のタスク単位消費を見ます。 |
| `internal_objects_dealloc_page_count` | 内部オブジェクトから解放したページ数 | 内部作業終了後の解放量です。 |

## 3. 読み方のコツ

tempdb を多く消費した要求を見つけたいときは、`session_id` と `request_id` で集約し、その後 `exec_context_id` ごとに分解すると、要求全体の問題なのか、特定タスクだけが偏っているのかを把握しやすくなります。並列クエリでは、同じ要求の中でもタスクごとに tempdb 消費量が偏ることがあります。[official]

内部オブジェクトの割り当てが大きい場合は、明示的一時テーブルではなく、ソートやハッシュが tempdb を使っている可能性が高くなります。ユーザー オブジェクトと内部オブジェクトを分けて読むのが重要です。

## 4. 注意点

この DMV は tempdb にのみ適用されます。ページ カウンターは要求開始時に 0 で初期化され、要求完了時にセッション レベルへ集約されます。つまり、`sys.dm_db_session_space_usage` はこの DMV の結果が要求終了後にまとめ上げられたものだと考えると理解しやすいです。[official]

また、作業テーブル キャッシュ、一時テーブル キャッシュ、遅延ドロップはカウンターへ影響します。IAM ページも含まれません。厳密な物理使用量ではなく、運用上の消費傾向をつかむ指標として使うのが適切です。[official]

権限は、SQL Server 2022 以降では `VIEW SERVER PERFORMANCE STATE`、それ以前では `VIEW SERVER STATE` が基本です。[official]

[official]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-db-task-space-usage-transact-sql?view=sql-server-ver17
