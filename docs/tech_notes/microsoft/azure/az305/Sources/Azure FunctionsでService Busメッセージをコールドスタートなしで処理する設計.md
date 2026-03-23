[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Functions]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Service Bus]]
# Azure FunctionsでService Busメッセージをコールドスタートなしで処理する設計

## 1. 背景（シナリオ）

あるシステムでは、Azure Service Bus のキューに送信されたメッセージを Azure Functions で処理する必要があります。  
この処理はイベント駆動型で自動的に実行される必要があります。

また、メッセージ到着時の遅延を最小化するため、**コールドスタート（Cold Start）が発生しない構成**で関数を実行する必要があります。

この要件を満たす Azure Functions の **ホスティングプラン**と**トリガーの組み合わせ**を選択する必要があります。

## 2. 要件整理

この問題の要件を整理すると次の通りです。

- Service Bus キューのメッセージを処理する
- イベント駆動型で処理する
- コールドスタートを回避する
- メッセージ処理は継続的かつ低遅延で行う

これを満たすためには以下のポイントが重要です。

- Service Bus メッセージを処理する適切なトリガー
- コールドスタートを回避できる Azure Functions のホスティングプラン

## 3. 技術の基本概念

### Azure Functions のホスティングプラン

Azure Functions には主に以下のホスティングプランがあります。

**消費プラン (Consumption Plan)**  
- 使用した分だけ課金されるサーバーレスモデル  
- スケールは自動で行われる  
- 一定時間実行がない場合はインスタンスが停止する  
- 再起動時に **コールドスタートが発生する**

**プレミアムプラン (Premium Plan)**  
- 事前ウォームされたインスタンス（pre-warmed instances）を保持  
- コールドスタートを回避できる  
- 自動スケーリングが可能  
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Network (VNet)]]
- VNET統合などの高度な機能も利用可能

**App Service プラン / 専用プラン (Dedicated Plan)**  
- 常時実行可能  
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
- VMベースの固定リソース  
- サーバーレスの自動スケールとは異なる

### Azure Functions トリガー

**Service Bus Trigger**

- Service Bus キューやトピックのメッセージ到着で実行
- Event-driven（イベント駆動）
- PeekLock / ReceiveAndDelete モード
- Dead-letter キュー対応

**HTTP Trigger**

- HTTPリクエストで実行
- REST API 用途

**Timer Trigger**

- スケジュールベース
- 定期実行ジョブ

## 4. アーキテクチャまたは設計のポイント

Service Bus キューからメッセージを処理する場合、最も自然なアーキテクチャは以下です。

Service Bus Queue  
↓  
Azure Functions (Service Bus Trigger)

この構成では次の特徴があります。

- メッセージ到着時に自動実行
- メッセージロック制御
- 再試行・デッドレター処理

しかし、Functions のホスティングプランによっては **コールドスタートが発生する**ため注意が必要です。

### コールドスタート回避の方法

コールドスタートを回避する代表的な方法は以下です。

- Premium Plan を使用する
- Always-ready instance を維持する

Premium Plan は **常にウォーム状態のインスタンスを維持**するため、メッセージ処理時の遅延を防げます。

## 5. 設計判断（なぜこの構成になるか）

この問題の要件は以下の2点です。

1. Service Bus メッセージ処理
2. コールドスタート回避

これを同時に満たす構成は

**Premium Plan + Service Bus Trigger**

です。

理由：

- Service Bus Trigger がキューメッセージ処理に最適
- Premium Plan は pre-warmed instances を保持
- コールドスタートなしで実行可能
- スケールアウトにも対応

そのため、この組み合わせが最も適切です。

## 6. 他の選択肢が誤りな理由

### A. 消費プラン + Service Bus Trigger

Service Bus Trigger 自体は正しいですが、  
**Consumption Plan はコールドスタートが発生する可能性があります。**

関数が一定時間アイドル状態になるとインスタンスが破棄されるため、次回実行時に起動遅延が発生します。

したがって要件を満たしません。

### C. App Service Plan + HTTP Trigger

問題点はトリガーです。

HTTP Trigger は HTTP リクエストで実行されるため、  
Service Bus のメッセージ処理には適していません。

イベント駆動型のキュー処理では Service Bus Trigger を使用する必要があります。

### D. 専用プラン + Timer Trigger

Dedicated Plan はコールドスタートを回避できますが、  
Timer Trigger は **スケジュール実行**です。

Service Bus の新しいメッセージをイベントとして処理することができないため、要件に適合しません。

## 7. 最終回答

**B. プレミアムプラン + Service Bus トリガー**

## 8. まとめ

- Azure Functions の Consumption Plan ではコールドスタートが発生する可能性がある  
- コールドスタートを回避するには **Premium Plan** を使用する  
- Service Bus メッセージ処理には **Service Bus Trigger** が最適  
- HTTP Trigger や Timer Trigger はキューメッセージ処理には適していない  

試験では  
**「コールドスタート回避 = Premium Plan」**  
という組み合わせを覚えておくことが重要です。