# sys.dm_exec_requests

この DMV は、SQL Server で現在実行中の要求を 1 行ずつ返す瞬間観測用の DMV です。ブロッキング、待機、CPU 使用量、I/O、進捗率をその場で確認できるため、いま遅い処理や詰まっている処理を特定するときの最初の入口になります。[official]

重要なのは、この DMV が履歴ではなく「今」を返すことです。区間比較には向きませんが、セッション単位では見えにくい要求単位の状態を見られるので、ブロック調査や長時間実行クエリの切り分けでは最も使いやすい部類に入ります。

## 1. 主な使いどころ

この DMV は、次の問いに答えるときに役立ちます。

- いま実行中の要求は何か。
- どの要求がどの待機で止まっているか。
- 誰が誰をブロックしているか。
- 長時間実行されている処理が CPU なのか I/O なのか、あるいはロック待ちなのか。
- バックアップやリストア、インデックス再構成のような進捗率を持つ処理がどこまで進んだか。

## 2. 列の整理

列数は多いですが、実務で最初に見るべき列はある程度決まっています。まずは状態、待機、資源消費、SQL テキスト特定用の列を押さえると十分です。

| 列 | 意味 | 実務での見方 |
| --- | --- | --- |
| `session_id` / `request_id` | 要求が属するセッション ID と、そのセッション内での要求 ID | `sys.dm_exec_sessions` や `sys.dm_os_tasks` と結合する起点です。 |
| `status` | 要求の現在状態 | `running`、`runnable`、`suspended` などを見て、CPU 待ちなのか、何かの完了待ちなのかを判断します。 |
| `command` | 実行中コマンドの種別 | `SELECT`、`INSERT`、`BACKUP DATABASE`、`DBCC` などが見えるため、処理の種類をざっくり把握できます。 |
| `start_time` | 要求の開始時刻 | 長時間実行要求の抽出に使います。 |
| `database_id` | 実行対象のデータベース ID | どの DB の処理かを切り分ける入口になります。 |
| `blocking_session_id` | この要求をブロックしているセッション ID | `0` や `NULL` 以外なら、まずブロッキングを疑います。 `-2` は孤立分散トランザクション、`-3` は遅延復旧トランザクション、`-4` と `-5` はラッチ系や所有者未確定の特殊値です。 |
| `wait_type` | 現在の待機種別 | 何待ちで止まっているかを示す主列です。複数タスクを使う並列要求では、要求単位では代表値しか見えません。 |
| `wait_time` | 現在の待機時間 | いまの待機がどれだけ継続しているかを見ます。 |
| `last_wait_type` | 直近の待機種別 | 現在は走っていても、直前に何待ちだったかを補足できます。 |
| `wait_resource` | 待機中のリソース | ページ、キー、オブジェクト、ラッチ対象などの詳細を確認するときに重要です。 |
| `transaction_id` / `open_transaction_count` | 関連するトランザクション ID と開いているトランザクション数 | 長いトランザクションやロック保持の調査に使います。 |
| `cpu_time` | この要求が消費した CPU 時間 | CPU 偏重の処理かどうかの当たりをつけます。 |
| `total_elapsed_time` | 要求受信からの経過時間 | 待機を含む総経過時間なので、ユーザー体感に近い時間です。 |
| `reads` / `writes` / `logical_reads` | 物理読み取り、書き込み、論理読み取り | I/O とバッファ使用量の当たりをつける基本列です。 |
| `row_count` | 返却済み行数 | 処理が実際に結果を吐いているか、どの程度進んだかを補足できます。 |
| `sql_handle` | 実行バッチやストアドプロシージャを識別するハンドル | `sys.dm_exec_sql_text` と組み合わせて SQL テキストを取得します。 |
| `statement_start_offset` / `statement_end_offset` | バッチ内で現在実行中の文の位置 | 1 バッチ中のどの文が詰まっているかを特定できます。 |
| `plan_handle` | 実行プランを識別するハンドル | `sys.dm_exec_query_plan` と組み合わせてプラン確認に使います。 |
| `percent_complete` | 一部の長時間処理に対する進捗率 | `BACKUP`、`RESTORE`、`DBCC`、`ALTER INDEX REORGANIZE` などで有効です。 |
| `dop` / `parallel_worker_count` | 並列度と予約済み並列ワーカー数 | 並列クエリかどうか、どの程度ワーカーを使っているかを見ます。 |
| `query_hash` / `query_plan_hash` | 類似 SQL と類似プランを束ねるハッシュ値 | リテラル違いをまとめて傾向を見るときに有効です。 |
| `page_resource` | ページ待機時の 8 バイト表現 | SQL Server 2019 以降で、ページ資源の詳細解析に使えます。 |

## 3. 読み方のコツ

この DMV は、単独でも有用ですが、次のように読むと解釈を誤りにくくなります。

- `status = suspended` かつ `wait_type` がロック系なら、まずブロッキングを疑います。
- `total_elapsed_time` が長いのに `cpu_time` が小さい場合は、CPU 以外の待機やブロッキングの可能性が高くなります。
- `reads` や `logical_reads` が大きい場合は、実行計画やアクセスパスを確認した方が早いです。
- `sql_handle` とオフセット列があれば、バッチ全体ではなく現在の文だけを切り出せます。

## 4. 注意点

並列要求では、行モード並列実行時にコーディネーター スレッドのみが見えるため、`reads`、`writes`、`logical_reads`、`row_count`、`wait_type`、`wait_time` などが要求全体を完全には表さない場合があります。並列実行の詳細を追うときは `sys.dm_os_tasks` や `sys.dm_os_waiting_tasks` と併用する方が安全です。[official]

また、この DMV の時間列には、SQL Server の外側のコードをプリエンプティブ モードで実行している時間は含まれません。したがって、外部要因を含む処理では、見えている時間だけで全体時間を説明し切れないことがあります。[official]

権限は、SQL Server 2022 以降では `VIEW SERVER PERFORMANCE STATE`、それ以前では `VIEW SERVER STATE` が基本です。Azure SQL Database では `VIEW SERVER STATE` を一般に付与できないため、通常は現在の接続分だけが見えます。[official]

[official]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-exec-requests-transact-sql?view=sql-server-ver17
