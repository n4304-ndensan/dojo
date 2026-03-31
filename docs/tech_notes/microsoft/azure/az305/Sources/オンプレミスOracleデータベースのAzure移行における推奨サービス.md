---
分類: cloud/azure
tags:
  - cloud/azure/data-factory
  - cloud/azure/datalake
  - cloud/azure/expressroute
  - cloud/azure/databricks
  - cloud/azure/synapse
  - cloud/architecture/data-migration
  - exam/azure/az-305
---

# オンプレミスOracleデータベースのAzure移行における推奨サービス

## 1. 背景（シナリオ）
組織は、オンプレミスのOracleデータベースをAzureに移行し、Azure Databricksでデータ変換を行った上で、処理済みデータをAzure Synapse Analyticsで分析する必要があります。この移行では、データの整合性を維持しつつ、安全かつ低遅延で効率的にデータを転送することが求められます。

## 2. 要件整理
- **安全なデータ転送**: オンプレミスからAzureへの機密データの移動時にプライバシーを確保
- **低遅延の効率的な処理**: データ変換と読み込みが迅速に行えること
- **データ整合性の維持**: 移行中および変換後もデータの正確性を保証
- **Azureサービスとの統合**: Databricks と Synapse Analytics へのシームレスな接続

## 3. 技術の基本概念
- **Azure Data Factory (ADF) 統合ランタイム (IR)**: データ移行と変換のパイプラインを自動化するクラウドベースサービス。オンプレミスデータベースとの安全な接続、Databricksとの統合、データ整合性の検証が可能。
- **Azure ExpressRoute**: オンプレミスとAzure間のプライベート専用接続を提供。インターネットを経由せず、安全かつ低遅延で大容量データを転送できる。

## 4. アーキテクチャまたは設計のポイント
- **ADF 統合ランタイム** を使用してデータの抽出、変換、ロード(ETL)パイプラインを構築
- **ExpressRoute** で専用回線を構築し、オンプレミスOracleデータベースからAzureへの安全で高速なデータ転送を実現
- Databricksで変換したデータをSynapse Analyticsにロードし、分析処理を最適化

## 5. 設計判断（なぜこの構成になるか）
- ADFにより、移行パイプラインの自動化とデータ整合性の確保が可能
- ExpressRouteにより、低遅延かつセキュアなデータ転送を実現
- DatabricksとSynapseとの統合により、分析ワークフローが効率化され、全体の移行戦略をサポート

## 6. 他の選択肢が誤りな理由
- **Azure Data Lake Storage Gen2**: 中間データ格納に有効だが、移行の安全性や遅延削減の要件を直接満たさない
- **Azure Data Box**: 大容量データの物理移行に適するが、リアルタイム転送や継続的ETLには不向き

## 7. 最終回答
**A. 統合ランタイムを備えたAzure Data Factory**  
**C. Azure ExpressRoute 専用ネットワーク接続用**

## 8. まとめ
Azure Data FactoryとExpressRouteを組み合わせることで、オンプレミスOracleデータベースからAzureへの安全で低遅延なデータ転送が可能になります。ADFはデータ抽出・変換・ロードを自動化し、ExpressRouteはセキュアかつ効率的な通信を提供します。この構成により、データ整合性を維持しつつ移行戦略の目標を達成できます。