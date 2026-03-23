---
分類: DataStreaming
tags:
  - cloud/azure
  - cloud/azure/event-hubs
  - cloud/azure/stream-analytics
  - cloud/azure/power-bi
  - cloud/iot
  - cloud/real-time-processing
  - cloud/data-pipeline
  - exam/azure
---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Event Hubs]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Stream Analytics]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Power BI]]
# IoT リアルタイムデータ処理パイプライン（Event Hubs + Stream Analytics + Power BI）

## 1. 背景（シナリオ）

IoT ソリューションでは、多数のデバイスから継続的にデータが送信されます。例えば工場設備、センサー、車両、スマートメーターなどの IoT デバイスは、温度、位置、状態、パフォーマンスなどのテレメトリをリアルタイムで送信します。

今回のシナリオでは、数千台の IoT デバイスからデータがストリーミングされます。これらのデータはリアルタイムに近い形で処理され、運用担当者が状況を確認できるように可視化する必要があります。

つまり、このアーキテクチャでは **リアルタイムデータ取り込み → ストリーム処理 → 可視化**というデータパイプラインを設計する必要があります。

さらに、IoT 環境ではデバイス数やデータ量が増える可能性があるため、システムは非常に高いスケーラビリティを持つ必要があります。また、インフラ管理を最小限にするため、ソリューションは **完全にマネージドサービスで構成すること**が望ましいです。

これらの要件を満たす Azure のサービス構成を選択する必要があります。

---

## 2. 要件整理

この問題の重要なポイントは、IoT データ処理の典型的なパイプラインを理解することです。

まず、大量のデバイスから送信されるストリーミングデータを受け取る **高スループットのデータ取り込みサービス**が必要になります。

次に、そのデータをリアルタイムで処理する **ストリーム処理エンジン**が必要です。ここではフィルタリングや集計、時間ウィンドウ分析などが実行されます。

さらに、分析結果を運用者が理解できるようにするため **リアルタイムの可視化ダッシュボード**が必要です。

この問題の要件を整理すると次のようになります。

- IoT デバイスからの大量ストリーミングデータ  
- 低遅延のリアルタイム処理  
- 高いスケーラビリティ  
- 完全にマネージドサービス  
- リアルタイムダッシュボード  

このようなリアルタイムデータ処理アーキテクチャは、Azure では **Event Hubs + Stream Analytics + Power BI** の組み合わせで実現されます。

---

## 3. Event Hubs（データ取り込み）

Azure Event Hubs は、大量のイベントデータを取り込むためのストリーミングプラットフォームです。

このサービスは Apache Kafka と似たイベントストリーミングアーキテクチャを採用しており、数百万イベント/秒レベルのデータ取り込みが可能です。

Event Hubs の主な役割は **データの取り込み（Ingestion）**です。

IoT デバイスやアプリケーションは Event Hubs にイベントを送信し、Event Hubs がそれを受信してストリームとして保持します。

Event Hubs の特徴は次の通りです。

- 高スループットのイベント取り込み  
- パーティションによるスケーラビリティ  
- IoT デバイスとの統合  
- 完全マネージドサービス  

そのため、大量の IoT デバイスからのデータ取り込みに最適です。

---

## 4. Stream Analytics（リアルタイム処理）

Azure Stream Analytics は、リアルタイムのストリーミングデータを処理するサービスです。

このサービスでは SQL に似たクエリ言語を使用してストリーミングデータを処理できます。

例えば次のような処理が可能です。

- データのフィルタリング  
- 集計処理  
- 時間ウィンドウ分析  
- 異常検知  
- データのエンリッチメント  

Stream Analytics は Event Hubs とネイティブに統合されているため、Event Hubs に取り込まれたデータをリアルタイムで処理できます。

さらに、処理結果をさまざまなサービスへ送信できます。

---

## 5. Power BI（リアルタイム可視化）

Power BI は Microsoft のデータ可視化および BI（Business Intelligence）サービスです。

Stream Analytics の出力を Power BI に送ることで、リアルタイムダッシュボードを構築できます。

例えば次のような可視化が可能です。

- センサー値のリアルタイムグラフ  
- 異常検知アラート  
- 地理データのマップ表示  
- IoT デバイスの状態監視  

Power BI のストリーミングデータセットを利用すると、ほぼリアルタイムでダッシュボードを更新できます。

これにより、運用チームは IoT システムの状態をリアルタイムに監視できます。

---

## 6. アーキテクチャ全体の流れ

このリアルタイムデータパイプラインは次のように動作します。

まず、IoT デバイスがテレメトリデータを Azure Event Hubs に送信します。

次に、Azure Stream Analytics が Event Hubs のデータストリームを読み取り、リアルタイムで分析処理を行います。

最後に、処理結果が Power BI に送信され、リアルタイムダッシュボードとして表示されます。

この構成は完全にマネージドサービスで構築されているため、インフラ管理は不要です。また、IoT デバイス数が増えても自動的にスケールできます。

---

## 7. 他の選択肢が適さない理由

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Logic Apps]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Service Bus]]
### Service Bus + Logic Apps + SQL Database

Service Bus はエンタープライズメッセージングサービスであり、リアルタイムストリーミング分析向けではありません。Logic Apps もワークフロー自動化ツールであり、大量ストリーミング処理には適していません。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Functions]]
### Queue Storage + Azure Functions + Data Lake

この構成はバッチ処理や非同期処理には適していますが、リアルタイムストリーミング分析には最適ではありません。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Cosmos DB]]
### Notification Hubs + App Service + Cosmos DB

Notification Hubs はプッシュ通知サービスであり、IoT テレメトリ取り込み用途ではありません。

---

## 8. 最終回答

A. Azure Event Hubs、Azure Stream Analytics、および Power BI