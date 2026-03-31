
---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Microsoft Entra ID]]
# Azure AD を利用した ASP.NET Core アプリのユーザー委任アクセス設計

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#App Registration]]
(App Registration + Delegated Permissions)

---

# 1 背景

企業では **ASP.NET Core アプリケーション**を Azure 上の仮想マシン環境で運用している。  
本シナリオでは、以下のような構成のアプリケーションが存在する。

- ASP.NET Core アプリケーションを **100 台の仮想マシン**に展開している
    
- 2種類のアプリケーションが存在する
    

|アプリ|機能|
|---|---|
|App1|ユーザーのカレンダーを読み取る|
|App2|ユーザーのカレンダーを書き込む|

ユーザーは **Azure Active Directory（Azure AD）** を使用してアプリケーションにサインインする。

また、これらのアプリケーションは **Microsoft Graph API** を利用してユーザーのカレンダー情報にアクセスする。

---

# 2 要件整理

問題文から読み取れる重要な要件は次の通りである。

### ① ユーザーは Azure AD でサインインする

アプリケーションはユーザー認証を Azure AD に委任する必要がある。

つまり

```
ユーザーの認証情報
↓
Azure AD
↓
アプリケーション
```

という認証フローになる。

---

### ② ユーザーのカレンダーにアクセスする

アプリケーションは Microsoft Graph API を使用して、  
ユーザーのカレンダー情報にアクセスする必要がある。

---

### ③ 最小権限（Least Privilege）

セキュリティのベストプラクティスとして

**必要最小限の権限のみ付与する**

設計が求められている。

---

### ④ 管理作業を最小化する

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
アプリケーションは **100 台の VM にデプロイ**されているため、

- 認証設定
    
- 権限管理
    

を簡単に管理できる設計が必要となる。

---

# 3 Azure AD におけるアプリ認証

Azure AD でアプリケーションを認証する場合は  
**App Registration（アプリ登録）**を使用する。

App Registration は次の役割を持つ。

- アプリケーションの識別
    
- OAuth 2.0 / OpenID Connect 認証
    
- Microsoft Graph API へのアクセス制御
    
- API 権限管理
    

アーキテクチャは次のようになる。

```
User
 │
 ▼
Azure AD
 │
 ▼
App Registration
 │
 ▼
ASP.NET Core App
```

この App Registration を通じて、アプリケーションは Azure AD からトークンを取得し、Graph API にアクセスできる。

---

# 4 Microsoft Graph カレンダーアクセス

Microsoft Graph では、カレンダーアクセスに対して専用の権限が定義されている。

|権限|説明|
|---|---|
|Calendars.Read|カレンダーの読み取り|
|Calendars.ReadWrite|カレンダーの読み取りと書き込み|

今回の要件は次の通り。

|アプリ|必要権限|
|---|---|
|App1|Calendars.Read|
|App2|Calendars.ReadWrite|

---

# 5 Azure AD の権限モデル

Azure AD では API アクセス権限に **2つのモデル**が存在する。

|権限タイプ|説明|
|---|---|
|Delegated Permissions|ユーザーの代理として API を呼び出す|
|Application Permissions|アプリケーション単独で API を呼び出す|

---

# 6 Delegated Permissions

Delegated Permissions は

```
User + Application
```

という組み合わせで動作する。

つまりアプリケーションは

```
ユーザーの代理として
Microsoft Graph API を呼び出す
```

という形になる。

アクセス範囲は

```
ユーザーが本来アクセスできる範囲
```

に制限される。

これは **最小権限の原則に適合する**。

---

# 7 Application Permissions

Application Permissions は

```
Application only
```

で動作する。

特徴

- ユーザー不要
    
- テナント全体のデータへアクセス可能
    

例えば

```
全ユーザーのメールボックス
```

などにアクセスできる。

しかしこれは

**権限が過剰になりやすい**

という問題がある。

---

# 8 Delegated Permissions が適切な理由

今回の要件では

```
ユーザーが Azure AD にサインイン
```

する必要がある。

つまり

```
User Context
```

を利用する必要がある。

Delegated Permissions を使用すると次のフローになる。

```
User Login
     ↓
Azure AD
     ↓
App Registration
     ↓
Application
     ↓
Microsoft Graph
     ↓
User Calendar
```

この方法では

- ユーザーのデータのみアクセス
    
- 最小権限の原則を維持
    

することができる。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Managed Identity]]
# 9 Managed Identity が不適切な理由

Managed Identity は Azure リソースの認証に使用される仕組みである。

主な用途

|用途|例|
|---|---|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Key Vault]]
|VM → Key Vault|秘密情報取得|
|VM → Storage|ストレージアクセス|
|VM → SQL Database|データベース接続|

しかし今回のシナリオは

```
User → Graph API
```

である。

Managed Identity は

**ユーザーの代理として Microsoft Graph を呼び出す用途ではない**

ため適切ではない。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Role-Based Access Control (RBAC)]]
# 10 Azure RBAC が不適切な理由

Azure RBAC は

```
Azure Resource 管理
```

のための権限制御である。

例

- Virtual Machine
    
- Storage Account
    
- Key Vault
    

などの **Azure リソースアクセス制御**に使用される。

しかし

```
Microsoft Graph のユーザーデータ
```

には使用されない。

---

# 11 推奨アーキテクチャ

最適な構成は次の通りである。

```
User
 │
 ▼
Azure AD Login
 │
 ▼
App Registration
 │
 ▼
ASP.NET Core Application
 │
 ▼
Microsoft Graph API
 │
 ▼
User Calendar
```

---

# 12 App1 / App2 の権限設計

|アプリ|Delegated Permission|
|---|---|
|App1|Calendars.Read|
|App2|Calendars.ReadWrite|

この構成により

- 必要最小限の権限
    
- セキュアなアクセス
    

が実現される。

---

# 13 大規模 VM 環境でも管理が容易

App Registration を使用することで

- 認証設定を一元管理
    
- VM 数に依存しない
    
- 権限設定も一箇所
    

というメリットがある。

構成イメージ

```
100 VM
   │
   ▼
Single App Registration
```

---

# 14 最終回答

正解

**C**

```
Authentication : App Registration
Authorization  : Delegated Permissions
```

---

# 15 まとめ

今回の設計ポイント

|要件|解決方法|
|---|---|
|ユーザーサインイン|Azure AD|
|Graph API アクセス|App Registration|
|最小権限|Delegated Permissions|
|大規模デプロイ|App Registration 共有|

そのため最適な設計は

```
Azure AD App Registration
+
Delegated Permissions
```

である。

これは **Microsoft Graph を利用するアプリケーションの標準的なアーキテクチャ**であり、Microsoft の推奨パターンでもある。

---

もし希望があれば、次に

**Azure試験で最重要のまとめ**

- Delegated vs Application Permissions
    
- Managed Identity vs Service Principal
    
- Graph vs RBAC
    

を **1枚で理解できる図**を作ります。  
（これ作ると Azure 問題かなり解けるようになります）