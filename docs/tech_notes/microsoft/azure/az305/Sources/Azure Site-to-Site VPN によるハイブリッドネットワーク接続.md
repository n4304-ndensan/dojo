---
分類: Cloud
tags:
  - cloud/azure
  - cloud/azure/vpn-gateway
  - cloud/azure/virtual-network
  - cloud/architecture/hybrid-cloud
  - cloud/architecture/network-connectivity
  - networking/vpn
  - networking/site-to-site
  - exam/azure/fundamentals
---

# Azure Site-to-Site VPN によるハイブリッドネットワーク接続

## 1. 背景（シナリオ）

企業がクラウドを導入する際、既存のオンプレミスネットワークとクラウド環境を接続する **ハイブリッドクラウド構成**を採用することがよくあります。この構成では、オンプレミスで稼働しているシステムとAzure上のサービスが相互に通信できる必要があります。

特に、企業のアプリケーションがAzureの仮想ネットワーク（VNet）内のリソースにアクセスする場合、通信は安全でなければなりません。インターネットを経由する場合でも、暗号化されたトンネルを利用することで、安全に通信を行うことができます。

この問題では、オンプレミスネットワークをVPN経由でAzureに接続し、Azure VNetのリソースへ安全にアクセスするためのVPN構成を選択する必要があります。

## 2. 要件整理

問題文の要件を整理すると、ハイブリッドネットワーク設計に関する重要なポイントが見えてきます。

このシナリオでは、次の条件があります。

・オンプレミスネットワークが存在する  
・Azureの仮想ネットワーク（VNet）と接続する必要がある  
・VPNを使用する  
・接続は安全である必要がある  
・オンプレミスネットワーク全体からAzureへアクセスできる必要がある  

ここで重要なのは、「個々のユーザーではなく **ネットワーク全体を接続する**」という点です。

## 3. 技術の基本概念

AzureではVPN接続の方式として主に次の2種類があります。

まず **Site-to-Site VPN（S2S VPN）** です。これは、オンプレミスネットワークとAzure Virtual Networkの間にVPNトンネルを作成する方式です。企業ネットワーク全体をAzureに接続する場合に使用されます。

次に **Point-to-Site VPN（P2S VPN）** です。これは、個々のユーザーのデバイス（ノートPCなど）からAzureへVPN接続する方式です。リモートユーザーがAzureリソースへアクセスする場合に使用されます。

また、Azureには次のようなネットワーク接続サービスもあります。

・Azure ExpressRoute  
・Azure Virtual WAN  

しかし、これらはVPN接続とは目的が異なります。

## 4. アーキテクチャまたは設計のポイント

このシナリオでは、オンプレミスネットワーク全体とAzure VNetを接続する必要があります。そのため、Site-to-Site VPNを使用します。

Site-to-Site VPNでは、オンプレミスのVPNデバイスとAzure VPN Gatewayの間に **IPsec/IKEトンネル**が作成されます。

この接続の特徴には次のようなものがあります。

・オンプレミスネットワーク全体がAzureに接続される  
・IPsec/IKEによる暗号化通信  
・Azure VNetのリソースへ安全にアクセス可能  
・インターネットを経由するVPNトンネル  

この仕組みにより、オンプレミス環境からAzureの仮想ネットワーク内の仮想マシンやサービスへ安全にアクセスできます。

## 5. 設計判断（なぜこの構成になるか）

この問題の要件は次の通りです。

・オンプレミスネットワークをAzureに接続する  
・VPNを使用する  
・ネットワーク全体がAzureへアクセスする  
・セキュアな接続を確保する  

これらの条件を満たすVPN構成が **Site-to-Site VPN Gateway** です。

Site-to-Site VPNは、オンプレミスネットワークのVPNデバイスとAzure VPN Gatewayの間に安全なトンネルを構築し、ネットワーク全体を接続するため、ハイブリッドクラウド構成に最も適しています。

## 6. 他の選択肢が誤りな理由

まず Azure ExpressRoute について説明します。ExpressRoute は専用回線を使用したプライベート接続サービスであり、VPN接続ではありません。高性能な接続を提供しますが、問題の条件である「VPN接続」には該当しません。

次に Point-to-Site VPN です。この方式は個々のユーザーがAzureに接続するためのVPNです。リモートユーザーのアクセスには適していますが、オンプレミスネットワーク全体を接続する用途には適していません。

最後に Azure Virtual WAN です。Virtual WAN は大規模なネットワーク接続を管理するサービスですが、基本的なオンプレミス接続には過剰なソリューションとなることが多く、この問題の要件には直接対応していません。

## 7. 最終回答

B. サイト間VPNゲートウェイ（Site-to-Site VPN Gateway）

## 8. まとめ

Azureでオンプレミスネットワークとクラウド環境を接続する場合、VPN接続には主にSite-to-Site VPNとPoint-to-Site VPNがあります。

Site-to-Site VPNは、オンプレミスネットワーク全体をAzure Virtual Networkに接続する方式であり、ハイブリッドクラウド環境で最も一般的に使用されます。IPsec/IKE暗号化トンネルを使用するため、安全に通信することができます。

そのため、オンプレミスネットワークからAzure VNetへ安全に接続する必要がある場合には **Site-to-Site VPN Gateway** を使用するのが最適なソリューションです。