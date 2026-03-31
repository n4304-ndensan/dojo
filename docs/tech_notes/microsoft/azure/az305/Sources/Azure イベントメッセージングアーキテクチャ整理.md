# Azure イベント駆動アーキテクチャ完全ドキュメント

## 1. 概要

クラウドシステムでは、システムのスケーラビリティや柔軟性を高めるために **イベント駆動アーキテクチャ (Event-Driven Architecture)** が広く採用されています。

イベント駆動アーキテクチャでは、システム内で発生するイベント（データ更新、ユーザー操作、センサーデータなど）をメッセージング基盤へ送信し、そのイベントをトリガーとして複数のサービスが処理を実行します。

Azureではこの仕組みを実現するために、以下のようなサービスを組み合わせて利用します。

主なAzureサービス

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure IoT Hub]]
- Azure IoT Hub
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Event Hubs]]
- Azure Event Hubs
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Service Bus]]
- Azure Service Bus
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Event Grid]]
- Azure Event Grid
    
- Azure Queue Storage
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Stream Analytics]]
- Azure Stream Analytics
    
- Azure Data Lake
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Functions]]
- Azure Functions
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Power BI]]
- Power BI
    

これらを組み合わせることで、リアルタイム処理・非同期処理・大規模データ処理などのシステムを構築できます。

---

# 2. Azureイベントメッセージングの分類

Azureのメッセージングサービスは大きく **3つのカテゴリ**に分類できます。

## ストリーム処理系

大量のイベントデータをリアルタイムで処理するためのサービスです。

主なサービス

- Azure Event Hubs
    
- Azure Stream Analytics
    

特徴

- 高スループット
    
- リアルタイム処理
    
- IoT向け
    

主な用途

- センサーデータ
    
- アプリケーションログ
    
- テレメトリ
    

---

## メッセージブローカー系

サービス間通信や非同期処理を実現するためのメッセージング基盤です。

主なサービス

- Azure Service Bus
    
- Azure Queue Storage
    

特徴

- メッセージ永続化
    
- 信頼性メッセージング
    
- 非同期処理
    

主な用途

- マイクロサービス通信
    
- バックグラウンド処理
    

---

## イベント通知系

イベントが発生したことを通知するためのサービスです。

主なサービス

- Azure Event Grid
    

特徴

- プッシュ型イベント
    
- 軽量通知
    
- サーバーレス統合
    

主な用途

- リソース変更通知
    
- サーバーレス処理
    

---

# 3. IoT Devices

IoT Devicesとは、インターネットに接続された物理デバイスを指します。

例

- 温度センサー
    
- スマートメーター
    
- GPSトラッカー
    
- 工場設備
    

IoTデバイスは定期的にデータを送信します。

例

- 温度
    
- 湿度
    
- 加速度
    
- 位置情報
    

IoTシステムでは、数万〜数百万のデバイスが同時にデータを送信することがあります。

そのため、高スループットなイベント処理基盤が必要になります。

---

# 4. Azure IoT Hub

Azure IoT Hubは、IoTデバイスとの通信を管理するサービスです。

主な機能

- デバイス認証
    
- セキュア通信
    
- デバイス管理
    
- テレメトリ収集
    

IoT HubはIoTデバイスからデータを受信し、そのデータを他のAzureサービスへ送信します。

---

# 5. Azure Event Hubs

Azure Event Hubsは、大規模イベントストリーミングサービスです。

特徴

- 数百万イベント/秒の処理
    
- パーティション分散
    
- Consumer Group
    
- イベント保持
    
- Kafka互換
    

Event Hubsはログ型ストリームシステムとして動作します。

そのため

- 再処理
    
- 並列処理
    
- 複数コンシューマー
    

が可能になります。

用途

- IoTテレメトリ
    
- ログ収集
    
- リアルタイム分析
    

---

# 6. Azure Stream Analytics

Azure Stream Analyticsはリアルタイムストリーム処理サービスです。

Event Hubsなどから送られてくるイベントデータをリアルタイムに処理できます。

主な機能

- ウィンドウ処理
    
- イベント時間処理
    
- 遅延データ処理
    
- SQLライククエリ
    

例

- 1分間平均温度
    
- 異常検知
    
- リアルタイム統計
    

---

# 7. Azure Data Lake

Azure Data Lakeは大規模データ保存用ストレージです。

特徴

- ビッグデータ保存
    
- 分析向けストレージ
    
- Hadoop互換
    
- 高スケーラビリティ
    

IoTデータやログデータなどを長期保存するために利用されます。

---

# 8. Power BI

Power BIはデータ可視化サービスです。

主な機能

- ダッシュボード
    
- レポート作成
    
- リアルタイム分析
    
- データ可視化
    

Power BIはData Lakeやデータウェアハウスのデータを可視化します。

---

# 9. Azure Service Bus

Azure Service Busはエンタープライズメッセージングサービスです。

用途

- マイクロサービス通信
    
- ワークフロー処理
    
- 非同期メッセージング
    

特徴

- Queue
    
- Topic / Subscription
    
- Dead Letter Queue
    
- トランザクション
    

Service Busは信頼性の高いメッセージ処理を提供します。

---

# 10. Azure Event Grid

Azure Event Gridはイベント通知サービスです。

イベント例

- Blob作成
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
- VM作成
    
- リソース変更
    

Event Gridはイベント発生を通知し、Azure Functionsなどをトリガーします。

---

# 11. Azure Functions

Azure Functionsはサーバーレスコンピューティングサービスです。

特徴

- イベント駆動
    
- 自動スケール
    
- インフラ管理不要
    

Event GridやQueueなどと組み合わせて利用します。

---

# 12. Azure Queue Storage

Azure Queue Storageはシンプルなメッセージキューサービスです。

用途

- 非同期処理
    
- バックグラウンド処理
    

例

Webアプリが重い処理をQueueへ送信し、Workerが処理を行います。

---

# 13. Worker / Batch Processing

Workerはバックグラウンド処理プログラムです。

処理例

- 画像変換
    
- メール送信
    
- データ集計
    

Batch Processingは大量データをまとめて処理する方法です。

---

# 14. 実践アーキテクチャ例

## IoTリアルタイム分析

IoTセンサーデータをリアルタイムに分析する構成です。

```
IoT Devices
↓
Event Hubs
↓
Stream Analytics
↓
Data Lake
↓
Power BI
```

用途

- センサーデータ監視
    
- リアルタイム分析
    

---

## マイクロサービスアーキテクチャ

サービス間通信を非同期で行う構成です。

```
API Service
↓
Service Bus
↓
Order Service
↓
Billing Service
↓
Notification Service
```

用途

- マイクロサービス通信
    
- 非同期処理
    

---

## サーバーレスイベント処理

イベント発生時に処理を実行する構成です。

```
Blob Upload
↓
Event Grid
↓
Azure Functions
↓
Image Processing
```

用途

- 画像処理
    
- ファイル処理
    

---

## Webバックグラウンド処理

重い処理を非同期化する構成です。

```
Web App
↓
Queue Storage
↓
Worker
↓
Batch Processing
```

用途

- メール送信
    
- バッチ処理
    

---

## IoT大規模テレメトリ

大規模IoTシステムの構成です。

```
100,000 Devices
↓
IoT Hub
↓
Event Hubs
↓
Stream Processing
↓
Data Lake
↓
Analytics
```

特徴

- 高スループット
    
- 大規模スケール
    
- リアルタイム分析
    

---

# 15. Azureメッセージングサービス比較

|サービス|用途|
|---|---|
|Event Hubs|大量イベントストリーム|
|Service Bus|信頼性メッセージング|
|Event Grid|イベント通知|
|Queue Storage|シンプルキュー|
|Notification Hubs|モバイル通知|

---

# 16. Azureアーキテクチャ設計パターン

要件別の選択

|要件|サービス|
|---|---|
|大量イベント処理|Event Hubs|
|IoTテレメトリ|IoT Hub|
|マイクロサービス通信|Service Bus|
|イベント通知|Event Grid|
|非同期処理|Queue Storage|
|サーバーレス処理|Azure Functions|

---

# 17. まとめ

Azureイベント駆動アーキテクチャでは、以下の役割分担が基本となります。

イベント収集

- IoT Hub
    
- Event Hubs
    

リアルタイム処理

- Stream Analytics
    

データ保存

- Data Lake
    

サービス通信

- Service Bus
    

イベント通知

- Event Grid
    

非同期処理

- Queue Storage
    

サーバーレス処理

- Azure Functions
    

これらを組み合わせることで

- IoTシステム
    
- リアルタイム分析
    
- マイクロサービス
    
- イベント駆動システム
    

などのスケーラブルなクラウドアーキテクチャを構築できます。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Databricks]]
必要なら次に **このドキュメントをさらにレベルアップした「Azureデータパイプライン完全版（Databricks / Synapse / Data Factory含む）」** を作れます。  
実はAzure試験は **ここに出てくるサービス＋その拡張だけで8割解ける構造**になっています。