# Azure IoT テレメトリストリーム処理設計

（5秒以内アラート処理 + 可変データ量）

---

# 1 背景

IoT システムでは、多数のデバイスから継続的にテレメトリデータが送信される。これらのデータはリアルタイムに処理され、異常検知やアラート通知に利用されることが多い。例えば、工場設備監視、スマートビルディング、交通管理、環境センサーなどの分野では、センサーから送信されるデータを数秒以内に分析し、異常を検知することが重要になる。

今回のシステムでは、IoT デバイスから送信されるテレメトリデータを処理し、**最大5秒の遅延以内にアラートを発生させる**必要がある。また、データ量は常に一定ではなく、時間帯やイベントによって大きく変動する。さらに、処理ロジックは運用の中で頻繁に変更される可能性があるため、迅速なデプロイや更新が可能な仕組みが求められている。

このような要件を満たすストリーム処理基盤として最適な Azure サービスは

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Functions]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Event Hubs]]
**Azure Functions（Event Hubs トリガー）**

である。

---

# 2 IoT ストリーム処理の基本構成

IoT システムでは一般的に次のようなデータ処理パイプラインが構築される。

```text
IoT Devices
      │
      ▼
Event Ingestion
      │
      ▼
Stream Processing
      │
      ▼
Alert / Storage / Analytics
```

まず IoT デバイスからイベントデータが送信され、イベントストリームとしてクラウドに取り込まれる。その後、リアルタイム処理エンジンがデータを分析し、必要に応じてアラートを発生させたり、データを保存したりする。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure IoT Hub]]
Azure では、このイベント取り込みサービスとして **Azure Event Hubs** や **Azure IoT Hub** が利用されることが多い。Event Hubs は大規模イベントストリームを高速に取り込むことができるサービスであり、数百万イベント/秒の処理能力を持つ。

---

# 3 Azure Functions（Event Hubs トリガー）

Azure Functions は、Azure が提供する **サーバーレスコンピューティングサービス**である。イベント駆動型アーキテクチャを採用しており、特定のイベントが発生した際に自動的にコードを実行する。

Event Hubs トリガーを利用すると、Event Hubs に送信されたイベントをトリガーとして Azure Functions が実行される。これにより、IoT データをリアルタイムに処理することができる。

Azure Functions の主な特徴は次の通りである。

|特徴|内容|
|---|---|
|サーバーレス|インフラ管理不要|
|イベント駆動|イベント発生時に自動実行|
|自動スケーリング|負荷に応じてインスタンス増減|
|高速処理|低レイテンシ処理|

この仕組みにより、IoT テレメトリを受信してから数秒以内に処理を完了することが可能になる。

---

# 4 システムアーキテクチャ

Azure Functions を利用した IoT ストリーム処理構成は次のようになる。

```text
IoT Devices
   │
   ▼
Azure IoT Hub / Event Hubs
   │
   ▼
Azure Functions (Event Hub Trigger)
   │
   ▼
Telemetry Processing
   │
   ▼
Alert Generation
   │
   ├─ Notification
   ├─ Database Storage
   └─ Monitoring Dashboard
```

この構成では、IoT デバイスが送信したイベントが Event Hubs に取り込まれ、そのイベントをトリガーとして Azure Functions が実行される。関数内でテレメトリデータを解析し、閾値を超えた場合にアラートを生成する。

---

# 5 処理フロー

処理の流れは次のようになる。

1. IoT デバイスがテレメトリデータを送信
    
2. Event Hubs がイベントを受信
    
3. Event Hub トリガーにより Azure Functions が起動
    
4. Functions がデータを解析
    
5. 条件に一致する場合アラートを生成
    

この処理はイベント到着と同時に実行されるため、**5秒以内のアラート生成**が可能になる。

---

# 6 この構成のメリット

Azure Functions を利用することで、IoT ストリーム処理システムにおいて重要なメリットを得ることができる。

まず、サーバーレスアーキテクチャであるため、インフラ管理が不要であり、運用負荷が大幅に削減される。イベント量が増加した場合でも Azure が自動的にスケールアウトするため、急激なデータ増加にも対応できる。

また、処理ロジックはコードとして実装されるため、ロジック変更が頻繁に発生する場合でも迅速に更新することができる。これは SQL ベースのストリーム処理サービスよりも柔軟性が高い。

さらに、Event Hubs と組み合わせることで、数百万イベント/秒規模の IoT データを処理できる高いスケーラビリティを実現できる。

---

# 7 他の選択肢が適切でない理由

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Stream Analytics]]
Azure Stream Analytics はリアルタイムストリーム分析サービスであり、SQL ライクなクエリでデータ処理を行うことができる。しかし処理ロジックが頻繁に変更される場合、コードベースの処理よりも柔軟性が低くなる可能性がある。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Databricks]]
Azure Databricks は Apache Spark ベースのビッグデータ処理プラットフォームであり、大規模分析や機械学習には非常に強力であるが、クラスタ管理やジョブ管理が必要であり、5秒以内の低レイテンシ処理には適さないケースが多い。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Kubernetes Service (AKS)]]
Azure Kubernetes Service と Kafka を組み合わせることで高度なストリーム処理基盤を構築することも可能であるが、Kubernetes クラスタの運用や Kafka の管理が必要になるため、シンプルな IoT アラート処理システムには過剰な構成となる。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Synapse Analytics]]
Azure Synapse Analytics はデータウェアハウスやビッグデータ分析のためのサービスであり、バッチ処理や大規模分析には適しているが、リアルタイムストリーム処理には向いていない。

---

# 8 まとめ

今回の要件は次の通りである。

- IoT テレメトリデータのリアルタイム処理
    
- 最大5秒以内のアラート生成
    
- データ量が大きく変動する
    
- 処理ロジックが頻繁に変更される
    

これらの要件を満たす最適な Azure サービスは

**Azure Functions（Event Hubs トリガー）**

である。

この構成により、低レイテンシでスケーラブルな IoT ストリーム処理基盤を構築でき、処理ロジックの変更にも柔軟に対応することが可能になる。