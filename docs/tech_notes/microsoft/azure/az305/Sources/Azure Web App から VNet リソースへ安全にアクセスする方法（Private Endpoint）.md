---
分類: Networking
tags:
  - cloud/azure
  - cloud/azure/app-service
  - cloud/azure/private-endpoint
  - cloud/networking/private-link
  - cloud/networking/vnet
  - cloud/networking/dns
  - exam/azure
---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Network (VNet)]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Private Endpoint]]
# Azure Web App から VNet リソースへ安全にアクセスする方法（Private Endpoint）

## 1. 背景（シナリオ）

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure App Service]]
Azure App Service（Web App）は、多くの場合 PaaS サービスとしてインターネット上に公開されます。しかし実際のアプリケーションでは、Web アプリがバックエンドのリソースへアクセスする必要があります。

例えば次のような構成です。

- Web App  
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
- Azure SQL Database  
- Storage Account  
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Key Vault]]
- Key Vault  
- 内部 API  

これらのリソースはセキュリティの観点から **仮想ネットワーク（VNet）内に配置されることが多い**です。

今回のシナリオでは、Azure Web App が VNet 内のリソースへアクセスする必要があります。

さらに、次の重要な要件があります。

まず、ネットワークパフォーマンスは可能な限り高い必要があります。つまり、トラフィックはインターネットではなく Azure の内部ネットワークを通る必要があります。

また、仮想ネットワークのリソースは **パブリックエンドポイントを公開してはいけません。**

つまり、このシナリオでは **完全にプライベートなネットワークアクセス**が必要になります。

この要件を満たす Azure のネットワーク機能を選択する必要があります。

---

## 2. 要件整理

この問題では、次の要件が明確に示されています。

まず、Azure Web App は仮想ネットワーク内のリソースにアクセスする必要があります。

次に、ネットワークパフォーマンスは可能な限り最良である必要があります。つまり、通信は Azure のバックボーンネットワークを通るべきです。

さらに、仮想ネットワークリソースは **公開エンドポイントを持たない必要があります。**

つまり、この環境ではインターネット経由のアクセスは許可されません。

これらの条件をまとめると次のようになります。

- Web App → VNet リソースへアクセス  
- パブリックエンドポイントを使用しない  
- Azure 内部ネットワークで通信  
- 高いネットワークパフォーマンス  

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Private Link]]
この要件を満たすのは **Azure Private Link（Private Endpoint）**です。

---

## 3. Private Endpoint の基本概念

Private Endpoint は Azure Private Link の機能の一つであり、Azure PaaS サービスへ **VNet 内のプライベート IP を使って接続する仕組み**です。

通常、Azure の多くの PaaS サービスはパブリックエンドポイントを持っています。

しかし Private Endpoint を使用すると、そのサービスへ **VNet 内のプライベート IP を割り当てることができます。**

つまり、クライアントはインターネットを経由せず、VNet 内のプライベートアドレスを使ってアクセスできます。

この仕組みにより、次のメリットが得られます。

- パブリックインターネットを経由しない  
- Azure バックボーンネットワークのみ使用  
- パブリックエンドポイントを無効化できる  
- セキュリティとパフォーマンスの向上  

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure DNS]]
## 4. DNS の重要性

Private Endpoint を使用する場合、DNS 設定が重要になります。

Azure の PaaS サービスは通常、パブリック DNS 名を使用します。

例えば次のような名前です。

- storageaccount.blob.core.windows.net  
- database.windows.net  

Private Endpoint を使用する場合、この DNS 名を **プライベート IP に解決する必要があります。**

そのため Azure では **Private DNS Zone** を使用します。

DNS が正しく設定されると、アプリケーションは通常の FQDN を使用しながら、実際にはプライベート IP に接続することになります。

---

## 5. Web App から Private Endpoint へ接続する仕組み

Azure Web App が Private Endpoint を利用する場合、一般的に次の構成になります。

まず、Web App は **VNet Integration** を使用して仮想ネットワークに接続します。

次に、バックエンドリソースに **Private Endpoint** を作成します。

その後、Private DNS Zone を設定して、リソースの名前がプライベート IP に解決されるようにします。

この構成により、Web App は次のような通信を行います。

Web App → VNet → Private Endpoint → Azure サービス

この通信はすべて Azure 内部ネットワークで行われます。

---

## 6. なぜ Private Endpoint が最適なのか

今回の問題では **パブリックエンドポイントを使用してはいけない**という要件があります。

Service Endpoint は VNet から Azure サービスへのアクセスを制御できますが、サービス自体は依然としてパブリックエンドポイントを持っています。

つまり、この要件を完全には満たしません。

Private Endpoint は Azure サービスを VNet 内のプライベート IP として公開するため、パブリックエンドポイントを完全に排除できます。

さらに通信は Azure バックボーンを使用するため、ネットワークパフォーマンスも最適になります。

そのため、このシナリオでは Private Endpoint が最適なソリューションになります。

---

## 7. 他の選択肢が適さない理由

### Web App の VNet Integration

VNet Integration は Web App を VNet に接続する機能です。

しかし、これだけではバックエンドサービスをプライベート化することはできません。

---

### Service Endpoint

Service Endpoint は VNet から Azure サービスへのアクセスを制限できます。

しかし、サービスは依然としてパブリックエンドポイントを持ち続けます。

そのため、公開エンドポイントを禁止する要件には適していません。

---

### Point-to-Site VPN

Point-to-Site VPN はクライアントデバイス向けの接続方式です。

App Service のネットワーク接続には適していません。

---

## 8. 最終回答

C. リソースのプライベートエンドポイントと、プライベート IP に解決する DNS を設定する