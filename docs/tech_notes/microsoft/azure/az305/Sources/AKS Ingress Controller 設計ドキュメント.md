[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Kubernetes Service (AKS)]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Ingress Controller]]
# AKS Ingress Controller 設計ドキュメント

（TLS終了・パスベースルーティング・WebSocket対応）

## 1. 概要

Azure Kubernetes Service (AKS) を使用したマイクロサービスアーキテクチャでは、  
外部からのアクセスを **Ingress Controller** を通じて制御するのが一般的です。

Ingress Controller は Kubernetes クラスタの外部から内部サービスへのトラフィックを管理し、以下のような機能を提供します。

主な機能

- TLS 終了 (HTTPS termination)
    
- パスベースルーティング
    
- WebSocket対応
    
- 負荷分散
    
- API公開制御
    

本ドキュメントでは、AKSにおけるIngress Controllerの選択と設計について体系的に説明します。

---

# 2. KubernetesにおけるIngress

Kubernetesでは、PodやServiceは通常クラスタ内部ネットワークでのみ通信可能です。

外部からアクセスするためには、次の方法があります。

|方法|用途|
|---|---|
|ClusterIP|クラスタ内部通信|
|NodePort|ノードポート公開|
|LoadBalancer|外部ロードバランサ|
|Ingress|HTTP/HTTPSルーティング|

Ingressは **HTTPレベルのルーティング制御** を提供します。

---

## Ingress構造

```
Internet
↓
Ingress Controller
↓
Ingress Resource
↓
Kubernetes Service
↓
Pods
```

Ingressは以下を管理します。

- URLルーティング
    
- TLS証明書
    
- ホストベースルーティング
    

---

# 3. Ingress Controllerとは

Ingress Controllerは、Ingressリソースを実装する実体です。

Ingressリソースは単なる設定であり、  
実際にトラフィックを処理するのがIngress Controllerです。

代表的なIngress Controller

|Ingress Controller|特徴|
|---|---|
|NGINX Ingress Controller|最も一般的|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Application Gateway]]
|Azure Application Gateway Ingress Controller (AGIC)|Azureネイティブ|
|Traefik|軽量・クラウドネイティブ|
|HAProxy|高性能ロードバランサ|

---

# 4. マイクロサービスアーキテクチャとIngress

マイクロサービスでは複数のAPIが存在します。

例

```
User Service
Order Service
Billing Service
Notification Service
```

これらを次のように公開する必要があります。

- 外部公開API
    
- 内部API
    

典型構成

```
Internet
↓
Ingress Controller
↓
API Gateway
↓
Internal Services
```

---

# 5. 要件分析

問題の要件

1. AKSマイクロサービス
    
2. 1つのAPIのみ公開
    
3. 他サービスは内部のみ
    
4. TLS termination
    
5. パスベースルーティング
    
6. WebSocketサポート
    

これらを満たすIngress Controllerを選択する必要があります。

---

# 6. TLS Termination

TLS terminationとは

HTTPS通信の暗号化処理を **ロードバランサまたはIngressで終了する仕組み**です。

```
Client
HTTPS
↓
Ingress (TLS Termination)
↓
HTTP
↓
Service
```

メリット

- Pod負荷軽減
    
- 証明書集中管理
    
- セキュリティ強化
    

---

# 7. Path-based Routing

パスベースルーティングとは  
URLのパスによってサービスを振り分ける仕組みです。

例

```
api.example.com/users → User Service
api.example.com/orders → Order Service
api.example.com/billing → Billing Service
```

Ingress ControllerはURLパスを解析して適切なサービスへ転送します。

---

# 8. WebSocket

WebSocketはリアルタイム通信プロトコルです。

用途

- チャット
    
- リアルタイムダッシュボード
    
- ゲーム
    
- IoT
    

Ingress Controllerは長時間接続を維持する必要があります。

---

# 9. Azure Application Gateway Ingress Controller (AGIC)

AGICはAzure Application Gatewayと連携するIngress Controllerです。

構成

```
Internet
↓
Azure Application Gateway
↓
AGIC
↓
AKS Services
```

---

## Application Gatewayとは

Application GatewayはAzureのL7ロードバランサです。

機能

- TLS termination
    
- WAF
    
- パスベースルーティング
    
- WebSocket
    
- 自動スケール
    

---

# 10. AGICの特徴

AGICはAzure環境でAKSとネイティブ統合されています。

主な特徴

### Azure統合

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Load Balancer]]
- Azure Load Balancer不要
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Network (VNet)]]
- Azure VNET統合
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Monitor]]
- Azure Monitor統合
    

### TLS管理

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Key Vault]]
Azure Key Vault統合

```
Key Vault
↓
Application Gateway
↓
TLS Certificates
```

### WAF統合

Application Gateway WAF

- SQL Injection防御
    
- XSS防御
    
- Bot防御
    

---

# 11. AGICアーキテクチャ

```
Internet
↓
Application Gateway
↓
Public Listener
↓
Path-based routing
↓
AKS Ingress
↓
Services
↓
Pods
```

---

# 12. 公開APIと内部API

要件

- 1つのAPIのみ公開
    
- 他は内部通信
    

AGIC構成

```
Internet
↓
Application Gateway
↓
Public API
↓
AKS Service

Internal Services
↕
Cluster Network
```

内部サービスは外部公開されません。

---

# 13. 他のIngress Controller比較

|Controller|特徴|Azure適合|
|---|---|---|
|NGINX|最も一般的|中|
|AGIC|Azureネイティブ|高|
|Traefik|軽量|中|
|HAProxy|高性能|中|

---

## NGINX Ingress Controller

特徴

- OSS
    
- 柔軟
    
- Kubernetes標準
    

欠点

- Azure統合が弱い
    
- TLS管理手動
    

---

## Traefik

特徴

- クラウドネイティブ
    
- 自動構成
    

欠点

- Azure統合弱い
    

---

## HAProxy

特徴

- 高性能
    
- L4/L7対応
    

欠点

- Kubernetes統合弱い
    

---

# 14. AGICが最適な理由

AGICは次の要件を満たします。

|要件|対応|
|---|---|
|TLS termination|Application Gateway|
|Path routing|Application Gateway|
|WebSocket|サポート|
|単一API公開|Listener設定|
|Azure統合|ネイティブ|

---

# 15. 典型AKSアーキテクチャ

```
Internet
↓
Azure Front Door
↓
Application Gateway (AGIC)
↓
AKS
├ User Service
├ Order Service
├ Billing Service
└ Notification Service
```

---

# 16. セキュリティ構成

推奨構成

```
Internet
↓
Azure Front Door
↓
WAF
↓
Application Gateway
↓
AKS
```

---

# 17. AKSネットワーク構成

```
Virtual Network
│
├ Application Gateway Subnet
│
└ AKS Subnet
```

---

# 18. まとめ

AKSのIngress Controller選択では次の指針が重要です。

|要件|選択|
|---|---|
|Azureネイティブ|AGIC|
|OSS中心|NGINX|
|軽量|Traefik|
|高性能|HAProxy|

今回の要件

- TLS termination
    
- Path routing
    
- WebSocket
    
- 単一公開API
    

これを満たす最適解は

**Azure Application Gateway Ingress Controller (AGIC)**

です。

---
