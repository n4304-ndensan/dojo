---
分類: Cloud
tags:
  - cloud/azure
  - cloud/azure/data-factory
  - cloud/azure/private-endpoint
  - cloud/azure/private-link
  - cloud/azure/virtual-machines
  - cloud/data-engineering
  - cloud/data-engineering/data-integration
  - cloud/security/network-security
  - cloud/security/private-networking
  - exam/azure/architecture
---

# Azure Data Factory Self-hosted Integration Runtime と Private Endpoint による安全なデータ統合

## 1. 背景（シナリオ）

ある企業では、Windows Server 2022 を実行する Azure 仮想マシン **VM2** に 1TB の機密データファイルを保存しています。このデータは Azure Data Factory を使用して処理および変換され、その後 Azure Blob Storage に保存される予定です。

しかし、このデータは機密情報を含んでいるため、データ処理と転送のすべてのプロセスは安全なネットワーク内で行われる必要があります。特に、データがパブリックインターネットを通過しないことが重要な要件です。

また、1TB という大容量データを処理するため、データ転送の遅延を最小化し、高速なデータ転送を実現するアーキテクチャが必要になります。

## 2. 要件整理

このシナリオから読み取れる要件を整理すると、次のようになります。

まず、Azure Data Factory を使用して VM2 のデータを処理する必要があります。

次に、データ転送は仮想ネットワーク内で行われ、パブリックインターネットを経由しないようにする必要があります。

さらに、1TB のデータを処理するため、データ転送の遅延を最小化し、高速な処理が可能である必要があります。

これらをまとめると、次の要件になります。

- Azure Data Factory を使用したデータ統合  
- VM2 上の機密データへの安全なアクセス  
- パブリックインターネットを経由しない通信  
- 低遅延かつ高速なデータ転送  

## 3. 技術の基本概念

Azure Data Factory は Azure のデータ統合サービスであり、複数のデータソースからデータを取得して処理・変換できます。

Data Factory では、データ処理を実行するためのコンポーネントとして **Integration Runtime（IR）** が使用されます。Integration Runtime には主に次の種類があります。

- Azure Integration Runtime  
- Self-hosted Integration Runtime  
- Azure SSIS Integration Runtime  

このシナリオでは、VM 上のデータを安全に処理する必要があるため **Self-hosted Integration Runtime** が適しています。

Self-hosted Integration Runtime はオンプレミス環境や仮想マシン上にインストールされ、ローカル環境のデータに直接アクセスできます。

## 4. アーキテクチャまたは設計のポイント

このシナリオでは、VM2 上に Self-hosted Integration Runtime を配置することで、ローカルに保存されている機密データに直接アクセスできます。

これにより、データを一度パブリックインターネットに送信する必要がなくなり、セキュリティが向上します。

さらに、Azure Blob Storage へのアクセスには **Private Endpoint（Private Link）** を使用します。

Private Endpoint を使用すると、Blob Storage へのアクセスは Azure 仮想ネットワーク内のプライベート IP アドレスを通じて行われます。これにより、通信は Azure のバックボーンネットワーク内で完結し、パブリックインターネットを通過しません。

この構成により、セキュリティとパフォーマンスの両方を最適化できます。

## 5. 設計判断（なぜこの構成になるか）

この問題の正解は **A. Azure Data Factory Self-hosted Integration Runtime と Private Endpoint** です。

Self-hosted Integration Runtime を VM2 上に配置することで、機密データをローカル環境で処理できます。これにより、データを外部ネットワークに公開する必要がありません。

また、Azure Blob Storage に Private Endpoint を設定することで、データ転送は Azure 仮想ネットワーク内で完結します。

この構成は次のメリットを提供します。

- 機密データを安全に処理  
- パブリックインターネットを経由しない通信  
- 高速なデータ転送  
- Azure Data Factory の統合機能の活用  

## 6. 他の選択肢が誤りな理由

Azure Data Box Gateway は大量データを Azure に転送するための物理デバイスまたは仮想アプライアンスであり、継続的なデータ統合処理には適していません。

Azure Blob Storage の Private Link は安全な接続を提供しますが、それだけでは VM 上のデータ処理を実現することはできません。データ統合処理には Integration Runtime が必要です。

Azure Logic Apps はワークフロー自動化サービスであり、大量データの処理や高速データ統合には適していません。

## 7. 最終回答

A. Azure Data Factory Self-hosted Integration Runtime と Private Endpoint

## 8. まとめ

この問題は Azure Data Factory のデータ統合アーキテクチャとネットワークセキュリティの理解を確認する問題です。

機密データを安全に処理する場合、Self-hosted Integration Runtime を使用してローカル環境でデータ処理を行うことが重要です。

さらに、Azure Storage への接続には Private Endpoint を使用することで、通信を Azure 仮想ネットワーク内に限定できます。

この組み合わせにより、安全かつ高速なデータ統合パイプラインを構築できます。