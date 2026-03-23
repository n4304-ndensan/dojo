# Azure用語集

この用語集は、再構築後の `Topic` と `Scenario` から最短で参照するための索引です。詳細な背景や元問題文は [[SourceIndex.md]] と [[Sources/]] 配下を参照します。

## Microsoft Entra ID

概要  
Azure の ID 基盤。

詳細  
ユーザー、グループ、アプリ登録、外部 ID、トークン発行、条件付きアクセスをまとめて扱う。

関連サービス  
Managed Identity / Azure Key Vault / Azure App Service / Azure API Management

## Managed Identity

概要  
Azure リソースに割り当てるワークロード ID。

詳細  
シークレットを埋め込まずに Azure リソースへ認証できる。AKS では Workload Identity、App Service や VM ではシステム割り当てまたはユーザー割り当てを使う。

関連サービス  
Microsoft Entra ID / Azure Key Vault / Azure SQL Database / Azure Storage

## Azure Key Vault

概要  
シークレット、キー、証明書を保護するサービス。

詳細  
RBAC と Entra ID を使ってアクセスを制御し、シークレットのバージョン管理やローテーションを行う。

関連サービス  
Managed Identity / Azure App Service / Azure Functions / Azure Virtual Machines

## Management Group

概要  
サブスクリプションを束ねるガバナンス階層。

詳細  
Azure Policy や RBAC を広いスコープで適用するときの起点になる。

関連サービス  
Azure Policy / Azure RBAC / Subscription

## Azure RBAC

概要  
Azure の標準権限制御モデル。

詳細  
管理グループ、サブスクリプション、リソースグループ、個別リソースの単位でロールを割り当てる。

関連サービス  
Management Group / Azure Policy / Managed Identity

## Azure Policy

概要  
構成ルールを評価し、監査・拒否・自動修復するサービス。

詳細  
リージョン制限、タグ強制、暗号化強制、AKS ガードレールなどを組織横断で適用する。

関連サービス  
Management Group / Azure RBAC / Azure Kubernetes Service (AKS)

## Azure Virtual Network

概要  
Azure の基本ネットワーク境界。

詳細  
サブネット、NSG、ルート、Private Endpoint、Peering、VPN/ExpressRoute を受ける基盤になる。

関連サービス  
Private Endpoint / Azure Firewall / VPN Gateway / ExpressRoute

## Azure Private Endpoint

概要  
PaaS にプライベート IP で到達させる Private Link 接続。

詳細  
VNet 内に NIC を作成し、Storage、SQL Database、Key Vault などへの通信を Microsoft バックボーンに閉じる。

関連サービス  
Azure Virtual Network / Azure DNS / Azure Storage / Azure SQL Database

## Azure Application Gateway

概要  
L7 ロードバランサ兼 Web アプリケーション ファイアウォール。

詳細  
HTTPS 終端、URL ベース ルーティング、WAF、AKS Ingress 連携に使う。

関連サービス  
Azure Front Door / Azure Kubernetes Service (AKS) / Web App

## Azure Front Door

概要  
グローバルなアプリ配信サービス。

詳細  
エッジでの TLS 終端、WAF、ルーティング最適化、ヘルスプローブによるリージョン フェイルオーバーを提供する。

関連サービス  
Azure Application Gateway / Traffic Manager / App Service

## Azure Kubernetes Service (AKS)

概要  
マネージド Kubernetes。

詳細  
クラスター基盤、ノードプール、HPA、Cluster Autoscaler、Ingress、Workload Identity を組み合わせてコンテナ基盤を構築する。

関連サービス  
Azure Container Registry / Azure Files / Managed Identity

## Azure App Service

概要  
Web アプリ向け PaaS 実行基盤。

詳細  
コード デプロイ、Easy Auth、Deployment Slots、マネージド証明書、監視をまとめて扱える。

関連サービス  
Microsoft Entra ID / Azure Key Vault / Application Insights

## Azure Functions

概要  
イベント駆動のサーバーレス実行基盤。

詳細  
HTTP、Queue、Service Bus、Event Hubs などのトリガーでコードを実行し、Consumption と Premium でスケール特性が変わる。

関連サービス  
Azure Service Bus / Azure Event Hubs / Managed Identity

## Azure Virtual Machines

概要  
IaaS の汎用コンピュート。

詳細  
OS とミドルウェアを細かく制御でき、SQL Server on VM、特殊ワークロード、レガシー移行で使う。

関連サービス  
Azure Backup / Azure Site Recovery / Azure Disk Storage

## Azure Storage Account

概要  
Storage 系サービスの論理コンテナ。

詳細  
Blob、Files、Queues、Tables の提供単位であり、冗長性、ネットワーク制御、暗号化、アクセス制御の判断起点になる。

関連サービス  
Azure Blob Storage / Azure Files / Azure Data Lake Storage Gen2

## Azure Blob Storage

概要  
オブジェクト ストレージ。

詳細  
Hot/Cool/Archive、バージョニング、WORM、ライフサイクル管理、SAS、Private Endpoint を組み合わせて設計する。

関連サービス  
Azure Storage Account / Azure Data Factory / Azure Key Vault

## Azure Files

概要  
SMB/NFS ベースのマネージド ファイル共有。

詳細  
AKS の RWX 永続ボリュームやオンプレミスとのハイブリッド共有に向く。

関連サービス  
Azure File Sync / Azure Kubernetes Service (AKS) / App Service

## Azure Data Lake Storage Gen2

概要  
分析向けの階層名前空間付きストレージ。

詳細  
HDFS 互換の名前空間を持ち、Data Factory、Databricks、Synapse の保存先として使う。

関連サービス  
Azure Databricks / Azure Synapse Analytics / Azure Data Factory

## Recovery Services Vault

概要  
バックアップと災害復旧の管理リソース。

詳細  
Azure Backup と Site Recovery のポリシー、保管、復旧操作を統合する。

関連サービス  
Azure Backup / Azure Site Recovery / Azure Virtual Machines

## Azure SQL Database

概要  
PaaS のリレーショナル データベース。

詳細  
vCore/DTU、Serverless、Elastic Pool、TDE、PITR、Failover Group を組み合わせて設計する。

関連サービス  
Azure Policy / Private Endpoint / Azure Cache for Redis

## Azure SQL Managed Instance

概要  
SQL Server 互換性を高く保った PaaS。

詳細  
インスタンス レベル機能や既存 SQL Server 互換性を優先するときに使う。

関連サービス  
Azure SQL Database / Azure Virtual Network / Azure Database Migration Service

## Azure Cosmos DB

概要  
グローバル分散型 NoSQL データベース。

詳細  
複数 API、整合性モデル、マルチリージョン複製、低レイテンシを提供する。

関連サービス  
Azure Event Hubs / Azure Functions / Azure Synapse Link

## Azure Cache for Redis

概要  
インメモリ キャッシュ。

詳細  
セッション共有、読み取り負荷軽減、マルチリージョン キャッシュ戦略に使う。

関連サービス  
Azure SQL Database / App Service / Front Door

## Azure API Management

概要  
API ゲートウェイ。

詳細  
トークン検証、レート制限、ポリシー変換、VNet 接続、開発者ポータルを提供する。

関連サービス  
Microsoft Entra ID / Logic Apps / Virtual Network

## Azure Logic Apps

概要  
ワークフロー統合サービス。

詳細  
SaaS コネクタ、オンプレミス データゲートウェイ、スケジュール実行、イベント起動をまとめて扱う。

関連サービス  
Azure API Management / Azure Service Bus / On-premises Data Gateway

## Azure Service Bus

概要  
エンタープライズ向けメッセージ ブローカー。

詳細  
Queue、Topic/Subscription、セッション、デッドレター、トランザクションで疎結合な非同期連携を実現する。

関連サービス  
Azure Functions / Logic Apps / API Management

## Azure Event Hubs

概要  
高スループットのイベント ストリーミング基盤。

詳細  
テレメトリ、ログ、IoT ストリームを大量に受け、Capture や Functions/Stream Analytics 連携で後段処理へ流す。

関連サービス  
Azure Functions / Azure Stream Analytics / Azure Cosmos DB

## Azure Event Grid

概要  
イベント通知ルーター。

詳細  
Azure リソースの状態変化を購読し、Functions や Logic Apps にプッシュする。

関連サービス  
Azure Functions / Azure Logic Apps / Azure Storage

## Azure Data Factory

概要  
データ統合パイプライン サービス。

詳細  
Copy Activity、Mapping Data Flow、Self-hosted Integration Runtime でオンプレミスと Azure をつなぐ。

関連サービス  
Azure Data Lake Storage Gen2 / Azure Synapse Analytics / SQL Database

## Azure Synapse Analytics

概要  
分析基盤を統合するサービス。

詳細  
SQL、Spark、Pipelines、Data Explorer 的な分析ワークロードを横断的に扱う。

関連サービス  
Azure Data Factory / Azure Data Lake Storage Gen2 / Azure Databricks

## Azure Databricks

概要  
Apache Spark ベースの分析プラットフォーム。

詳細  
データレイク処理、機械学習、メダリオン アーキテクチャ、Parquet/Delta Lake 運用に向く。

関連サービス  
Azure Data Lake Storage Gen2 / Azure Synapse Analytics / Azure Machine Learning

## Azure Monitor

概要  
Azure の監視基盤。

詳細  
メトリクス、ログ、アラート、診断設定、Application Insights、Log Analytics を束ねる。

関連サービス  
Log Analytics Workspace / Application Insights / AMPLS

## Log Analytics Workspace

概要  
Azure Monitor ログの中心保管先。

詳細  
AMA と DCR からログを受け、KQL で検索し、アラートや Workbook の土台になる。

関連サービス  
Azure Monitor / Azure Monitor Agent / AMPLS

## Application Insights

概要  
アプリケーション監視サービス。

詳細  
応答時間、失敗率、依存関係、分散トレーシング、可用性テストを取得する。

関連サービス  
Azure Monitor / Azure App Service / Azure Functions

