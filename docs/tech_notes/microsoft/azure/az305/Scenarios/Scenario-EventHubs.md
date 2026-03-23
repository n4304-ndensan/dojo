# Scenario: Azure Event Hubs

## シナリオ一覧

- Event Hubs + Functions + Cosmos DB でリアルタイム処理する
- Capture でコールドパス分析を作る
- Event Grid でインフラ運用を自動化する

## event-hubs--functions--cosmos-db-でリアルタイム処理する

シナリオ  
大量イベントを受けて即時に処理し、NoSQL へ保存する。

構成  
Producers  
↓  
Azure Event Hubs  
↓  
Azure Functions  
↓  
Azure Cosmos DB

ポイント  
- 高スループット入口とサーバーレス処理を組み合わせやすい
- テレメトリや監査イベント向けの典型構成

関連リソース  
Azure Event Hubs / Azure Functions / Azure Cosmos DB

出典  
- [[Sources/リアルタイムデータ処理アーキテクチャ（Azure Event Hubs + Azure Functions + Cosmos DB）.md]]
- [[Sources/Azure AD のユーザー作成およびロール割り当てイベントをキャプチャし、Azure Cosmos DB に保存する設計.md]]

## capture-でコールドパス分析を作る

シナリオ  
ストリームをそのまま Data Lake / Blob へ保管し、後段分析に回す。

構成  
Azure Event Hubs  
↓  
Capture  
↓  
ADLS Gen2 / Blob Storage

ポイント  
- ホットパスとコールドパスを分離できる
- Parquet 出力で後段分析しやすい

関連リソース  
Azure Event Hubs / Azure Data Lake Storage Gen2

出典  
- [[Sources/Event Hubs Capture を利用したコールドパス分析アーキテクチャ設計.md]]

## event-grid-でインフラ運用を自動化する

シナリオ  
Azure リソースの変更イベントを受けて Logic Apps や Functions を起動する。

構成  
Azure Resource Event  
↓  
Event Grid  
↓  
Logic Apps / Functions

ポイント  
- ポーリング不要で運用自動化できる
- Event Hubs とは用途が違う

関連リソース  
Azure Event Grid / Azure Logic Apps / Azure Functions

出典  
- [[Sources/Azure イベント駆動オートメーション.md]]
- [[Sources/AKS で Event Grid イベントを処理するためのスケーリング構成.md]]
