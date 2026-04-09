# sys.dm_os_sys_memory

この DMV は、SQL Server プロセス内部ではなく、オペレーティング システム側から見たメモリ状態を返します。SQL Server が今どれくらいメモリを使っているかを見る `sys.dm_os_sys_info` と違い、こちらは OS 全体に物理メモリの余裕があるか、メモリ不足シグナルが立っているかを判断するための DMV です。[official]

SQL Server のメモリ問題は、SQL Server 自身の使用量だけではなく、OS 側に空きがあるかどうかでも見え方が変わります。そのため、SQL Server 内部のメモリと OS 全体のメモリを分けて観測するのが基本です。

## 1. 主な使いどころ

この DMV は、次のような問いに向いています。

- Windows またはホスト OS 全体でメモリ圧力が起きているか。
- SQL Server 以外も含めた物理メモリ不足の有無を確認したいとき。
- 高メモリ シグナルと低メモリ シグナルの状態を知りたいとき。

## 2. 列の整理

この DMV は OS 側の状態を見る列で構成されています。

| 列 | 意味 | 実務での見方 |
| --- | --- | --- |
| `total_physical_memory_kb` | OS で利用可能な物理メモリ総量 | サーバー全体のメモリ規模です。 |
| `available_physical_memory_kb` | 使用可能な物理メモリ量 | ここが小さいと OS 全体のメモリ圧力を疑います。 |
| `total_page_file_kb` | OS が報告するコミット上限 | ページ ファイルを含めたコミット限界の把握に使います。 |
| `available_page_file_kb` | 未使用のページ ファイル量 | コミット余力の補助指標です。 |
| `system_cache_kb` | システム キャッシュ総量 | OS のキャッシュ状況をざっくり見ます。 |
| `kernel_paged_pool_kb` | ページ カーネル プール量 | OS カーネル側のメモリ利用を補足します。 |
| `kernel_nonpaged_pool_kb` | 非ページ カーネル プール量 | ドライバーやカーネル常駐領域の圧力確認に使います。 |
| `system_high_memory_signal_state` | 高メモリ シグナル状態 | `1` なら OS は十分なメモリ余裕があると判断しています。 |
| `system_low_memory_signal_state` | 低メモリ シグナル状態 | `1` なら OS 側でメモリ不足が発生しています。 |
| `system_memory_state_desc` | メモリ状態の説明文 | 高シグナル・低シグナルの組み合わせを説明付きで返します。 |

## 3. 読み方のコツ

最初に見るべきは `available_physical_memory_kb` と、二つのシグナル列です。`system_low_memory_signal_state = 1` であれば、SQL Server だけでなく OS 全体としてメモリ不足圧力を受けている可能性が高いです。逆に `system_high_memory_signal_state = 1` なら、少なくとも OS は物理メモリに余裕があると判断しています。[official]

`system_memory_state_desc` は、二つのシグナル列の読み替え用として便利です。高シグナルのみなら余裕あり、低シグナルのみなら不足、両方 0 なら安定状態、両方 1 に見える場合は遷移中と解釈します。[official]

## 4. 注意点

この DMV は SQL Server プロセス固有の使用量ではなく、OS 全体の状態を返します。そのため、SQL Server が大量メモリを消費しているのか、別プロセスの影響なのかは、これだけでは確定できません。SQL Server 自身のメモリ量を見るには `sys.dm_os_sys_info` と併用するのが基本です。[official]

権限は、SQL Server 2022 以降では `VIEW SERVER PERFORMANCE STATE`、それ以前では `VIEW SERVER STATE` が必要です。[official]

[official]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-os-sys-memory-transact-sql?view=sql-server-ver17
