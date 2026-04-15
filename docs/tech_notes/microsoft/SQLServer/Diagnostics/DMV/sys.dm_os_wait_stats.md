# sys.dm_os_wait_stats

この DMV は、SQL Server インスタンス全体で発生した待機の累積統計を返します。いま待っているものではなく、完了した待機の合計を集計しているため、単発の瞬間観測よりも、一定時間の差分比較やボトルネックの傾向把握に向いています。[official]

性能分析で重要なのは、待機の名前を暗記することではなく、どの種類の待機が増えているかを区間差分で見ることです。ロック待機が増えたのか、I/O 待機が増えたのか、CPU でスケジューリング待ちが増えたのかをここで見分けます。

## 1. 主な使いどころ

この DMV は、次のような問いに向いています。

- テスト前後で何待ちが増えたか。
- インスタンス全体の主要ボトルネックがロック、I/O、CPU のどこに寄っているか。
- Query Store や実行計画を見る前に、まず全体の圧力方向を知りたいとき。

## 2. 列の整理

列自体は少なく、意味も比較的明快です。

| 列                     | 意味                             | 実務での見方                                                                   |
| --------------------- | ------------------------------ | ------------------------------------------------------------------------ |
| `wait_type`           | 待機種別名                          | `LCK_%`、`PAGEIOLATCH_%`、`WRITELOG`、`SOS_SCHEDULER_YIELD` など、原因の方向性を表します。 |
| `waiting_tasks_count` | その待機が発生した回数                    | 待機の頻度を見ます。短い待機が大量に起きているケースで効きます。                                         |
| `wait_time_ms`        | その待機種別で消費した総待機時間               | 累積待機の主指標です。`signal_wait_time_ms` を含みます。                                  |
| `max_wait_time_ms`    | 1 回あたりの最大待機時間                  | 極端に長い単発待機があったかを見る補助列です。                                                  |
| `signal_wait_time_ms` | リソース解放後、実際に CPU で再実行されるまでの待ち時間 | CPU スケジューリング待ちの強さを見る補助列です。                                               |
| `pdw_node_id`         | 分散ノード識別子                       | Azure Synapse Analytics / APS のみで使います。                                   |

## 3. 読み方のコツ

`wait_time_ms` が大きい待機だけを見ると、アイドル系や正常系の待機も混ざります。そのため、意味のある待機だけを絞って差分比較するのが基本です。たとえばロック競合なら `LCK_%`、ストレージ遅延なら `PAGEIOLATCH_%`、ログ書き込みなら `WRITELOG`、CPU 圧力なら `SOS_SCHEDULER_YIELD` を起点に見ます。

`signal_wait_time_ms` が大きい場合は、単なる I/O やロック待ちではなく、待機解除後に CPU を得られていない可能性があります。逆に `wait_time_ms` の大半がシグナル待ち以外なら、資源そのものの待機が強いと解釈できます。

## 4. 注意点

この DMV は、現在待機中のタスクを返しません。あくまで完了した待機の累積なので、「いま何待ちで止まっているか」を見るには `sys.dm_os_waiting_tasks` や `sys.dm_exec_requests` を使う必要があります。[official]

統計は SQL Server サービス再起動で初期化され、`DBCC SQLPERF ('sys.dm_os_wait_stats', CLEAR);` でもリセットできます。したがって、絶対値をそのまま比較するのではなく、測定開始時と終了時の差分を取るのが原則です。[official]

権限は、SQL Server 2022 以降では `VIEW SERVER PERFORMANCE STATE`、それ以前では `VIEW SERVER STATE` が基本です。Azure SQL Database ではサービス階層によって追加条件があります。[official]

[official]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-os-wait-stats-transact-sql?view=sql-server-ver17
