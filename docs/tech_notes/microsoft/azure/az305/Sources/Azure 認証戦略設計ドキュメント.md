# Azure 認証戦略設計ドキュメント

（社内ユーザー + 外部パートナー / SSO / リスクベース条件付きアクセス）

---

# 1 背景

ある組織では、新しいアプリケーションを構築しており、以下のユーザーが利用する予定である。

- 社内従業員
    
- 外部パートナー
    

外部パートナーはそれぞれ異なる **Identity Provider（IdP）** を使用している可能性がある。

例

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Microsoft Entra ID]]
- Azure AD
    
- Google
    
- Facebook
    
- 他企業のAD
    
- SAML IdP
    

このアプリケーションでは、次のセキュリティ要件が求められている。

### 必須要件

1. **Single Sign-On（SSO）**
    
2. **ユーザーリスクベースの条件付きアクセス**
    
3. **複数IdPのサポート**
    
4. **社内ユーザー + 外部パートナーの統合管理**
    

この要件を満たす Azure サービスは

**Microsoft Identity Platform + External Identities**

である。

---

# 2 Microsoft Identity Platform

Microsoft Identity Platform は Azure AD（現在は Entra ID）のアイデンティティ基盤であり、以下の機能を提供する。

### 主な機能

|機能|説明|
|---|---|
|認証|OAuth2 / OpenID Connect|
|SSO|複数アプリケーション|
|条件付きアクセス|リスクベース制御|
|外部ID|B2B / B2C|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Multi-Factor Authentication (MFA)]]
|MFA|多要素認証|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Identity Governance]]
|Identity Governance|アクセス制御|

---

# 3 External Identities

External Identities は Azure AD の外部ユーザー管理機能である。

対応するユーザー

```text
Internal Users
Employees
↓
Azure AD

External Users
Partners
Suppliers
Customers
↓
External Identity Providers
```

---

# 4 認証アーキテクチャ

今回の構成は以下のようになる。

```text
Users
 │
 ├ Internal Employees
 │     │
 │     └ Azure AD
 │
 └ External Partners
       │
       ├ Partner Azure AD
       ├ Google
       ├ Facebook
       └ SAML IdP

        ↓
Microsoft Identity Platform
        ↓
Application
```

---

# 5 Single Sign-On (SSO)

SSOによりユーザーは一度の認証で複数のアプリケーションにアクセスできる。

### SSOの流れ

```text
User
 │
 ▼
Identity Platform
 │
 ▼
Authentication
 │
 ▼
Token Issued
 │
 ▼
Application Access
```

使用プロトコル

- OAuth2
    
- OpenID Connect
    
- SAML
    

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Conditional Access]]
# 6 条件付きアクセス（Conditional Access）

条件付きアクセスは、ユーザーのリスク状態に応じてアクセス制御を行う。

### 例

|条件|アクション|
|---|---|
|通常ログイン|アクセス許可|
|不審なIP|MFA要求|
|高リスクユーザー|アクセス拒否|

---

### リスク判定

Azure AD Identity Protection により

```text
User Sign-in
      │
      ▼
Risk Evaluation
      │
      ├ Low Risk → Access
      ├ Medium Risk → MFA
      └ High Risk → Block
```

---

# 7 外部パートナー認証

External Identities は多様な IdP をサポートする。

### サポートIdP

|IdP|説明|
|---|---|
|Azure AD|他企業|
|Google|ソーシャルログイン|
|Facebook|ソーシャルログイン|
|SAML|企業IdP|
|OpenID|外部IdP|

---

### 外部ユーザーアクセス

```text
Partner User
     │
     ▼
External IdP
     │
     ▼
Microsoft Identity Platform
     │
     ▼
Application
```

---

# 8 内部ユーザー認証

社内ユーザーは Azure AD を利用する。

```text
Employee
   │
   ▼
Azure AD
   │
   ▼
SSO Token
   │
   ▼
Application
```

---

# 9 セキュリティ機能

Microsoft Identity Platform は以下のセキュリティ機能を提供する。

### MFA

- SMS
    
- Authenticator
    
- FIDO2
    

---

### Identity Protection

ユーザーのリスクを評価

|リスク|説明|
|---|---|
|Impossible Travel|不可能な移動|
|Leaked Credentials|漏洩パスワード|
|Suspicious Login|不審ログイン|

---

### Identity Governance

アクセス管理

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Access Reviews]]
- Access reviews
    
- Privileged Identity Management
    
- Entitlement management
    

---

# 10 他の選択肢の評価

## Azure AD B2B Collaboration

用途

- パートナー企業とのコラボレーション
    

問題

- 外部IdPの幅広い統合が弱い
    
- 包括的ID戦略ではない
    

---

## Azure AD B2C

用途

- 顧客向けアプリ
    

問題

- 社内ユーザー管理が対象外
    

---

## Azure AD Application Proxy

用途

- オンプレミスアプリ公開
    

問題

- 認証基盤ではない
    

---

## Azure AD Domain Services

用途

- LDAP / Kerberos
    

問題

- モダン認証ではない
    

---

# 11 推奨アーキテクチャ

```text
Users
 │
 ├ Employees
 │     │
 │     └ Azure AD
 │
 └ Partners
       │
       ├ Azure AD
       ├ Google
       ├ Facebook
       └ SAML

        ↓
Microsoft Identity Platform
        ↓
Conditional Access
        ↓
Application
```

---

# 12 利点

Microsoft Identity Platform + External Identities を使用する利点

### 統合ID管理

内部 + 外部ユーザーを統一管理

---

### セキュリティ

- MFA
    
- Risk Based Access
    
- Conditional Access
    

---

### SSO

複数アプリケーション統合

---

### 拡張性

- 多様なIdP
    
- B2B
    
- B2C
    

---

# 13 まとめ

今回の要件

- 社内ユーザー
    
- 外部パートナー
    
- 複数IdP
    
- SSO
    
- リスクベース条件付きアクセス
    

これらを満たす最適なサービスは

**Microsoft Identity Platform + External Identities**

である。

この構成により

- 統合認証基盤
    
- セキュアなアクセス管理
    
- スケーラブルな外部連携
    

を実現できる。