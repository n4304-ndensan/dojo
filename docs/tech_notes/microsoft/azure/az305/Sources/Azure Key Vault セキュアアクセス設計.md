[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Key Vault]]
# Azure Key Vault セキュアアクセス設計

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Managed Identity]]
（PIM + Managed Identity）

---

# 1 背景

ある組織では、**Azure Key Vault** を利用してアプリケーションのシークレット（APIキー、接続文字列、証明書など）を管理する Web アプリケーションを開発している。

このシステムでは、複数のチームが Key Vault に関連するアクセスを必要としている。

### 関係チーム

- セキュリティチーム
    
- 開発チーム
    
- QAチーム
    

それぞれのチームに対して、異なるセキュリティ要件が定義されている。

---

# 2 要件

### セキュリティチーム

- Key Vault 管理者アクセス
    
- **常時権限は禁止**
    
- **ジャストインタイム（JIT）アクセス**
    
- 承認フローが必要
    

---

### 開発チーム

- Webアプリから Key Vault のシークレットを取得
    
- **コードに認証情報を埋め込まない**
    
- 安全なサービス認証
    

---

### QAチーム

- テスト時のみ **一時的に高権限**
    
- 常時管理者権限は不要
    
- 時間制限付きアクセス
    

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Microsoft Entra ID]]
# 3 推奨 Azure AD 構成

これらの要件を満たす構成は次の通り。

|チーム|Azure AD 機能|
|---|---|
|セキュリティ|Azure AD Privileged Identity Management (PIM)|
|開発|Managed Identity|
|QA|Azure AD Privileged Identity Management (PIM)|

---

# 4 Azure AD Privileged Identity Management（PIM）

**Azure AD Privileged Identity Management（PIM）** は、Azure AD の特権ロールに対して **ジャストインタイムアクセス**を提供する機能である。

PIM を使用すると、ユーザーは通常は特権ロールを持たず、必要なときだけ一時的に昇格できる。

### PIM の特徴

|機能|説明|
|---|---|
|JITアクセス|必要時のみ権限付与|
|承認ワークフロー|管理者承認|
|時間制限|アクセス期限|
|監査ログ|操作履歴|

---

### PIM アクセスフロー

```text
User
 │
 ▼
Request role activation
 │
 ▼
Approval workflow
 │
 ▼
Temporary role assignment
 │
 ▼
Role expires automatically
```

---

### セキュリティチームでの利用

Key Vault 管理者は通常はロールを持たず、必要なときだけアクセスする。

例

```text
Key Vault Administrator
Activation duration: 1 hour
Approval required
```

これにより

- 常時管理者権限を防止
    
- 内部不正リスク低減
    
- 監査対応
    

が実現される。

---

# 5 Managed Identity

**Managed Identity** は、Azure リソースが Azure AD を利用して認証を行う仕組みである。

これによりアプリケーションは

- パスワード
    
- APIキー
    
- クライアントシークレット
    

を **コードに保存する必要がなくなる**。

---

### Managed Identity の仕組み

```text
Web App
 │
 ▼
Managed Identity
 │
 ▼
Azure AD Token
 │
 ▼
Key Vault
```

アプリは Azure AD からトークンを取得し、そのトークンを使用して Key Vault にアクセスする。

---

### メリット

|メリット|説明|
|---|---|
|資格情報不要|コードに秘密情報不要|
|自動ローテーション|証明書管理不要|
|セキュア|Azure AD認証|

---

# 6 QAチームの一時的権限

QA チームはテスト時に **一時的に高権限**が必要になる。

この場合も **PIM** を使用する。

### QAアクセス例

```text
Role: Key Vault Contributor
Access duration: 2 hours
Approval required
```

これにより

- テスト時のみアクセス
    
- 権限自動失効
    
- セキュリティ維持
    

が実現できる。

---

# 7 アーキテクチャ

全体構成

```text
Developers
     │
     ▼
Web Application
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

管理アクセス

```text
Security / QA
      │
      ▼
Azure AD PIM
      │
      ▼
Temporary Role Activation
      │
      ▼
Azure Key Vault Administration
```

---

# 8 他の選択肢が不適切な理由

### Azure AD Identity Protection

Identity Protection は

- リスク検知
    
- 異常ログイン検出
    

を行うサービスであり、JITアクセス管理は提供しない。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure AD Connect]]
### Azure AD Connect

Azure AD Connect は

- オンプレミス AD
    
- Azure AD
    

の **ID同期ツール**であり、アクセス制御とは無関係。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Conditional Access]]
### Conditional Access

条件付きアクセスは

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Multi-Factor Authentication (MFA)]]
- MFA
    
- デバイス制御
    
- ロケーション制御
    

を行うが、JITアクセスや一時的権限管理は提供しない。

---

# 9 セキュリティメリット

この構成には次のメリットがある。

### 最小権限アクセス

ユーザーは必要なときだけ特権を取得する。

---

### 認証情報排除

アプリコードにパスワードや秘密を保存しない。

---

### 監査性

すべてのアクセスが Azure AD ログに記録される。

---

### 自動権限失効

テスト終了後に権限が自動で削除される。

---

# 10 まとめ

今回の要件

- Key Vault 管理者の JIT アクセス
    
- Web アプリの安全なシークレット取得
    
- QA の一時的高権限
    

これを実現する Azure AD 機能は次の通り。

|チーム|Azure機能|
|---|---|
|セキュリティ|Azure AD Privileged Identity Management|
|開発|Managed Identity|
|QA|Azure AD Privileged Identity Management|

この構成により、**Key Vault のセキュアなアクセス管理と最小権限の原則**を実現できる。