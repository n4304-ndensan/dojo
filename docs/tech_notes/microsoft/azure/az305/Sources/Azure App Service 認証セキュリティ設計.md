[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure App Service]]
# Azure App Service 認証セキュリティ設計

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Microsoft Entra ID]]
（Azure AD + 条件付きアクセス）

---

# 1 背景

ある企業では、**Azure App Service** 上に多層 Web アプリケーションを展開している。  
このアプリケーションは **Azure Active Directory（Azure AD / Microsoft Entra ID）** を利用してユーザー認証を行う構成になっている。

セキュリティポリシーとして、組織は以下のアクセス制御要件を定義している。

### セキュリティ要件

- 管理されていないデバイスからのアクセスを **完全にブロックする**
    
- 企業所有の **Azure AD参加デバイス** のみアクセス可能
    
- 企業デバイスからは **追加認証なしでシームレスにアクセス（SSO）**
    
- Azure App Service は **Azure AD認証を利用**
    

この要件を満たすには、次の2つの Azure 機能を組み合わせる必要がある。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#App Registration]]
**Azure AD アプリ登録  
Azure AD 条件付きアクセスポリシー**

---

# 2 Azure AD アプリ登録

Azure AD アプリ登録は、アプリケーションを Azure AD に登録し、  
**ID プロバイダーとして Azure AD を使用するための構成**を提供する。

App Service で Azure AD 認証を有効化すると、次のような認証フローが利用される。

```text
User
 │
 ▼
Azure AD Login
 │
 ▼
Token (OIDC / OAuth)
 │
 ▼
Azure App Service
```

アプリ登録では次の情報が定義される。

|設定項目|内容|
|---|---|
|Application ID|アプリ識別子|
|Redirect URI|認証リダイレクト先|
|API permissions|APIアクセス権|
|Client secret / certificate|アプリ認証|

App Service の **Authentication / Authorization（Easy Auth）** 機能と統合することで、  
Azure AD を利用した認証を簡単に実装できる。

---

# 3 Azure AD 条件付きアクセス

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Conditional Access]]
**Azure AD 条件付きアクセス（Conditional Access）** は、ユーザーやデバイスの状態に応じてアクセス制御を行うポリシーエンジンである。

条件付きアクセスでは次のような条件を評価できる。

|条件|例|
|---|---|
|ユーザー|特定ユーザーまたはグループ|
|デバイス状態|準拠デバイスのみ|
|場所|企業ネットワーク|
|リスク|高リスクユーザー|

今回の要件では **デバイス状態** が重要になる。

---

# 4 管理されていないデバイスのブロック

条件付きアクセスポリシーを設定することで、  
**管理されていないデバイスからのアクセスを拒否**できる。

ポリシー例

```text
Condition

Device state = compliant
```

結果

```text
Managed Device → Access allowed
Unmanaged Device → Access blocked
```

この機能は通常 **Microsoft Intune のデバイス準拠ポリシー**と組み合わせて使用される。

---

# 5 シームレスサインイン（SSO）

Azure AD に参加したデバイスでは、  
ユーザーは **追加の認証プロンプトなしでアプリにアクセス**できる。

認証フロー

```text
Azure AD Joined Device
      │
      ▼
Azure AD SSO
      │
      ▼
App Service
```

この動作は次の条件で成立する。

- Azure AD joined device
    
- 条件付きアクセスで準拠デバイス許可
    
- Azure AD 認証トークンキャッシュ
    

結果として、企業デバイスでは **シームレスサインイン（Single Sign-On）** が実現される。

---

# 6 アーキテクチャ

この構成は次のようになる。

```text
User Device
   │
   ▼
Azure AD Authentication
   │
   ▼
Conditional Access Policy
   │
   ├─ Managed Device → Allow
   │
   └─ Unmanaged Device → Block
   │
   ▼
Azure App Service
```

Azure AD が認証とアクセス制御の両方を担う。

---

# 7 他の選択肢が適切でない理由

Azure AD Application Proxy は、主に **オンプレミスアプリケーションを Azure AD 経由で公開するためのサービス**であり、今回のように Azure App Service に直接 Azure AD 認証を適用するケースでは不要である。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Managed Identity]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Key Vault]]
Azure AD 管理済み ID（Managed Identity）は、アプリケーションが Azure リソース（Key Vault、Storage など）にアクセスするための **サービス間認証**に使用される。これはユーザー認証やデバイス制御には関係しない。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Container Registry]]
Azure Container Registry や Defender などのセキュリティサービスも、ユーザー認証のポリシー制御には関係しない。

---

# 8 メリット

この構成には次のメリットがある。

### セキュリティ強化

管理されていないデバイスからのアクセスを完全にブロックできる。

### シームレスなユーザー体験

企業デバイスでは追加認証なしでアクセス可能。

### 中央管理

アクセス制御は Azure AD ポリシーとして集中管理できる。

### クラウドネイティブ

追加インフラなしで App Service と統合できる。

---

# 9 まとめ

今回の要件

- Azure AD 認証
    
- 管理されていないデバイスのブロック
    
- Azure AD参加デバイスのシームレスアクセス
    

これを実現するために必要な Azure サービスは次の組み合わせである。

**Azure AD アプリ登録  
Azure AD 条件付きアクセスポリシー**

この構成により、Azure App Service アプリケーションに対して **デバイスベースのアクセス制御とシームレスな SSO** を実現できる。