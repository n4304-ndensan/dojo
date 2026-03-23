# Topic-08-Analytics

## 学習ゴール

データ取り込みから分析、IoT、機械学習までを `データパイプライン` として捉え、各サービスの役割分担を整理する。

## このTopicの全体像

この Topic では、Data Factory、Synapse、Databricks、Stream Analytics、IoT、Azure Machine Learning を扱う。  
判断軸は「バッチかストリームか」「ETL か分析か」「ノーコードかコードか」。

---

# 第1章 学習マップ

## 1.1 学習順序

1. Data Factory でデータ取り込みの基本を押さえる。
2. ADLS Gen2 を前提に Synapse と Databricks の役割差を見る。
3. Stream Analytics と IoT でリアルタイム処理を補う。
4. Azure Machine Learning と動画分析で推論ワークロードまで広げる。

## 1.2 Azureリソース一覧

- Azure Data Factory
- Azure Synapse Analytics
- Azure Databricks
- Azure Stream Analytics
- Azure IoT Hub / Time Series Insights / Power BI
- Azure Machine Learning

---

# 第2章 Azureリソース解説

## Resource: Azure Data Factory

### 概要

[[Azure用語集.md#Azure Data Factory]] はデータ パイプラインの制御レイヤ。

### できること

- Copy Activity
- Mapping Data Flow
- Self-hosted Integration Runtime
- スケジュール / トリガー実行

### 技術仕様

- オンプレミス接続では Self-hosted IR が重要。
- データ取り込みとオーケストレーションを分離して考える。
- Blob、SQL、Synapse、ADLS Gen2 のハブになりやすい。

### SDK / API

- ADF REST API
- ARM / Azure CLI

### 他サービスとの比較

- Data Factory vs Logic Apps: データ移送/ETL は Data Factory。
- Data Factory vs AzCopy: 繰り返し運用するパイプラインなら Data Factory。

### どのようなときに使うか

- オンプレミス DB やファイルを Azure に継続取り込みしたいとき
- ETL パイプラインを定期運用したいとき

### 関連シナリオ

- [[Scenarios/Scenario-DataPlatform.md#data-factory-でオンプレミスデータをデータレイクへ取り込む]]
- [[Scenarios/Scenario-DataPlatform.md#blob-から-sql-database-へデータ転送する]]

### 主な出典

- [[Sources/Topic-09.md]]
- [[Sources/Azure Data Factory 大容量データコピー設計.md]]
- [[Sources/Azure Data Factory を利用したBlob StorageからSQL Databaseへの自動データ転送.md]]
- [[Sources/Azure Data Factory Self-hosted Integration Runtime と Private Endpoint による安全なデータ統合.md]]

## Resource: Azure Synapse Analytics

### 概要

[[Azure用語集.md#Azure Synapse Analytics]] は SQL、Spark、Pipelines を一体化した分析基盤。

### できること

- Dedicated SQL Pool
- Serverless SQL / Spark
- Pipelines
- Data Share 連携

### 技術仕様

- データウェアハウスとデータレイク分析を同一ワークスペースで扱える。
- Synapse Pipelines は Data Factory と近い操作感を持つ。
- Spark / SQL の役割を用途で分ける。

### SDK / API

- Synapse REST API
- SQL / Spark
- ARM / CLI

### 他サービスとの比較

- Synapse vs Databricks: SQL/DWH 統合重視なら Synapse。
- Synapse vs SQL Database: 分析基盤全体を持ちたいなら Synapse。

### どのようなときに使うか

- データレイクと DWH を統合的に扱いたいとき
- 組織横断分析基盤を整えたいとき

### 関連シナリオ

- [[Scenarios/Scenario-DataPlatform.md#synapse-でデータレイク分析とデータウェアハウスを統合する]]
- [[Scenarios/Scenario-DataPlatform.md#partner-企業へ-data-share-で分析データを配布する]]

### 主な出典

- [[Sources/Topic-09.md]]
- [[Sources/Azure Synapse Analytics におけるデータレイク分析アーキテクチャ設計.md]]
- [[Sources/Azure Synapse Pipelines と Azure Data Share を組み合わせたデータ統合・共有アーキテクチャ.md]]
- [[Sources/Azure データウェアハウス ETL アーキテクチャ設計.md]]

## Resource: Azure Databricks

### 概要

[[Azure用語集.md#Azure Databricks]] は Spark ベースのデータ処理・機械学習実行基盤。

### できること

- 大規模変換処理
- Parquet / Delta Lake 運用
- メダリオン アーキテクチャ
- ノートブック分析

### 技術仕様

- ADLS Gen2 が標準保存先になりやすい。
- Partitioned Parquet / Delta Lake の設計が性能を左右する。
- 認証は Entra ID / Credential Passthrough などを整理して考える。

### SDK / API

- Spark API
- Databricks REST API

### 他サービスとの比較

- Databricks vs Synapse Spark: ノートブック中心か、Synapse 統合か。
- Databricks vs Stream Analytics: 低レイテンシの連続クエリなら Stream Analytics。

### どのようなときに使うか

- 大規模変換、データレイク処理、特徴量生成
- データサイエンスとデータエンジニアリングを一体で回したいとき

### 関連シナリオ

- [[Scenarios/Scenario-DataPlatform.md#databricks-で-parquetdelta-中心のデータレイクを構成する]]

### 主な出典

- [[Sources/Topic-09.md]]
- [[Sources/Azure Databricks 分析基盤におけるデータレイクファイル形式設計.md]]
- [[Sources/Azure Databricks Credential Passthrough による安全なデータアクセス.md]]
- [[Sources/Azure メダリオンアーキテクチャのデータレイク設計.md]]

## Resource: Stream Analytics and IoT Analytics

### 概要

`Azure Stream Analytics` はストリーム クエリ、IoT 系サービスはイベント取り込みと可視化を支える。

### できること

- ストリーム集約
- 時系列分析
- Power BI へのリアルタイム出力
- IoT テレメトリ処理

### 技術仕様

- Event Hubs / IoT Hub を入口にする。
- 数秒以内のアラートや集約に向く。
- TSI / Power BI は可視化側の選択肢になる。

### SDK / API

- Stream Analytics query
- IoT Hub / Event Hubs SDK

### 他サービスとの比較

- Stream Analytics vs Databricks: 低レイテンシ ストリームなら Stream Analytics。

### どのようなときに使うか

- センサーデータのリアルタイム集約
- Power BI へ即時表示したいとき

### 関連シナリオ

- [[Scenarios/Scenario-DataPlatform.md#iot-テレメトリを-stream-analytics-で集計する]]
- [[Scenarios/Scenario-EventHubs.md#event-hubs--functions--cosmos-db-でリアルタイム処理する]]

### 主な出典

- [[Sources/Topic-10.md]]
- [[Sources/Azure IoT テレメトリストリーム処理設計.md]]
- [[Sources/Azure Stream Analytics を利用したリアルタイムテレメトリデータ統合.md]]
- [[Sources/IoT リアルタイムデータ処理パイプライン（Event Hubs + Stream Analytics + Power BI）.md]]

## Resource: Azure Machine Learning

### 概要

Azure Machine Learning はモデル学習・配布・推論を管理する ML プラットフォーム。

### できること

- Managed Online Endpoint
- GPU 推論
- モデル登録
- REST API 公開

### 技術仕様

- 低レイテンシ推論は Managed Online Endpoint が標準。
- GPU 要件や特殊デプロイは AKS 連携も使える。
- 監視と再デプロイ戦略を合わせて設計する。

### SDK / API

- Azure ML SDK / CLI
- REST endpoint

### 他サービスとの比較

- Azure ML vs App Service / Functions: モデル管理と推論配布まで含めるなら Azure ML。

### どのようなときに使うか

- 不正検知やスコアリング API を Azure で本番提供したいとき

### 関連シナリオ

- [[Scenarios/Scenario-DataPlatform.md#azure-machine-learning-でリアルタイム推論-api-を公開する]]

### 主な出典

- [[Sources/Topic-10.md]]
- [[Sources/Azure Machine Learning によるリアルタイム不正検出モデルのデプロイ設計.md]]
- [[Sources/Azure Machine Learning モデルデプロイアーキテクチャ.md]]

---

# 第3章 設計判断ガイド

## 3.1 パイプラインを選ぶとき

- コピー主体なら Data Factory。
- 変換/ノートブック主体なら Databricks。
- SQL/DWH 統合なら Synapse。

## 3.2 リアルタイム分析を選ぶとき

- 秒単位集約なら Stream Analytics。
- 高スループット入口は Event Hubs / IoT Hub。

## 3.3 推論基盤を選ぶとき

- モデル配布と運用まで含めるなら Azure ML。
- 単純な前処理/後処理は Functions と組み合わせる。

---

# 第4章 関連シナリオ

- [[Scenarios/Scenario-DataPlatform.md]]

