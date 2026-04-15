---
aliases:
  - SQL Server ドキュメントガイド
tags:
  - sqlserver
  - index
status: current
updated: 2026-04-15
---

# SQL Server

このフォルダは、SQL Server の学習ノートを「基礎理解」「同時実行制御」「診断・観測」「Azure SQL Managed Instance 固有補足」に分けて管理するための入口です。トップレベルにはカテゴリごとの `index.md` だけを置き、個別ノートは責務に応じたサブフォルダへ寄せています。

- 目的: SQL Server 関連ノートの読み順と置き場を明確にし、文書追加時の迷いを減らします。
- 想定読者: SQL Server を学び始めた人、RCSI や blocking を整理したい人、診断用ノートの入口を探している人。
- 前提知識: SQL の基本文法とトランザクションの概念を薄く知っていると読みやすいですが、入門ノートから読み始めれば前提は最小限で構いません。
- 関連文書: 基礎は [[Fundamentals/index|Fundamentals]]、同時実行制御は [[Concurrency/index|Concurrency]]、診断は [[Diagnostics/index|Diagnostics]]、Azure SQL Managed Instance 固有事項は [[ManagedInstance/index|ManagedInstance]] を参照してください。
- 正本性: このファイルは `SQLServer/` フォルダ全体の正本となる入口です。
- 利用場面: どこから読むべきか、または新しいノートをどこへ置くべきかを判断したいときに使います。

## 1. フォルダの目的

以前の `SQLServer/` は、入門資料、分離レベルの比較、DMV 個票、Managed Instance 固有メモが同じ階層に並んでいました。そのままでも読めますが、入口が一つに定まらず、AI も人も「最初に何を読むか」「新しいノートをどこへ置くか」を毎回考える必要がありました。

現在は、役割ごとにサブフォルダを分けています。製品共通の基礎は `Fundamentals/`、RCSI や lock / blocking は `Concurrency/`、DMV と XE は `Diagnostics/`、Azure SQL Managed Instance 固有事項は `ManagedInstance/` で扱います。

## 2. 文書一覧

| 文書 | 責務 | 正本性 | ステータス | 最終更新 |
|---|---|---|---|---|
| [Fundamentals](Fundamentals/index.md) | SQL Server 基礎理解と学習導線の入口 | 正本 | 現行 | 2026-04-15 |
| [Concurrency](Concurrency/index.md) | 分離レベル、RCSI、lock / blocking の整理入口 | 正本 | 現行 | 2026-04-15 |
| [Diagnostics](Diagnostics/index.md) | DMV、XE、観測リファレンスの入口 | 正本 | 現行 | 2026-04-15 |
| [ManagedInstance](ManagedInstance/index.md) | Azure SQL Managed Instance 固有ノートの入口 | 正本 | 現行 | 2026-04-15 |

## 3. 読み順ガイド

SQL Server をこれから学ぶ場合は、次の順番で読むと流れがつながります。

1. [[Fundamentals/index|Fundamentals]]: 製品全体像と学習の起点を押さえます。
2. [[Concurrency/index|Concurrency]]: RCSI、SNAPSHOT、lock / blocking の違いを整理します。
3. [[Diagnostics/index|Diagnostics]]: どの観測オブジェクトを使うかを把握します。
4. [[ManagedInstance/index|ManagedInstance]]: Azure SQL Managed Instance を対象にするときだけ読み進めます。

## 4. 文書責務の網羅性チェック

SQL Server 学習ノートとして最低限ほしい責務と、現状の対応先を整理します。

| 必要な責務 | 対応文書 | 状態 |
|---|---|---|
| 基礎理解の入口 | [Fundamentals](Fundamentals/index.md) | 存在 |
| 同時実行制御と分離レベルの整理 | [Concurrency](Concurrency/index.md) | 存在 |
| 観測・診断の入口 | [Diagnostics](Diagnostics/index.md) | 存在 |
| Azure SQL Managed Instance 固有の補足 | [ManagedInstance](ManagedInstance/index.md) | 存在 |
| 用語集や FAQ の横断入口 | — | 未作成。必要になった時点で追加します。 |

## 5. 分類軸

このフォルダでは、次の分類軸でノートを置き分けます。

- `Fundamentals/`: SQL Server 全般で共通する基礎理解、全体像、名前解決など。
- `Concurrency/`: Isolation Level、RCSI、SNAPSHOT、lock / blocking、version store など。
- `Diagnostics/`: DMV、XE、観測や切り分けのためのリファレンス。
- `ManagedInstance/`: Azure SQL Managed Instance 固有の前提、評価設計、サービスメーター解説。

## 6. 更新ルール

この構成を維持するため、追加時は次のルールで揃えます。

- SQL Server 全般に共通する基礎ノートは `Fundamentals/` に置きます。
- 同時実行制御や分離レベルの比較ノートは `Concurrency/` に置きます。
- DMV / DMF の個票や観測用メモは `Diagnostics/` に置き、個票は `Diagnostics/DMV/` に寄せます。
- Azure SQL Managed Instance 固有の内容は `ManagedInstance/` に置きます。
- トップレベルには個別ノートを増やさず、カテゴリ入口の `index.md` だけを維持します。
