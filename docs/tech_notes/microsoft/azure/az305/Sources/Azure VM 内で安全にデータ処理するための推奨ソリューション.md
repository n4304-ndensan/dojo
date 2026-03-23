---
分類: cloud/azure
tags:
  - cloud/azure/data-factory
  - cloud/azure/integration-runtime
  - cloud/azure/blob-storage
  - cloud/architecture/security
  - exam/azure/az-305
---

# Azure VM 内で安全にデータ処理するための推奨ソリューション

## 1. 背景（シナリオ）
Windows Server 2022 を実行している Azure 仮想マシン VM2 上で、1 TB のデータファイルを処理し、変換後のデータを Azure Blob ストレージに保存する必要があります。データ処理は仮想ネットワーク内で行われ、公共のインターネットに公開されてはいけません。さらに、Azure Data Factory を使用してデータ統合を行う必要があります。

## 2. 要件整理
- **安全なデータ処理**: VM 内で処理を行い、データがパブリックインターネットを通過しない
- **Azure Data Factory との統合**: データフロー活動や変換を自動化
- **効率的な処理**: 大容量データ (1 TB) の処理に対応
- **仮想ネットワーク内で完結**: セキュリティとコンプライアンス要件を満たす

## 3. 技術の基本概念
- **Azure Data Factory Integration Runtime (Self-hosted)**:
  - 仮想マシン上にデプロイされ、オンプレミスやVM内のデータソースとAzureクラウドサービスを安全に接続
  - データコピーや変換を安全に実行
  - 仮想ネットワーク内でデータを処理可能、公共インターネットを経由しない
  - Azure Data Factory とシームレスに統合可能

- **その他の選択肢**:
  - **Azure File Sync**: ファイルの同期専用。データ変換やADF統合には不向き
  - **Azure VPN Gateway / ExpressRoute**: セキュア接続を提供するが、データ処理や変換の機能はない

## 4. アーキテクチャまたは設計のポイント
- VM2 に Self-hosted Integration Runtime をインストール
- データは仮想ネットワーク内で処理され、安全に Azure Blob ストレージにコピー
- データフローと変換作業を ADF パイプラインでオーケストレーション

## 5. 設計判断（なぜこの構成になるか）
- 大容量データ処理が可能で、ADFと直接統合できる
- 仮想ネットワーク内完結で、公共インターネットに露出せずセキュア
- データ整合性を維持しつつ、効率的な変換と移動を実現

## 6. 他の選択肢が誤りな理由
- **Azure File Sync**: データ同期のみで、変換やADF統合はサポートされない
- **Azure VPN Gateway / ExpressRoute**: 接続は安全だが、データ処理機能は提供しない

## 7. 最終回答
**A. Azure Data Factory 統合ランタイム (セルフホスティング済み)**

## 8. まとめ
Self-hosted Integration Runtime を使用することで、VM 内で安全かつ効率的にデータを処理し、Azure Blob ストレージに安全に格納できます。公共インターネットを経由せず、ADFと連携して大容量データを統合するため、このシナリオに最適なソリューションです。