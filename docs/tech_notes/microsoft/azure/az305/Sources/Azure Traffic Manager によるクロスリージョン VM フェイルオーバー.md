---
分類: Cloud
tags:
  - cloud/azure
  - cloud/azure/traffic-manager
  - cloud/azure/networking
  - cloud/architecture/high-availability
  - cloud/architecture/load-balancing
  - cloud/networking/dns-load-balancing
  - exam/azure/networking
---

# Azure Traffic Manager によるクロスリージョン VM フェイルオーバー

## 1. 背景（シナリオ）

Azure サブスクリプションには複数の仮想ネットワークが存在しています。  

- **VNet1 ↔ VNet2**
- **VNet1 ↔ VNet3**

の間で **VNet Peering** が設定されています。

環境には以下の仮想マシンが存在します。

- **VM2**
- **VM3**

この構成では、VM3 が故障した場合に **VM2 へ自動的にトラフィックを誘導**する必要があります。  
また逆に **VM2 が故障した場合には VM3 へトラフィックを誘導**する必要があります。

さらにこのソリューションは **リージョンを跨いで動作する必要があります。**

---

# 2. 要件整理

この問題の重要ポイントは次の通りです。

### 必須条件

- VM レベルの負荷分散
- VM のヘルスチェック
- 自動フェイルオーバー
- **クロスリージョン対応**

つまり

**「リージョンを跨いだフェイルオーバー型ロードバランシング」**

が必要です。

---

# 3. Azure のロードバランシングサービス整理

Azure には複数のロードバランサーがあります。

|サービス|レイヤー|範囲|用途|
|---|---|---|---|
|Azure Load Balancer|L4|リージョン内|VM の TCP/UDP 負荷分散|
|Application Gateway|L7|リージョン内|Web アプリ負荷分散|
|Front Door|L7|グローバル|HTTP/HTTPS Web|
|Traffic Manager|DNS|グローバル|クロスリージョン負荷分散|

---

# 4. Azure Traffic Manager の特徴

**Azure Traffic Manager** は

**DNS ベースのグローバルロードバランサー**

です。

特徴

- クロスリージョン対応
- ヘルスチェック
- フェイルオーバー
- DNS レベルルーティング

---

## Traffic Manager のルーティング方式

主なルーティング方法

|方式|説明|
|---|---|
|Priority|フェイルオーバー|
|Weighted|重み付き|
|Performance|最も近いリージョン|
|Geographic|地域別|
|Multi-value|複数IP|

今回のシナリオでは

**Priority (Failover)**

が最適です。

---

# 5. アーキテクチャ

```
    Client
       │
       ▼

Azure Traffic Manager  
│  
┌──────┴──────┐  
▼ ▼  
VM2 VM3  
(Region A) (Region B)

```

Traffic Manager が

- VM2
- VM3

の **ヘルスチェックを実施**

もし


VM3 DOWN


なら


Traffic → VM2


へ自動ルーティングされます。

---

# 6. 他の選択肢が間違いな理由

## A Azure Front Door

Front Door は

- **HTTP/HTTPS専用**
- Webアプリ用

VM ベースの一般トラフィックには適さない。

---

## B Azure Application Gateway

Application Gateway は

- **リージョン内ロードバランサー**

クロスリージョン対応ではない。

---

## C Azure Load Balancer

Load Balancer は

- **リージョン内 L4 Load Balancer**

クロスリージョン負荷分散はできない。

---

# 7. 最終回答

**D. Azure Traffic Manager（カスタムルーティング）**

---

# 8. 試験ポイント（超重要）

AZ-104 / AZ-305 / AZ-700 で頻出。

覚えるべきルール

```

リージョン内 VM LB  
→ Azure Load Balancer

リージョン内 Web LB  
→ Application Gateway

グローバル Web LB  
→ Front Door

グローバル DNS LB  
→ Traffic Manager

```

---

# まとめ

今回の問題のキーワード

- クロスリージョン
- VM フェイルオーバー
- ヘルスチェック
- DNS ベース

この条件を満たすサービスは

**Azure Traffic Manager**

のみです。
