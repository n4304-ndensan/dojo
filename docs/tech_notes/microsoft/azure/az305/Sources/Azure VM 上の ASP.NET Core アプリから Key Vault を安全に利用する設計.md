[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Key Vault]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
# Azure VM 上の ASP.NET Core アプリから Key Vault を安全に利用する設計

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Managed Identity]]
（Managed Identity + OAuth2 Client Credentials Flow）

---

# 1 背景

企業では **ASP.NET Core アプリケーション**を Azure 環境で運用している。  
今回のシナリオでは、アプリケーションは **Windows Server 2022 Azure Edition の仮想マシン (VM)** 上にデプロイされている。

このアプリケーションは、外部サービスや内部システムとの連携のために **機密情報（シークレット）**を使用する必要がある。  
例としては以下のような情報が考えられる。

- API キー
    
- 接続文字列
    
- パスワード
    
- 証明書情報
    

これらの機密情報をアプリケーションの設定ファイルやソースコードに直接保存すると、セキュリティ上のリスクが高くなる。そのため Azure では **Azure Key Vault** を使用して機密情報を安全に管理することが推奨されている。

今回の要件は次の通りである。

- ASP.NET Core アプリが **Key Vault から秘密情報を取得する**
    
- アプリケーションコードの変更を **最小限に抑える**
    
- 認証情報（パスワードなど）をコードに保存しない
    
- Azure VM の **システム割り当て Managed Identity** を使用する
    

この要件を満たすためには、適切な **OAuth 2.0 フロー**と **トークンエンドポイント**を理解する必要がある。

---

# 2 要件整理

問題文から読み取れる重要な要件は次の通りである。

### ① システム割り当て Managed Identity を使用する

VM に割り当てられた **Managed Identity** を利用して認証を行う。

つまりアプリケーションは

```text
認証情報なし
```

で Azure リソースにアクセスする必要がある。

---

### ② Key Vault にアクセスする

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Microsoft Entra ID]]
Key Vault にアクセスするためには **Azure AD のアクセストークン**を取得する必要がある。

---

### ③ ユーザーは存在しない

このシナリオでは

```text
User Login
```

が存在しない。

つまり

```text
Service-to-Service communication
```

である。

---

### ④ 最小限のコード変更

Azure SDK では次のような方法が推奨される。

```csharp
DefaultAzureCredential
```

この認証方式は Managed Identity を自動的に使用する。

---

# 3 Managed Identity とは

Managed Identity は Azure が提供する **自動管理されたサービス ID**である。

Azure リソース（VM、App Service、Function など）に対して **Azure AD の ID**を割り当てることができる。

特徴

- パスワード管理不要
    
- 秘密情報をコードに保存しない
    
- Azure AD で認証
    
- 自動ローテーション
    

今回の構成は次のようになる。

```text
ASP.NET Core App
      │
      ▼
Managed Identity
      │
      ▼
Azure AD
      │
      ▼
Azure Key Vault
```

---

# 4 OAuth 2.0 フローの種類

OAuth 2.0 には複数の認証フローが存在する。

|フロー|用途|
|---|---|
|Authorization Code Flow|ユーザーサインイン|
|Implicit Flow|ブラウザアプリ|
|Client Credentials Flow|サービス間通信|

---

# 5 Authorization Code Flow

Authorization Code Flow は次の用途で使用される。

```text
User Login
```

例

- Web アプリ
    
- SaaS アプリ
    
- Microsoft 365 アプリ
    

フロー

```text
User
 ↓
Login
 ↓
Authorization Code
 ↓
Token
```

このフローは **ユーザーが必要**である。

今回のシナリオには適していない。

---

# 6 Implicit Flow

Implicit Flow は主に **ブラウザベースのアプリ**で使用される。

例

- SPA (Single Page Application)
    

しかし現在では **セキュリティ上の理由から非推奨**である。

---

# 7 Client Credentials Flow

Client Credentials Flow は

```text
Application → Application
```

の通信で使用される。

特徴

- ユーザー不要
    
- サービス間通信
    
- バックエンド処理
    

典型例

```text
VM Application
     │
     ▼
Azure AD
     │
     ▼
Access Token
     │
     ▼
Azure Key Vault
```

このフローは **Managed Identity の内部動作でも使用される**。

---

# 8 Azure AD トークンエンドポイント

Azure AD はアクセストークンを発行するための **トークンエンドポイント**を提供する。

Managed Identity の場合、アプリケーションは内部的に次の処理を行う。

```text
Managed Identity
     │
     ▼
Azure AD Token Endpoint
     │
     ▼
Access Token
```

このトークンを使用して Key Vault にアクセスする。

---

# 9 IMDS（Instance Metadata Service）

VM では **Azure Instance Metadata Service (IMDS)** が利用される。

IMDS は VM に関する情報を提供するサービスであり、Managed Identity のトークン取得にも使用される。

内部処理の流れ

```text
Application
   │
   ▼
IMDS
   │
   ▼
Azure AD Token Endpoint
   │
   ▼
Access Token
```

---

# 10 DefaultAzureCredential の利用

ASP.NET Core アプリでは、Azure SDK を使用すると簡単に Key Vault にアクセスできる。

例

```csharp
var client = new SecretClient(
    new Uri("https://myvault.vault.azure.net/"),
    new DefaultAzureCredential());
```

DefaultAzureCredential は以下の順序で認証を試みる。

1. Managed Identity
    
2. Visual Studio
    
3. Azure CLI
    
4. Environment Variables
    

Azure VM 上では **Managed Identity**が自動的に使用される。

---

# 11 他の選択肢が不適切な理由

### 認証コードフロー

問題点

- ユーザー操作が必要
    
- サービス間通信に不適切
    

---

### 暗黙のフロー

問題点

- ブラウザアプリ用
    
- 現在は非推奨
    

---

### Microsoft Identity Platform エンドポイント

これは SPA やユーザー認証用途であり、今回の VM アプリには適していない。

---

# 12 推奨アーキテクチャ

最適な構成は次の通りである。

```text
ASP.NET Core App
       │
       ▼
System Assigned Managed Identity
       │
       ▼
Azure Instance Metadata Service
       │
       ▼
Azure AD Token Endpoint
       │
       ▼
Access Token
       │
       ▼
Azure Key Vault
```

---

# 13 最終回答

正解

**B**

```text
OAuth Flow: Client Credentials Flow
Token Endpoint: Azure AD Token Endpoint
```

---

# 14 まとめ

今回の設計ポイント

|要件|解決方法|
|---|---|
|認証情報管理不要|Managed Identity|
|Key Vault アクセス|Azure AD Token|
|ユーザー不要|Client Credentials Flow|
|コード変更最小|DefaultAzureCredential|

そのため最適な設計は

```text
Managed Identity
+
Client Credentials Flow
+
Azure AD Token Endpoint
```

である。

これは **Azure VM から Key Vault を利用する際の標準的なセキュリティアーキテクチャ**であり、Microsoft が推奨する方法でもある。