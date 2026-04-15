---
aliases:
  - ManagedInstance ドキュメントガイド
tags:
  - sqlserver
  - managed-instance
  - index
status: current
updated: 2026-04-15
---

# Azure SQL Managed Instance

このフォルダは、Azure SQL Managed Instance 固有の前提、観測指標、評価設計だけを集める入口です。SQL Server 全般に共通する基礎は上位フォルダの `Fundamentals/` と `Concurrency/` に残し、このフォルダでは Managed Instance で解釈が変わる部分に集中します。

- 目的: Managed Instance 固有ノートの責務を明確にし、上位の共通知識と混ざらない状態を保ちます。
- 想定読者: Azure SQL Managed Instance で RCSI や観測設計を整理したい人。
- 前提知識: [[../Fundamentals/index|Fundamentals]] と [[../Concurrency/index|Concurrency]] の基礎を先に押さえていると読みやすくなります。
- 関連文書: SQL Server 全体の入口は [[../index|SQL Server]]、診断系ノートは [[../Diagnostics/index|Diagnostics]] を参照してください。
- 正本性: このファイルは `ManagedInstance/` フォルダ全体の正本となる入口です。
- 利用場面: Azure SQL Managed Instance 固有の前提や評価設計だけをまとめて見たいときに使います。

## 1. フォルダの目的

Managed Instance では、ADR 常時有効、PVS、`sys.server_resource_stats` の粒度と遅延など、SQL Server 共通ノートだけでは吸収しきれない前提があります。このフォルダは、それらの環境依存要素だけをまとめて、製品共通の基礎ノートと切り分けるために存在します。

## 2. 文書一覧

| 文書 | 責務 | 正本性 | ステータス | 最終更新 |
|---|---|---|---|---|
| [[Azure SQL Managed Instance における RCSI 評価の前提整理]] | Managed Instance 評価の入口と前提整理 | 正本 | 現行 | 2026-04-14 |
| [[Azure SQL Managed Instance における RCSI 評価スクリプトの測定項目整理]] | 測定項目の意味と読み分けの整理 | 参考資料 | 現行 | 2026-04-15 |
| [[Azure SQL Managed Instance における RCSI 評価スクリプト設計ガイド]] | 評価スクリプト設計原則と参照実装 | 正本 | 現行 | 2026-04-14 |
| [[sys.server_resource_stats 詳細解説]] | `sys.server_resource_stats` 単体の詳説 | 参考資料 | 現行 | 2026-04-14 |

## 3. 読み順ガイド

Managed Instance 向けの RCSI 評価を整理するなら、次の順番で読むと流れがつながります。

1. [[Azure SQL Managed Instance における RCSI 評価の前提整理]]: まず Managed Instance で固定すべき前提を押さえます。
2. [[Azure SQL Managed Instance における RCSI 評価スクリプトの測定項目整理]]: どの列を何の責務で見るかを切り分けます。
3. [[Azure SQL Managed Instance における RCSI 評価スクリプト設計ガイド]]: 評価スクリプトの設計原則と参照実装を確認します。
4. [[sys.server_resource_stats 詳細解説]]: 個別ビューの意味を掘り下げたいときに参照します。

## 4. 文書責務の網羅性チェック

| 必要な責務 | 対応文書 | 状態 |
|---|---|---|
| Managed Instance 固有前提の入口 | [[Azure SQL Managed Instance における RCSI 評価の前提整理]] | 存在 |
| 測定項目の意味整理 | [[Azure SQL Managed Instance における RCSI 評価スクリプトの測定項目整理]] | 存在 |
| 評価設計と参照実装 | [[Azure SQL Managed Instance における RCSI 評価スクリプト設計ガイド]] | 存在 |
| サービスメーター個票 | [[sys.server_resource_stats 詳細解説]] | 存在 |
| 障害時の切り戻しや運用手順 | — | 未作成。必要になった時点で追加します。 |

## 5. 更新ルール

- Managed Instance でしか成立しない注意点だけをこのフォルダに置きます。
- SQL Server 全般に共通する概念説明は上位の [[../Fundamentals/index|Fundamentals]] または [[../Concurrency/index|Concurrency]] に寄せます。
- 新しいトップレベル文書を追加したら、この `index.md` と上位の [[../index|SQL Server]] を両方更新します。
