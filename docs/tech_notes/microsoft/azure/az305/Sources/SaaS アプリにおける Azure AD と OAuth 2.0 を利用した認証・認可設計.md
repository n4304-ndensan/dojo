[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Microsoft Entra ID]]
# SaaS アプリにおける Azure AD と OAuth 2.0 を利用した認証・認可設計

（Web アプリと Web API のトークン発行と認可の役割）

---

# 1 背景（シナリオ）

クラウドアプリケーション、特に **SaaS（Software as a Service）アプリケーション**では、ユーザー認証と API アクセス制御を安全に実装する必要がある。

今回のシナリオでは、次のような構成のアプリケーションを構築している。

|コンポーネント|役割|
|---|---|
|Web アプリ|ユーザーが直接操作するフロントエンド|
|Web API|ビジネスロジックを提供するバックエンド|
|Azure AD|認証・トークン発行を行う ID プロバイダー|

アプリケーションの要求事項は次の通り。

- Web アプリは **Azure Active Directory（Azure AD）でユーザーを認証する**
    
- Web アプリは **OAuth 2.0 のベアラートークンを使用して Web API を呼び出す**
    
- Web API は **トークンを検証し、アクセス制御を行う**
    

この構成は **Azure AD + OAuth 2.0 を利用した標準的なクラウド認証アーキテクチャ**である。

---

# 2 アーキテクチャ概要

今回のアプリケーション構成は次のようになる。

```text
User
 │
 ▼
Web App（フロントエンド）
 │
 │ OAuth 2.0 Bearer Token
 ▼
Web API（バックエンド）
 │
 ▼
Azure AD（トークン発行）
```

この構成では **認証（Authentication）**と **認可（Authorization）** が異なる場所で処理される。

---

# 3 OAuth 2.0 の基本

OAuth 2.0 は **リソースへの安全なアクセスを提供する認可フレームワーク**である。

主なコンポーネントは次の通り。

|役割|説明|
|---|---|
|Resource Owner|ユーザー|
|Client|Web アプリ|
|Authorization Server|Azure AD|
|Resource Server|Web API|

この構成では

```text
Azure AD = Authorization Server
Web API = Resource Server
```

となる。

---

# 4 アクセストークンとは

アクセストークンは **API へのアクセス権を証明するトークン**である。

Azure AD ではアクセストークンは **JWT（JSON Web Token）形式**で発行される。

JWT トークンには次のような情報が含まれる。

|クレーム|説明|
|---|---|
|iss|発行者（Azure AD）|
|aud|対象 API|
|exp|有効期限|
|scp|許可されたスコープ|
|roles|ユーザー権限|

このトークンを Web API に送信することで API 呼び出しが可能になる。

---

# 5 トークン生成の仕組み

OAuth 2.0 の標準フローでは **トークンは Authorization Server が生成する**。

今回のシナリオでは Authorization Server は

```text
Azure Active Directory
```

である。

つまり

```text
アクセストークンは Azure AD が生成する
```

---

# 6 トークン取得フロー

Web アプリと Web API を利用する典型的なフローは次の通り。

### Step 1

ユーザーが Web アプリにアクセスする。

### Step 2

Web アプリが Azure AD にリダイレクトする。

### Step 3

ユーザーが Azure AD で認証する。

### Step 4

Azure AD がアクセストークンを発行する。

### Step 5

Web アプリがアクセストークンを取得する。

### Step 6

Web アプリがアクセストークンを使用して Web API を呼び出す。

---

# 7 認可（Authorization）の実行場所

トークンを発行するだけでは API アクセスは許可されない。

実際のアクセス制御は **リソースサーバー側**で行われる。

今回の構成では

```text
Web API
```

がリソースサーバーである。

Web API は次の処理を行う。

1. トークン署名の検証
    
2. トークンの発行者確認
    
3. トークンの有効期限確認
    
4. スコープまたはロールの検証
    

これにより **API アクセスの認可決定**が行われる。

---

# 8 Web API のトークン検証

Web API は受信した HTTP リクエストの

```text
Authorization: Bearer <token>
```

ヘッダーを確認する。

検証項目

|項目|内容|
|---|---|
|署名|Azure AD 公開キー|
|issuer|Azure AD|
|audience|API の App ID|
|expiration|トークン有効期限|
|scope|API アクセス権|

これらが正しければ API リクエストは許可される。

---

# 9 Web アプリの役割

Web アプリは **トークン発行者ではない**。

役割は次の通り。

|役割|説明|
|---|---|
|ユーザー認証開始|Azure AD にリダイレクト|
|トークン取得|Azure AD からトークン受信|
|API 呼び出し|Bearer トークン使用|

つまり

```text
Web App = Client
```

である。

---

# 10 他の選択肢が不正解の理由

### A

トークン: Web App

これは誤りである。

トークンを発行するのは **Authorization Server（Azure AD）**であり、Web アプリではない。

---

### C

トークン: Web API

Web API は **リソースサーバー**でありトークン発行は行わない。

---

### D

認証: Web App

API へのアクセス制御は **Web API 側で実施**される。

---

# 11 正しいアーキテクチャ

正しいフローは次の通り。

```text
User
 │
 ▼
Web App
 │
 │ OAuth 2.0 Authorization
 ▼
Azure AD
 │
 │ Access Token
 ▼
Web App
 │
 │ Bearer Token
 ▼
Web API
 │
 │ Token Validation
 ▼
Authorization Decision
```

---

# 12 最終回答

正解

```text
B

トークン生成: Azure AD  
認可決定: Web API
```

---

# 13 まとめ

今回の問題の重要ポイントは **OAuth 2.0 の役割分担**である。

|役割|コンポーネント|
|---|---|
|認証 / トークン発行|Azure AD|
|トークン使用|Web App|
|認可決定|Web API|

つまり

```text
Token Issuer = Azure AD
Authorization Enforcement = Web API
```

である。

このモデルは **Azure AD を利用した SaaS アプリの標準的なセキュリティアーキテクチャ**である。