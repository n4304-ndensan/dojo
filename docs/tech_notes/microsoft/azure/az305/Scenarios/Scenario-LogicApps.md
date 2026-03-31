# Scenario: Azure Logic Apps

## シナリオ一覧

- オンプレミス SQL Server へ安全に接続する
- SAP 連携をスケジュール実行する

## オンプレミス-sql-server-へ安全に接続する

シナリオ  
Logic Apps からインターネット非公開の SQL Server へデータを書き込む。

構成  
Azure Logic Apps  
↓  
On-premises Data Gateway  
↓  
SQL Server

ポイント  
- ゲートウェイ経由で安全に接続できる
- VPN や公開 DB を前提にしなくてよい

関連リソース  
Azure Logic Apps / On-premises Data Gateway / SQL Server

出典  
- [[Sources/Azure Logic Apps からオンプレミス SQL Server へ接続するアーキテクチャ.md]]

## sap-連携をスケジュール実行する

シナリオ  
定時バッチや業務ワークフローとして SAP とクラウド サービスをつなぐ。

構成  
Azure Logic Apps  
↓  
Connector / Gateway  
↓  
On-prem SAP

ポイント  
- スケジュール実行とコネクタ活用が中心
- 業務フロー自動化に向く

関連リソース  
Azure Logic Apps / SAP Connector

出典  
- [[Sources/Azure Logic Apps とオンプレミス SAP システムのスケジュール統合.md]]
- [[Sources/Azure Logic Apps によるワークフローオーケストレーション.md]]
