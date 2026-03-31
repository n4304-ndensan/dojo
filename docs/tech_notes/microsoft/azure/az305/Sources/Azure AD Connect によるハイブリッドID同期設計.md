[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Microsoft Entra ID]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure AD Connect]]
# Azure AD Connect によるハイブリッドID同期設計

（グループベースのフィルタリング）

---

# 1 背景

多くの企業では、オンプレミスの **Active Directory（AD）** とクラウドの **Azure Active Directory（Azure AD / Microsoft Entra ID）** を併用する **ハイブリッドID環境**を採用している。

この構成では、オンプレミスのユーザーやグループをクラウドに同期することで、次のようなメリットを得られる。

- クラウドサービス（Microsoft 365、Azure、SaaS）への統一認証
    
- シングルサインオン
    
- ID 管理の一元化
    
- セキュリティポリシーの統合
    

しかし、大規模な Active Directory 環境では、すべてのユーザーやグループを同期する必要はない。特定の組織単位（OU）やグループのみを同期対象とし、それ以外のオブジェクトは除外する必要がある。

今回のシナリオでは、以下の要件を満たすハイブリッドIDソリューションを設計する必要がある。

- 特定のオンプレミス AD グループのみ Azure AD に同期する
    
- 一部の OU を除外する
    
- パスワードハッシュ同期をサポートする
    
- カスタム PowerShell スクリプトを使用しない
    
- ネイティブのフィルタリング機能を利用する
    

これらの要件を満たす最適な構成は

**Azure AD Connect + グループベースフィルタリング**

である。

---

# 2 Azure AD Connect

Azure AD Connect は、オンプレミス Active Directory と Azure AD の間でディレクトリ情報を同期するツールである。

主な機能は次の通りである。

- ユーザー同期
    
- グループ同期
    
- パスワードハッシュ同期
    
- フェデレーション統合
    
- フィルタリング
    

基本アーキテクチャは次のようになる。

```text
On-Premises Active Directory
            │
            ▼
      Azure AD Connect
            │
            ▼
        Azure AD
```

Azure AD Connect は定期的にオンプレミスのディレクトリをスキャンし、変更を Azure AD に同期する。

---

# 3 パスワードハッシュ同期

パスワードハッシュ同期（Password Hash Synchronization）は、オンプレミス AD のパスワードハッシュを Azure AD にコピーする仕組みである。

仕組み

```text
User Password
      │
      ▼
Active Directory
      │
      ▼
Password Hash
      │
      ▼
Azure AD
```

ユーザーは同じパスワードで次のサービスにログインできる。

- Microsoft 365
    
- Azure
    
- SaaS アプリケーション
    

メリット

- フェデレーション不要
    
- 高可用性
    
- シンプルな構成
    

---

# 4 フィルタリングの必要性

企業の Active Directory には多くのオブジェクトが存在する。

例

- 社員ユーザー
    
- サービスアカウント
    
- テストアカウント
    
- 管理者グループ
    

すべてをクラウドに同期すると、次の問題が発生する。

- セキュリティリスク
    
- 不要なライセンス消費
    
- 管理コスト増加
    

そのため、Azure AD Connect では **同期対象を制御するフィルタリング機能**が提供されている。

---

# 5 Azure AD Connect のフィルタリング方式

Azure AD Connect では主に次の3種類のフィルタリングが利用できる。

### 1 Domain Filtering

同期対象ドメインを制御する。

```text
corp.local
branch.local
```

一部のドメインのみ同期する場合に使用する。

---

### 2 OU Filtering

特定の **OU（組織単位）** のみ同期する。

例

```text
OU=Employees
OU=IT
OU=Finance
```

この方法はシンプルだが、OU構造に依存するため柔軟性が低い。

---

### 3 Group-Based Filtering

特定の **Active Directory グループ**に属するオブジェクトのみ同期する方法である。

今回のシナリオでは、この方法が最も適している。

---

# 6 グループベースフィルタリング

グループベースフィルタリングでは、特定のグループに属するユーザーのみ Azure AD に同期する。

例

```text
AzureSyncGroup
   ├ UserA
   ├ UserB
   └ UserC
```

同期対象

```text
AzureSyncGroup members
```

同期対象外

```text
Other AD users
```

アーキテクチャ

```text
On-Prem AD
   │
   ├ AzureSyncGroup
   │      ├ User1
   │      ├ User2
   │      └ User3
   │
   └ OtherUsers

           │
           ▼
     Azure AD Connect
           │
           ▼
        Azure AD
```

この方法のメリット

- OU構造に依存しない
    
- 柔軟な同期制御
    
- 運用が簡単
    

---

# 7 OU除外の実現

グループベースフィルタリングを利用すると、OU構造を変更する必要がない。

例えば次の OU が存在するとする。

```text
OU=Employees
OU=ServiceAccounts
OU=TestUsers
```

AzureSyncGroup に含まれるユーザーのみ同期すれば

- ServiceAccounts OU
    
- TestUsers OU
    

は自動的に除外される。

---

# 8 他の選択肢との比較

## Domain / OU Filtering

メリット

- シンプル
    
- 設定が簡単
    

デメリット

- OU構造に依存
    
- 柔軟性が低い
    

---

## Azure AD Connect Cloud Sync

Cloud Sync は軽量な同期エージェントを使用する新しい同期方式である。

しかし

- 高度なグループフィルタリングが限定的
    
- 一部機能が従来の AD Connect より少ない
    

---

## AD FS

AD FS は認証フェデレーションサービスであり、同期ツールではない。

役割

- シングルサインオン
    
- フェデレーション認証
    

ユーザーやグループの同期は行わない。

---

# 9 推奨アーキテクチャ

最適なハイブリッドID構成は次の通りである。

```text
On-Prem Active Directory
        │
        │  (Group Filter)
        ▼
Azure AD Connect
        │
        │  Password Hash Sync
        ▼
Azure AD
        │
        ▼
Cloud Applications
```

この構成により

- 必要なグループのみ同期
    
- OU除外
    
- シンプルな管理
    
- 高いセキュリティ
    

を実現できる。

---

# 10 まとめ

今回の要件は

- 特定の AD グループのみ同期
    
- OU除外
    
- パスワードハッシュ同期
    
- カスタムスクリプトなし
    

これらを満たす最適な Azure AD Connect 構成は

**グループベースのフィルタリング**

である。

この方法は

- OU構造に依存しない
    
- 柔軟な同期管理
    
- ネイティブ機能のみで実装可能
    

という理由から、ハイブリッドID環境における最も効率的な同期方法の一つである。