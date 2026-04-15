---
tags:
  - sqlserver
  - concurrency
  - index
status: current
updated: 2026-04-15
---

# SQL Server Concurrency

このフォルダは、Isolation Level、RCSI、SNAPSHOT、lock / blocking、version store といった同時実行制御の話題をまとめる入口です。性能評価や整合性判断に関わるノートをここへ集め、基礎ノートと診断ノートから切り離しています。

- 目的: 同時実行制御に関する比較ノートの置き場を揃え、RCSI 評価で参照する順序を明確にします。
- 想定読者: RCSI の導入可否や blocking の見え方を整理したい人。
- 前提知識: [[../Fundamentals/index|Fundamentals]] を先に読んでいると、用語の理解が早くなります。
- 関連文書: SQL Server 全体の入口は [[../index|SQL Server]]、基礎は [[../Fundamentals/index|Fundamentals]]、観測系は [[../Diagnostics/index|Diagnostics]] を参照してください。
- 正本性: このファイルは `Concurrency/` フォルダ全体の正本となる入口です。
- 利用場面: 分離レベルの比較、RCSI と SNAPSHOT の違い、lock / blocking の整理が必要なときに使います。

## 1. フォルダの目的

このフォルダでは、「何が競合し、どこまで一貫性を求めるか」を扱います。実装方式の差や概念の比較を主責務とし、具体的な観測クエリや DMV 個票は `Diagnostics/` に委譲します。

## 2. 文書一覧

| 文書 | 責務 | 正本性 | ステータス | 最終更新 |
|---|---|---|---|---|
| [[SQL Server の Lock と Blocking と Version Store の整理]] | lock / blocking / version store の差分整理 | 正本 | 現行 | 2026-04-15 |
| [[SQL Server の Isolation Level（分離レベル）詳細解説]] | Isolation Level 全体の体系解説 | 正本 | 現行 | 2026-04-15 |
| [[SQL Server Snapshot Isolation と RCSI と Read Committed の違い]] | RCSI、SNAPSHOT、READ COMMITTED の選び分け | 正本 | 現行 | 2026-04-14 |
| [[SQL Server で ADR ON のとき PVS は更新のたびに必ず増えるのか]] | ADR、PVS、in-row / off-row の関係整理 | 正本 | 現行 | 2026-04-15 |

## 3. 読み順ガイド

1. [[SQL Server の Lock と Blocking と Version Store の整理]]: 何が競合し、何が誤解されやすいかを先に押さえます。
2. [[SQL Server で ADR ON のとき PVS は更新のたびに必ず増えるのか]]: `pvs_size_mb` と実際の versioning の関係を整理します。
3. [[SQL Server の Isolation Level（分離レベル）詳細解説]]: 分離レベル全体を体系的に整理します。
4. [[SQL Server Snapshot Isolation と RCSI と Read Committed の違い]]: 実際の選び分けに落とし込みます。

## 4. 文書責務の網羅性チェック

| 必要な責務 | 対応文書 | 状態 |
|---|---|---|
| lock / blocking / version store の概念整理 | [[SQL Server の Lock と Blocking と Version Store の整理]] | 存在 |
| ADR と PVS と in-row / off-row の整理 | [[SQL Server で ADR ON のとき PVS は更新のたびに必ず増えるのか]] | 存在 |
| Isolation Level 全体の体系化 | [[SQL Server の Isolation Level（分離レベル）詳細解説]] | 存在 |
| RCSI と SNAPSHOT の比較判断 | [[SQL Server Snapshot Isolation と RCSI と Read Committed の違い]] | 存在 |
| 実測に使う観測オブジェクト | [Diagnostics](../Diagnostics/index.md) | 他フォルダで対応 |

## 5. 更新ルール

- 分離レベル、row versioning、lock 方式の比較や判断ノートはこのフォルダに置きます。
- 個別 DMV や XE の使い方は [[../Diagnostics/index|Diagnostics]] に置きます。
- 環境依存で Managed Instance にしか当てはまらない話は [[../ManagedInstance/index|ManagedInstance]] に置きます。
