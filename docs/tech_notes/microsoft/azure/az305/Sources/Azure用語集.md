# Azure用語集

この用語集は `docs/tech_notes/microsoft/azure/az305/` 配下のドキュメントから Azure 関連用語を抽出し、表記ゆれを統一したものです。

## Microsoft Entra ID

概要
Azure の ID 管理基盤。

詳細
ユーザー、外部 ID、アプリ認証、トークン発行を統合管理する。

関連サービス
Conditional Access / Managed Identity

出典
- [[AKS から Azure SQL Database に安全に接続する認証方式.md]]
- [[AKS における Azure リソース認証アーキテクチャ.md]]
- [[Azure AD Connect によるハイブリッドID同期設計.md]]
- [[Azure AD と API Management を用いた内部 API セキュリティ設計.md]]
- [[Azure AD のユーザー作成およびロール割り当てイベントをキャプチャし、Azure Cosmos DB に保存する設計.md]]
- [[Azure AD を利用した ASP.NET Core アプリのユーザー委任アクセス設計.md]]
- [[Azure API Management の外部モードと同一 VNet 内バックエンド接続の判断.md]]
- [[Azure App Service で Key Vault の秘密を利用する設計.md]]
- [[Azure App Service と Microsoft Entra ID を利用した認証・認可アーキテクチャ.md]]
- [[Azure App Service 認証セキュリティ設計.md]]
- [[Azure Entra ID Governance（アクセスガバナンス）体系整理.md]]
- [[Azure Entra ID におけるシングルサインオン方式.md]]
- [[Azure Entra ID における外部ユーザーアクセス管理（B2B とアクセスレビュー）.md]]
- [[Azure Entra ID 外部ユーザーアクセス.md]]
- [[Azure Entra ID（Azure AD）アプリ登録と API ロールをトークンに含める設計.md]]
- [[Azure Identity Governance ドキュメント.md]]
- [[Azure Key Vault セキュアアクセス設計.md]]
- [[Azure Key Vault と Managed Identity を使用したシークレットアクセス制御.md]]
- [[Azure Key Vault を用いた API キー管理アーキテクチャ.md]]
- [[Azure Logic Apps からオンプレミス SQL Server へ接続するアーキテクチャ.md]]
- [[Azure VM 上の ASP.NET Core アプリから Key Vault を安全に利用する設計.md]]
- [[Azure サブスクリプションのテナント変更とアクセス管理.md]]
- [[Azure における金融機関向けプライベートブロックチェーン設計.md]]
- [[Azure ワークロード用 ID.md]]
- [[Azure 仮想マシンをインターネットから安全に管理する設計.md]]
- [[Azure 管理アクセスを MFA で保護する方法（Conditional Access）.md]]
- [[Azure 認証戦略設計ドキュメント.md]]
- [[Microsoft Entra ID ハイブリッド認証方式ドキュメント.md]]
- [[Microsoft Entra ID を利用した Azure Web アプリケーション認証アーキテクチャ.md]]
- [[SaaS アプリにおける Azure AD と OAuth 2.0 を利用した認証・認可設計.md]]
- [[Workday と Microsoft Entra ID を利用したユーザープロビジョニング設計ドキュメント.md]]
- [[ハイブリッド ID 環境でオンプレミス障害に耐える認証方式.md]]
- [[外部ユーザーアクセスの定期監査（Azure AD Access Reviews）.md]]

## Conditional Access

概要
条件付きアクセス機能。

詳細
ユーザー リスクやデバイス状態に応じてアクセス条件を制御する。

関連サービス
Microsoft Entra ID / MFA

出典
- [[Azure App Service と Microsoft Entra ID を利用した認証・認可アーキテクチャ.md]]
- [[Azure App Service 認証セキュリティ設計.md]]
- [[Azure Entra ID Governance（アクセスガバナンス）体系整理.md]]
- [[Azure Key Vault セキュアアクセス設計.md]]
- [[Azure 仮想マシンをインターネットから安全に管理する設計.md]]
- [[Azure 管理アクセスを MFA で保護する方法（Conditional Access）.md]]
- [[Azure 認証戦略設計ドキュメント.md]]
- [[Microsoft Entra ID を利用した Azure Web アプリケーション認証アーキテクチャ.md]]

## Multi-Factor Authentication (MFA)

概要
多要素認証。

詳細
追加要素を要求して高リスク サインインや管理アクセスを保護する。

関連サービス
Conditional Access / Microsoft Entra ID

出典
- [[Azure App Service と Microsoft Entra ID を利用した認証・認可アーキテクチャ.md]]
- [[Azure Entra ID Governance（アクセスガバナンス）体系整理.md]]
- [[Azure Entra ID 外部ユーザーアクセス.md]]
- [[Azure Identity Governance ドキュメント.md]]
- [[Azure Key Vault セキュアアクセス設計.md]]
- [[Azure Recovery Services Vault 技術ドキュメント.md]]
- [[Azure 仮想マシンをインターネットから安全に管理する設計.md]]
- [[Azure 管理アクセスを MFA で保護する方法（Conditional Access）.md]]
- [[Azure 認証戦略設計ドキュメント.md]]

## Role-Based Access Control (RBAC)

概要
Azure の権限管理モデル。

詳細
スコープ単位でロールを割り当て、最小権限を実装する。

関連サービス
Management Group / Managed Identity

出典
- [[AKS における Azure リソース認証アーキテクチャ.md]]
- [[Azure AD を利用した ASP.NET Core アプリのユーザー委任アクセス設計.md]]
- [[Azure ARM テンプレートデプロイ制御設計.md]]
- [[Azure Backup とハイブリッドバックアップアーキテクチャ.md]]
- [[Azure Blueprints による標準化デプロイ設計.md]]
- [[Azure Blueprints を利用した大規模 Azure 環境のガバナンス設計.md]]
- [[Azure Container Registry SKU 比較ドキュメント.md]]
- [[Azure Entra ID 外部ユーザーアクセス.md]]
- [[Azure Key Vault と Managed Identity を使用したシークレットアクセス制御.md]]
- [[Azure Key Vault を用いた API キー管理アーキテクチャ.md]]
- [[Azure Key Vault 技術ドキュメント.md]]
- [[Azure Monitor による VM セキュリティイベント監視.md]]
- [[Azure Policy による Azure SQL Database TDE 強制設計.md]]
- [[Azure Policy を管理グループで適用する理由（リージョン制限ポリシー）.md]]
- [[Azure RBAC とスコープ階層によるリソース作成権限の判断.md]]
- [[Azure Recovery Services Vault 技術ドキュメント.md]]
- [[Azure イベント駆動オートメーション.md]]
- [[Azure サブスクリプションのテナント変更とアクセス管理.md]]
- [[Azure における金融機関向けプライベートブロックチェーン設計.md]]
- [[管理グループ階層で仮想ネットワーク管理を委任する RBAC 設計.md]]

## Managed Identity

概要
Azure リソース用のマネージド ID。

詳細
シークレットを埋め込まずに Azure リソースへアクセスさせる。

関連サービス
Microsoft Entra ID / Azure Key Vault

出典
- [[AKS から Azure SQL Database に安全に接続する認証方式.md]]
- [[AKS における Azure リソース認証アーキテクチャ.md]]
- [[Azure AD を利用した ASP.NET Core アプリのユーザー委任アクセス設計.md]]
- [[Azure App Service で Key Vault の秘密を利用する設計.md]]
- [[Azure App Service 認証セキュリティ設計.md]]
- [[Azure ARM テンプレートデプロイ制御設計.md]]
- [[Azure Container Registry SKU 比較ドキュメント.md]]
- [[Azure Key Vault セキュアアクセス設計.md]]
- [[Azure Key Vault と Managed Identity を使用したシークレットアクセス制御.md]]
- [[Azure Key Vault を用いた API キー管理アーキテクチャ.md]]
- [[Azure Key Vault 技術ドキュメント.md]]
- [[Azure Policy による Azure SQL Database TDE 強制設計.md]]
- [[Azure VM 上の ASP.NET Core アプリから Key Vault を安全に利用する設計.md]]
- [[Azure ワークロード用 ID.md]]
- [[Microsoft Entra ID を利用した Azure Web アプリケーション認証アーキテクチャ.md]]

## Workload Identity

概要
AKS ワークロード向け ID 連携。

詳細
Kubernetes サービス アカウントと Entra ID を接続する。

関連サービス
Azure Kubernetes Service (AKS) / Managed Identity

出典
- [[AKS から Azure SQL Database に安全に接続する認証方式.md]]
- [[AKS における Azure リソース認証アーキテクチャ.md]]

## App Registration

概要
アプリ登録機能。

詳細
クライアント ID、リダイレクト URI、API 権限を定義する。

関連サービス
Microsoft Entra ID / Azure API Management

出典
- [[Azure AD と API Management を用いた内部 API セキュリティ設計.md]]
- [[Azure AD を利用した ASP.NET Core アプリのユーザー委任アクセス設計.md]]
- [[Azure App Service 認証セキュリティ設計.md]]
- [[Azure Entra ID 外部ユーザーアクセス.md]]
- [[Azure Entra ID（Azure AD）アプリ登録と API ロールをトークンに含める設計.md]]
- [[Microsoft Entra ID を利用した Azure Web アプリケーション認証アーキテクチャ.md]]
- [[Workday と Microsoft Entra ID を利用したユーザープロビジョニング設計ドキュメント.md]]

## Access Reviews

概要
アクセス棚卸し機能。

詳細
外部ユーザーや高権限アクセスの継続必要性を定期確認する。

関連サービス
Identity Governance / Microsoft Entra ID

出典
- [[Azure Entra ID Governance（アクセスガバナンス）体系整理.md]]
- [[Azure Entra ID における外部ユーザーアクセス管理（B2B とアクセスレビュー）.md]]
- [[Azure Identity Governance ドキュメント.md]]
- [[Azure 認証戦略設計ドキュメント.md]]
- [[外部ユーザーアクセスの定期監査（Azure AD Access Reviews）.md]]

## Identity Governance

概要
ID ライフサイクル管理機能。

詳細
アクセス要求、レビュー、プロビジョニングを通じて ID 運用を統制する。

関連サービス
Access Reviews / Microsoft Entra ID

出典
- [[Azure Entra ID Governance（アクセスガバナンス）体系整理.md]]
- [[Azure Entra ID における外部ユーザーアクセス管理（B2B とアクセスレビュー）.md]]
- [[Azure Identity Governance ドキュメント.md]]
- [[Azure 認証戦略設計ドキュメント.md]]
- [[外部ユーザーアクセスの定期監査（Azure AD Access Reviews）.md]]

## Azure AD Connect

概要
ハイブリッド ID 同期機能。

詳細
オンプレミス AD と Microsoft Entra ID の同期を行う。

関連サービス
Hybrid Identity / Microsoft Entra ID

出典
- [[Azure AD Connect によるハイブリッドID同期設計.md]]
- [[Azure Key Vault セキュアアクセス設計.md]]
- [[Microsoft Entra ID ハイブリッド認証方式ドキュメント.md]]
- [[Workday と Microsoft Entra ID を利用したユーザープロビジョニング設計ドキュメント.md]]
- [[ハイブリッド ID 環境でオンプレミス障害に耐える認証方式.md]]

## Hybrid Identity

概要
オンプレミスとクラウドをまたぐ ID 構成。

詳細
AD と Microsoft Entra ID を接続し、認証とユーザー管理を継続する。

関連サービス
Azure AD Connect / Microsoft Entra ID

出典
- [[ハイブリッド ID 環境でオンプレミス障害に耐える認証方式.md]]

## Azure Key Vault

概要
シークレットとキーの保護サービス。

詳細
シークレット、キー、証明書を集中管理し、アクセス制御を適用する。

関連サービス
Managed Identity / Azure App Service

出典
- [[AKS Ingress Controller 設計ドキュメント.md]]
- [[AKS から Azure SQL Database に安全に接続する認証方式.md]]
- [[AKS における Azure リソース認証アーキテクチャ.md]]
- [[Azure AD を利用した ASP.NET Core アプリのユーザー委任アクセス設計.md]]
- [[Azure App Service HTTPS 構成設計.md]]
- [[Azure App Service で Key Vault の秘密を利用する設計.md]]
- [[Azure App Service 認証セキュリティ設計.md]]
- [[Azure DevOps Pipeline を利用した AKS への継続的デプロイ（ACR イメージ更新トリガー）.md]]
- [[Azure Key Vault セキュアアクセス設計.md]]
- [[Azure Key Vault と Managed Identity を使用したシークレットアクセス制御.md]]
- [[Azure Key Vault を用いた API キー管理アーキテクチャ.md]]
- [[Azure Key Vault 技術ドキュメント.md]]
- [[Azure Monitor による VM セキュリティイベント監視.md]]
- [[Azure PaaS サービスへのプライベート接続設計（Private Endpoint）.md]]
- [[Azure SQL Managed Instance 接続設計ドキュメント.md]]
- [[Azure VM 上の ASP.NET Core アプリから Key Vault を安全に利用する設計.md]]
- [[Azure Web App から VNet リソースへ安全にアクセスする方法（Private Endpoint）.md]]
- [[Azure ハイブリッド接続アーキテクチャ設計ドキュメント.md]]
- [[Azure ワークロード用 ID.md]]
- [[Microsoft Entra ID を利用した Azure Web アプリケーション認証アーキテクチャ.md]]

## Azure Policy

概要
構成ガバナンス機能。

詳細
許可リージョンや暗号化などのルールを評価・適用する。

関連サービス
Management Group / Azure Resource Manager (ARM)

出典
- [[AKS コンテナセキュリティガバナンス設計.md]]
- [[Azure ARM テンプレートデプロイ制御設計.md]]
- [[Azure Blueprints による標準化デプロイ設計.md]]
- [[Azure Blueprints を利用した大規模 Azure 環境のガバナンス設計.md]]
- [[Azure Policy による Azure SQL Database TDE 強制設計.md]]
- [[Azure Policy を管理グループで適用する理由（リージョン制限ポリシー）.md]]
- [[Azure SQL Database の透過的データ暗号化（TDE）を自動修復付きで強制するガバナンス設計.md]]
- [[Microsoft Entra ID を利用した Azure Web アプリケーション認証アーキテクチャ.md]]

## Azure Blueprints

概要
標準構成の再利用機能。

詳細
ポリシー、ロール、テンプレートをまとめて標準環境を配布する。

関連サービス
Azure Policy / Azure Resource Manager (ARM)

出典
- [[Azure ARM テンプレートデプロイ制御設計.md]]
- [[Azure Blueprints による標準化デプロイ設計.md]]
- [[Azure Blueprints を利用した大規模 Azure 環境のガバナンス設計.md]]
- [[Azure SQL Database の透過的データ暗号化（TDE）を自動修復付きで強制するガバナンス設計.md]]

## Management Group

概要
サブスクリプション上位の管理単位。

詳細
複数サブスクリプションへポリシーや RBAC をまとめて適用する。

関連サービス
Azure Policy / RBAC

出典
- [[AKS における Azure リソース認証アーキテクチャ.md]]
- [[Azure ARM テンプレートデプロイ制御設計.md]]
- [[Azure Blueprints による標準化デプロイ設計.md]]
- [[Azure Blueprints を利用した大規模 Azure 環境のガバナンス設計.md]]
- [[Azure Policy を管理グループで適用する理由（リージョン制限ポリシー）.md]]
- [[Azure RBAC とスコープ階層によるリソース作成権限の判断.md]]
- [[Azure SQL Database の透過的データ暗号化（TDE）を自動修復付きで強制するガバナンス設計.md]]
- [[管理グループ階層で仮想ネットワーク管理を委任する RBAC 設計.md]]

## Azure Resource Manager (ARM)

概要
Azure のデプロイ制御レイヤー。

詳細
テンプレート、ポリシー、RBAC を通じて宣言的にリソースを管理する。

関連サービス
Azure Policy / Management Group

出典
- [[Azure ARM テンプレートデプロイ制御設計.md]]
- [[Azure Blueprints による標準化デプロイ設計.md]]
- [[Azure Blueprints を利用した大規模 Azure 環境のガバナンス設計.md]]
- [[Azure DevOps Pipeline を利用した AKS への継続的デプロイ（ACR イメージ更新トリガー）.md]]
- [[Azure Key Vault 技術ドキュメント.md]]
- [[Azure Policy による Azure SQL Database TDE 強制設計.md]]

## Azure App Service

概要
フルマネージド Web アプリ実行基盤。

詳細
Web アプリや API を PaaS としてホストし、認証や構成管理を提供する。

関連サービス
Deployment Slots / Application Insights

出典
- [[Azure App Service HTTPS 構成設計.md]]
- [[Azure App Service アプリケーション監視アーキテクチャ.md]]
- [[Azure App Service で Key Vault の秘密を利用する設計.md]]
- [[Azure App Service と Microsoft Entra ID を利用した認証・認可アーキテクチャ.md]]
- [[Azure App Service とオンプレミスファイルサーバーのハイブリッドファイル共有設計.md]]
- [[Azure App Service のゼロダウンタイムデプロイ（Deployment Slots）.md]]
- [[Azure App Service のパフォーマンス監視と依存関係トラッキング.md]]
- [[Azure App Service 構成管理アーキテクチャ設計.md]]
- [[Azure App Service 高可用性設計.md]]
- [[Azure App Service 認証セキュリティ設計.md]]
- [[Azure Logic Apps からオンプレミス SQL Server へ接続するアーキテクチャ.md]]
- [[Azure SQL Database のサービス層設計ガイド.md]]
- [[Azure Web App から VNet リソースへ安全にアクセスする方法（Private Endpoint）.md]]
- [[Azure での多層アプリケーション設計（Web層とデータベース層の最適化）.md]]
- [[DotNETアプリケーションのAzure移行設計.md]]
- [[Microsoft Entra ID を利用した Azure Web アプリケーション認証アーキテクチャ.md]]
- [[コンテナ化アプリケーションのためのマネージド Kubernetes（Azure Kubernetes Service）.md]]

## Deployment Slots

概要
App Service の段階的リリース機能。

詳細
ステージング環境で検証後にスワップで切り替える。

関連サービス
Azure App Service / Azure Front Door

出典
- [[Azure App Service のゼロダウンタイムデプロイ（Deployment Slots）.md]]

## Azure Functions

概要
イベント駆動のサーバーレス実行基盤。

詳細
小さな処理単位をイベントやスケジュールで実行し、自動スケールする。

関連サービス
Azure Service Bus / Azure Event Grid

出典
- [[Azure AD のユーザー作成およびロール割り当てイベントをキャプチャし、Azure Cosmos DB に保存する設計.md]]
- [[Azure Container Registry SKU 比較ドキュメント.md]]
- [[Azure Functions のホスティングプランとスケーリング（Consumption Plan）.md]]
- [[Azure FunctionsでService Busメッセージをコールドスタートなしで処理する設計.md]]
- [[Azure IoT テレメトリストリーム処理設計.md]]
- [[Azure Logic Apps とオンプレミス SAP システムのスケジュール統合.md]]
- [[Azure イベントメッセージングアーキテクチャ整理.md]]
- [[Azure イベント駆動オートメーション.md]]
- [[Azure データ分析アーキテクチャ整理.md]]
- [[Azure ワークロード用 ID.md]]
- [[Event Hubs Capture を利用したコールドパス分析アーキテクチャ設計.md]]
- [[IoT リアルタイムデータ処理パイプライン（Event Hubs + Stream Analytics + Power BI）.md]]

## Azure Virtual Machines

概要
IaaS 仮想マシン サービス。

詳細
OS やミドルウェアを含めた制御が必要なワークロードを実行する。

関連サービス
Availability Set / Azure Site Recovery

出典
- [[Azure AD を利用した ASP.NET Core アプリのユーザー委任アクセス設計.md]]
- [[Azure App Service アプリケーション監視アーキテクチャ.md]]
- [[Azure App Service とオンプレミスファイルサーバーのハイブリッドファイル共有設計.md]]
- [[Azure App Service 構成管理アーキテクチャ設計.md]]
- [[Azure Backup とハイブリッドバックアップアーキテクチャ.md]]
- [[Azure Batch におけるコスト最適化と HPC ワークロード設計.md]]
- [[Azure Batch による GPU バッチワークロードのコスト最適化.md]]
- [[Azure Data Factory 大容量データコピー設計.md]]
- [[Azure Dedicated Host による物理的分離インフラ設計.md]]
- [[Azure DevOps Pipeline を利用した AKS への継続的デプロイ（ACR イメージ更新トリガー）.md]]
- [[Azure Functions のホスティングプランとスケーリング（Consumption Plan）.md]]
- [[Azure FunctionsでService Busメッセージをコールドスタートなしで処理する設計.md]]
- [[Azure Key Vault を用いた API キー管理アーキテクチャ.md]]
- [[Azure Key Vault 技術ドキュメント.md]]
- [[Azure Kubernetes Service (AKS) スケーリング設計.md]]
- [[Azure Machine Learning によるリアルタイム不正検出モデルのデプロイ設計.md]]
- [[Azure Machine Learning モデルデプロイアーキテクチャ.md]]
- [[Azure Monitor _ Application Monitoring アーキテクチャ.md]]
- [[Azure Monitor Agent を利用したログ収集設.md]]
- [[Azure Monitor Private Link Scope（AMPLS）設計ガイド.md]]
- [[Azure Monitor による VM セキュリティイベント監視.md]]
- [[Azure Monitor による Windows VM ログ集中監視設計.md]]
- [[Azure RBAC とスコープ階層によるリソース作成権限の判断.md]]
- [[Azure Recovery Services Vault 技術ドキュメント.md]]
- [[Azure SQL Database のポイントインタイム復元（PITR）.md]]
- [[Azure SQL Database の購入モデルと展開設計.md]]
- [[Azure SQL Server 高可用性アーキテクチャ.md]]
- [[Azure SQL 移行サービス整理.md]]
- [[Azure VM 上で SQL Server を高性能かつ低コストで運用する設計.md]]
- [[Azure VM 上の ASP.NET Core アプリから Key Vault を安全に利用する設計.md]]
- [[Azure VM 上の SQL Server におけるディスクキャッシュ設定設計.md]]
- [[Azure イベントメッセージングアーキテクチャ整理.md]]
- [[Azure イベント駆動オートメーション.md]]
- [[Azure データウェアハウス ETL アーキテクチャ設計.md]]
- [[Azure での多層アプリケーション設計（Web層とデータベース層の最適化）.md]]
- [[Azure における金融機関向けプライベートブロックチェーン設計.md]]
- [[Azure ハイブリッド接続設計.md]]
- [[Azure への OLTP データベース移行アーキテクチャ設計.md]]
- [[Azure マルチテナントデータベース設計.md]]
- [[Azure ワークロード用 ID.md]]
- [[Azure 仮想マシンのディザスタリカバリ設計（Azure Site Recovery）.md]]
- [[Azure 仮想マシンのバックアップ戦略（RPO と不変バックアップ）.md]]
- [[Azure 仮想マシンの高可用性とパフォーマンスアーキテクチャ.md]]
- [[Azure 仮想マシンをインターネットから安全に管理する設計.md]]
- [[Azure 仮想マシン向けネットワークセキュリティ設計.md]]
- [[Azure 高可用性とディザスタリカバリー設計ドキュメント.md]]
- [[DotNETアプリケーションのAzure移行設計.md]]
- [[Event Hubs Capture を利用したコールドパス分析アーキテクチャ設計.md]]
- [[Linux VM 向けの高性能共有ファイルシステム（Azure NetApp Files）.md]]
- [[Microsoft Entra ID を利用した Azure Web アプリケーション認証アーキテクチャ.md]]
- [[SQL Server → Azure SQL Managed Instance 移行方式ガイド.md]]
- [[SQL Server Always On データベース移行設計ドキュメント.md]]
- [[VMware vSphere 仮想マシンを Azure へ移行するアーキテクチャ.md]]
- [[グローバル負荷分散アーキテクチャ.md]]
- [[マルチリージョン Azure アプリ設計.md]]
- [[可用性ゾーン障害に耐える Azure VM 配置設計（Dedicated Host.md]]
- [[Azure Batch ソリューションにおけるプールタイプと仮想マシン構成の選択]]

## Azure Batch

概要
大規模バッチ実行基盤。

詳細
並列処理や HPC ワークロード向けに計算ノードをまとめて確保する。

関連サービス
Azure Virtual Machines / Azure Storage Account

出典
- [[Azure Batch におけるコスト最適化と HPC ワークロード設計.md]]
- [[Azure Batch による GPU バッチワークロードのコスト最適化.md]]

## Azure Dedicated Host

概要
専有ホスト提供機能。

詳細
物理分離やライセンス要件に対応するため専有ホスト上に VM を配置する。

関連サービス
Azure Virtual Machines / Availability Zone

出典
- [[Azure Dedicated Host による物理的分離インフラ設計.md]]
- [[可用性ゾーン障害に耐える Azure VM 配置設計（Dedicated Host.md]]

## Availability Set

概要
同一データセンター内の冗長配置機能。

詳細
Fault Domain と Update Domain を分散して単一障害の影響を抑える。

関連サービス
Azure Virtual Machines / Availability Zone

出典
- [[Azure Dedicated Host による物理的分離インフラ設計.md]]
- [[Azure 高可用性とディザスタリカバリー設計ドキュメント.md]]
- [[Azure Batch ソリューションにおけるプールタイプと仮想マシン構成の選択]]

## Availability Zone

概要
リージョン内の物理分離ゾーン。

詳細
電源・ネットワーク・冷却が独立したゾーンに分散配置する。

関連サービス
Azure Virtual Machines / Azure SQL Database

出典
- [[Azure App Service 高可用性設計.md]]
- [[Azure SQL Database のサービス層設計（Business Critical）.md]]
- [[Azure 仮想マシンの高可用性とパフォーマンスアーキテクチャ.md]]
- [[Azure 高可用性とディザスタリカバリー設計ドキュメント.md]]
- [[可用性ゾーン障害に耐える Azure VM 配置設計（Dedicated Host.md]]
- [[Azure Batch ソリューションにおけるプールタイプと仮想マシン構成の選択]]

## Azure Kubernetes Service (AKS)

概要
マネージド Kubernetes サービス。

詳細
コンテナ化アプリを Kubernetes で運用し、スケーリングや更新を簡素化する。

関連サービス
Azure Container Registry / Workload Identity

出典
- [[AKS Ingress Controller 設計ドキュメント.md]]
- [[AKS から Azure SQL Database に安全に接続する認証方式.md]]
- [[AKS コンテナセキュリティガバナンス設計.md]]
- [[AKS ステートフルアプリケーションの永続ストレージ（Azure Files RWX）.md]]
- [[AKS ステートフルアプリケーションの共有ストレージ設計.md]]
- [[AKS における Azure リソース認証アーキテクチャ.md]]
- [[AKS マイクロサービスアーキテクチャ設計ガイド.md]]
- [[AKS マルチリージョンアプリケーションの安全なリリース戦略.md]]
- [[AKS を使用したリージョン障害耐性のある Kubernetes アーキテクチャ.md]]
- [[Azure Batch による GPU バッチワークロードのコスト最適化.md]]
- [[Azure Container Registry SKU 比較ドキュメント.md]]
- [[Azure DevOps Pipeline を利用した AKS への継続的デプロイ（ACR イメージ更新トリガー）.md]]
- [[Azure IoT テレメトリストリーム処理設計.md]]
- [[Azure Kubernetes Service (AKS) スケーリング設計.md]]
- [[Azure Machine Learning によるリアルタイム不正検出モデルのデプロイ設計.md]]
- [[Azure Machine Learning モデルデプロイアーキテクチャ.md]]
- [[Azure における金融機関向けプライベートブロックチェーン設計.md]]
- [[Azure 高可用性とディザスタリカバリー設計ドキュメント.md]]
- [[DotNETアプリケーションのAzure移行設計.md]]
- [[コンテナ化アプリケーションのためのマネージド Kubernetes（Azure Kubernetes Service）.md]]

## Azure Container Registry

概要
コンテナ イメージ レジストリ。

詳細
AKS や CI/CD と連携してイメージの保管と配布を行う。

関連サービス
Azure Kubernetes Service (AKS) / Managed Identity

出典
- [[AKS コンテナセキュリティガバナンス設計.md]]
- [[AKS における Azure リソース認証アーキテクチャ.md]]
- [[Azure App Service 構成管理アーキテクチャ設計.md]]
- [[Azure App Service 認証セキュリティ設計.md]]
- [[Azure Container Registry SKU 比較ドキュメント.md]]
- [[Azure DevOps Pipeline を利用した AKS への継続的デプロイ（ACR イメージ更新トリガー）.md]]
- [[Azure Machine Learning によるリアルタイム不正検出モデルのデプロイ設計.md]]
- [[コンテナ化アプリケーションのためのマネージド Kubernetes（Azure Kubernetes Service）.md]]

## Ingress Controller

概要
Kubernetes の入口制御コンポーネント。

詳細
HTTP/HTTPS 受信トラフィックを Kubernetes サービスへルーティングする。

関連サービス
Azure Kubernetes Service (AKS) / Azure Application Gateway

出典
- [[AKS Ingress Controller 設計ドキュメント.md]]
- [[AKS マイクロサービスアーキテクチャ設計ガイド.md]]

## Dapr

概要
分散アプリ機能の抽象化ランタイム。

詳細
サービス間通信、Pub/Sub、状態管理をサイドカーで提供する。

関連サービス
Azure Kubernetes Service (AKS) / Azure Service Bus

出典
- [[AKS マイクロサービスアーキテクチャ設計ガイド.md]]

## Istio

概要
Kubernetes 向けサービス メッシュ。

詳細
mTLS、トラフィック制御、可観測性をサービス間通信へ適用する。

関連サービス
Azure Kubernetes Service (AKS) / Azure Monitor

出典
- [[AKS マイクロサービスアーキテクチャ設計ガイド.md]]

## Azure Virtual Network (VNet)

概要
Azure の基本ネットワーク境界。

詳細
サブネット、ルーティング、セキュリティ制御を提供する。

関連サービス
Azure Private Endpoint / Azure DNS

出典
- [[AKS Ingress Controller 設計ドキュメント.md]]
- [[Azure API Management の外部モードと同一 VNet 内バックエンド接続の判断.md]]
- [[Azure Cache for Redis を用いたマルチリージョンWebアプリケーション設計.md]]
- [[Azure DevOps Pipeline を利用した AKS への継続的デプロイ（ACR イメージ更新トリガー）.md]]
- [[Azure ExpressRoute Global Reach を用いたマルチサイトネットワーク設計.md]]
- [[Azure FunctionsでService Busメッセージをコールドスタートなしで処理する設計.md]]
- [[Azure Key Vault 技術ドキュメント.md]]
- [[Azure Monitor Private Link Scope（AMPLS）設計ガイド.md]]
- [[Azure PaaS サービスへのプライベート接続設計（Private Endpoint）.md]]
- [[Azure Private Endpoint を使用した SQL Database の DNS 名前解決設計.md]]
- [[Azure SQL Managed Instance 接続設計ドキュメント.md]]
- [[Azure Standard Internal Load Balancer によるアプリケーション層のロードバランシング.md]]
- [[Azure Storage アカウントのネットワーク制御（Private Endpoint と Storage Firewall）.md]]
- [[Azure Virtual WAN 設計.md]]
- [[Azure Web App から VNet リソースへ安全にアクセスする方法（Private Endpoint）.md]]
- [[Azure グローバルルーティングおよびWeb配信サービス設計ドキュメント.md]]
- [[Azure ハイブリッド接続アーキテクチャ設計ドキュメント.md]]
- [[Azure ハイブリッド接続設計.md]]
- [[Azure 仮想マシンをインターネットから安全に管理する設計.md]]
- [[Azure 仮想マシン向けネットワークセキュリティ設計.md]]
- [[Linux VM 向けの高性能共有ファイルシステム（Azure NetApp Files）.md]]
- [[VMware vSphere 仮想マシンを Azure へ移行するアーキテクチャ.md]]
- [[マルチリージョン Azure アプリ設計.md]]

## Azure Front Door

概要
グローバル レイヤー 7 負荷分散サービス。

詳細
リージョンをまたぐ Web 配信、ヘルス判定、WAF 連携を提供する。

関連サービス
Azure App Service / Azure Application Gateway

出典
- [[AKS Ingress Controller 設計ドキュメント.md]]
- [[Azure App Service HTTPS 構成設計.md]]
- [[Azure App Service と Microsoft Entra ID を利用した認証・認可アーキテクチャ.md]]
- [[Azure App Service 高可用性設計.md]]
- [[Azure Standard Internal Load Balancer によるアプリケーション層のロードバランシング.md]]
- [[Azure グローバルルーティングおよびWeb配信サービス設計ドキュメント.md]]
- [[Azure 仮想マシン向けネットワークセキュリティ設計.md]]
- [[Azure 高可用性とディザスタリカバリー設計ドキュメント.md]]
- [[グローバル負荷分散アーキテクチャ.md]]
- [[マルチリージョン Web アプリケーションのグローバル負荷分散（Azure Front Door）.md]]
- [[マルチリージョン Web アプリケーションのグローバル負荷分散（Azure Front Door）2.md]]

## Azure Application Gateway

概要
アプリケーション層ロード バランサー。

詳細
URL ルーティング、TLS 終端、WAF を提供する。

関連サービス
Azure Front Door / Ingress Controller

出典
- [[AKS Ingress Controller 設計ドキュメント.md]]
- [[Azure グローバルルーティングおよびWeb配信サービス設計ドキュメント.md]]
- [[Azure 仮想マシンの高可用性とパフォーマンスアーキテクチャ.md]]
- [[グローバル負荷分散アーキテクチャ.md]]
- [[マルチリージョン Web アプリケーションのグローバル負荷分散（Azure Front Door）.md]]
- [[マルチリージョン Web アプリケーションのグローバル負荷分散（Azure Front Door）2.md]]

## Azure Load Balancer

概要
レイヤー 4 負荷分散サービス。

詳細
TCP/UDP ベースのトラフィックを VM 間で分散する。

関連サービス
Azure Virtual Machines / Azure Application Gateway

出典
- [[AKS Ingress Controller 設計ドキュメント.md]]
- [[Azure Kubernetes Service (AKS) スケーリング設計.md]]
- [[Azure SQL Server 高可用性アーキテクチャ.md]]
- [[Azure Standard Internal Load Balancer によるアプリケーション層のロードバランシング.md]]
- [[Azure 仮想マシンの高可用性とパフォーマンスアーキテクチャ.md]]
- [[グローバル負荷分散アーキテクチャ.md]]
- [[マルチリージョン Web アプリケーションのグローバル負荷分散（Azure Front Door）.md]]
- [[マルチリージョン Web アプリケーションのグローバル負荷分散（Azure Front Door）2.md]]

## Azure Private Endpoint

概要
PaaS 向けプライベート IP 接続。

詳細
VNet 内のプライベート IP 経由で Azure サービスへ到達する。

関連サービス
Azure Private Link / Azure DNS

出典
- [[Azure Key Vault 技術ドキュメント.md]]
- [[Azure Monitor Private Link Scope（AMPLS）設計ガイド.md]]
- [[Azure PaaS サービスへのプライベート接続設計（Private Endpoint）.md]]
- [[Azure Private Endpoint を使用した SQL Database の DNS 名前解決設計.md]]
- [[Azure SQL Managed Instance 接続設計ドキュメント.md]]
- [[Azure Storage アカウントのネットワーク制御（Private Endpoint と Storage Firewall）.md]]
- [[Azure Web App から VNet リソースへ安全にアクセスする方法（Private Endpoint）.md]]
- [[Azure ハイブリッド接続アーキテクチャ設計ドキュメント.md]]
- [[グローバル負荷分散アーキテクチャ.md]]

## Azure Private Link

概要
Azure サービスの私設接続基盤。

詳細
Azure バックボーン経由で PaaS へ安全に接続する。

関連サービス
Azure Private Endpoint / Azure Virtual Network (VNet)

出典
- [[Azure Container Registry SKU 比較ドキュメント.md]]
- [[Azure Monitor Agent を利用したログ収集設.md]]
- [[Azure Monitor Private Link Scope（AMPLS）設計ガイド.md]]
- [[Azure PaaS サービスへのプライベート接続設計（Private Endpoint）.md]]
- [[Azure SQL Managed Instance 接続設計ドキュメント.md]]
- [[Azure Storage アカウントのネットワーク制御（Private Endpoint と Storage Firewall）.md]]
- [[Azure Web App から VNet リソースへ安全にアクセスする方法（Private Endpoint）.md]]
- [[Azure ハイブリッド接続アーキテクチャ設計ドキュメント.md]]

## Azure ExpressRoute

概要
閉域接続サービス。

詳細
オンプレミスと Azure をインターネット非経由で接続する。

関連サービス
Azure Virtual WAN / Azure Virtual Network (VNet)

出典
- [[Azure ExpressRoute Global Reach を用いたマルチサイトネットワーク設計.md]]
- [[Azure Logic Apps からオンプレミス SQL Server へ接続するアーキテクチャ.md]]
- [[Azure Monitor Private Link Scope（AMPLS）設計ガイド.md]]
- [[Azure SQL Managed Instance 接続設計ドキュメント.md]]
- [[Azure Virtual WAN 設計.md]]
- [[Azure ハイブリッド接続アーキテクチャ設計ドキュメント.md]]
- [[Azure ハイブリッド接続設計.md]]
- [[ExpressRoute と VPN を組み合わせたハイブリッドネットワーク接続設計.md]]

## Azure Virtual WAN

概要
広域ネットワークのハブ基盤。

詳細
複数拠点や VNet をハブ型に接続し、ルーティングを集約する。

関連サービス
Azure ExpressRoute / VPN Gateway

出典
- [[Azure Virtual WAN 設計.md]]
- [[Azure ハイブリッド接続設計.md]]

## Azure DNS

概要
名前解決サービス。

詳細
パブリック/プライベート DNS ゾーンと Azure 内名前解決を管理する。

関連サービス
Azure Private Endpoint / Azure Virtual Network (VNet)

出典
- [[AKS を使用したリージョン障害耐性のある Kubernetes アーキテクチャ.md]]
- [[Azure App Service HTTPS 構成設計.md]]
- [[Azure App Service のゼロダウンタイムデプロイ（Deployment Slots）.md]]
- [[Azure App Service 高可用性設計.md]]
- [[Azure Monitor Private Link Scope（AMPLS）設計ガイド.md]]
- [[Azure Private Endpoint を使用した SQL Database の DNS 名前解決設計.md]]
- [[Azure SQL Managed Instance 接続設計ドキュメント.md]]
- [[Azure Standard Internal Load Balancer によるアプリケーション層のロードバランシング.md]]
- [[Azure Web App から VNet リソースへ安全にアクセスする方法（Private Endpoint）.md]]
- [[Azure グローバルルーティングおよびWeb配信サービス設計ドキュメント.md]]
- [[Azure 仮想マシンのディザスタリカバリ設計（Azure Site Recovery）.md]]
- [[Azure 仮想マシンの高可用性とパフォーマンスアーキテクチャ.md]]
- [[Azure 高可用性とディザスタリカバリー設計ドキュメント.md]]
- [[グローバル負荷分散アーキテクチャ.md]]
- [[マルチリージョン Web アプリケーションのグローバル負荷分散（Azure Front Door）.md]]
- [[マルチリージョン Web アプリケーションのグローバル負荷分散（Azure Front Door）2.md]]

## Azure Bastion

概要
ブラウザー経由の VM 管理接続。

詳細
RDP/SSH ポートを公開せずに VNet 内 VM を操作する。

関連サービス
Azure Virtual Machines / Azure Virtual Network (VNet)

出典
- [[Azure ハイブリッド接続設計.md]]
- [[Azure 仮想マシンをインターネットから安全に管理する設計.md]]

## Azure Storage Account

概要
Azure ストレージの管理単位。

詳細
Blob、File などのストレージ サービスをまとめて管理する。

関連サービス
Azure Blob Storage / Azure Files

出典
- [[AKS から Azure SQL Database に安全に接続する認証方式.md]]
- [[AKS ステートフルアプリケーションの共有ストレージ設計.md]]
- [[AKS における Azure リソース認証アーキテクチャ.md]]
- [[Azure Backup とハイブリッドバックアップアーキテクチャ.md]]
- [[Azure Blob Storage における WORM（Write Once Read Many）データ保持.md]]
- [[Azure Blob Storage における機密データの長期保持設計.md]]
- [[Azure Cache for Redis を用いたマルチリージョンWebアプリケーション設計.md]]
- [[Azure Functions のホスティングプランとスケーリング（Consumption Plan）.md]]
- [[Azure Key Vault と Managed Identity を使用したシークレットアクセス制御.md]]
- [[Azure Monitor による Windows VM ログ集中監視設計.md]]
- [[Azure PaaS サービスへのプライベート接続設計（Private Endpoint）.md]]
- [[Azure Recovery Services Vault 技術ドキュメント.md]]
- [[Azure SQL Database Diagnostics とログ保持設計.md]]
- [[Azure SQL Database のサービス層設計ガイド.md]]
- [[Azure SQL Managed Instance 接続設計ドキュメント.md]]
- [[Azure Storage アカウントのネットワーク制御（Private Endpoint と Storage Firewall）.md]]
- [[Azure Storage 設計ドキュメント.md]]
- [[Azure Synapse Pipelines と Azure Data Share を組み合わせたデータ統合・共有アーキテクチャ.md]]
- [[Azure ハイブリッド接続アーキテクチャ設計ドキュメント.md]]
- [[Azure への OLTP データベース移行アーキテクチャ設計.md]]
- [[Azure 高可用性とディザスタリカバリー設計ドキュメント.md]]
- [[Azure 大容量データ移行サービスまとめ.md]]
- [[Azure 大容量データ転送設計.md]]
- [[SQL Server から Azure へのデータ移行ツールの選択.md]]
- [[SQL Server から Azure へのデータ移行ツールの選択2.md]]
- [[オンプレミス NAS から Azure Data Lake Storage Gen2 へのデータ移行設計.md]]
- [[大規模ログ分析に適した Azure サービス（Azure Data Explorer）.md]]

## Azure Blob Storage

概要
オブジェクト ストレージ サービス。

詳細
非構造データ、バックアップ、アーカイブを大容量に格納する。

関連サービス
Azure Storage Account / WORM

出典
- [[AKS ステートフルアプリケーションの共有ストレージ設計.md]]
- [[Azure App Service とオンプレミスファイルサーバーのハイブリッドファイル共有設計.md]]
- [[Azure Blob Storage における WORM（Write Once Read Many）データ保持.md]]
- [[Azure Blob Storage における機密データの長期保持設計.md]]
- [[Azure Data Factory を用いたオンプレミスファイルサーバーから Azure Blob Storage へのデータ移行設計.md]]
- [[Azure Storage 設計ドキュメント.md]]
- [[Azure 大容量データ移行サービスまとめ.md]]
- [[Azure 大容量データ転送設計.md]]
- [[Event Hubs Capture を利用したコールドパス分析アーキテクチャ設計.md]]
- [[Linux VM 向けの高性能共有ファイルシステム（Azure NetApp Files）.md]]
- [[SQL Server から Azure へのデータ移行ツールの選択.md]]
- [[SQL Server から Azure へのデータ移行ツールの選択2.md]]
- [[オンプレミス NAS から Azure Data Lake Storage Gen2 へのデータ移行設計.md]]

## Azure Files

概要
共有ファイル ストレージ。

詳細
SMB/NFS ベースの共有を提供し、AKS やハイブリッド共有に使う。

関連サービス
Azure Storage Account / Azure Kubernetes Service (AKS)

出典
- [[AKS ステートフルアプリケーションの永続ストレージ（Azure Files RWX）.md]]
- [[AKS ステートフルアプリケーションの共有ストレージ設計.md]]
- [[Azure App Service とオンプレミスファイルサーバーのハイブリッドファイル共有設計.md]]
- [[Azure Backup とハイブリッドバックアップアーキテクチャ.md]]
- [[Azure Data Factory を用いたオンプレミスファイルサーバーから Azure Blob Storage へのデータ移行設計.md]]
- [[Azure Recovery Services Vault 技術ドキュメント.md]]
- [[Azure Storage 設計ドキュメント.md]]
- [[Event Hubs Capture を利用したコールドパス分析アーキテクチャ設計.md]]
- [[Linux VM 向けの高性能共有ファイルシステム（Azure NetApp Files）.md]]
- [[オンプレミス NAS から Azure Data Lake Storage Gen2 へのデータ移行設計.md]]

## Azure Data Lake Storage Gen2

概要
分析向け階層型ストレージ。

詳細
階層構造と高スループットを持つデータレイク基盤を提供する。

関連サービス
Azure Blob Storage / Azure Synapse Analytics

出典
- [[Azure Databricks 分析基盤におけるデータレイクファイル形式設計.md]]
- [[Azure IoT データ処理アーキテクチャ.md]]
- [[Azure Synapse Pipelines と Azure Data Share を組み合わせたデータ統合・共有アーキテクチャ.md]]
- [[Azure データエンジニアリングアーキテクチャ.md]]
- [[Azure メダリオンアーキテクチャのデータレイク設計.md]]
- [[Event Hubs Capture を利用したコールドパス分析アーキテクチャ設計.md]]
- [[オンプレミス NAS から Azure Data Lake Storage Gen2 へのデータ移行設計.md]]

## Azure NetApp Files

概要
高性能ファイル ストレージ。

詳細
低レイテンシと高スループットが必要な共有ファイル用途に使う。

関連サービス
Azure Virtual Machines / Azure Files

出典
- [[AKS ステートフルアプリケーションの共有ストレージ設計.md]]
- [[Linux VM 向けの高性能共有ファイルシステム（Azure NetApp Files）.md]]

## Azure Backup

概要
マネージド バックアップ サービス。

詳細
VM、SQL、ファイル共有などのバックアップと保持ポリシーを管理する。

関連サービス
Recovery Services Vault / Azure Site Recovery

出典
- [[Azure App Service 高可用性設計.md]]
- [[Azure Backup とハイブリッドバックアップアーキテクチャ.md]]
- [[Azure Dedicated Host による物理的分離インフラ設計.md]]
- [[Azure Recovery Services Vault 技術ドキュメント.md]]
- [[Azure 仮想マシンのディザスタリカバリ設計（Azure Site Recovery）.md]]
- [[Azure 仮想マシンのバックアップ戦略（RPO と不変バックアップ）.md]]
- [[Azure 高可用性とディザスタリカバリー設計ドキュメント.md]]
- [[RTO・RPO 要件に基づく Azure 災害復旧（DR）ソリューションの選択.md]]
- [[Azure Batch ソリューションにおけるプールタイプと仮想マシン構成の選択]]

## Recovery Services Vault

概要
バックアップと復旧の管理コンテナー。

詳細
Azure Backup や Site Recovery の保護データと構成を保持する。

関連サービス
Azure Backup / Azure Site Recovery

出典
- [[Azure Backup とハイブリッドバックアップアーキテクチャ.md]]
- [[Azure Recovery Services Vault 技術ドキュメント.md]]
- [[Azure 仮想マシンのバックアップ戦略（RPO と不変バックアップ）.md]]
- [[RTO・RPO 要件に基づく Azure 災害復旧（DR）ソリューションの選択.md]]

## Azure Site Recovery

概要
ディザスタ リカバリー サービス。

詳細
レプリケーションとフェイルオーバーで DR を実現する。

関連サービス
Azure Virtual Machines / Recovery Services Vault

出典
- [[Azure Backup とハイブリッドバックアップアーキテクチャ.md]]
- [[Azure Recovery Services Vault 技術ドキュメント.md]]
- [[Azure 仮想マシンのディザスタリカバリ設計（Azure Site Recovery）.md]]
- [[Azure 仮想マシンのバックアップ戦略（RPO と不変バックアップ）.md]]
- [[Azure 高可用性とディザスタリカバリー設計ドキュメント.md]]
- [[RTO・RPO 要件に基づく Azure 災害復旧（DR）ソリューションの選択.md]]

## WORM

概要
変更不可保持機能。

詳細
一定期間データの変更や削除を禁止して保持する。

関連サービス
Azure Blob Storage / Azure Storage Account

出典
- [[Azure Blob Storage における WORM（Write Once Read Many）データ保持.md]]
- [[Azure Blob Storage における機密データの長期保持設計.md]]
- [[Azure 仮想マシンのバックアップ戦略（RPO と不変バックアップ）.md]]

## RTO

概要
復旧時間目標。

詳細
障害から再開までに許容される最大時間を示す。

関連サービス
RPO / Azure Site Recovery

出典
- [[Azure App Service 高可用性設計.md]]
- [[Azure 仮想マシンのディザスタリカバリ設計（Azure Site Recovery）.md]]
- [[Azure 高可用性とディザスタリカバリー設計ドキュメント.md]]
- [[RTO・RPO 要件に基づく Azure 災害復旧（DR）ソリューションの選択.md]]

## RPO

概要
復旧時点目標。

詳細
障害時に許容できるデータ損失量を時間で表す。

関連サービス
RTO / Azure Backup

出典
- [[Azure 仮想マシンのディザスタリカバリ設計（Azure Site Recovery）.md]]
- [[Azure 仮想マシンのバックアップ戦略（RPO と不変バックアップ）.md]]
- [[Azure 高可用性とディザスタリカバリー設計ドキュメント.md]]
- [[RTO・RPO 要件に基づく Azure 災害復旧（DR）ソリューションの選択.md]]

## Azure SQL Database

概要
フルマネージド リレーショナル データベース。

詳細
可用性、暗号化、スケーリングを備えた SQL PaaS を提供する。

関連サービス
TDE / Failover Group

出典
- [[AKS から Azure SQL Database に安全に接続する認証方式.md]]
- [[Azure Key Vault と Managed Identity を使用したシークレットアクセス制御.md]]
- [[Azure PaaS サービスへのプライベート接続設計（Private Endpoint）.md]]
- [[Azure Policy による Azure SQL Database TDE 強制設計.md]]
- [[Azure Private Endpoint を使用した SQL Database の DNS 名前解決設計.md]]
- [[Azure SQL Database Diagnostics とログ保持設計.md]]
- [[Azure SQL Database データ保護・暗号化機能整理.md]]
- [[Azure SQL Database のクエリパフォーマンス自動改善.md]]
- [[Azure SQL Database のサービス層設計（Business Critical）.md]]
- [[Azure SQL Database のサービス層設計ガイド.md]]
- [[Azure SQL Database のポイントインタイム復元（PITR）.md]]
- [[Azure SQL Database の購入モデルと展開設計.md]]
- [[Azure SQL Database の診断設定とログ送信先の制約.md]]
- [[Azure SQL Database の診断設定とログ送信先の制約2.md]]
- [[Azure SQL Database の透過的データ暗号化（TDE）を自動修復付きで強制するガバナンス設計.md]]
- [[Azure SQL Database 購入モデル設計ドキュメント.md]]
- [[Azure SQL Database 設計ドキュメント.md]]
- [[Azure SQL Server 高可用性アーキテクチャ.md]]
- [[Azure SQL 移行サービス整理.md]]
- [[Azure Synapse Pipelines と Azure Data Share を組み合わせたデータ統合・共有アーキテクチャ.md]]
- [[Azure Web App から VNet リソースへ安全にアクセスする方法（Private Endpoint）.md]]
- [[Azure データウェアハウス ETL アーキテクチャ設計.md]]
- [[Azure での多層アプリケーション設計（Web層とデータベース層の最適化）.md]]
- [[Azure ハイブリッド接続アーキテクチャ設計ドキュメント.md]]
- [[Azure への OLTP データベース移行アーキテクチャ設計.md]]
- [[Azure マルチテナントデータベース設計.md]]
- [[Azure ワークロード用 ID.md]]
- [[Azure 高可用性とディザスタリカバリー設計ドキュメント.md]]
- [[SQL Server から Azure へのデータ移行ツールの選択.md]]
- [[SQL Server から Azure へのデータ移行ツールの選択2.md]]
- [[大規模ログ分析に適した Azure サービス（Azure Data Explorer）.md]]

## Azure SQL Managed Instance

概要
SQL Server 互換性を高めた PaaS DB。

詳細
既存 SQL Server 資産の移行に適したマネージド インスタンスを提供する。

関連サービス
Azure SQL Database / Azure Virtual Network (VNet)

出典
- [[Azure SQL Database の購入モデルと展開設計.md]]
- [[Azure SQL Managed Instance 接続設計ドキュメント.md]]
- [[Azure SQL 移行サービス整理.md]]
- [[Azure への OLTP データベース移行アーキテクチャ設計.md]]
- [[SQL Server → Azure SQL Managed Instance 移行方式ガイド.md]]

## Transparent Data Encryption (TDE)

概要
保存時暗号化機能。

詳細
データベース ファイルやバックアップを暗号化する。

関連サービス
Azure SQL Database / Azure Policy

出典
- [[Azure Policy による Azure SQL Database TDE 強制設計.md]]
- [[Azure SQL Database データ保護・暗号化機能整理.md]]
- [[Azure SQL Database の透過的データ暗号化（TDE）を自動修復付きで強制するガバナンス設計.md]]
- [[Azure SQL Database 設計ドキュメント.md]]

## Always Encrypted

概要
列レベル暗号化機能。

詳細
機密列をクライアント側で暗号化して DB 管理者からも保護する。

関連サービス
Azure SQL Database / Azure Key Vault

出典
- [[Azure SQL Database データ保護・暗号化機能整理.md]]
- [[Azure SQL Database 設計ドキュメント.md]]

## Failover Group

概要
複数リージョン冗長化機能。

詳細
自動フェイルオーバーと固定エンドポイントで継続性を確保する。

関連サービス
Azure SQL Database / Availability Zone

出典
- [[Azure SQL Database 設計ドキュメント.md]]
- [[Azure SQL Server 高可用性アーキテクチャ.md]]

## Point-in-Time Restore (PITR)

概要
特定時点への復元機能。

詳細
バックアップ履歴から任意の時点へデータベースを戻す。

関連サービス
Azure SQL Database / Azure Backup

出典
- [[Azure SQL Database のポイントインタイム復元（PITR）.md]]

## Azure Cosmos DB

概要
グローバル分散 NoSQL データベース。

詳細
低レイテンシ、一貫性選択、マルチリージョン構成を提供する。

関連サービス
Analytical Store / Azure Synapse Analytics

出典
- [[AKS における Azure リソース認証アーキテクチャ.md]]
- [[Azure AD のユーザー作成およびロール割り当てイベントをキャプチャし、Azure Cosmos DB に保存する設計.md]]
- [[Azure Blueprints による標準化デプロイ設計.md]]
- [[Azure Cosmos DB Analytical Store と Azure Synapse Analytics を用いた Near Real-Time 分析アーキテクチャ.md]]
- [[Azure Cosmos DB API ドキュメント.md]]
- [[Azure Cosmos DB における一貫性モデルとマルチリージョン書き込み設計.md]]
- [[Azure IoT データ処理アーキテクチャ.md]]
- [[Azure SQL Database のポイントインタイム復元（PITR）.md]]
- [[Azure データ分析アーキテクチャ整理.md]]
- [[Azure ハイブリッド接続アーキテクチャ設計ドキュメント.md]]
- [[Azure マルチテナントデータベース設計.md]]
- [[IoT リアルタイムデータ処理パイプライン（Event Hubs + Stream Analytics + Power BI）.md]]
- [[SQL Server から Azure へのデータ移行ツールの選択.md]]
- [[SQL Server から Azure へのデータ移行ツールの選択2.md]]

## Azure Cache for Redis

概要
インメモリ キャッシュ サービス。

詳細
読み取り負荷軽減やセッション共有に使う。

関連サービス
Azure App Service / Azure Virtual Network (VNet)

出典
- [[AKS における Azure リソース認証アーキテクチャ.md]]
- [[AKS マイクロサービスアーキテクチャ設計ガイド.md]]
- [[Azure App Service アプリケーション監視アーキテクチャ.md]]
- [[Azure Cache for Redis を用いたマルチリージョンWebアプリケーション設計.md]]
- [[Azure Monitor _ Application Monitoring アーキテクチャ.md]]

## Azure API Management

概要
API ゲートウェイ サービス。

詳細
API の公開、保護、変換、レート制限を一元化する。

関連サービス
Microsoft Entra ID / Azure Virtual Network (VNet)

出典
- [[Azure AD と API Management を用いた内部 API セキュリティ設計.md]]
- [[Azure API Management の外部モードと同一 VNet 内バックエンド接続の判断.md]]
- [[Azure App Service と Microsoft Entra ID を利用した認証・認可アーキテクチャ.md]]
- [[Azure Logic Apps とオンプレミス SAP システムのスケジュール統合.md]]

## Azure Logic Apps

概要
ワークフロー自動化サービス。

詳細
SaaS やオンプレミスをコネクタで接続し、自動処理を構成する。

関連サービス
Azure API Management / Azure Service Bus

出典
- [[Azure AD のユーザー作成およびロール割り当てイベントをキャプチャし、Azure Cosmos DB に保存する設計.md]]
- [[Azure Logic Apps からオンプレミス SQL Server へ接続するアーキテクチャ.md]]
- [[Azure Logic Apps とオンプレミス SAP システムのスケジュール統合.md]]
- [[Azure イベント駆動オートメーション.md]]
- [[Azure データウェアハウス ETL アーキテクチャ設計.md]]
- [[IoT リアルタイムデータ処理パイプライン（Event Hubs + Stream Analytics + Power BI）.md]]
- [[SAP HANA → Azure Synapse Analytics 増分データパイプライン設計.md]]

## Azure Service Bus

概要
エンタープライズ メッセージング サービス。

詳細
キューやトピックで信頼性の高い非同期連携を実現する。

関連サービス
Azure Functions / Azure Logic Apps

出典
- [[AKS における Azure リソース認証アーキテクチャ.md]]
- [[AKS マイクロサービスアーキテクチャ設計ガイド.md]]
- [[Azure FunctionsでService Busメッセージをコールドスタートなしで処理する設計.md]]
- [[Azure Kubernetes Service (AKS) スケーリング設計.md]]
- [[Azure Logic Apps とオンプレミス SAP システムのスケジュール統合.md]]
- [[Azure イベントメッセージングアーキテクチャ整理.md]]
- [[Azure イベント駆動オートメーション.md]]
- [[IoT リアルタイムデータ処理パイプライン（Event Hubs + Stream Analytics + Power BI）.md]]

## Azure Event Hubs

概要
高スループット イベント取り込みサービス。

詳細
大量テレメトリやログをストリームとして受け取る。

関連サービス
Azure Stream Analytics / Azure IoT Hub

出典
- [[Azure AD のユーザー作成およびロール割り当てイベントをキャプチャし、Azure Cosmos DB に保存する設計.md]]
- [[Azure IoT センサーデータのリアルタイムダッシュボード設計.md]]
- [[Azure IoT データ処理アーキテクチャ.md]]
- [[Azure IoT テレメトリストリーム処理設計.md]]
- [[Azure Monitor による Windows VM ログ集中監視設計.md]]
- [[Azure SQL Database Diagnostics とログ保持設計.md]]
- [[Azure SQL Database の診断設定とログ送信先の制約.md]]
- [[Azure SQL Database の診断設定とログ送信先の制約2.md]]
- [[Azure イベントメッセージングアーキテクチャ整理.md]]
- [[Azure イベント駆動オートメーション.md]]
- [[Azure データ分析アーキテクチャ整理.md]]
- [[Event Hubs Capture を利用したコールドパス分析アーキテクチャ設計.md]]
- [[IoT リアルタイムデータ処理パイプライン（Event Hubs + Stream Analytics + Power BI）.md]]

## Azure Event Grid

概要
イベント ルーティング サービス。

詳細
リソース発生イベントを購読者へ配信して自動化につなげる。

関連サービス
Azure Functions / Azure Logic Apps

出典
- [[Azure AD のユーザー作成およびロール割り当てイベントをキャプチャし、Azure Cosmos DB に保存する設計.md]]
- [[Azure Logic Apps とオンプレミス SAP システムのスケジュール統合.md]]
- [[Azure イベントメッセージングアーキテクチャ整理.md]]
- [[Azure イベント駆動オートメーション.md]]
- [[Azure リアルタイム動画分析アーキテクチャ.md]]

## Azure Monitor

概要
統合監視プラットフォーム。

詳細
メトリック、ログ、アラート、ダッシュボードを提供する。

関連サービス
Application Insights / Log Analytics

出典
- [[AKS Ingress Controller 設計ドキュメント.md]]
- [[AKS マルチリージョンアプリケーションの安全なリリース戦略.md]]
- [[Azure AD のユーザー作成およびロール割り当てイベントをキャプチャし、Azure Cosmos DB に保存する設計.md]]
- [[Azure App Service アプリケーション監視アーキテクチャ.md]]
- [[Azure App Service のパフォーマンス監視と依存関係トラッキング.md]]
- [[Azure Backup とハイブリッドバックアップアーキテクチャ.md]]
- [[Azure DevOps Pipeline を利用した AKS への継続的デプロイ（ACR イメージ更新トリガー）.md]]
- [[Azure Monitor _ Application Monitoring アーキテクチャ.md]]
- [[Azure Monitor Agent を利用したログ収集設.md]]
- [[Azure Monitor Private Link Scope（AMPLS）設計ガイド.md]]
- [[Azure Monitor による VM セキュリティイベント監視.md]]
- [[Azure Monitor による Windows VM ログ集中監視設計.md]]
- [[Azure Recovery Services Vault 技術ドキュメント.md]]
- [[Azure SQL Database Diagnostics とログ保持設計.md]]
- [[Azure SQL Database の診断設定とログ送信先の制約.md]]
- [[Azure SQL Database の診断設定とログ送信先の制約2.md]]
- [[Azure 仮想マシン向けネットワークセキュリティ設計.md]]
- [[コンテナ化アプリケーションのためのマネージド Kubernetes（Azure Kubernetes Service）.md]]
- [[大規模ログ分析に適した Azure サービス（Azure Data Explorer）.md]]

## Application Insights

概要
アプリケーション可観測性サービス。

詳細
レスポンス時間、依存関係、例外、ユーザー行動を追跡する。

関連サービス
Azure Monitor / Log Analytics

出典
- [[AKS マルチリージョンアプリケーションの安全なリリース戦略.md]]
- [[Azure App Service アプリケーション監視アーキテクチャ.md]]
- [[Azure App Service のパフォーマンス監視と依存関係トラッキング.md]]
- [[Azure Monitor _ Application Monitoring アーキテクチャ.md]]
- [[Azure Monitor Private Link Scope（AMPLS）設計ガイド.md]]

## Log Analytics

概要
ログ分析エンジン。

詳細
Workspace に集約したログを KQL で横断分析する。

関連サービス
Azure Monitor / Azure Monitor Agent

出典
- [[Azure AD のユーザー作成およびロール割り当てイベントをキャプチャし、Azure Cosmos DB に保存する設計.md]]
- [[Azure App Service アプリケーション監視アーキテクチャ.md]]
- [[Azure Monitor _ Application Monitoring アーキテクチャ.md]]
- [[Azure Monitor Agent を利用したログ収集設.md]]
- [[Azure Monitor Private Link Scope（AMPLS）設計ガイド.md]]
- [[Azure Monitor による VM セキュリティイベント監視.md]]
- [[Azure Monitor による Windows VM ログ集中監視設計.md]]
- [[Azure SQL Database Diagnostics とログ保持設計.md]]
- [[Azure SQL Database の診断設定とログ送信先の制約.md]]
- [[Azure SQL Database の診断設定とログ送信先の制約2.md]]
- [[Azure 仮想マシン向けネットワークセキュリティ設計.md]]
- [[大規模ログ分析に適した Azure サービス（Azure Data Explorer）.md]]

## Azure Monitor Agent

概要
ログ収集エージェント。

詳細
VM などから Azure Monitor と Log Analytics へデータを送る。

関連サービス
Azure Monitor / Log Analytics

出典
- [[Azure Monitor Agent を利用したログ収集設.md]]
- [[Azure Monitor による VM セキュリティイベント監視.md]]
- [[Azure Monitor による Windows VM ログ集中監視設計.md]]

## Azure Monitor Private Link Scope (AMPLS)

概要
Azure Monitor の私設接続スコープ。

詳細
Azure Monitor 関連サービスへの通信を Private Link 経由に集約する。

関連サービス
Azure Monitor / Azure Private Link

出典
- [[Azure Monitor Private Link Scope（AMPLS）設計ガイド.md]]

## Azure Data Factory

概要
データ統合と ETL/ELT サービス。

詳細
コピー、変換、スケジュール実行でデータ移送を制御する。

関連サービス
Azure Synapse Analytics / Azure Blob Storage

出典
- [[Azure Data Factory を用いたオンプレミスファイルサーバーから Azure Blob Storage へのデータ移行設計.md]]
- [[Azure Data Factory 大容量データコピー設計.md]]
- [[Azure Synapse Pipelines と Azure Data Share を組み合わせたデータ統合・共有アーキテクチャ.md]]
- [[Azure データウェアハウス ETL アーキテクチャ設計.md]]
- [[Azure データエンジニアリングアーキテクチャ.md]]
- [[Azure データ分析アーキテクチャ整理.md]]
- [[Azure 大容量データ移行サービスまとめ.md]]
- [[Azure 大容量データ転送設計.md]]
- [[SAP HANA → Azure Synapse Analytics 増分データパイプライン設計.md]]
- [[SQL Server から Azure へのデータ移行ツールの選択.md]]
- [[SQL Server から Azure へのデータ移行ツールの選択2.md]]

## Azure Synapse Analytics

概要
分析統合プラットフォーム。

詳細
SQL、Spark、パイプラインを統合し、分析基盤を一体管理する。

関連サービス
Azure Data Lake Storage Gen2 / Azure Data Factory

出典
- [[Azure Cosmos DB Analytical Store と Azure Synapse Analytics を用いた Near Real-Time 分析アーキテクチャ.md]]
- [[Azure Data Factory 大容量データコピー設計.md]]
- [[Azure IoT データ処理アーキテクチャ.md]]
- [[Azure IoT テレメトリストリーム処理設計.md]]
- [[Azure SQL 移行サービス整理.md]]
- [[Azure Synapse Analytics におけるデータレイク分析アーキテクチャ設計.md]]
- [[Azure Synapse Pipelines と Azure Data Share を組み合わせたデータ統合・共有アーキテクチャ.md]]
- [[Azure データウェアハウス ETL アーキテクチャ設計.md]]
- [[Azure データエンジニアリングアーキテクチャ.md]]
- [[Azure データ分析アーキテクチャ整理.md]]
- [[Azure への OLTP データベース移行アーキテクチャ設計.md]]
- [[Event Hubs Capture を利用したコールドパス分析アーキテクチャ設計.md]]
- [[SAP HANA → Azure Synapse Analytics 増分データパイプライン設計.md]]
- [[オンプレミス NAS から Azure Data Lake Storage Gen2 へのデータ移行設計.md]]

## Azure Databricks

概要
Spark ベース分析プラットフォーム。

詳細
データ エンジニアリングや機械学習の大規模処理を行う。

関連サービス
Azure Data Lake Storage Gen2 / Azure Synapse Analytics

出典
- [[Azure Databricks 分析基盤におけるデータレイクファイル形式設計.md]]
- [[Azure IoT データ処理アーキテクチャ.md]]
- [[Azure IoT テレメトリストリーム処理設計.md]]
- [[Azure Machine Learning モデルデプロイアーキテクチャ.md]]
- [[Azure Synapse Analytics におけるデータレイク分析アーキテクチャ設計.md]]
- [[Azure イベントメッセージングアーキテクチャ整理.md]]
- [[Azure データウェアハウス ETL アーキテクチャ設計.md]]
- [[Azure データエンジニアリングアーキテクチャ.md]]
- [[Azure メダリオンアーキテクチャのデータレイク設計.md]]
- [[Event Hubs Capture を利用したコールドパス分析アーキテクチャ設計.md]]
- [[SAP HANA → Azure Synapse Analytics 増分データパイプライン設計.md]]
- [[オンプレミス NAS から Azure Data Lake Storage Gen2 へのデータ移行設計.md]]

## Azure Data Explorer

概要
大規模ログ・時系列分析サービス。

詳細
Kusto により大量ログや時系列データを高速集計する。

関連サービス
Azure Monitor / Azure Event Hubs

出典
- [[Azure IoT センサーデータのリアルタイムダッシュボード設計.md]]
- [[大規模ログ分析に適した Azure サービス（Azure Data Explorer）.md]]

## Azure Data Share

概要
データ共有サービス。

詳細
組織間でデータセットを安全に配布・同期する。

関連サービス
Azure Synapse Analytics / Azure Data Factory

出典
- [[Azure Synapse Pipelines と Azure Data Share を組み合わせたデータ統合・共有アーキテクチャ.md]]

## Medallion Architecture

概要
データレイク層構造パターン。

詳細
Bronze、Silver、Gold でデータを段階的に精製する。

関連サービス
Azure Data Lake Storage Gen2 / Azure Databricks

出典
- [[Azure メダリオンアーキテクチャのデータレイク設計.md]]

## Azure Stream Analytics

概要
ストリーム分析サービス。

詳細
イベントを SQL ライクに集計・変換して後続処理へ渡す。

関連サービス
Azure Event Hubs / Power BI

出典
- [[Azure AD のユーザー作成およびロール割り当てイベントをキャプチャし、Azure Cosmos DB に保存する設計.md]]
- [[Azure IoT データ処理アーキテクチャ.md]]
- [[Azure IoT テレメトリストリーム処理設計.md]]
- [[Azure イベントメッセージングアーキテクチャ整理.md]]
- [[Azure データ分析アーキテクチャ整理.md]]
- [[Azure リアルタイム動画分析アーキテクチャ.md]]
- [[Event Hubs Capture を利用したコールドパス分析アーキテクチャ設計.md]]
- [[IoT リアルタイムデータ処理パイプライン（Event Hubs + Stream Analytics + Power BI）.md]]

## Power BI

概要
可視化と BI サービス。

詳細
分析結果やストリーム データをダッシュボードで共有する。

関連サービス
Azure Stream Analytics / Azure Synapse Analytics

出典
- [[Azure IoT センサーデータのリアルタイムダッシュボード設計.md]]
- [[Azure Logic Apps からオンプレミス SQL Server へ接続するアーキテクチャ.md]]
- [[Azure Logic Apps とオンプレミス SAP システムのスケジュール統合.md]]
- [[Azure Synapse Pipelines と Azure Data Share を組み合わせたデータ統合・共有アーキテクチャ.md]]
- [[Azure イベントメッセージングアーキテクチャ整理.md]]
- [[Azure データ分析アーキテクチャ整理.md]]
- [[Event Hubs Capture を利用したコールドパス分析アーキテクチャ設計.md]]
- [[IoT リアルタイムデータ処理パイプライン（Event Hubs + Stream Analytics + Power BI）.md]]
- [[大規模ログ分析に適した Azure サービス（Azure Data Explorer）.md]]

## Azure IoT Hub

概要
IoT デバイス接続サービス。

詳細
多数デバイスとの双方向通信とメッセージ取り込みを管理する。

関連サービス
Azure Event Hubs / Azure Stream Analytics

出典
- [[Azure IoT データ処理アーキテクチャ.md]]
- [[Azure IoT テレメトリストリーム処理設計.md]]
- [[Azure イベントメッセージングアーキテクチャ整理.md]]
- [[Azure データ分析アーキテクチャ整理.md]]

## Azure Machine Learning

概要
機械学習開発・運用基盤。

詳細
学習からデプロイ、監視までの MLOps を支援する。

関連サービス
Managed Online Endpoint / Azure Data Lake Storage Gen2

出典
- [[Azure Machine Learning によるリアルタイム不正検出モデルのデプロイ設計.md]]
- [[Azure Machine Learning モデルデプロイアーキテクチャ.md]]

## Managed Online Endpoint

概要
Azure ML のリアルタイム推論エンドポイント。

詳細
モデルを API として公開し、マネージドに運用する。

関連サービス
Azure Machine Learning / Application Insights

出典
- [[Azure Machine Learning によるリアルタイム不正検出モデルのデプロイ設計.md]]
- [[Azure Machine Learning モデルデプロイアーキテクチャ.md]]

## Azure Migrate

概要
移行評価とオーケストレーション機能。

詳細
オンプレミス資産を評価し、Azure 移行を計画・実行する。

関連サービス
Azure Database Migration Service / Azure Virtual Machines

出典
- [[SQL Server → Azure SQL Managed Instance 移行方式ガイド.md]]
- [[SQL Server Always On データベース移行設計ドキュメント.md]]
- [[VMware vSphere 仮想マシンを Azure へ移行するアーキテクチャ.md]]

## Azure Database Migration Service

概要
データベース移行サービス。

詳細
SQL Server などを Azure SQL 系サービスへ移行する。

関連サービス
Azure SQL Database / Azure SQL Managed Instance

出典
- [[SQL Server → Azure SQL Managed Instance 移行方式ガイド.md]]
- [[大規模 SQL Server データベースを最小ダウンタイムで Azure に移行する設計.md]]

## Azure Data Box

概要
大容量オフライン データ転送サービス。

詳細
物理アプライアンスで大量データを Azure へ移送する。

関連サービス
Azure Blob Storage / Azure Data Lake Storage Gen2

出典
- [[Azure App Service とオンプレミスファイルサーバーのハイブリッドファイル共有設計.md]]
- [[Azure Synapse Pipelines と Azure Data Share を組み合わせたデータ統合・共有アーキテクチャ.md]]
- [[Azure 大容量データ移行サービスまとめ.md]]
- [[Azure 大容量データ転送設計.md]]

