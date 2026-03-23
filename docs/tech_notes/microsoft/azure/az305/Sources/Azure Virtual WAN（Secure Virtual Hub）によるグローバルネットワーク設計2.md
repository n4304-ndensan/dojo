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

ある企業は世界中に複数のオフィスを展開しており、現在オンプレミス環境からAzureへの移行を計画しています。クラウド移行に伴い、ネットワーク設計には複数の重要な要件があります。

まず、ユーザーや拠点からAzureへ安全に接続できる **ポイントツーサイト（Point-to-Site）VPN接続** を提供する必要があります。

次に、複数の仮想ネットワーク（VNet）間で **トランジティブルーティング（Transit Routing）** を実現する必要があります。これは、複数のネットワークが中央のハブを通じて相互通信できる構成を意味します。

さらに、セキュリティ要件として、ネットワークトラフィックを **完全修飾ドメイン名（FQDN）ベースでフィルタリング** する必要があります。つまりIPアドレスではなくドメイン名に基づいて通信制御を行う必要があります。

## 2. 要件整理

このシナリオから読み取れる主な要件は次の通りです。

- 世界中の拠点からAzureへ接続する必要がある  
- Point-to-Site VPNを提供する  
- 仮想ネットワーク間のトランジティブルーティングを実現する  
- FQDNベースでトラフィックをフィルタリングする  
- ネットワーク管理を簡素化する

これらの要件は、単純なVNet接続ではなく **グローバルネットワークサービス** を利用することで効率的に実現できます。

## 3. 技術の基本概念

Azureには複数のネットワーク接続を統合管理するためのサービスとして **Azure Virtual WAN** が提供されています。

Azure Virtual WANは次の接続を一元管理できるサービスです。

- Site-to-Site VPN  
- Point-to-Site VPN  
- ExpressRoute  
- VNet接続  

Virtual WANの中心的なコンポーネントは **Virtual Hub** です。Virtual Hubはハブアンドスポークアーキテクチャの中心となるネットワークハブとして機能します。

さらに **Secure Virtual Hub** を使用すると、Azure Firewallなどのセキュリティサービスを統合することができます。

## 4. アーキテクチャまたは設計のポイント

このシナリオでは **Azure Virtual WAN + Secure Virtual Hub** を利用することで、すべての要件を満たすことができます。

まず、Virtual WANは **Point-to-Site VPN接続をネイティブでサポート** しています。これにより、世界中のユーザーがインターネット経由で安全にAzureへ接続できます。

次に、Virtual Hubを中心とした **ハブアンドスポークネットワーク** を構成することで、複数のVNet間でトランジティブルーティングを実現できます。

さらに、Secure Virtual Hubでは **Azure Firewall** を統合できます。Azure Firewallは **FQDNベースのトラフィックフィルタリング** をサポートしているため、ドメイン名に基づいたアクセス制御を実装できます。

この構成により、ネットワーク接続とセキュリティポリシーを一元管理できます。

## 5. 設計判断（なぜこの構成になるか）

この問題の重要なポイントは次の3つです。

- Point-to-Site VPN接続  
- トランジティブルーティング  
- FQDNベースのトラフィック制御  

Azure Virtual WANはこれらすべての機能を統合的に提供するサービスです。

特にSecure Virtual Hubを使用することで、Azure Firewallと統合され、FQDNベースのトラフィックフィルタリングが可能になります。

そのため、グローバルネットワーク環境ではAzure Virtual WANが最も適したソリューションになります。

## 6. 他の選択肢が誤りな理由

まず、ExpressRouteとAzure Route Serverの組み合わせは、ハイブリッド接続のルーティング管理には有効ですが、Point-to-Site VPN接続を提供するサービスではありません。

次に、Virtual Network PeeringとNSGの組み合わせはVNet同士の接続は可能ですが、トランジティブルーティングをサポートしていません。またNSGはIPアドレスとポートベースの制御のみであり、FQDNベースのフィルタリングはできません。

さらに、Network WatcherとApplication Gatewayは主にネットワーク監視やHTTPロードバランシングに使用されるサービスであり、VPN接続やVNet間ルーティングの用途には適していません。

## 7. 最終回答

これらの要件を満たす最適なサービスは次です。

**セキュアな仮想ハブを備えた Azure Virtual WAN**

したがって **正解は A** です。

## 8. まとめ

Azure Virtual WANは、グローバル企業のネットワーク接続を簡素化するためのサービスです。

Virtual WANを利用すると、次の機能を統合できます。

- Site-to-Site VPN  
- Point-to-Site VPN  
- ExpressRoute接続  
- VNet接続  

さらに **Secure Virtual Hub + Azure Firewall** を利用することで、FQDNベースのトラフィックフィルタリングも実装できます。

そのため、複数拠点を持つ企業のAzureネットワーク設計では **Azure Virtual WANが最もスケーラブルで管理しやすいソリューション**になります。