---
tags:
  - sqlserver
  - diagnostics
  - index
status: current
updated: 2026-04-15
---

# SQL Server Diagnostics

このフォルダは、DMV、DMF、XE など観測と切り分けに使うノートをまとめる入口です。概念比較ではなく、「何を判断するためにどのオブジェクトを見るか」を主責務にしています。

- 目的: 観測用ノートの入口を一つにまとめ、個別 DMV と上位一覧の関係を明確にします。
- 想定読者: blocking、待機、tempdb、ログ使用量、XE による追跡を調べたい人。
- 前提知識: [[../Fundamentals/index|Fundamentals]] と [[../Concurrency/index|Concurrency]] を先に読んでおくと、観測値の意味を理解しやすくなります。
- 関連文書: SQL Server 全体の入口は [[../index|SQL Server]]、Azure SQL Managed Instance 固有の補足は [[../ManagedInstance/index|ManagedInstance]] を参照してください。
- 正本性: このファイルは `Diagnostics/` フォルダ全体の正本となる入口です。
- 利用場面: どの観測オブジェクトから見るべきか、または個別 DMV のノートをどこへ置くべきかを判断したいときに使います。

## 1. フォルダの目的

診断系ノートは、上位の一覧ノートと個別ノートが混ざると探しにくくなります。このフォルダでは、一覧系と個票系を分離し、`SQL Server DMV一覧` から `DMV/index.md` と各個票へ辿れる構造にしています。

## 2. 文書一覧

| 文書 | 責務 | 正本性 | ステータス | 最終更新 |
|---|---|---|---|---|
| [[SQL Server DMV一覧]] | SQL Server の観測オブジェクト一覧と使い分け | 正本 | 現行 | 2026-04-14 |
| [DMV 個票一覧](DMV/index.md) | 個別 DMV / DMF ノートの入口 | 正本 | 現行 | 2026-04-15 |
| [XEでSQL実行内容を確認するための整理](XE_SQLイベント_比較.md) | XE による SQL 実行内容の調査メモ | 参考資料 | 現行 | 2026-04-14 |

## 3. 読み順ガイド

1. [[SQL Server DMV一覧]]: まず何を判断するためにどのオブジェクトを見るかを把握します。
2. [[DMV/index|DMV 個票一覧]]: 必要な DMV / DMF の個票に進みます。
3. [[XE_SQLイベント_比較|XEでSQL実行内容を確認するための整理]]: 実行イベントを追跡したいときに参照します。

## 4. 文書責務の網羅性チェック

| 必要な責務 | 対応文書 | 状態 |
|---|---|---|
| 観測オブジェクトの一覧 | [[SQL Server DMV一覧]] | 存在 |
| 個別 DMV / DMF の個票 | [DMV 個票一覧](DMV/index.md) | 存在 |
| XE によるイベント追跡 | [XEでSQL実行内容を確認するための整理](XE_SQLイベント_比較.md) | 存在 |
| Query Store 専用の独立ガイド | — | 未作成。必要になった時点で追加します。 |

## 5. 更新ルール

- 個別 DMV / DMF のノートは `DMV/` 配下に置きます。
- 何を見るためにどのオブジェクトを使うかという一覧ノートは、このフォルダ直下に置きます。
- Managed Instance 固有ビューの詳細は [[../ManagedInstance/index|ManagedInstance]] に置きます。
