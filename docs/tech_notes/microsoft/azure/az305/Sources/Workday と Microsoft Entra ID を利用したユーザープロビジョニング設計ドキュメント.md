[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Microsoft Entra ID]]
## Workday と Microsoft Entra ID を利用したユーザープロビジョニング設計ドキュメント

### ― オンプレミス AD DS と Microsoft Entra ID の統合アーキテクチャ ―

---

# 1 概要

本シナリオでは、組織のユーザー管理を **クラウドベースのHRシステム（Workday）** を起点として自動化することが目的である。  
現在は従業員（employees）および請負業者（contractors）のオンボーディングが手動で行われているが、これを自動プロビジョニングへ移行する。

対象環境は以下の構成を持つ。

- オンプレミス Active Directory Domain Services (AD DS) フォレスト
    
- Microsoft Entra ID テナント
    
- Workday HR システム
    
- Microsoft Entra Connect Sync
    

この構成では **Workdayをユーザー情報の「信頼できるソース（Source of Truth）」** とし、ユーザーが登録された際に自動的に以下へアカウントを作成する。

- オンプレミス AD の子ドメイン
    
- Microsoft Entra ID テナント
    

さらに、従業員と請負業者ではプロビジョニングルールを変える必要がある。

---

# 2 現在の環境構成

オンプレミスには **AD DS フォレスト** が存在する。

構造は次の通り。

```text
AD Forest
│
├ Root Domain
│
├ Child Domain 1
├ Child Domain 2
└ Child Domain 3
```

このフォレスト内のユーザーは **Server1 上の Microsoft Entra Connect Sync** により Microsoft Entra ID へ同期されている。

```text
On-prem AD DS
      │
      │ Azure AD Connect Sync
      ▼
Microsoft Entra ID
```

現在のユーザー作成フローは次の通り。

```text
Workday
   │
   ▼
Manual user creation
   │
   ├ AD Domain
   └ Entra ID
```

これは手動作業が必要であり、管理負荷が高い。

---

# 3 目標アーキテクチャ

目標は **Workday → Entra → AD** の自動プロビジョニングである。

```text
Workday (HR system)
        │
        ▼
Microsoft Entra ID
        │
        ▼
On-prem Active Directory
```

ユーザー作成フロー

```text
HR registers employee in Workday
        │
        ▼
Entra ID provisioning
        │
        ├ Create Entra user
        └ Create AD user
```

この方式により

- 人事登録 = アカウント作成
    
- 手動作業不要
    

となる。

---

# 4 Microsoft Entra Provisioning の仕組み

Microsoft Entra ID には **SCIMベースの自動プロビジョニング機能**がある。

この機能は

- SaaS
    
- HRシステム
    
- オンプレミスAD
    

などへユーザーを自動作成できる。

基本構造

```text
Source System
    │
    ▼
Microsoft Entra ID
    │
    ▼
Target System
```

今回の構成では

```text
Workday
   │
   ▼
Microsoft Entra ID
   │
   ├ Entra user
   └ AD DS user
```

となる。

---

# 5 Application Registration 数

問題の1つ目の質問

**必要なアプリケーション登録数**

答え

**1**

理由は次の通り。

Microsoft Entra ID のプロビジョニングでは

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#App Registration]]
- 1つのアプリ登録
    
- 複数のプロビジョニングルール
    

を設定できる。

つまり

```text
Workday App Registration
        │
        ├ Employee rules
        └ Contractor rules
```

のように

**同一アプリ内で異なるプロビジョニングロジックを定義可能**。

例

|ユーザータイプ|プロビジョニング|
|---|---|
|Employee|AD + Entra|
|Contractor|Entraのみ|

このような制御は **Scoping Filter** を使って実装できる。

---

# 6 Scoping Filter の役割

Scoping Filter は

**どのユーザーをプロビジョニング対象にするか**

を決定する条件である。

例えば Workday の属性を利用できる。

```text
WorkerType = Employee
```

または

```text
WorkerType = Contractor
```

例

```text
IF WorkerType = Employee
    → AD + Entra
```

```text
IF WorkerType = Contractor
    → Entra only
```

これにより **アプリ登録を増やす必要がない**。

---

# 7 Provisioning Agent の役割

次の質問は

**Entra Connect Provisioning Agent の最小数**

である。

答え

**1**

理由

Provisioning Agent は

**Microsoft Entra → On-prem AD**

の通信を実現する。

構造

```text
Microsoft Entra ID
        │
        ▼
Provisioning Agent
        │
        ▼
Active Directory
```

1つのフォレストであれば

**単一エージェントで複数ドメインを管理可能**。

今回の構成

```text
AD Forest
│
├ Child Domain 1
├ Child Domain 2
└ Child Domain 3
```

同一フォレストなので

**Agent 1台で対応できる**。

---

# 8 Provisioning Agent の配置

Agentは通常

- Domain joined server
    
- ADアクセス可能
    

なサーバーに配置する。

例

```text
Server1
   │
   ├ Entra Connect Sync
   └ Provisioning Agent
```

---

# 9 完成アーキテクチャ

最終構成

```text
              Workday (HR System)
                      │
                      ▼
           Microsoft Entra ID
            (Provisioning Service)
                      │
        ┌─────────────┴─────────────┐
        ▼                           ▼
 Entra User                    Provisioning Agent
                                     │
                                     ▼
                              Active Directory
                              (Child Domain)
```

---

# 10 プロビジョニングフロー

ユーザー作成プロセス

```text
1 HR creates employee in Workday
2 Workday sends data to Entra
3 Entra provisioning rules evaluate
4 Entra creates user
5 Provisioning agent creates AD account
```

---

# 11 管理のメリット

この設計の利点

### シンプル

- App registration: 1
    
- Agent: 1
    

---

### HR中心管理

```text
HR system = identity source
```

---

### 自動化

- 手動アカウント作成不要
    
- 入社/退職自動処理
    

---

### セキュリティ

- HR属性ベース制御
    
- Role assignment可能
    

---

# 12 まとめ

今回の設計の重要ポイント

|項目|答え|
|---|---|
|App registration|1|
|Provisioning Agent|1|

理由

- Entra ID は **単一アプリで複数プロビジョニングルールを定義可能**
    
- 同一フォレストは **単一エージェントで管理可能**
    

最終アーキテクチャ

```text
Workday
   │
   ▼
Microsoft Entra ID
   │
   ▼
Provisioning Agent
   │
   ▼
Active Directory
```

この構成により

- 自動ユーザープロビジョニング
    
- 管理負荷最小
    
- HR主導のIdentity管理
    

を実現できる。