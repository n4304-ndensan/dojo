[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Microsoft Entra ID]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure API Management]]
# Azure AD と API Management を用いた内部 API セキュリティ設計

（Azure AD Authorization + APIM JWT Validation）

---

# 1 背景

企業では複数の内部サービスを Azure 上で運用しており、内部 API を **Azure API Management（APIM）** を介して公開している。

今回のシステム構成は次の通りである。

- **内部 Web API：20件**
    
- **Web アプリケーション：10件**
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#App Registration]]
- すべて **Azure AD にアプリ登録されている**
    
- API は **Azure API Management を経由して公開**
    

システム構成イメージ

```text
User / Client App
        │
        ▼
Azure AD (Authentication)
        │
        ▼
Web Applications
        │
        ▼
Azure API Management
        │
        ▼
Internal APIs (20)
```

これらの API は Azure AD の **クレーム（claims）** を利用してアクセス制御を行う。

---

# 2 要件

問題文から重要な要件を整理する。

### ① Azure AD のクレームを使用する

API へのアクセス制御は **JWT トークンのクレーム情報**を利用する。

代表的なクレーム

|クレーム|意味|
|---|---|
|iss|トークン発行者|
|aud|対象 API|
|scope|アクセス権|
|roles|ロール|

---

### ② 不正なリクエストをブロックする

Azure AD で発行された **正しいトークンのみ**を API に到達させる必要がある。

---

### ③ 最小限の設定

API は **20 個存在する**ため、

- 各 API に認証ロジックを実装する
    
- 各 API にトークン検証コードを書く
    

といった構成は **運用コストが高い**。

---

### ④ スケーラブルな管理

中央で認証・認可を管理できる構成が求められる。

---

# 3 Azure AD による権限管理

Azure AD は **認可（Authorization）管理の中心**として使用する。

具体的には以下を定義する。

### API スコープ

Azure AD の **App Registration** で API を公開する。

例

```text
api://internal-api
```

ここにスコープを定義する。

例

```text
read_api
write_api
admin_api
```

---

### クライアントアプリの権限

Web アプリケーションには API アクセス権を付与する。

```text
Client App
    │
    ▼
Azure AD
    │
    ▼
Access Token
    │
    ▼
API
```

このトークンには次の情報が含まれる。

```json
{
 "aud": "api://internal-api",
 "scope": "read_api"
}
```

---

# 4 JWT トークンとは

Azure AD は **JWT (JSON Web Token)** を発行する。

JWT は次の構造を持つ。

```text
Header
Payload
Signature
```

Payload にはクレーム情報が含まれる。

例

```json
{
 "iss": "https://login.microsoftonline.com/{tenant}",
 "aud": "api://internal-api",
 "scope": "read_api"
}
```

---

# 5 JWT 検証の役割

API はトークンの次の項目を検証する必要がある。

|検証項目|内容|
|---|---|
|issuer|Azure AD|
|audience|API|
|signature|トークン署名|
|scope|必要な権限|

---

# 6 API Management を使う理由

JWT 検証を **各 API に実装することも可能**だが、問題がある。

問題点

- API 20個すべてに実装が必要
    
- メンテナンスが複雑
    
- 認証ロジックが分散する
    

そのため **API Management で検証するのが最適**である。

---

# 7 API Management による JWT 検証

APIM には **validate-jwt ポリシー**が存在する。

例

```xml
<validate-jwt header-name="Authorization">
    <openid-config url="https://login.microsoftonline.com/{tenant}/v2.0/.well-known/openid-configuration" />
    <audiences>
        <audience>api://internal-api</audience>
    </audiences>
</validate-jwt>
```

このポリシーにより

- トークン署名
    
- 発行者
    
- audience
    
- 有効期限
    

が自動的に検証される。

---

# 8 認証フロー

実際のフローは次のようになる。

```text
Client App
    │
    ▼
Azure AD
(Authentication + Authorization)
    │
    ▼
Access Token (JWT)
    │
    ▼
API Management
(JWT validation)
    │
    ▼
Internal API
```

APIM が **セキュリティゲート**として機能する。

---

# 9 他の選択肢が不適切な理由

## A

Azure AD 権限付与 + Web API JWT 検証

問題

- API 20個すべてに JWT 検証実装
    
- 運用コスト増加
    

---

## B

API Management 権限付与

APIM は **認証システムではない**。

権限管理は Azure AD で行うべきである。

---

## D

Web API で権限付与 + Azure AD JWT 検証

責務が逆転している。

Azure AD は **トークン発行者**であり  
JWT 検証は API 側または APIM 側で行う必要がある。

---

# 10 推奨アーキテクチャ

最適構成

```text
Client Applications (10)
        │
        ▼
Azure AD
(Token issuance + Authorization)
        │
        ▼
API Management
(JWT validation)
        │
        ▼
Internal APIs (20)
```

この構成により

- セキュリティ集中管理
    
- API 軽量化
    
- スケーラビリティ
    

が実現される。

---

# 11 運用メリット

### セキュリティ統一

すべての API が **同一ポリシーで保護される**。

---

### 管理コスト削減

JWT 検証は **APIM で1回設定するだけ**。

---

### 監査ログ

API Management でアクセスログを集中管理できる。

---

### キーローテーション

Azure AD が署名キーを自動管理する。

---

# 12 実装手順（概要）

### Azure AD

1. API を App Registration
    
2. スコープ定義
    
3. クライアントアプリに権限付与
    

---

### API Management

1. API を登録
    
2. validate-jwt ポリシー設定
    
3. audience / issuer 設定
    

---

# 13 最終回答

正解

**C**

```text
Authorization : Azure AD
JWT Validation : API Management
```

---

# 14 まとめ

今回の設計のポイント

|要件|設計|
|---|---|
|権限管理|Azure AD|
|トークン発行|Azure AD|
|トークン検証|API Management|
|バックエンドAPI|軽量化|

つまり

```text
Azure AD → Authorization
API Management → JWT Validation
```

という構成が最も適している。

これは **大規模 API 環境における Azure の推奨アーキテクチャ**でもある。