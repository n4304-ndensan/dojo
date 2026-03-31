# Azure グローバルルーティングおよびWeb配信サービス設計ドキュメント

## 1. 概要

Azureでは、グローバル規模のWebアプリケーションを提供するために複数のトラフィック管理サービスが提供されています。  
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure DNS]]
それぞれのサービスは **動作レイヤー（DNS / L7 / キャッシュ）やスコープ（リージョン / グローバル）** が異なります。

主な目的

- グローバルユーザーへの低遅延配信
    
- 可用性の向上
    
- トラフィック分散
    
- セキュリティ強化
    
- コンテンツ配信の高速化
    

Azureの主要サービス

|サービス|役割|
|---|---|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Front Door]]
|Azure Front Door|グローバルL7ロードバランサ|
|Azure Traffic Manager|DNSベースルーティング|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Application Gateway]]
|Azure Application Gateway|リージョン内L7ロードバランサ|
|Azure CDN|静的コンテンツ配信|

---

# 2. Azure Front Door

Azure Front Door は **グローバルレイヤー7ロードバランサー**です。

Microsoftのグローバルエッジネットワークを利用して、ユーザーの最寄りのバックエンドへトラフィックをルーティングします。

## 主な機能

- グローバルロードバランシング
    
- TLS終端
    
- URLベースルーティング
    
- WAF（Web Application Firewall）
    
- DDoS軽減
    
- カスタムドメイン
    
- 管理SSL証明書
    
- APIおよびWeb配信
    

## アーキテクチャ

```id="fd01"
User
 ↓
Azure Front Door (Edge Network)
 ↓
Region A Backend
Region B Backend
```

## 特徴

- HTTP / HTTPS L7ルーティング
    
- エッジネットワーク処理
    
- 最寄りリージョンへ自動ルーティング
    
- 高速レスポンス
    

## 主な用途

- グローバルWebアプリ
    
- SaaSアプリケーション
    
- APIゲートウェイ
    
- マイクロサービス
    

---

# 3. Azure Traffic Manager

Azure Traffic Manager は **DNSベースのトラフィックルーティングサービス**です。

DNS応答を利用してユーザーを最適なエンドポイントへ誘導します。

## アーキテクチャ

```id="tm01"
User
 ↓
DNS Query
 ↓
Traffic Manager
 ↓
Region A
Region B
```

## ルーティング方式

|方式|説明|
|---|---|
|Priority|フェイルオーバー|
|Weighted|重み付け|
|Performance|最寄りリージョン|
|Geographic|地理ベース|

## 特徴

- DNSレベルルーティング
    
- HTTP処理なし
    
- TLS終端なし
    
- URLルーティングなし
    

## 主な用途

- グローバルフェイルオーバー
    
- シンプルなリージョン分散
    
- DNSベーストラフィック管理
    

---

# 4. Azure Application Gateway

Azure Application Gateway は **リージョン内L7ロードバランサー**です。

仮想ネットワーク内で動作し、HTTPトラフィックを処理します。

## アーキテクチャ

```id="ag01"
User
 ↓
Application Gateway
 ↓
Backend Pool
```

## 主な機能

- URLベースルーティング
    
- TLS終端
    
- Web Application Firewall
    
- Cookieベースセッション
    

## 特徴

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Network (VNet)]]
- VNET統合
    
- 内部サービス保護
    
- リージョンスコープ
    

## 主な用途

- Webアプリロードバランサ
    
- マイクロサービスルーティング
    
- APIゲートウェイ
    

---

# 5. Azure CDN

Azure CDN は **コンテンツ配信ネットワーク**です。

世界中のエッジキャッシュを利用して静的コンテンツを高速配信します。

## アーキテクチャ

```id="cdn01"
User
 ↓
CDN Edge
 ↓
Origin Server
```

## 主な機能

- 静的コンテンツキャッシュ
    
- レイテンシ削減
    
- 帯域削減
    

## 配信対象

- 画像
    
- CSS
    
- JavaScript
    
- 動画
    
- ダウンロードファイル
    

## 特徴

- APIルーティング不可
    
- グローバルロードバランスなし
    

---

# 6. Azureサービス比較

|機能|Front Door|Traffic Manager|App Gateway|CDN|
|---|---|---|---|---|
|グローバルルーティング|○|○|×|×|
|URLルーティング|○|×|○|×|
|TLS終端|○|×|○|○|
|WAF|○|×|○|×|
|DNSルーティング|×|○|×|×|
|静的キャッシュ|○|×|×|○|

---

# 7. 設計パターン

## 7.1 グローバルWebアプリ

```id="arch01"
User
 ↓
Azure Front Door
 ↓
Region A App Service
Region B App Service
```

目的

- グローバル負荷分散
    
- 高可用性
    
- 高速配信
    

---

## 7.2 静的コンテンツ高速化

```id="arch02"
User
 ↓
Azure CDN
 ↓
Storage / Web App
```

目的

- キャッシュ配信
    
- レイテンシ削減
    

---

## 7.3 DNSフェイルオーバー

```id="arch03"
User
 ↓
Traffic Manager
 ↓
Primary Region
Secondary Region
```

目的

- 災害対策
    
- DNSベースフェイルオーバー
    

---

# 8. サービス選択指針

Azureでのサービス選択は以下の基準で判断します。

## グローバルHTTPロードバランサ

→ Azure Front Door

## DNSレベルルーティング

→ Azure Traffic Manager

## リージョン内Webロードバランサ

→ Application Gateway

## 静的コンテンツ配信

→ Azure CDN

---

# 9. 試験対策判断フロー

Azure試験では以下のキーワードで判断します。

|要件|選択サービス|
|---|---|
|グローバルWebアプリ|Front Door|
|最寄りリージョン|Front Door|
|URLルーティング|Front Door / App Gateway|
|DNSルーティング|Traffic Manager|
|静的配信|CDN|

---

# まとめ

AzureのWeb配信アーキテクチャは、  
**Front Door を中心に各サービスを組み合わせることで構築されます。**

基本構成

- グローバルルーティング → Front Door
    
- リージョン内ロードバランス → Application Gateway
    
- 静的コンテンツ → CDN
    
- DNS制御 → Traffic Manager
    

これらを適切に組み合わせることで、  
**高可用性・低遅延・高セキュリティのWebアプリケーションを構築できます。**