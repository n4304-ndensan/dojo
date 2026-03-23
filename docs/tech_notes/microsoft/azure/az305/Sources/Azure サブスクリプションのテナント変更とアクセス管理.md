# Azure サブスクリプションのテナント変更とアクセス管理

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Microsoft Entra ID]]
（Azure AD テナントアソシエーション変更時のロール設計）

---

# 1 背景（シナリオ）

企業が Azure を大規模に利用している場合、複数の **Azure Active Directory（Azure AD / Microsoft Entra ID）テナント**と **多数のサブスクリプション**を管理する必要がある。

今回のシナリオでは次の構成が存在する。

|項目|内容|
|---|---|
|Azure AD テナント|2つ（contoso.com / fabrikam.com）|
|Azure サブスクリプション|100個|

この環境で **サブスクリプションの Azure AD テナント関連付け（tenant association）を変更する必要**がある。

つまり

```text
サブスクリプション
↓
別の Azure AD テナントへ移動
```

する操作である。

---

# 2 要件

今回の問題では次の 2 つの要件がある。

### 要件1

User1 は

```text
サブスクリプションの Azure AD テナントアソシエーション変更
```

を実行できる必要がある。

つまり

```text
サブスクリプションのテナント変更
```

を行える権限が必要である。

---

### 要件2

テナント変更後

```text
管理者アクセスが失われる
```

可能性がある。

そのため

```text
User2 に管理アクセスを付与
```

する必要がある。

---

# 3 Azure サブスクリプションと Azure AD テナント

Azure では

```text
Azure AD テナント
↓
サブスクリプション
```

という関係で管理される。

構造

```text
Azure AD Tenant
      │
      ▼
Subscriptions
      │
      ▼
Resources
```

サブスクリプションは **必ず 1 つの Azure AD テナントに関連付けられる**。

---

# 4 テナント変更とは

サブスクリプションのテナント変更とは

```text
contoso tenant
↓
fabrikam tenant
```

のように

```text
サブスクリプションのディレクトリ変更
```

を行う操作である。

Azure Portal では

```text
Change directory
```

という操作になる。

---

# 5 テナント変更時の注意点

サブスクリプションのテナント変更を行うと

```text
RBAC ロールが削除される
```

場合がある。

つまり

```text
管理者アクセス消失
```

が発生する可能性がある。

そのため

```text
新しいテナント側で管理者を設定
```

する必要がある。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Role-Based Access Control (RBAC)]]
# 6 Azure RBAC の代表的ロール

Azure RBAC には複数のロールがある。

|ロール|説明|
|---|---|
|Owner|すべての権限|
|Contributor|リソース管理|
|User Access Administrator|アクセス管理のみ|
|Reader|閲覧のみ|

---

# 7 Owner ロール

Owner は Azure RBAC の **最上位ロール**である。

Owner は次の操作が可能。

|機能|可否|
|---|---|
|リソース作成|可能|
|リソース削除|可能|
|アクセス管理|可能|
|RBAC変更|可能|

つまり

```text
フル管理者
```

である。

---

# 8 テナント変更に必要な権限

Azure の仕様では

```text
サブスクリプションのテナント変更
```

を行うには

```text
Owner ロール
```

が必要である。

理由

- サブスクリプションの所有権変更
    
- RBAC 再構成
    
- ディレクトリ関連付け変更
    

などを行うためである。

---

# 9 User1 に必要な権限

User1 は

```text
テナントアソシエーション変更
```

を行う。

そのため必要なのは

```text
Owner
```

である。

---

# 10 User2 の役割

問題では

```text
テナント変更後
管理アクセスを失う可能性
```

がある。

そのため

```text
User2 に管理アクセス付与
```

する必要がある。

この場合

```text
Owner
```

を割り当てるのが最も確実である。

---

# 11 他のロールでは不十分な理由

### User Access Administrator

このロールは

```text
RBAC変更のみ
```

可能である。

しかし

```text
サブスクリプション管理
```

はできない。

---

### Service Administrator

これは **旧クラシックロール**である。

現在の Azure RBAC 管理では

```text
推奨されない
```

場合が多い。

---

### Co-administrator

これも

```text
クラシック管理モデル
```

のロールであり

RBAC の完全代替ではない。

---

# 12 最適構成

今回の要件を整理すると

|ユーザー|必要機能|
|---|---|
|User1|テナント変更|
|User2|管理アクセス維持|

この条件を満たすロールは

```text
Owner
```

である。

---

# 13 最終回答

正解

```text
C

User1: Owner  
User2: Owner
```

---

# 14 アーキテクチャイメージ

```text
Before

Tenant: contoso.com
     │
     └ Subscription
           │
           ├ Owner : User1
           └ Owner : User2

↓

Change Directory

↓

After

Tenant: fabrikam.com
     │
     └ Subscription
           │
           ├ Owner : User1
           └ Owner : User2
```

---

# 15 まとめ

この問題のポイントは

```text
サブスクリプションのテナント変更
```

に必要な **RBAC 権限**である。

重要ポイント

|項目|内容|
|---|---|
|テナント変更|Owner 必須|
|管理アクセス維持|Owner 付与|

そのため最適な回答は

```text
User1: Owner  
User2: Owner
```

となる。