# Scenario: Azure Functions

## シナリオ一覧

- Storage Queue トリガーで自動スケールする
- Service Bus メッセージをコールドスタートなしで処理する
- Event Hubs イベントをリアルタイム処理する

## storage-queue-トリガーで自動スケールする

シナリオ  
キュー長に応じて Functions を自動スケールし、未使用時はゼロまで落とす。

構成  
Storage Queue  
↓  
Azure Functions (Consumption)

ポイント  
- 変動負荷に強い
- 実行ベース課金でコスト効率がよい

関連リソース  
Azure Functions / Azure Storage Queue

出典  
- [[Sources/Azure Functions のホスティングプランとスケーリング（Consumption Plan）.md]]

## service-bus-メッセージをコールドスタートなしで処理する

シナリオ  
Service Bus キューからの処理でコールドスタートを避ける。

構成  
Azure Service Bus  
↓  
Azure Functions Premium

ポイント  
- Premium で常時ウォームなインスタンスを持てる
- VNet や長時間実行要件にも寄せやすい

関連リソース  
Azure Functions / Azure Service Bus

出典  
- [[Sources/Azure FunctionsでService Busメッセージをコールドスタートなしで処理する設計.md]]
- [[Sources/Azure Functions Premium プランによる長時間イベント処理とスケーリング.md]]

## event-hubs-イベントをリアルタイム処理する

シナリオ  
Event Hubs に流れた大量イベントを Functions で処理し、後段へ保存する。

構成  
Event Hubs  
↓  
Azure Functions  
↓  
Cosmos DB / Storage / SQL

ポイント  
- サーバーレスでスケールしやすい
- 前処理やルーティング関数として使いやすい

関連リソース  
Azure Functions / Azure Event Hubs / Azure Cosmos DB

出典  
- [[Sources/リアルタイムデータ処理アーキテクチャ（Azure Event Hubs + Azure Functions + Cosmos DB）.md]]
- [[Sources/Azure AD のユーザー作成およびロール割り当てイベントをキャプチャし、Azure Cosmos DB に保存する設計.md]]
