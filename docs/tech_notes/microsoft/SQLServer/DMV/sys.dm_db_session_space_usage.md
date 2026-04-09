# sys.dm_db_session_space_usage

この DMV は、tempdb において各セッションが割り当てたページ数と割り当て解除したページ数を返します。セッション単位で tempdb の使い方を追えるので、「どの接続が tempdb を多く消費したか」を見るときに役立ちます。[official]

ただし、この DMV はリアルタイムの現在値ではなく、セッション内で完了したタスクの累積を返します。実行中タスクの消費をそのまま反映しない点が、`sys.dm_db_task_space_usage` との大きな違いです。[official]

## 1. 主な使いどころ

この DMV は、次のような場面で有効です。

- tempdb を多く使ったセッションを洗い出したいとき。
- 一時テーブルや内部作業の tempdb 使用傾向をセッション単位で見たいとき。
- セッション終了まで含めた累積 tempdb 消費を見たいとき。

## 2. 列の整理

この DMV は tempdb のセッション単位カウンターに絞られています。

| 列 | 意味 | 実務での見方 |
| --- | --- | --- |
| `session_id` | セッション ID | `sys.dm_exec_sessions` と結合する起点です。 |
| `database_id` | データベース ID | tempdb 監視で使います。 |
| `user_objects_alloc_page_count` | ユーザー オブジェクトに割り当てたページ数 | 一時テーブル、テーブル変数などの消費量を見ます。 |
| `user_objects_dealloc_page_count` | ユーザー オブジェクトから解放されたページ数 | 使い終わった量を見て純消費量を推定します。 |
| `internal_objects_alloc_page_count` | 内部オブジェクトに割り当てたページ数 | ソート、ハッシュ、スプールなど内部作業の消費量です。 |
| `internal_objects_dealloc_page_count` | 内部オブジェクトから解放されたページ数 | 内部作業の解放量を見ます。 |
| `user_objects_deferred_dealloc_page_count` | 遅延割り当て解除対象のページ数 | 解放待ちの領域がどれだけあるかを補足します。 |

## 3. 読み方のコツ

tempdb の純消費量を見たいときは、割り当て数と割り当て解除数の差を取ると分かりやすくなります。たとえば `user_objects_alloc_page_count - user_objects_dealloc_page_count` は、そのセッションがユーザー オブジェクトで抱えてきた純増分の目安になります。

また、ユーザー オブジェクト消費と内部オブジェクト消費を分けて見ると、一時テーブル主導なのか、クエリ処理内部のソートやハッシュ主導なのかを切り分けやすくなります。[official]

## 4. 注意点

この DMV は tempdb にのみ適用されます。さらに、ページ カウンターはセッション開始時に 0 で初期化され、タスクが終了した時点でのみ更新されます。つまり、実行中のタスクの消費は即時には反映されません。[official]

IAM ページは割り当て／割り当て解除カウントに含まれません。このため、実ディスク使用量と完全一致するわけではなく、tempdb 利用傾向を見るための運用指標として使うのが適切です。[official]

権限は、SQL Server 2022 以降では `VIEW SERVER PERFORMANCE STATE`、それ以前では `VIEW SERVER STATE` が基本です。[official]

[official]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-db-session-space-usage-transact-sql?view=sql-server-ver17
