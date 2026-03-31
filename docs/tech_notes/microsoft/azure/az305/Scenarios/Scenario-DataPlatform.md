# Scenario: Data Platform

## シナリオ一覧

- Data Factory でオンプレミスデータをデータレイクへ取り込む
- Blob から SQL Database へデータ転送する
- Synapse でデータレイク分析とデータウェアハウスを統合する
- Partner 企業へ Data Share で分析データを配布する
- Databricks で Parquet/Delta 中心のデータレイクを構成する
- IoT テレメトリを Stream Analytics で集計する
- Azure Machine Learning でリアルタイム推論 API を公開する

## data-factory-でオンプレミスデータをデータレイクへ取り込む

シナリオ  
オンプレミスの SQL Server やファイルを Azure 側へ継続コピーする。

構成  
On-prem Data  
↓  
Self-hosted IR  
↓  
Azure Data Factory  
↓  
ADLS Gen2 / Blob

ポイント  
- ネットワーク内のデータソースを安全に取り込める
- 定期実行と監視を一体化しやすい

関連リソース  
Azure Data Factory / ADLS Gen2

出典  
- [[Sources/Azure Data Factory 大容量データコピー設計.md]]
- [[Sources/Azure Data Factory を用いたオンプレミスファイルサーバーから Azure Blob Storage へのデータ移行設計.md]]

## blob-から-sql-database-へデータ転送する

シナリオ  
Blob に置いたファイルを加工またはロードして SQL Database 側へ取り込む。

構成  
Blob Storage  
↓  
Azure Data Factory  
↓  
Azure SQL Database

ポイント  
- バッチ取り込みの標準パターン
- コピーだけでなく変換処理も足しやすい

関連リソース  
Azure Data Factory / Azure Blob Storage / Azure SQL Database

出典  
- [[Sources/Azure Data Factory を利用したBlob StorageからSQL Databaseへの自動データ転送.md]]
- [[Sources/Azure Blob ストレージから Azure SQL Database への自動データ転送設計.md]]

## synapse-でデータレイク分析とデータウェアハウスを統合する

シナリオ  
データレイクと DWH を 1 つの分析基盤で扱う。

構成  
ADLS Gen2  
↓  
Synapse SQL / Spark  
↓  
BI / Analytics

ポイント  
- SQL と Spark の役割分担が重要
- ワークスペース統合で運用を簡素化できる

関連リソース  
Azure Synapse Analytics / ADLS Gen2

出典  
- [[Sources/Azure Synapse Analytics におけるデータレイク分析アーキテクチャ設計.md]]
- [[Sources/Azure データウェアハウス ETL アーキテクチャ設計.md]]

## partner-企業へ-data-share-で分析データを配布する

シナリオ  
パートナー企業へデータセットを定期共有し、自社内のパイプラインと切り分ける。

構成  
Synapse / Lake  
↓  
Azure Data Share  
↓  
Partner Tenant

ポイント  
- 自社の取り込みパイプラインと共有手段を分離できる

関連リソース  
Azure Data Share / Azure Synapse Analytics

出典  
- [[Sources/Azure Synapse Pipelines と Azure Data Share を組み合わせたデータ統合・共有アーキテクチャ.md]]

## databricks-で-parquetdelta-中心のデータレイクを構成する

シナリオ  
データレイクを Parquet / Delta で標準化し、大規模変換と分析を行う。

構成  
ADLS Gen2  
↓  
Azure Databricks  
↓  
Curated Data

ポイント  
- ファイル形式設計が性能を大きく左右する
- メダリオン アーキテクチャとの相性がよい

関連リソース  
Azure Databricks / ADLS Gen2

出典  
- [[Sources/Azure Databricks 分析基盤におけるデータレイクファイル形式設計.md]]
- [[Sources/Azure メダリオンアーキテクチャのデータレイク設計.md]]

## iot-テレメトリを-stream-analytics-で集計する

シナリオ  
IoT の高頻度データを数秒単位で集計し、アラートやダッシュボードへ出す。

構成  
IoT Hub / Event Hubs  
↓  
Stream Analytics  
↓  
Power BI / Storage / Alerts

ポイント  
- 低レイテンシ集計に向く
- Databricks より運用が軽い

関連リソース  
Azure Stream Analytics / Azure Event Hubs / Power BI

出典  
- [[Sources/Azure IoT テレメトリストリーム処理設計.md]]
- [[Sources/IoT リアルタイムデータ処理パイプライン（Event Hubs + Stream Analytics + Power BI）.md]]

## azure-machine-learning-でリアルタイム推論-api-を公開する

シナリオ  
学習済みモデルを REST API として低レイテンシに公開する。

構成  
Model Registry  
↓  
Managed Online Endpoint  
↓  
Client Apps

ポイント  
- モデル配布とスケールを Azure ML に寄せられる
- GPU 要件や監視も一体で考えやすい

関連リソース  
Azure Machine Learning

出典  
- [[Sources/Azure Machine Learning によるリアルタイム不正検出モデルのデプロイ設計.md]]
- [[Sources/Azure Machine Learning モデルデプロイアーキテクチャ.md]]
