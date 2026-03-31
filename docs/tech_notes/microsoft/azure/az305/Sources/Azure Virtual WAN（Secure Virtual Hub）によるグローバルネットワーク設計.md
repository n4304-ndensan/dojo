---
分類: Networking
tags:
  - cloud/azure
  - cloud/azure/virtual-wan
  - cloud/azure/virtual-hub
  - cloud/azure/firewall
  - cloud/networking/vpn
  - cloud/networking/point-to-site
  - cloud/networking/hub-spoke
  - cloud/security/fqdn-filtering
  - exam/azure
---

# Azure Virtual WAN（Secure Virtual Hub）によるグローバルネットワーク設計

## 1. 背景（シナリオ）

ある企業は世界中に複数のオフィスを展開しており、現在オンプレミス環境からAzureへの移行を計画しています。

このクラウド移行に伴い、ネットワーク設計にはいくつかの重要な要件があります。

まず、ユーザーや拠点からAzureへの **ポイントツーサイト（P2S）VPN接続** をシームレスに提供する必要があります。

次に、複数の仮想ネットワーク（VNet）間で **トランジティブルーティング（Transit Routing）** を可能にする必要があります。

さらに、セキュリティ要件として、ネットワークトラフィックを **完全修飾ドメイン名（FQDN）ベースでフィルタリング** できる必要があります。

このようなグローバルネットワーク環境では、スケーラブルで管理しやすいネットワークアーキテクチャが必要になります。

## 2. 要件整理

この問題の要件を整理すると次のようになります。

- 世界中の拠点からAzureへ接続
- Point-to-Site VPN接続
- 仮想ネットワーク間のトランジティブルーティング
- FQDNベースのトラフィックフィルタリング
- スケーラブルなネットワークアーキテクチャ

これらの要件は、単一のVNet構成ではなく **グローバルネットワークサービス** を利用することで効率的に実現できます。

## 3. 技術の基本概念

Azureには、複数のネットワーク接続を統合管理するためのサービスとして **Azure Virtual WAN** が提供されています。

Azure Virtual WANは、次のようなネットワーク接続を統合管理するためのサービスです。

- Site-to-Site VPN
- Point-to-Site VPN
- ExpressRoute
- VNet接続

Virtual WANの中心となるコンポーネントが **Virtual Hub** です。

Virtual Hubはハブアンドスポークネットワークの中心として機能し、複数のVNetやVPN接続を統合します。

さらに **Secure Virtual Hub** を利用すると、Azure Firewallなどのセキュリティ機能を統合することができます。

## 4. アーキテクチャまたは設計のポイント

このシナリオでは、Virtual WANの **Secure Virtual Hub** を利用することで、複数の要件を同時に満たすことができます。

まず、Virtual WANはネイティブで **Point-to-Site VPN接続** をサポートしています。そのため、ユーザーやリモート拠点が安全にAzureへ接続できます。

次に、Virtual Hubを中心とした **ハブアンドスポークアーキテクチャ** により、複数のVNet間でトランジティブルーティングが可能になります。

さらに、Secure Virtual Hubでは **Azure Firewall** を統合することができます。

Azure Firewallは **FQDNベースのトラフィックフィルタリング** をサポートしており、ドメイン名に基づいたアクセス制御を実現できます。

この構成により、ネットワーク接続とセキュリティを一元管理できます。

## 5. 設計判断（なぜこの構成になるか）

この問題では次の3つの要件が同時に求められています。

- Point-to-Site VPN
- トランジティブルーティング
- FQDNベースのフィルタリング

Azure Virtual WANはこれらすべてをネイティブにサポートしています。

特にSecure Virtual Hubを利用すると、Azure Firewallと統合できるため、FQDNベースのトラフィック制御が可能になります。

また、Virtual WANはグローバル企業向けに設計されたサービスであり、複数拠点の接続管理を簡素化できます。

そのため、このシナリオではAzure Virtual WANが最適なソリューションになります。

## 6. 他の選択肢が誤りな理由

### B. Azure Route Server + ExpressRoute

Azure Route ServerはBGPルーティングの管理を簡素化するサービスです。

しかし、Point-to-Site VPN接続を提供するサービスではありません。

### C. VNet Peering + NSG

VNet PeeringはVNet同士を接続することはできますが、トランジティブルーティングはサポートされません。

また、NSGはIPアドレスとポートベースの制御のみであり、FQDNベースのフィルタリングはできません。

### D. Network Watcher + Application Gateway

Network Watcherはネットワーク監視ツールです。

Application Gatewayはレイヤー7ロードバランサーであり、VPN接続やVNet間ルーティングには適していません。

## 7. 最終回答

この要件を満たす最適なサービスは次です。

**Secure Virtual Hub を備えた Azure Virtual WAN**

したがって **正解は A** です。

## 8. まとめ

Azure Virtual WANは、グローバルネットワーク接続を簡素化するためのサービスです。

Virtual WANを利用すると次の機能を統合できます。

- Site-to-Site VPN
- Point-to-Site VPN
- ExpressRoute
- VNet接続

さらに **Secure Virtual Hub + Azure Firewall** を利用することで、FQDNベースのトラフィックフィルタリングも実装できます。

そのため、グローバル企業のネットワーク設計では **Azure Virtual WANが最もスケーラブルで管理しやすいソリューション**になります。