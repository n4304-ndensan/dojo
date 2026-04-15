---
tags:
  - sqlserver
  - fundamentals
  - index
status: current
updated: 2026-04-15
---

# SQL Server Fundamentals

このフォルダは、SQL Server を理解するための基礎ノートをまとめる入口です。製品全体像、学習の起点、名前解決のような共通知識をここに集約し、同時実行制御や診断系ノートとは分けて管理します。

- 目的: SQL Server の共通知識を一か所から辿れるようにし、初見者が迷わず読み始められるようにします。
- 想定読者: SQL Server を学び始めた人、RCSI 評価の前に土台を固めたい人。
- 前提知識: 特になし。必要に応じて SQL の基本文法だけ分かっていれば十分です。
- 関連文書: 上位入口は [[../index|SQL Server]]、同時実行制御は [[../Concurrency/index|Concurrency]]、診断は [[../Diagnostics/index|Diagnostics]] を参照してください。
- 正本性: このファイルは `Fundamentals/` フォルダ全体の正本となる入口です。
- 利用場面: まずどこから学ぶべきか、または製品共通知識のノートをどこへ置くべきかを判断したいときに使います。

## 1. フォルダの目的

このフォルダでは、SQL Server の全体像と共通知識を扱います。特定の isolation level や DMV の個票ではなく、製品理解の土台になるノートを置く場所です。

## 2. 文書一覧

| 文書                                            | 責務                            | 正本性  | ステータス | 最終更新       |
| --------------------------------------------- | ----------------------------- | ---- | ----- | ---------- |
| [[SQL Server 入門から始める RCSI 性能評価研修]]            | 学習の起点となる主教材                   | 正本   | 現行    | 2026-04-15 |
| [[SQL Server の全体像 詳細解説]]                      | Database Engine 全体像の深掘り       | 参考資料 | 現行    | 2026-04-14 |
| [[SQL Server の User、Schema、tempdb.sys の名前解決]] | 名前解決、schema、system object の整理 | 参考資料 | 現行    | 2026-04-14 |

## 3. 読み順ガイド

1. [[SQL Server 入門から始める RCSI 性能評価研修]]: 学習の起点として全体像をつかみます。
2. [[SQL Server の全体像 詳細解説]]: Query Processor、Storage Engine、tempdb、DMV のつながりを深掘りします。
3. [[SQL Server の User、Schema、tempdb.sys の名前解決]]: スキーマと名前解決の混乱を整理します。

## 4. 文書責務の網羅性チェック

| 必要な責務 | 対応文書 | 状態 |
|---|---|---|
| 入門教材 | [[SQL Server 入門から始める RCSI 性能評価研修]] | 存在 |
| 製品全体像の深掘り | [[SQL Server の全体像 詳細解説]] | 存在 |
| 名前解決とオブジェクト境界の整理 | [[SQL Server の User、Schema、tempdb.sys の名前解決]] | 存在 |
| 分離レベルや RCSI の体系整理 | [Concurrency](../Concurrency/index.md) | 他フォルダで対応 |

## 5. 更新ルール

- SQL Server 全般で共通する概念ノートはこのフォルダに置きます。
- RCSI、SNAPSHOT、lock / blocking のような同時実行制御の話は [[../Concurrency/index|Concurrency]] に置きます。
- DMV や XE など観測オブジェクト中心の話は [[../Diagnostics/index|Diagnostics]] に置きます。
