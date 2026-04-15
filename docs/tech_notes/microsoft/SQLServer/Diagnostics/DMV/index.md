---
tags:
  - sqlserver
  - dmv
  - index
status: current
updated: 2026-04-15
---

# SQL Server DMV 個票一覧

このフォルダは、個別の DMV / DMF ノートをまとめる入口です。上位の [[../SQL Server DMV一覧]] が「何を見るか」を整理し、このフォルダでは「各列をどう読むか」を個票ごとに扱います。

- 目的: 個別 DMV / DMF ノートの所在と役割を一覧化し、必要な個票へすぐ辿れるようにします。
- 想定読者: 具体的な DMV / DMF の列や読み方を確認したい人。
- 前提知識: 先に [[../SQL Server DMV一覧]] を読んでおくと、個票を引く理由が明確になります。
- 関連文書: 診断系全体の入口は [[../index|Diagnostics]]、SQL Server 全体の入口は [[../../index|SQL Server]] を参照してください。
- 正本性: このファイルは `Diagnostics/DMV/` フォルダ全体の正本となる入口です。
- 利用場面: どの DMV 個票があるかを把握したいとき、または新しい個票を追加したいときに使います。

## 1. フォルダの目的

個票ノートは一つずつ読むと分かりやすい反面、数が増えると探しにくくなります。この `index.md` は、上位一覧と個票の橋渡しを担います。

## 2. 文書一覧

| 文書 | 責務 | 正本性 | ステータス | 最終更新 |
|---|---|---|---|---|
| [[sys.dm_exec_requests]] | 実行中リクエストと blocking の確認 | 正本 | 現行 | 2026-04-13 |
| [[sys.dm_os_wait_stats]] | 待機統計の全体傾向確認 | 正本 | 現行 | 2026-04-14 |
| [[sys.dm_os_waiting_tasks]] | 現在待機中タスクの詳細確認 | 正本 | 現行 | 2026-04-09 |
| [[sys.dm_os_tasks]] | worker / task 単位の補助確認 | 正本 | 現行 | 2026-04-09 |
| [[sys.dm_os_sys_info]] | CPU、scheduler、起動情報の確認 | 正本 | 現行 | 2026-04-14 |
| [[sys.dm_os_sys_memory]] | OS メモリ状態の確認 | 正本 | 現行 | 2026-04-14 |
| [[sys.dm_io_virtual_file_stats]] | ファイル単位 I/O の確認 | 正本 | 現行 | 2026-04-14 |
| [[sys.dm_db_file_space_usage]] | tempdb / version store の容量確認 | 正本 | 現行 | 2026-04-09 |
| [[sys.dm_db_session_space_usage]] | session 単位の領域使用量確認 | 正本 | 現行 | 2026-04-09 |
| [[sys.dm_db_task_space_usage]] | task 単位の領域使用量確認 | 正本 | 現行 | 2026-04-09 |
| [[sys.dm_db_log_space_usage]] | ログ使用量の確認 | 正本 | 現行 | 2026-04-09 |

## 3. 読み順ガイド

症状別に、次の順番で個票へ入ると迷いにくくなります。

1. 実行中の詰まりを見たいときは [[sys.dm_exec_requests]] と [[sys.dm_os_waiting_tasks]] を先に読みます。
2. 全体的な待機傾向を把握したいときは [[sys.dm_os_wait_stats]] を読みます。
3. tempdb、version store、ログ圧力を見たいときは [[sys.dm_db_file_space_usage]]、[[sys.dm_db_session_space_usage]]、[[sys.dm_db_task_space_usage]]、[[sys.dm_db_log_space_usage]] を参照します。
4. CPU、メモリ、I/O の前提を押さえたいときは [[sys.dm_os_sys_info]]、[[sys.dm_os_sys_memory]]、[[sys.dm_io_virtual_file_stats]] を参照します。

## 4. 文書責務の網羅性チェック

| 必要な責務 | 対応文書 | 状態 |
|---|---|---|
| 実行中リクエストの観測 | [[sys.dm_exec_requests]] | 存在 |
| 待機の全体統計 | [[sys.dm_os_wait_stats]] | 存在 |
| 現在待機中タスクの観測 | [[sys.dm_os_waiting_tasks]] | 存在 |
| CPU / メモリの前提確認 | [[sys.dm_os_sys_info]] / [[sys.dm_os_sys_memory]] | 存在 |
| tempdb / ログの領域確認 | [[sys.dm_db_file_space_usage]] / [[sys.dm_db_log_space_usage]] | 存在 |

## 5. 更新ルール

- 個票ノートは「その DMV / DMF 単体の読み方」に責務を限定します。
- 何を見るためにその個票を使うかという上位説明は [[../SQL Server DMV一覧]] へ寄せます。
- Managed Instance 固有ビューはこのフォルダへ混ぜず、[[../../ManagedInstance/index|ManagedInstance]] に置きます。
