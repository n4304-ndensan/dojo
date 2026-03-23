# Topic-07-Integration

## 学習ゴール

API 連携、ワークフロー、キュー、イベント ストリーミングを分けて理解し、Azure の疎結合設計パターンを説明できるようにする。

## このTopicの全体像

この Topic では、API Management、Logic Apps、Service Bus、Event Hubs、Event Grid を扱う。  
判断軸は「同期か非同期か」「メッセージかイベント通知か」「コードかコネクタか」。

---

# 第1章 学習マップ

## 1.1 学習順序

1. API Management で API の入口設計を押さえる。
2. Logic Apps でワークフロー統合を理解する。
3. Service Bus で信頼性の高い非同期メッセージングへ進む。
4. Event Hubs / Event Grid でストリームとイベント通知を区別する。

## 1.2 Azureリソース一覧

- Azure API Management
- Azure Logic Apps
- Azure Service Bus
- Azure Event Hubs
- Azure Event Grid

---

# 第2章 Azureリソース解説

## Resource: Azure API Management

### 概要

[[Azure用語集.md#Azure API Management]] は API の公開、保護、変換、監視を担うゲートウェイ。

### できること

- JWT 検証
- レート制限
- ポリシー変換
- VNet 接続
- 開発者ポータル

### 技術仕様

- Entra ID のトークン検証を入口で共通化できる。
- 内部モード / 外部モードでネットワーク設計が変わる。
- Logic Apps や内部 API の公開境界として使える。

### SDK / API

- Management REST API
- APIM policy XML
- Azure CLI / ARM

### 他サービスとの比較

- APIM vs Application Gateway: API ポリシーや開発者向け公開まで必要なら APIM。
- APIM vs 直接公開: 認証、制限、変換を共通化したいなら APIM。

### どのようなときに使うか

- 複数 API を一貫した入口で公開したいとき
- トークン検証やレート制限をアプリ外へ出したいとき

### 関連シナリオ

- [[Scenarios/Scenario-APIManagement.md#entra-id-トークンを-apim-で検証する]]
- [[Scenarios/Scenario-APIManagement.md#logic-apps-を-apim-越しに外部公開する]]

### 主な出典

- [[Sources/Topic-08.md]]
- [[Sources/Azure AD と API Management を用いた内部 API セキュリティ設計.md]]
- [[Sources/Azure API Management の外部モードと同一 VNet 内バックエンド接続の判断.md]]
- [[Sources/Azure API Management を利用した Logic Apps の外部公開.md]]

## Resource: Azure Logic Apps

### 概要

[[Azure用語集.md#Azure Logic Apps]] はコネクタ中心のワークフロー統合基盤。

### できること

- SaaS 連携
- オンプレミス データゲートウェイ経由接続
- スケジュール実行
- イベント駆動ワークフロー

### 技術仕様

- On-premises Data Gateway で社内 SQL Server や SAP に接続する。
- ノーコード/ローコードで接続フローを組める。
- APIM、Service Bus、Event Grid と組み合わせやすい。

### SDK / API

- Logic Apps workflow definition
- connectors
- ARM / CLI

### 他サービスとの比較

- Logic Apps vs Functions: コネクタ駆動と業務ワークフローなら Logic Apps。
- Logic Apps vs Data Factory: 業務連携中心なら Logic Apps、データパイプライン中心なら Data Factory。

### どのようなときに使うか

- SaaS と社内システムをつなぎたいとき
- 人手の多い業務フローを自動化したいとき

### 関連シナリオ

- [[Scenarios/Scenario-LogicApps.md#オンプレミス-sql-server-へ安全に接続する]]
- [[Scenarios/Scenario-LogicApps.md#sap-連携をスケジュール実行する]]

### 主な出典

- [[Sources/Topic-08.md]]
- [[Sources/Azure Logic Apps からオンプレミス SQL Server へ接続するアーキテクチャ.md]]
- [[Sources/Azure Logic Apps とオンプレミス SAP システムのスケジュール統合.md]]
- [[Sources/Azure Logic Apps によるワークフローオーケストレーション.md]]

## Resource: Azure Service Bus

### 概要

[[Azure用語集.md#Azure Service Bus]] は信頼性を重視するエンタープライズ メッセージ ブローカー。

### できること

- Queue
- Topic / Subscription
- セッション
- デッドレター
- トランザクション

### 技術仕様

- 順序保証が必要ならセッションを使う。
- 複数購読先に配るなら Topic / Subscription。
- Functions や Logic Apps のトリガーとして後段を疎結合化できる。

### SDK / API

- Azure Messaging Service Bus SDK
- AMQP

### 他サービスとの比較

- Service Bus vs Event Hubs: 信頼性重視の業務メッセージなら Service Bus。
- Service Bus vs Event Grid: イベント通知ではなくキューイングが必要なら Service Bus。

### どのようなときに使うか

- 注文、決済、在庫のような業務メッセージ連携
- 再試行や順序保証が必要なとき

### 関連シナリオ

- [[Scenarios/Scenario-ServiceBus.md#ecサイトの業務メッセージを非同期化する]]
- [[Scenarios/Scenario-ServiceBus.md#セッションで順序保証と並列処理を両立する]]

### 主な出典

- [[Sources/Topic-08.md]]
- [[Sources/Azure Service Bus を使用した非同期メッセージング.md]]
- [[Sources/Azure Service Bus のメッセージセッションによる順序保証と並列処理.md]]
- [[Sources/Azure FunctionsでService Busメッセージをコールドスタートなしで処理する設計.md]]

## Resource: Azure Event Hubs and Event Grid

### 概要

[[Azure用語集.md#Azure Event Hubs]] は高スループット ストリーミング、[[Azure用語集.md#Azure Event Grid]] はイベント通知ルーター。

### できること

- 大量イベント取り込み
- Capture
- Functions / Stream Analytics 連携
- Azure リソースイベント通知

### 技術仕様

- Event Hubs はテレメトリやログを高レートで受ける。
- Event Grid は状態変化通知をプッシュする。
- Capture で Blob / ADLS Gen2 にコールドパスを作れる。

### SDK / API

- Azure Messaging Event Hubs SDK
- Event Grid publish/subscribe API

### 他サービスとの比較

- Event Hubs vs Service Bus: ストリームか、業務メッセージか。
- Event Grid vs Event Hubs: リソースイベントの通知か、連続データ取り込みか。

### どのようなときに使うか

- IoT / ログ / リアルタイム分析の入口
- ストレージ作成、リソース変更などのイベント自動化

### 関連シナリオ

- [[Scenarios/Scenario-EventHubs.md#event-hubs--functions--cosmos-db-でリアルタイム処理する]]
- [[Scenarios/Scenario-EventHubs.md#capture-でコールドパス分析を作る]]
- [[Scenarios/Scenario-EventHubs.md#event-grid-でインフラ運用を自動化する]]

### 主な出典

- [[Sources/Topic-08.md]]
- [[Sources/リアルタイムデータ処理アーキテクチャ（Azure Event Hubs + Azure Functions + Cosmos DB）.md]]
- [[Sources/Event Hubs Capture を利用したコールドパス分析アーキテクチャ設計.md]]
- [[Sources/Azure イベント駆動オートメーション.md]]

---

# 第3章 設計判断ガイド

## 3.1 同期と非同期を分ける

- API 呼び出しの入口は APIM。
- 人や業務ワークフロー連携は Logic Apps。
- 確実に積んで処理したい業務メッセージは Service Bus。
- 大量ストリームは Event Hubs。
- リソース通知は Event Grid。

## 3.2 コード量で選ぶ

- コネクタ中心なら Logic Apps。
- カスタム処理中心なら Functions と組み合わせる。

## 3.3 メッセージの性質で選ぶ

- 順序保証や DLQ が必要なら Service Bus。
- 秒間大量イベントなら Event Hubs。

---

# 第4章 関連シナリオ

- [[Scenarios/Scenario-APIManagement.md]]
- [[Scenarios/Scenario-LogicApps.md]]
- [[Scenarios/Scenario-ServiceBus.md]]
- [[Scenarios/Scenario-EventHubs.md]]

