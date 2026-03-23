# Topic-09-Monitoring

## 学習ゴール

[[Azure用語集.md#Azure Monitor]] を中心に、メトリクス、ログ、APM、アラート、閉域監視を一枚絵で説明できるようにする。

## このTopicの全体像

この Topic では、Azure Monitor、Log Analytics Workspace、Application Insights、診断設定、AMPLS を扱う。  
判断軸は「何を集めるか」「どこへ送るか」「どう隔離するか」。

---

# 第1章 学習マップ

## 1.1 学習順序

1. Azure Monitor と Log Analytics の役割分担を理解する。
2. Application Insights でアプリ可観測性を押さえる。
3. VM / SQL / プラットフォーム ログの収集と送信先制約を学ぶ。
4. AMPLS と Private Link で閉域監視を仕上げる。

## 1.2 Azureリソース一覧

- Azure Monitor
- Log Analytics Workspace
- Azure Monitor Agent / DCR
- Application Insights
- Diagnostic Settings / Alerts
- Azure Monitor Private Link Scope (AMPLS)

---

# 第2章 Azureリソース解説

## Resource: Azure Monitor

### 概要

[[Azure用語集.md#Azure Monitor]] はメトリクス、ログ、アラート、可観測性を束ねる基盤。

### できること

- メトリクス収集
- ログ収集
- アラート
- Workbook / ダッシュボード
- 診断設定

### 技術仕様

- メトリクスとログを区別して考える。
- Activity Log、Resource Logs、Guest OS Logs の出どころが違う。
- Log Analytics を中心に KQL で分析する。

### SDK / API

- Azure Monitor REST API
- KQL
- Azure CLI / ARM

### 他サービスとの比較

- Azure Monitor vs Application Insights: 前者が基盤、後者がアプリ特化。
- Log Analytics vs Storage Archive: 分析用途か、保存用途かで分ける。

### どのようなときに使うか

- Azure 全体の運用監視を標準化したいとき
- メトリクスとログを同じ運用フローで扱いたいとき

### 関連シナリオ

- [[Scenarios/Scenario-AzureMonitor.md#log-analytics-workspace-へ-vm-ログを集約する]]
- [[Scenarios/Scenario-AzureMonitor.md#subscription-全体の健全性をアラートで監視する]]

### 主な出典

- [[Sources/Topic-12.md]]
- [[Sources/Azure Monitor _ Application Monitoring アーキテクチャ.md]]
- [[Sources/Azure環境の包括的モニタリング（Azure Monitor）.md]]
- [[Sources/Azure Monitor によるサブスクリプション全体の監視.md]]

## Resource: Log Analytics Workspace and AMA

### 概要

[[Azure用語集.md#Log Analytics Workspace]] は Monitor ログの中心保管先で、AMA と DCR で収集を制御する。

### できること

- VM ログ収集
- DCR によるリージョン別 / 種別別収集
- KQL 検索
- Sentinel や Workbook の土台

### 技術仕様

- Azure Monitor Agent が新しい標準エージェント。
- DCR で不要ログを減らしコストを抑える。
- テーブル設計を理解すると VM、SQL、セキュリティ ログの読み分けがしやすい。

### SDK / API

- Azure Monitor Agent / DCR API
- KQL

### 他サービスとの比較

- AMA vs 旧エージェント: 新規設計は AMA 前提で考える。
- Workspace vs Storage: 検索分析するなら Workspace。

### どのようなときに使うか

- VM ログを一元収集したいとき
- 収集粒度を環境別に制御したいとき

### 関連シナリオ

- [[Scenarios/Scenario-AzureMonitor.md#log-analytics-workspace-へ-vm-ログを集約する]]
- [[Scenarios/Scenario-AzureMonitor.md#ama-と-dcr-で不要ログを抑制する]]

### 主な出典

- [[Sources/Topic-12.md]]
- [[Sources/Azure Monitor Agent を利用したログ収集設.md]]
- [[Sources/Azure Monitor による Windows VM ログ集中監視設計.md]]
- [[Sources/Azure Monitor を使用した VM の包括的モニタリング（Metrics Alerts + Log Analytics）.md]]

## Resource: Application Insights

### 概要

[[Azure用語集.md#Application Insights]] はアプリケーション可観測性に特化した監視リソース。

### できること

- 応答時間測定
- 失敗率監視
- 依存関係追跡
- 分散トレーシング
- 可用性テスト

### 技術仕様

- App Service や Functions に統合しやすい。
- コード変更を最小化して APM を入れたいケースでも相性がよい。
- 依存先のボトルネック分析に使う。

### SDK / API

- Application Insights SDK / OpenTelemetry
- REST API

### 他サービスとの比較

- Application Insights vs Azure Monitor Metrics: アプリ内部可観測性まで要るなら Application Insights。

### どのようなときに使うか

- Web/API の応答悪化原因を追いたいとき
- 依存関係の遅延を可視化したいとき

### 関連シナリオ

- [[Scenarios/Scenario-AzureMonitor.md#app-service-へ-application-insights-を組み込む]]

### 主な出典

- [[Sources/Topic-12.md]]
- [[Sources/Azure App Service アプリケーション監視アーキテクチャ.md]]
- [[Sources/Azure App Service のパフォーマンス監視と依存関係トラッキング.md]]
- [[Sources/Azure Application Insights によるアプリケーションパフォーマンス監視.md]]

## Resource: Diagnostics, Alerts, and SQL Monitoring

### 概要

診断設定とアラートは「何をどこへ送るか」を決める運用設計の中核。

### できること

- リソース ログ送信
- Log Analytics / Storage / Event Hubs への分岐
- メトリクス アラート
- SQL の診断ログ設計

### 技術仕様

- 同じログカテゴリを複数 Workspace へ送る設計には制約がある。
- Storage は長期保管先として使いやすい。
- SQL 監視では保管先と保持期間を同時に考える。

### SDK / API

- Diagnostic settings API
- Azure Monitor Alerts API

### 他サービスとの比較

- Log Analytics vs Storage: 検索分析か、保管と再取り込みか。

### どのようなときに使うか

- SQL / PaaS ログの送信先を整理したいとき
- サブスクリプション全体へアラートを整備したいとき

### 関連シナリオ

- [[Scenarios/Scenario-AzureMonitor.md#sql-database-の診断ログ送信先を設計する]]
- [[Scenarios/Scenario-AzureMonitor.md#subscription-全体の健全性をアラートで監視する]]

### 主な出典

- [[Sources/Topic-12.md]]
- [[Sources/Azure SQL Database Diagnostics とログ保持設計.md]]
- [[Sources/Azure SQL Database の診断設定とログ送信先の制約.md]]
- [[Sources/Azureリソースの健全性とパフォーマンス監視におけるアラート通知.md]]

## Resource: Azure Monitor Private Link Scope (AMPLS)

### 概要

AMPLS は Azure Monitor リソースを Private Link 化するときの束ね役。

### できること

- Log Analytics / Monitor への閉域接続
- 複数 VNet からのプライベート監視アクセス

### 技術仕様

- 接続元 VNet 数と Private Endpoint 数は必ずしも一致しない。
- Workspace と AMPLS の関連付け、各 VNet からの Private Endpoint 設計を分けて考える。

### SDK / API

- Azure Monitor Private Link Scope API

### 他サービスとの比較

- Private Endpoint 単体 vs AMPLS: Azure Monitor 系の閉域化には AMPLS が前段に必要。

### どのようなときに使うか

- 監視トラフィックもパブリックを通したくないとき
- 複数 VNet から 1 つの Workspace へ閉域接続したいとき

### 関連シナリオ

- [[Scenarios/Scenario-AzureMonitor.md#ampls-で-monitor-通信をプライベート化する]]

### 主な出典

- [[Sources/Topic-12.md]]
- [[Sources/Azure Monitor Private Link Scope（AMPLS）設計ガイド.md]]

---

# 第3章 設計判断ガイド

## 3.1 収集先を選ぶとき

- 検索と相関分析をするなら Log Analytics。
- 長期保管や監査を優先するなら Storage も併用する。

## 3.2 アプリ監視を入れるとき

- パフォーマンスと依存関係を追うなら Application Insights。
- プラットフォーム全体は Azure Monitor で受ける。

## 3.3 閉域監視にするとき

- Monitor 系リソースは AMPLS を前提に設計する。
- VNet ごとの Private Endpoint と Workspace 側の関連付けを分けて考える。

---

# 第4章 関連シナリオ

- [[Scenarios/Scenario-AzureMonitor.md]]

