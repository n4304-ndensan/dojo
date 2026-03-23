# Topic-10 IoT と AI/ML

## 学習ゴール

IoT のデータ取り込みからストリーム処理、可視化、機械学習推論、動画分析までを順番に理解する。

## この Topic の全体像

IoT、ストリーム処理、リアルタイム可視化、Azure Machine Learning を整理する。

対象ドキュメント数: 7 件

## 第1章 学習マップ

### 1.1 学習順序

1. IoT データ取り込み: IoT Hub や Event Hubs による取り込みを入口として押さえる。
2. リアルタイム処理と可視化: Stream Analytics、Power BI、TSI 系の流れで処理から可視化までを見る。
3. 機械学習推論: Azure Machine Learning の推論エンドポイントを理解する。
4. 動画と高度分析: リアルタイム動画やカメラ系のユースケースを最後に確認する。

### 1.2 セクション対応表

- IoT データ取り込み: 2 件 / [[Azure IoT データ処理アーキテクチャ.md]] / [[Azure IoT テレメトリストリーム処理設計.md]]
- リアルタイム処理と可視化: 2 件 / [[Azure IoT センサーデータのリアルタイムダッシュボード設計.md]] / [[IoT リアルタイムデータ処理パイプライン（Event Hubs + Stream Analytics + Power BI）.md]]
- 機械学習推論: 2 件 / [[Azure Machine Learning によるリアルタイム不正検出モデルのデプロイ設計.md]] / [[Azure Machine Learning モデルデプロイアーキテクチャ.md]]
- 動画と高度分析: 1 件 / [[Azure リアルタイム動画分析アーキテクチャ.md]]

## 第2章 基礎概念と構成要素

### 2.1 IoT データ取り込み

IoT Hub や Event Hubs による取り込みを入口として押さえる。

主な出典: [[Azure IoT データ処理アーキテクチャ.md]] / [[Azure IoT テレメトリストリーム処理設計.md]]

主要論点: 推奨サービス / IoTデータパイプライン / Azure Stream Analytics / ストリーム処理の例 / Azure Data Lake Storage / 保存データ例 / データフロー / リアルタイム異常検知 / 長期データ分析 / 分析ツール

### 2.2 リアルタイム処理と可視化

Stream Analytics、Power BI、TSI 系の流れで処理から可視化までを見る。

主な出典: [[Azure IoT センサーデータのリアルタイムダッシュボード設計.md]] / [[IoT リアルタイムデータ処理パイプライン（Event Hubs + Stream Analytics + Power BI）.md]]

主要論点: 全体アーキテクチャ / Azure Event Hubs の役割 / Azure Time Series Insights (TSI) / Time Series Insights の主な機能 / リアルタイムデータ取り込み / 時系列データの高速検索 / インタラクティブなフィルタリング / 集約処理 / ビルトインダッシュボード / Power BI Streaming Dataset

### 2.3 機械学習推論

Azure Machine Learning の推論エンドポイントを理解する。

主な出典: [[Azure Machine Learning によるリアルタイム不正検出モデルのデプロイ設計.md]] / [[Azure Machine Learning モデルデプロイアーキテクチャ.md]]

主要論点: Azure Machine Learning の推論アーキテクチャ / Managed Online Endpoint / GPU 推論 / 自動スケーリング / Docker コンテナのサポート / 他のコンピューティングターゲットとの比較 / Azure Kubernetes Service（GPU ノードプール） / Azure Container Instances（GPU） / Azure ML Compute Cluster / 機械学習モデルのデプロイとは

### 2.4 動画と高度分析

リアルタイム動画やカメラ系のユースケースを最後に確認する。

主な出典: [[Azure リアルタイム動画分析アーキテクチャ.md]]

主要論点: Azure Video Analyzer / Azure Custom Vision / システムアーキテクチャ / メタデータ例 / Azure Media Services / Azure Stream Analytics / Azure Event Grid / メリット

## 第3章 設計判断の軸

### 3.1 IoT データ取り込み

- ある IoT ソリューションでは **50,000台のデバイス**からテレメトリーデータを収集する必要がある。 ([[Azure IoT データ処理アーキテクチャ.md]])
- このため、IoT データ基盤は **ストリーミング処理 + 長期ストレージ**の両方を備える必要がある。 ([[Azure IoT データ処理アーキテクチャ.md]])
- 今回のシステムでは、IoT デバイスから送信されるテレメトリデータを処理し、**最大5秒の遅延以内にアラートを発生させる**必要がある。また、データ量は常に一定ではなく、時間帯やイベントによって大きく変動する。さらに、処理ロジックは運用の中で頻繁に変更される可能性があるため、迅速なデプロイや更新が可能な仕組みが求められている。 ([[Azure IoT テレメトリストリーム処理設計.md]])
- このような要件を満たすストリーム処理基盤として最適な Azure サービスは ([[Azure IoT テレメトリストリーム処理設計.md]])
- まず IoT デバイスからイベントデータが送信され、イベントストリームとしてクラウドに取り込まれる。その後、リアルタイム処理エンジンがデータを分析し、必要に応じてアラートを発生させたり、データを保存したりする。 ([[Azure IoT テレメトリストリーム処理設計.md]])
- Azure Databricks は Apache Spark ベースのビッグデータ処理プラットフォームであり、大規模分析や機械学習には非常に強力であるが、クラスタ管理やジョブ管理が必要であり、5秒以内の低レイテンシ処理には適さないケースが多い。 ([[Azure IoT テレメトリストリーム処理設計.md]])

### 3.2 リアルタイム処理と可視化

- 今回のシナリオでは、製造設備から送信されるセンサーデータをクラウドで収集し、リアルタイムダッシュボードで可視化する仕組みを構築する必要がある。データは Azure Event Hubs に取り込まれ、ユーザーはダッシュボード上で以下の操作を行うことが求められている。 ([[Azure IoT センサーデータのリアルタイムダッシュボード設計.md]])
- このような要件を満たす Azure サービスとして最も適しているのが **Azure Time Series Insights（TSI）** である。 ([[Azure IoT センサーデータのリアルタイムダッシュボード設計.md]])
- TSI は Event Hubs と直接接続できるため、データはほぼリアルタイムで取り込まれる。 ([[Azure IoT センサーデータのリアルタイムダッシュボード設計.md]])
- 製造設備のセンサーデータをリアルタイムに可視化する場合、以下の要件が重要になる。 ([[Azure IoT センサーデータのリアルタイムダッシュボード設計.md]])
- これらの要件を満たす最適な Azure サービスは ([[Azure IoT センサーデータのリアルタイムダッシュボード設計.md]])
- 今回のシナリオでは、数千台の IoT デバイスからデータがストリーミングされます。これらのデータはリアルタイムに近い形で処理され、運用担当者が状況を確認できるように可視化する必要があります。 ([[IoT リアルタイムデータ処理パイプライン（Event Hubs + Stream Analytics + Power BI）.md]])

### 3.3 機械学習推論

- 金融取引やオンライン決済システムでは、不正検出（Fraud Detection）のために機械学習モデルがリアルタイムで利用されることが多い。例えばクレジットカード取引では、ユーザーが決済を行った瞬間にその取引が不正であるかどうかを判定する必要がある。このような用途では、モデルの推論処理が数百ミリ秒以内に完了する必要があり、非常に低いレイテンシが求められる。 ([[Azure Machine Learning によるリアルタイム不正検出モデルのデプロイ設計.md]])
- 今回のシナリオでは、企業が不正検出のためのリアルタイム機械学習モデルを Azure 上に導入している。モデルは Docker コンテナとしてパッケージ化されており、以下の要件を満たす必要がある。 ([[Azure Machine Learning によるリアルタイム不正検出モデルのデプロイ設計.md]])
- 今回の要件は **リアルタイム推論**であるため、API エンドポイントとしてモデルを公開する必要がある。 ([[Azure Machine Learning によるリアルタイム不正検出モデルのデプロイ設計.md]])
- Managed Online Endpoint は Azure Machine Learning が提供する **フルマネージドのリアルタイム推論サービス**である。ユーザーはインフラの管理を行う必要がなく、モデルのデプロイ、スケーリング、監視は Azure によって管理される。 ([[Azure Machine Learning によるリアルタイム不正検出モデルのデプロイ設計.md]])
- これにより、リアルタイム不正検出のような低レイテンシ要件に対応できる。 ([[Azure Machine Learning によるリアルタイム不正検出モデルのデプロイ設計.md]])
- 企業が機械学習モデルを **REST API として公開**する場合、以下の要件がよく求められます。 ([[Azure Machine Learning モデルデプロイアーキテクチャ.md]])

### 3.4 動画と高度分析

- ある組織では、複数の監視カメラや IoT カメラから送信される **リアルタイム動画ストリーム**を分析し、特定の物体を検出するシステムを構築する必要がある。例えば、スマートシティ、工場の安全監視、交通監視、店舗分析などの分野では、カメラ映像をリアルタイムに分析し、人や車両、物体などを識別することが重要である。 ([[Azure リアルタイム動画分析アーキテクチャ.md]])
- このようなシステムでは、単に動画を保存するだけでなく、動画フレームを AI によって解析し、検出された物体の情報（メタデータ）を生成する必要がある。例えば、以下のようなデータが生成される。 ([[Azure リアルタイム動画分析アーキテクチャ.md]])
- このようなリアルタイム動画分析システムでは、以下の処理が必要になる。 ([[Azure リアルタイム動画分析アーキテクチャ.md]])
- これらの要件を満たす Azure サービスの組み合わせとして最適なのは次の2つである。 ([[Azure リアルタイム動画分析アーキテクチャ.md]])
- これらの要件を満たす Azure サービスの組み合わせは ([[Azure リアルタイム動画分析アーキテクチャ.md]])

## 第4章 ユースケースで理解する

### 4.1 IoT データ取り込みのユースケース

- Azure IoT データ処理アーキテクチャ （50,000デバイス / リアルタイム異常検知 / 長期分析）: ある IoT ソリューションでは **50,000台のデバイス**からテレメトリーデータを収集する必要がある。 デバイスは継続的にセンサー情報を送信し、そのデータは次の目的で利用される。 出典: [[Azure IoT データ処理アーキテクチャ.md]]
- Azure IoT テレメトリストリーム処理設計 （5秒以内アラート処理 + 可変データ量）: IoT システムでは、多数のデバイスから継続的にテレメトリデータが送信される。これらのデータはリアルタイムに処理され、異常検知やアラート通知に利用されることが多い。例えば、工場設備監視、スマートビルディング、交通管理、環境センサーなどの分野では、センサーから送信されるデータを数秒以内に分析し、異常を検知することが重要になる。 出典: [[Azure IoT テレメトリストリーム処理設計.md]]

### 4.2 リアルタイム処理と可視化のユースケース

- Azure IoT センサーデータのリアルタイムダッシュボード設計 （Azure Event Hubs + Azure Time Series Insights）: 製造業や設備監視の分野では、センサーから生成される大量のデータをリアルタイムで収集し、状態監視や異常検知を行うことが重要になる。例えば、工場の設備には温度、振動、圧力、電流などのセンサーが取り付けられており、これらの値を継続的に収集することで、機器の異常や故障の兆候を早期に検知できる。 出典: [[Azure IoT センサーデータのリアルタイムダッシュボード設計.md]]
- IoT リアルタイムデータ処理パイプライン（Event Hubs + Stream Analytics + Power BI）: IoT ソリューションでは、多数のデバイスから継続的にデータが送信されます。例えば工場設備、センサー、車両、スマートメーターなどの IoT デバイスは、温度、位置、状態、パフォーマンスなどのテレメトリをリアルタイムで送信します。 出典: [[IoT リアルタイムデータ処理パイプライン（Event Hubs + Stream Analytics + Power BI）.md]]

### 4.3 機械学習推論のユースケース

- Azure Machine Learning によるリアルタイム不正検出モデルのデプロイ設計 （Managed Online Endpoint + GPU 推論）: 金融取引やオンライン決済システムでは、不正検出（Fraud Detection）のために機械学習モデルがリアルタイムで利用されることが多い。例えばクレジットカード取引では、ユーザーが決済を行った瞬間にその取引が不正であるかどうかを判定する必要がある。このような用途では、モデルの推論処理が数百ミリ秒以内に完了する必要があり、非常に低いレイテ... 出典: [[Azure Machine Learning によるリアルタイム不正検出モデルのデプロイ設計.md]]
- Azure Machine Learning モデルデプロイアーキテクチャ （Managed Endpoints / GPU推論 / REST API）: 企業が機械学習モデルを **REST API として公開**する場合、以下の要件がよく求められます。 出典: [[Azure Machine Learning モデルデプロイアーキテクチャ.md]]

### 4.4 動画と高度分析のユースケース

- Azure リアルタイム動画分析アーキテクチャ （複数カメラ映像の物体検出）: ある組織では、複数の監視カメラや IoT カメラから送信される **リアルタイム動画ストリーム**を分析し、特定の物体を検出するシステムを構築する必要がある。例えば、スマートシティ、工場の安全監視、交通監視、店舗分析などの分野では、カメラ映像をリアルタイムに分析し、人や車両、物体などを識別することが重要である。 出典: [[Azure リアルタイム動画分析アーキテクチャ.md]]

## 第5章 学習チェックポイント

- まず IoT データ取り込み → リアルタイム処理と可視化 → 機械学習推論 → 動画と高度分析 の順で読むと、基礎から応用まで流れを追いやすい。
- 第2章でサービスや構成要素の位置づけを理解し、第3章で制約条件を確認してから第4章のユースケースを読む。
- 同じサービスでも、要件によって選定理由が変わるため、出典リンク先の問題設定を必ず確認する。
- 類似ケースが複数ある場合は、第4章のユースケースを比較して判断軸を揃える。

## 関連用語

- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Event Hubs]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Explorer]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Power BI]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Lake Storage Gen2]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Cosmos DB]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Synapse Analytics]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Databricks]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Stream Analytics]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure IoT Hub]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Functions]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Kubernetes Service (AKS)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Container Registry]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Machine Learning]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Managed Online Endpoint]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Event Grid]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Logic Apps]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Service Bus]]

## 出典ドキュメント

- [[Azure IoT センサーデータのリアルタイムダッシュボード設計.md]]
- [[Azure IoT データ処理アーキテクチャ.md]]
- [[Azure IoT テレメトリストリーム処理設計.md]]
- [[Azure Machine Learning によるリアルタイム不正検出モデルのデプロイ設計.md]]
- [[Azure Machine Learning モデルデプロイアーキテクチャ.md]]
- [[Azure リアルタイム動画分析アーキテクチャ.md]]
- [[IoT リアルタイムデータ処理パイプライン（Event Hubs + Stream Analytics + Power BI）.md]]
