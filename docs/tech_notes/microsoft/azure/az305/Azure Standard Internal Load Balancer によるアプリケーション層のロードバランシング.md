## Azure Standard Internal Load Balancer によるアプリケーション層のロードバランシング

### 1 概要

Azure Standard Internal Load Balancer は、Azure 仮想ネットワーク内の内部トラフィックを負荷分散する **L4ロードバランサー（Layer4）** である。  
主に同一VNet内のサービス間通信をスケールさせるために使用される。

今回のシナリオでは

- Webフロントエンド
    
- アプリケーション層
    

という **3層アーキテクチャの内部通信**が対象である。

この通信では

・ポートフォワーディング  
・HTTPSヘルスプローブ  
・可用性セットのバックエンド

が必要であり、これらの要件を満たすのが **Azure Standard Internal Load Balancer** である。

---

# 2 背景

Azure のロードバランシングサービスは複数存在する。

|サービス|レイヤ|
|---|---|
|Azure Load Balancer|L4|
|Azure Application Gateway|L7|
|Azure Front Door|L7|
|Azure Traffic Manager|DNS|

それぞれ役割が異なる。

典型的な **3層Webアーキテクチャ**は次の構造になる。

```
Internet
   │
   ▼
Web Tier
   │
   ▼
App Tier
   │
   ▼
Database
```

この構造ではロードバランサーも2種類使われる。

```
Internet → Web
Web → App
```

役割が異なるため **異なるサービスを使う設計になる**。

---

# 3 サービスの仕組み

Azure Load Balancer は **Layer4 (TCP/UDP)** レベルで動作する。

```
Client
  │
  ▼
Load Balancer
  │
  ├ VM
  ├ VM
  └ VM
```

内部ロードバランサーの場合は

```
VNet 内通信
```

のみ処理する。

```
Web Tier
   │
   ▼
Internal Load Balancer
   │
   ├ App VM
   ├ App VM
   └ App VM
```

---

# 4 主要機能

### ポートフォワーディング

Azure Load Balancer は

```
Frontend Port → Backend Port
```

のマッピングを行える。

例

```
80 → 8080
443 → 8443
```

これはアプリケーション層への通信で重要になる。

---

### ヘルスプローブ

バックエンド VM の正常性を監視する。

```
Load Balancer
    │
    ├ probe VM1
    ├ probe VM2
    └ probe VM3
```

HTTPS ヘルスチェックも可能。

異常ノードにはトラフィックを送らない。

---

### 可用性セット統合

Load Balancer のバックエンドプールには

- VM
    
- VMSS
    
- 可用性セット
    

を登録できる。

```
Availability Set
   │
   ├ VM1
   ├ VM2
   └ VM3
```

高可用構成を実現する。

---

### 内部ロードバランシング

Internal Load Balancer は

```
Private IP
```

で動作する。

```
Web VM
   │
   ▼
Internal LB (10.0.0.5)
   │
   ├ App VM
   └ App VM
```

外部公開されない。

---

# 5 関連Azureサービス

この問題の構成では通常次のようなサービスが組み合わされる。

|サービス|役割|
|---|---|
|Azure Application Gateway|Web Tier入口|
|Azure Load Balancer|App Tier負荷分散|
|Azure SQL / Database|データ層|
|Azure Virtual Network|ネットワーク|

典型構成

```
Internet
   │
   ▼
Application Gateway + WAF
   │
   ▼
Web Tier VM
   │
   ▼
Internal Load Balancer
   │
   ├ App VM
   └ App VM
```

---

# 6 アーキテクチャ

今回の問題の完全構造

```
Internet
   │
   ▼
Application Gateway (WAF)
   │
   │ URL routing
   │ Connection draining
   │ SQL injection protection
   │
   ▼
Web Frontend
   │
   ▼
Internal Standard Load Balancer
   │
   │ Port forwarding
   │ HTTPS health probe
   │
   ▼
Application Tier
   │
   ├ VM (Availability Set)
   ├ VM (Availability Set)
   └ VM (Availability Set)
```

---

# 7 他サービスとの違い

### Azure Application Gateway

Layer7ロードバランサー。

特徴

- URLルーティング
    
- Cookieセッション
    
- WAF
    

しかし

```
Web → App
```

内部通信には通常使わない。

---

### Azure Basic Load Balancer

旧世代サービス。

制限

- SLAなし
    
- 機能不足
    
- 推奨されない
    

AZ-305では

**Standardを選ぶのが基本**。

---

### Public Load Balancer

Public IPを持つ。

```
Internet → Azure
```

の通信で使用。

内部通信には不適。

---

# 8 ユースケース

### 3層アプリケーション

```
Internet
   │
Application Gateway
   │
Web Tier
   │
Internal LB
   │
App Tier
```

---

### マイクロサービス内部通信

```
Service A
   │
Internal LB
   │
Service B
```

---

### 高可用API層

```
Internal LB
   │
   ├ API VM
   ├ API VM
   └ API VM
```

---

# 9 設計指針

Azureアーキテクト設計では

|通信|推奨サービス|
|---|---|
|Internet → Web|Application Gateway|
|Web → App|Internal Load Balancer|
|Global routing|Front Door|

覚え方

```
L7 = Web入口
L4 = 内部負荷分散
```

AZ-305で頻出の設計パターン。

---

# 10 まとめ

今回の要件

Web → App 通信

必要機能

- ポートフォワーディング
    
- HTTPSヘルスプローブ
    
- 可用性セットバックエンド
    

これらを満たすのは

**Azure Standard Internal Load Balancer**

である。

典型アーキテクチャ

```
Internet
   │
Application Gateway (WAF)
   │
Web Tier
   │
Internal Standard Load Balancer
   │
Application Tier
```

これは **Azureの標準3層Webアーキテクチャ設計**であり、  
AZ-305試験でも非常に重要なパターンである。