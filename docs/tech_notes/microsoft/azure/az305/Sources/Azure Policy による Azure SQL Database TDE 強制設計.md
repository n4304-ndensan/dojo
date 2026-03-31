[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Policy]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Transparent Data Encryption (TDE)]]
# Azure Policy による Azure SQL Database TDE 強制設計

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Resource Manager (ARM)]]
（DeployIfNotExists + ARMテンプレート修復）

---

# 1 背景

クラウド環境では、セキュリティとコンプライアンスを維持するために、組織全体で統一されたセキュリティポリシーを適用する必要がある。特にデータベースを扱うシステムでは、保存データの暗号化が重要なセキュリティ要件となる。

Azure SQL Database では、保存データを暗号化する機能として **Transparent Data Encryption（TDE）** が提供されている。TDE を有効化することで、データベースファイルやバックアップが自動的に暗号化され、データ漏洩リスクを低減できる。

組織のセキュリティポリシーとして、以下の要件が定義されている。

- サブスクリプション内の **すべての Azure SQL Database で TDE を有効化**
    
- **既存のデータベースにも適用**
    
- **将来作成されるデータベースにも適用**
    
- 非準拠リソースを **自動修復**
    
- 修復処理は **ARMテンプレートを使用**
    

この要件を実現するためには、Azure Policy の **DeployIfNotExists エフェクト**を利用する必要がある。

---

# 2 Azure Policy の概要

Azure Policy は、Azure 環境のリソース構成を評価し、組織のルールやセキュリティ基準を強制するサービスである。

Azure Policy を使用することで次のことが可能になる。

- セキュリティ設定の強制
    
- コンフィグレーション標準化
    
- コンプライアンス監査
    
- 自動修復
    

Azure Policy は **Policy Definition** と **Policy Assignment** によって構成される。

```text
Policy Definition
      │
      ▼
Policy Assignment
      │
      ▼
Azure Resources Evaluation
```

リソースが作成または変更された際に、ポリシーが自動的に評価される。

---

# 3 Azure Policy のエフェクト

Azure Policy には複数のエフェクト（効果）があり、それぞれ異なる動作を行う。

|Effect|説明|
|---|---|
|Audit|違反を検出するのみ|
|Deny|違反リソースの作成を拒否|
|Append|リソースに設定を追加|
|Modify|リソース設定を変更|
|DeployIfNotExists|ARMテンプレートをデプロイして修復|

今回の要件では

- 非準拠リソースの検出
    
- ARMテンプレートによる修復
    

が必要であるため

**DeployIfNotExists**

を使用する。

---

# 4 DeployIfNotExists の仕組み

DeployIfNotExists は、リソースが特定の条件を満たしていない場合に **ARMテンプレートを自動的にデプロイして修復する**ポリシー効果である。

処理の流れは次のようになる。

```text
Azure Policy Evaluation
        │
        ▼
Resource configuration check
        │
        ▼
TDE Enabled ?
   │            │
  Yes          No
   │            │
   ▼            ▼
Compliant      ARM Template Deployment
                    │
                    ▼
                Enable TDE
```

この仕組みにより、TDE が無効な Azure SQL Database を検出すると、自動的に ARM テンプレートが実行され、TDE が有効化される。

---

# 5 ARMテンプレートによる修復

DeployIfNotExists ポリシーには、修復処理を行う **ARMテンプレート**が含まれる。

TDE を有効化するテンプレートの例は次のようになる。

```json
{
 "type": "Microsoft.Sql/servers/databases/transparentDataEncryption",
 "apiVersion": "2021-11-01",
 "name": "current",
 "properties": {
   "state": "Enabled"
 }
}
```

このテンプレートは、対象の Azure SQL Database に対して TDE を有効化する。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Role-Based Access Control (RBAC)]]
# 6 RBAC と修復アイデンティティ

DeployIfNotExists が ARM テンプレートを実行するためには、リソースを変更する権限が必要になる。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Managed Identity]]
そのため Azure Policy は **Managed Identity** を使用し、このアイデンティティに RBAC ロールを割り当てる必要がある。

構成は次のようになる。

```text
Azure Policy
      │
      ▼
Managed Identity
      │
      ▼
RBAC Role Assignment
      │
      ▼
ARM Template Deployment
```

通常は次のようなロールが使用される。

|Role|用途|
|---|---|
|SQL DB Contributor|SQL DB設定変更|
|Contributor|リソース変更|

この RBAC 設定により、ポリシーは自動的に修復処理を実行できる。

---

# 7 アーキテクチャ

Azure Policy による TDE 強制構成は次のようになる。

```text
Azure Subscription
        │
        ▼
Azure Policy Assignment
        │
        ▼
Policy Evaluation
        │
        ▼
Azure SQL Database
        │
        ├─ TDE Enabled → Compliant
        │
        └─ TDE Disabled
                │
                ▼
         Deploy ARM Template
                │
                ▼
           Enable TDE
```

この構成により、サブスクリプション内のすべての Azure SQL Database が自動的に暗号化される。

---

# 8 他のエフェクトが適さない理由

AuditIfNotExists はリソースのコンプライアンス違反を検出するだけであり、修復処理を実行することはできない。

Modify はリソースのプロパティを書き換える機能であり、ARM テンプレートを使用した修復処理には適していない。

Deny はポリシー違反のリソース作成を拒否するだけであり、既存リソースを修復することはできない。

---

# 9 メリット

DeployIfNotExists を使用することで、Azure SQL Database のセキュリティ設定を自動的に維持できる。

まず、既存のデータベースに対しても自動的に修復処理が実行されるため、手動設定の漏れを防ぐことができる。また、新規に作成されるデータベースにもポリシーが適用されるため、組織全体で暗号化が標準化される。

さらに ARM テンプレートによる修復処理は Azure Policy によって自動化されるため、運用管理の負担を大幅に削減できる。

---

# 10 まとめ

今回の要件

- Azure SQL Database の TDE 強制
    
- 既存および将来のリソース対象
    
- ARMテンプレートによる自動修復
    

これを実現する Azure Policy の設定は次の通りである。

**Policy Effect**

DeployIfNotExists

**追加設定**

RBAC ロールを割り当てた修復アイデンティティ

この構成により、サブスクリプション内の Azure SQL Database 全体で透過データ暗号化（TDE）を自動的に適用し、クラウド環境のセキュリティコンプライアンスを維持することができる。