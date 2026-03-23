# Azure イベント駆動オートメーション

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Logic Apps]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Event Grid]]
（Event Grid + Logic Apps によるインフラ監視）

## 1 概要

Azure環境では、リソースの状態や設定変更に応じて自動処理を実行する **イベント駆動アーキテクチャ**が重要になる。

今回のシナリオは

- 仮想マシンの設定変更を検知
    
- 管理者へ通知
    

という **インフラ監視の自動化**である。

この構成では次のサービスが使用される。

- **Azure Event Grid**
    
- **Azure Logic Apps**
    

典型構成

```text
Azure VM Configuration Change
            │
            ▼
Azure Event Grid
            │
            ▼
Logic App (Trigger)
            │
            ▼
Condition
            │
            ▼
Notification Action
```

これにより、Azureリソースのイベントを検知し、自動的にワークフローを実行できる。

---

# 2 背景

クラウド環境ではインフラが頻繁に変更される。

例

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
- VMサイズ変更
    
- ディスク追加
    
- ネットワーク設定変更
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Role-Based Access Control (RBAC)]]
- RBAC変更
    

これらを人手で監視することは困難である。

従来のオンプレミスでは

```text
Infrastructure
      │
      ▼
Monitoring Tool
      │
      ▼
Alert System
```

という監視ツール中心の構成だった。

Azureでは次のような **イベント駆動型監視**が主流になっている。

```text
Azure Resource Event
        │
        ▼
Event Grid
        │
        ▼
Automation / Logic App
```

これにより

- リアルタイム処理
    
- 自動化
    
- サーバレス運用
    

が可能になる。

---

# 3 サービスの仕組み

## Event Driven Architecture

Azureではリソースの変更がイベントとして発行される。

```text
Resource Change
      │
      ▼
Azure Resource Manager
      │
      ▼
Event Grid Event
```

イベントはサブスクライバーに配信される。

```text
Event Grid
   │
   ├ Logic Apps
   ├ Azure Functions
   ├ Webhook
   └ Service Bus
```

今回のケースでは

```text
Event Grid → Logic App
```

が使用される。

---

# 4 Logic Apps の主要コンポーネント

Logic Apps Designerではワークフローを次の構成で作る。

## 1 Trigger

ワークフローを開始するイベント。

今回の例

**Azure Event Grid Trigger**

```text
Event Grid
    │
    ▼
Logic App Trigger
```

役割

- Azureリソースイベント受信
    
- ワークフロー開始
    

---

## 2 Condition Control

条件分岐を行う。

```text
IF Condition
   ├ True → Action
   └ False → End
```

今回の例

- VM設定変更イベント
    
- 対象リソース確認
    

---

## 3 Action

実際に処理を実行する。

例

- Email送信
    
- Teams通知
    
- Webhook呼び出し
    

```text
Condition True
      │
      ▼
Send Notification
```

---

# 5 関連Azureサービス

この分野は **Event Architecture / Automation** に分類される。

主要サービス

|サービス|役割|
|---|---|
|Event Grid|イベント配信|
|Logic Apps|ワークフロー|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Functions]]
|Azure Functions|イベント処理|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Service Bus]]
|Service Bus|メッセージング|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Event Hubs]]
|Event Hub|ストリーム処理|

---

# 6 サービス比較

イベント処理サービス

|サービス|用途|
|---|---|
|Event Grid|リソースイベント|
|Event Hub|ストリーミング|
|Service Bus|メッセージキュー|
|Logic Apps|ワークフロー|

今回のケース

- **Event Grid → イベント検知**
    
- **Logic Apps → 自動処理**
    

---

# 7 アーキテクチャ

## VM設定変更通知

```text
Azure VM
   │
   ▼
Resource Manager Event
   │
   ▼
Event Grid
   │
   ▼
Logic App
   │
   ▼
Condition
   │
   ▼
Email / Teams Notification
```

---

## セキュリティ監査

```text
Resource Change
      │
      ▼
Event Grid
      │
      ▼
Logic App
      │
      ▼
Security Alert
```

---

# 8 ユースケース

## インフラ変更監視

対象

- VM
    
- Network
    
- Storage
    
- RBAC
    

変更イベントを検知し通知する。

---

## コンプライアンス監査

例

- NSG変更
    
- パブリックIP追加
    

検知してアラート送信。

---

## 自動修復

```text
Event
   │
   ▼
Logic App
   │
   ▼
Automation Script
```

例

- VM再起動
    
- 設定修復
    

---

# 9 設計指針

アーキテクトは次のポイントを判断する。

## 1 イベントサービス選択

|ケース|サービス|
|---|---|
|Azureリソース変更|Event Grid|
|IoTストリーム|Event Hub|
|メッセージ処理|Service Bus|

---

## 2 処理エンジン

|サービス|用途|
|---|---|
|Logic Apps|ワークフロー|
|Functions|コード処理|

---

## 3 通知方式

通知先

- Email
    
- Teams
    
- Slack
    
- Webhook
    

---

## 4 スケーラビリティ

Event Gridは

- 高スケール
    
- 低レイテンシ
    

でイベントを配信できる。

---

# 10 まとめ

Azureではインフラ監視や自動化を

**Event Driven Architecture**

で実装する。

今回のシナリオでは

- **Event Grid Trigger**
    
- **Condition Control**
    
- **Action**
    

の3つのLogic Appsコンポーネントを使用する。

構成

```text
Azure Resource Event
        │
        ▼
Event Grid
        │
        ▼
Logic App
   ├ Condition
   └ Action
```

この構成により

- VM設定変更検知
    
- 自動通知
    
- インフラ監視
    

を実現できる。