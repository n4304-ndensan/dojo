# Scenario: Azure Cosmos DB

## シナリオ一覧

- マルチリージョン読み取りを前提に整合性を設計する
- Event Hubs + Functions の処理結果を Cosmos DB へ保存する

## マルチリージョン読み取りを前提に整合性を設計する

シナリオ  
世界中のユーザーへ低遅延読み取りを提供しながら、整合性要件を満たす。

構成  
Global Clients  
↓  
Azure Cosmos DB  
↓  
Multi-region replicas

ポイント  
- Strong consistency と multi-write は両立しない前提で考える
- 一貫性、遅延、書き込みモデルの 3 つを同時に見る

関連リソース  
Azure Cosmos DB / Consistency Levels

出典  
- [[Sources/Azure Cosmos DB における一貫性モデルとマルチリージョン書き込み設計.md]]
- [[Sources/Azure Cosmos DB によるマルチリージョン高可用性アーキテクチャ.md]]

## event-hubs--functions-の処理結果を-cosmos-db-へ保存する

シナリオ  
Event Hubs で受けたイベントを Functions で処理し、NoSQL ストアへ格納する。

構成  
Azure Event Hubs  
↓  
Azure Functions  
↓  
Azure Cosmos DB

ポイント  
- サーバーレスなリアルタイム処理パターン
- 低レイテンシ書き込みとグローバル分散が相性よい

関連リソース  
Azure Event Hubs / Azure Functions / Azure Cosmos DB

出典  
- [[Sources/リアルタイムデータ処理アーキテクチャ（Azure Event Hubs + Azure Functions + Cosmos DB）.md]]
- [[Sources/Azure AD のユーザー作成およびロール割り当てイベントをキャプチャし、Azure Cosmos DB に保存する設計.md]]
