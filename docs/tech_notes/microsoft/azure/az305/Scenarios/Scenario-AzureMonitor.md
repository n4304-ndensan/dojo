# Scenario: Azure Monitor

## シナリオ一覧

- Log Analytics Workspace へ VM ログを集約する
- AMA と DCR で不要ログを抑制する
- App Service へ Application Insights を組み込む
- SQL Database の診断ログ送信先を設計する
- Subscription 全体の健全性をアラートで監視する
- AMPLS で Monitor 通信をプライベート化する

## log-analytics-workspace-へ-vm-ログを集約する

シナリオ  
複数 VM の OS ログを 1 つの Workspace へ集約し、KQL で分析する。

構成  
VMs  
↓  
Azure Monitor Agent  
↓  
Log Analytics Workspace

ポイント  
- 分析基盤を一元化できる
- テーブル設計の理解が重要

関連リソース  
Azure Monitor / Log Analytics Workspace / AMA

出典  
- [[Sources/Azure Monitor による Windows VM ログ集中監視設計.md]]
- [[Sources/Azure Monitor を使用した VM の包括的モニタリング（Metrics Alerts + Log Analytics）.md]]

## ama-と-dcr-で不要ログを抑制する

シナリオ  
収集対象を絞って、Workspace のコストとノイズを抑える。

構成  
Azure Monitor Agent  
↓  
Data Collection Rule  
↓  
Log Analytics Workspace

ポイント  
- まず全部集める設計はコストが高い
- リージョンや環境ごとに DCR を分ける発想が必要

関連リソース  
Azure Monitor Agent / DCR

出典  
- [[Sources/Azure Monitor Agent を利用したログ収集設.md]]

## app-service-へ-application-insights-を組み込む

シナリオ  
Web アプリの性能劣化や依存関係遅延を可視化する。

構成  
Azure App Service  
↓  
Application Insights  
↓  
Azure Monitor

ポイント  
- 失敗率、応答時間、依存関係追跡を一体で見られる
- コード変更最小で入れられるケースが多い

関連リソース  
Application Insights / Azure App Service

出典  
- [[Sources/Azure App Service アプリケーション監視アーキテクチャ.md]]
- [[Sources/Azure App Service のパフォーマンス監視と依存関係トラッキング.md]]

## sql-database-の診断ログ送信先を設計する

シナリオ  
SQL Database のログを Workspace と Storage へ適切に送る。

構成  
Azure SQL Database  
↓  
Diagnostic Settings  
↓  
Log Analytics / Storage

ポイント  
- 複数送信先の制約を理解しておく必要がある
- 検索用と保管用を分離すると整理しやすい

関連リソース  
Azure SQL Database / Azure Monitor

出典  
- [[Sources/Azure SQL Database Diagnostics とログ保持設計.md]]
- [[Sources/Azure SQL Database の診断設定とログ送信先の制約.md]]

## subscription-全体の健全性をアラートで監視する

シナリオ  
サブスクリプションや主要リソースの状態悪化を早期検知する。

構成  
Metrics / Activity Logs  
↓  
Azure Monitor Alerts  
↓  
Notification / Action Group

ポイント  
- 基盤監視は個別リソースごとの後追いより全体像が重要

関連リソース  
Azure Monitor / Alerts / Action Groups

出典  
- [[Sources/Azure Monitor によるサブスクリプション全体の監視.md]]
- [[Sources/Azureリソースの健全性とパフォーマンス監視におけるアラート通知.md]]

## ampls-で-monitor-通信をプライベート化する

シナリオ  
監視通信も Microsoft バックボーン内に閉じる。

構成  
VNet  
↓  
Private Endpoint  
↓  
AMPLS  
↓  
Workspace

ポイント  
- Workspace と VNet 側の設計を分けて考える
- VNet 数と AMPLS 数は必ずしも一致しない

関連リソース  
AMPLS / Log Analytics Workspace / Private Endpoint

出典  
- [[Sources/Azure Monitor Private Link Scope（AMPLS）設計ガイド.md]]
