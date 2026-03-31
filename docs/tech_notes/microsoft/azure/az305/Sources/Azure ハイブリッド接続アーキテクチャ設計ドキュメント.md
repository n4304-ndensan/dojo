# Azure ハイブリッド接続アーキテクチャ設計ドキュメント

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Private Link]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure ExpressRoute]]
（ExpressRoute / Private Link / VPN / Service Endpoint）

## 1. 概要

企業システムでは、オンプレミス環境とクラウド環境を接続する **ハイブリッドクラウドアーキテクチャ** が一般的です。

Azureではオンプレミス環境とAzureを接続するために複数のネットワークサービスが提供されています。

代表的な接続方法

- Azure VPN Gateway
    
- Azure ExpressRoute
    
- Azure Private Link
    
- Azure Service Endpoint
    

これらのサービスは **接続経路・セキュリティレベル・用途** が異なります。  
そのため、要件に応じて適切なサービスを選択する必要があります。

本ドキュメントでは、Azureのハイブリッド接続サービスを体系的に説明し、  
特に **ExpressRoute を利用したプライベート接続** の設計について解説します。

---

# 2. ハイブリッド接続とは

ハイブリッド接続とは、オンプレミス環境とクラウド環境をネットワーク接続する構成です。

典型構成

```
On-premises Network
        │
        │
Azure Network
```

目的

- 既存システムとクラウド連携
    
- データセンター拡張
    
- セキュアな通信
    

---

# 3. Azure PaaSサービス

問題の要件では次のPaaSサービスへのアクセスが求められています。

例

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Storage Account]]
- Azure Storage
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
- Azure SQL Database
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Cosmos DB]]
- Azure Cosmos DB
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Key Vault]]
- Azure Key Vault
    

PaaSサービスは通常 **パブリックエンドポイント** を持っています。

```
Storage Account
↓
Public Endpoint
```

しかし企業環境では

- パブリックアクセス禁止
    
- プライベートネットワークのみ
    

という要件が多くあります。

---

# 4. Azure VPN Gateway

Azure VPN GatewayはオンプレミスとAzureを接続する **IPSec VPNサービス**です。

接続方式

```
On-premises
↓
Internet
↓
VPN Gateway
↓
Azure VNet
```

特徴

- IPsec VPN
    
- 比較的低コスト
    
- セットアップ容易
    

しかし次の問題があります。

### 問題

VPNは **インターネット経由通信** です。

```
Internet
```

つまり

- 完全なプライベート接続ではない
    
- パブリックネットワークを通過する
    

そのため

**「パブリックインターネットを経由しない」**

という要件には適合しません。

---

# 5. Azure ExpressRoute

Azure ExpressRouteは **専用回線によるAzure接続サービス**です。

特徴

- インターネットを経由しない
    
- 専用回線
    
- 高帯域
    
- 高信頼性
    

構成

```
On-premises
↓
ExpressRoute Circuit
↓
Microsoft Backbone Network
↓
Azure
```

この接続は

**完全にプライベートネットワーク**

です。

---

## ExpressRoute接続タイプ

ExpressRouteには複数のピアリングがあります。

### Private Peering

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Network (VNet)]]
VNet接続

```
On-premises
↓
ExpressRoute
↓
Azure VNet
```

---

### Microsoft Peering

PaaS接続

```
On-premises
↓
ExpressRoute
↓
Azure PaaS Services
```

例

- Azure Storage
    
- Azure SQL Database
    

---

# 6. Azure Private Link

Azure Private Linkは **Azure PaaSサービスへのプライベート接続** を提供します。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Private Endpoint]]
Private Linkは **Private Endpoint** を使用します。

構成

```
Azure VNet
↓
Private Endpoint
↓
Azure PaaS
```

これにより

- PaaSサービスがプライベートIPで公開される
    

---

### 重要なポイント

Private Linkは

**オンプレミス接続サービスではありません**

つまり

```
On-premises → Azure
```

の接続は提供しません。

オンプレミス接続には

- ExpressRoute
    
- VPN
    

が必要です。

---

# 7. Azure Service Endpoint

Service Endpointは

**VNetからPaaSサービスへの接続をAzureバックボーン経由にする仕組み**

です。

構成

```
Azure VNet
↓
Service Endpoint
↓
Azure PaaS
```

メリット

- パブリックIP不要
    
- Azure内部通信
    

しかし

### 制限

オンプレミス接続は提供しません。

---

# 8. ハイブリッド接続サービス比較

|サービス|用途|インターネット|
|---|---|---|
|VPN Gateway|VPN接続|使用|
|ExpressRoute|専用回線|使用しない|
|Private Link|PaaS Private Endpoint|Azure内部|
|Service Endpoint|VNet → PaaS|Azure内部|

---

# 9. 要件分析

問題の要件

1. オンプレミス接続
    
2. Azure PaaSアクセス
    
3. パブリックインターネット不可
    
4. パブリックエンドポイント不要
    

この要件を満たすには

### 必要条件

- 専用回線
    
- プライベート接続
    
- PaaSアクセス
    

---

# 10. ExpressRouteによる解決

ExpressRouteは次の要件を満たします。

|要件|ExpressRoute|
|---|---|
|オンプレミス接続|○|
|インターネット回避|○|
|PaaS接続|○|
|高セキュリティ|○|

---

# 11. 典型アーキテクチャ

```
On-premises Data Center
        │
        │
ExpressRoute Circuit
        │
        │
Microsoft Backbone
        │
        │
Azure Virtual Network
        │
        │
Azure PaaS Services
```

---

# 12. セキュアPaaSアクセス

より安全な構成

```
On-premises
↓
ExpressRoute
↓
Azure VNet
↓
Private Endpoint
↓
Azure SQL / Storage
```

この構成では

- インターネット経由なし
    
- パブリックIPなし
    
- 完全プライベート通信
    

が実現します。

---

# 13. Azure試験の判断ポイント

試験問題では次のキーワードが重要です。

|キーワード|サービス|
|---|---|
|専用回線|ExpressRoute|
|インターネット経由不可|ExpressRoute|
|PaaS Private Access|Private Link|
|VNet → PaaS|Service Endpoint|
|VPN接続|VPN Gateway|

---

# 14. まとめ

Azureハイブリッド接続では次の使い分けが重要です。

|要件|サービス|
|---|---|
|オンプレミス接続|ExpressRoute|
|低コストVPN|VPN Gateway|
|PaaSプライベート接続|Private Link|
|VNet→PaaS通信|Service Endpoint|

今回の要件

- オンプレミス接続
    
- インターネットを通らない
    
- PaaSアクセス
    

この条件を満たすサービスは

**Azure ExpressRoute**

です。

---
