# Scenario: Azure Service Bus

## シナリオ一覧

- EC サイトの業務メッセージを非同期化する
- セッションで順序保証と並列処理を両立する

## ecサイトの業務メッセージを非同期化する

シナリオ  
注文、決済、在庫、配送の各サービスを疎結合にする。

構成  
Order Service  
↓  
Azure Service Bus Queue / Topic  
↓  
Payment / Inventory / Shipping

ポイント  
- メッセージ永続化と再試行に強い
- 業務メッセージの信頼性を担保できる

関連リソース  
Azure Service Bus / Azure Functions / Logic Apps

出典  
- [[Sources/Azure Service Bus を使用した非同期メッセージング.md]]

## セッションで順序保証と並列処理を両立する

シナリオ  
顧客や注文単位では順序を保ちつつ、全体では並列処理する。

構成  
Producer  
↓  
Service Bus Session  
↓  
Multiple Consumers

ポイント  
- セッション ID を軸に順序を閉じ込める
- 並列性と順序保証のバランスを取りやすい

関連リソース  
Azure Service Bus / Sessions

出典  
- [[Sources/Azure Service Bus のメッセージセッションによる順序保証と並列処理.md]]
