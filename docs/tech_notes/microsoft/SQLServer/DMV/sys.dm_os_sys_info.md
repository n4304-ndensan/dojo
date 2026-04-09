# sys.dm_os_sys_info

この DMV は、SQL Server が動作しているコンピューターの基礎情報と、SQL Server が利用できる資源、実際に使っている資源をまとめて返します。CPU 数、メモリの総量、SQL Server のコミット済みメモリ、スケジューラ数、起動時刻などを 1 行で確認できるため、環境の前提条件を把握する基準点として非常に有用です。[official]

性能分析では、待機や要求だけを見ても、そのサーバーがそもそも何 CPU で、どの程度のメモリ規模で、いつ再起動されたかが分からないと数字の意味が定まりません。この DMV は、その前提を揃えるための土台になります。

## 1. 主な使いどころ

この DMV は、次のような場面で使います。

- サーバーの CPU・NUMA・メモリ構成を確認したいとき。
- SQL Server の再起動時刻を確認したいとき。
- SQL Server が現在どの程度メモリを使っていて、どの程度まで増やそうとしているかを見たいとき。

## 2. 列の整理

列は多いですが、実務でまず押さえるべきものは次のとおりです。

| 列 | 意味 | 実務での見方 |
| --- | --- | --- |
| `cpu_count` | 論理 CPU 数 | サーバー規模の把握に使います。Azure SQL Database ではホスト側 CPU 数を返すことがあります。 |
| `hyperthread_ratio` | 1 物理ソケットあたりの論理コア比 | ハイパースレッディングの前提を把握する補助列です。 |
| `socket_count` | 使用可能なソケット数 | 物理構成の理解に使います。 |
| `cores_per_socket` | ソケットあたりのコア数 | CPU トポロジの把握に使います。 |
| `numa_node_count` | 使用可能な NUMA ノード数 | NUMA や soft-NUMA を考える前提になります。 |
| `physical_memory_kb` | マシンの物理メモリ総量 | OS 全体の物理メモリ規模です。 |
| `virtual_memory_kb` | ユーザー モード仮想アドレス空間総量 | 仮想アドレス空間の把握に使います。 |
| `committed_kb` | SQL Server メモリ マネージャーが現在コミットしているメモリ | SQL Server が今どれだけメモリを使っているかを見る列です。 |
| `committed_target_kb` | SQL Server が使用可能と判断している目標メモリ | `committed_kb` より大きければ、まだメモリを増やそうとしていると読めます。 |
| `visible_target_kb` | SQL Server が直接見えている目標メモリ | 実務では `committed_target_kb` とほぼ同じ意味合いで読みます。 |
| `max_workers_count` | 作成可能な最大ワーカー数 | `THREADPOOL` 待機やワーカー不足を考える前提になります。 |
| `scheduler_count` | SQL Server プロセスで構成されたユーザー スケジューラ数 | 実際に要求を捌くスケジューラ数の把握に使います。 |
| `scheduler_total_count` | SQL Server スケジューラ総数 | 隠しスケジューラを含む全体像を見たいときの補助列です。 |
| `ms_ticks` | OS 起動後のミリ秒 | 他 DMV の時刻差分や uptime 解釈の補助に使えます。 |
| `sqlserver_start_time` | SQL Server 最終起動時刻 | 累積 DMV を読む前に必ず確認したい列です。 |
| `softnuma_configuration_desc` | soft-NUMA の構成状態 | 自動か手動か、無効かを確認できます。 |
| `sql_memory_model_desc` | SQL Server のメモリ モデル | `CONVENTIONAL`、`LOCK_PAGES`、`LARGE_PAGES` を確認できます。 |
| `virtual_machine_type_desc` | 仮想化環境かどうかの説明 | 仮想基盤上かどうかの把握に使います。 |
| `container_type_desc` | コンテナー実行形態 | コンテナー上の SQL Server かどうかを判定できます。 |

## 3. 読み方のコツ

メモリを見るときは、`committed_kb` と `committed_target_kb` を並べて読むのが基本です。前者は現在使っている量、後者は SQL Server が目標としている量なので、両者の差から「まだメモリを伸ばしたいのか、ほぼ落ち着いているのか」が見えます。[official]

また、待機統計や I/O 統計を読む前に `sqlserver_start_time` を確認すると、累積カウンターがどれくらいの期間を背負っているのかが分かります。再起動直後の累積値と、何週間も動いた後の累積値は、そのまま比較できません。

## 4. 注意点

Azure SQL Database では、`cpu_count` や `physical_memory_kb` が実際にそのデータベースへ割り当てられた上限ではなく、ホスト マシン側の値を返す場合があります。したがって、PaaS では「見えている物理値」と「そのデータベースが使える上限」を混同しない方が安全です。[official]

権限は、SQL Server 2022 以降では `VIEW SERVER PERFORMANCE STATE`、SQL Server 2019 以前と SQL Managed Instance では `VIEW SERVER STATE` が基本です。[official]

[official]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-os-sys-info-transact-sql?view=sql-server-ver17
