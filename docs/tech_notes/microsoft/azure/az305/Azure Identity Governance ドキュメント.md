# Azure Identity Governance ドキュメント

（PIM / Access Reviews / Least Privilege / JIT Access）

## 1. 概要

企業のセキュリティガイドラインでは、管理者権限や特権アクセスを適切に管理することが重要です。特にクラウド環境では、過剰な権限が重大なセキュリティリスクになるため、次の原則が広く採用されています。

重要なセキュリティ原則

- **最小権限の原則（Least Privilege）**
    
- **適時アクセス（Just-In-Time Access / JIT）**
    
- **定期的なアクセスレビュー**
    
- **特権アカウント管理**
    

Azureではこれらを実現するために **Microsoft Entra ID（旧 Azure AD）Identity Governance** の機能を使用します。

主なサービス

- Privileged Identity Management (PIM)
    
- Access Reviews
    
- Entitlement Management
    
- Identity Protection
    

---

# 2. 最小権限の原則（Least Privilege）

最小権限の原則とは

> ユーザーには業務に必要な最小限の権限のみ付与する

というセキュリティ原則です。

例

|ユーザー|必要権限|
|---|---|
|開発者|Web App 管理|
|DB管理者|SQL 管理|
|一般ユーザー|読み取りのみ|

問題点

長期間の特権アクセスは以下のリスクがあります。

- 権限の過剰付与
    
- 内部不正
    
- アカウント乗っ取り
    

そのため **JITアクセス** が重要になります。

---

# 3. Just-In-Time Access（JIT）

JITとは

> 必要なときだけ一時的に権限を付与する仕組み

です。

通常状態

```id="nh4t1n"
User
 │
 ▼
No admin privilege
```

必要時

```id="b4l8g9"
User
 │ Request
 ▼
Privilege elevation
 │
 ▼
Temporary Admin Access
```

時間が経過すると

```id="3lfls2"
Privilege removed
```

この仕組みをAzureで提供するのが **PIM** です。

---

# 4. Privileged Identity Management (PIM)

PIMは **特権アクセス管理サービス**です。

目的

- 最小権限の強制
    
- JITアクセス
    
- 特権アカウント監視
    

主な機能

- JITロール割り当て
    
- 承認ワークフロー
    
- アクセス期限設定
    
- アラート
    
- アクセス履歴
    

構造

```id="sor3nb"
User
 │
 ▼
Request privilege
 │
 ▼
Approval
 │
 ▼
Temporary role assignment
```

---

## PIMの主な特徴

### 1. Just-In-Timeロール

ユーザーは **必要なときだけ管理者権限を取得**します。

例

```id="srfzvr"
Global Administrator
 │
 Eligible
 │
 Activate
 │
 Temporary access
```

---

### 2. 承認ワークフロー

権限昇格には承認を要求できます。

```id="nv1yij"
User request
   │
   ▼
Manager approval
   │
   ▼
Role activation
```

---

### 3. アクセス期限

例

```id="by8i9v"
Admin role
Duration = 2 hours
```

期限後は自動的に削除されます。

---

### 4. アクセス監査

PIMは以下を記録します。

- 誰が
    
- いつ
    
- どの権限を
    
- どれくらい使用したか
    

---

# 5. Access Reviews

Access Reviewsは **定期的なアクセス確認機能**です。

目的

- 不要なアクセス権削除
    
- 特権アクセス監査
    
- コンプライアンス
    

例

```id="ps4o1x"
Admin Role
 │
 ▼
Quarterly Review
 │
 ▼
Approve / Remove
```

---

## Access Reviewsの機能

主な機能

- 定期レビュー
    
- 自動レビュー
    
- マネージャーレビュー
    
- グループレビュー
    

例

```id="0bkg6y"
Admin Group
 │
 ▼
Review every 90 days
 │
 ▼
Remove inactive users
```

---

# 6. PIM + Access Reviews

この2つは **組み合わせて使用されます**。

構成

```id="hck3ue"
User
 │
 ▼
PIM (JIT Access)
 │
 ▼
Temporary Privilege
 │
 ▼
Access Review
 │
 ▼
Privilege validation
```

役割

|機能|サービス|
|---|---|
|JITアクセス|PIM|
|特権レビュー|Access Reviews|

---

# 7. Identity Protection

Identity Protectionは **リスクベースの認証保護**です。

機能

- 不審ログイン検出
    
- リスクユーザー検出
    
- 自動MFA
    

例

```id="49oz3u"
User login
 │
 ▼
Risk detection
 │
 ▼
Force MFA
```

しかし

- 最小権限
    
- JITアクセス
    

の管理とは直接関係ありません。

---

# 8. MFA

MFAは **多要素認証**です。

例

```id="pbfve3"
Password
+
Authenticator App
```

目的

- アカウント保護
    
- 認証強化
    

しかし

- 権限管理
    
- JITアクセス
    

とは別の機能です。

---

# 9. Identity Governance アーキテクチャ

Azure Identity Governance構成

```id="pkf7ly"
Users
 │
 ▼
Microsoft Entra ID
 │
 ├ PIM
 │
 ├ Access Reviews
 │
 ├ Entitlement Management
 │
 └ Identity Protection
```

---

# 10. セキュリティガイドライン対応

問題の要件

|要件|サービス|
|---|---|
|最小権限|PIM|
|JITアクセス|PIM|
|定期アクセスレビュー|Access Reviews|

---

# 11. 典型アーキテクチャ

```id="kngk1j"
Admin User
   │
   ▼
PIM
   │
   ▼
Temporary Privilege
   │
   ▼
Access Review
   │
   ▼
Privilege validation
```

---

# 12. 試験対策ポイント

Azure試験では以下のキーワードが重要です。

|キーワード|サービス|
|---|---|
|Just-In-Time access|PIM|
|Least privilege|PIM|
|Privileged role management|PIM|
|Periodic permission review|Access Reviews|
|Identity risk detection|Identity Protection|
|Authentication security|MFA|

---

# 13. まとめ

Azureで特権アクセス管理を行う場合

```id="eky4qi"
PIM
 │
 ▼
Just-In-Time Privilege
 │
 ▼
Access Reviews
 │
 ▼
Periodic Access Validation
```

この構成により

- 最小権限
    
- JITアクセス
    
- 定期レビュー
    

を実現できます。

そのためこの問題の正解は

**PIM + Access Reviews**

です。