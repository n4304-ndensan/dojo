[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Resource Manager (ARM)]]
# Azure ARM テンプレートデプロイ制御設計

（特定リソースグループへの限定デプロイ）

---

# 1 背景

ある組織では、Azure 環境におけるリソースのデプロイを開発者チームに許可する必要がある。しかし、クラウド環境では無制限にリソース作成を許可すると、以下のような問題が発生する可能性がある。

まず、開発者が誤って異なるリソースグループやサブスクリプションにリソースを作成してしまう可能性がある。これにより、環境構成が統一されず、運用管理が難しくなる。また、セキュリティやコスト管理の観点からも、リソース作成範囲を制限することは重要である。さらに、組織ではインフラの標準化を進めており、すべてのリソースは **事前定義された Azure Resource Manager (ARM) テンプレート** を利用してデプロイする方針となっている。

ARM テンプレートを利用することで、インフラ構成をコードとして管理できるようになり、同一構成の環境を再現可能にする **Infrastructure as Code (IaC)** の実現が可能になる。これは DevOps や CI/CD パイプラインの実装においても重要な要素である。

今回の要件は次の通りである。

- 開発者チームは Azure にリソースをデプロイできる
    
- ただし **特定のリソースグループのみ** に限定する
    
- デプロイは **事前定義された ARM テンプレートを利用する**
    

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Role-Based Access Control (RBAC)]]
この要件を満たす最も適切な Azure 機能は **Custom RBAC Role（カスタム RBAC ロール）** である。

---

# 2 Azure RBAC（Role-Based Access Control）

Azure RBAC は、Azure リソースに対するアクセス制御を管理する仕組みである。ユーザー、グループ、またはサービスプリンシパルに対して、どのリソースに対してどの操作を許可するかを定義できる。

RBAC は **最小権限の原則（Least Privilege Principle）** に基づいて設計されており、ユーザーが必要最低限の操作のみを実行できるように制御する。これにより、誤操作や不正操作のリスクを低減することができる。

Azure RBAC では次の3つの要素が重要である。

|要素|説明|
|---|---|
|セキュリティ主体|ユーザー、グループ、サービスプリンシパル|
|ロール定義|許可される操作|
|スコープ|権限が適用される範囲|

スコープは次の階層構造を持つ。

```text
Management Group
     │
Subscription
     │
Resource Group
     │
Resource
```

今回の要件では **Resource Group レベルで権限を制限する**必要がある。

---

# 3 カスタムRBACロール

Azure には組み込みロール（Owner、Contributor、Reader など）が存在するが、これらのロールでは権限が広すぎる場合がある。そのような場合には **カスタム RBAC ロール** を作成することで、必要な操作のみを許可することができる。

今回のケースでは、開発者チームは ARM テンプレートを使用してリソースをデプロイする必要があるため、次のような権限が必要になる。

- ARM テンプレートのデプロイ実行
    
- リソースグループ内でのリソース作成
    
- デプロイメントの読み取り
    

これらの操作に対応する Azure 権限は主に次の通りである。

|権限|用途|
|---|---|
|Microsoft.Resources/deployments/*|ARMテンプレートのデプロイ|
|Microsoft.Resources/subscriptions/resourceGroups/read|リソースグループ参照|
|Microsoft.Resources/subscriptions/resourceGroups/deployments/*|デプロイ操作|

これらを含む **カスタムロールを作成し、特定のリソースグループに割り当てる**ことで、開発者はそのリソースグループにのみリソースをデプロイできるようになる。

---

# 4 アーキテクチャ

この構成の概念図は次のようになる。

```text
Developer Team
      │
      ▼
Custom RBAC Role
      │
      ▼
Resource Group
      │
      ▼
ARM Template Deployment
```

この仕組みにより、開発者は指定されたリソースグループに対してのみ ARM テンプレートを実行できる。

---

# 5 デプロイフロー

実際のデプロイの流れは次の通りである。

```text
Developer
   │
   ▼
ARM Template
   │
   ▼
Azure Resource Manager
   │
   ▼
Resource Group
   │
   ▼
Azure Resources
```

ARM テンプレートは Azure Resource Manager を通じて実行され、RBAC によって許可されたスコープ内でのみリソースが作成される。

---

# 6 他の選択肢が適切でない理由

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Blueprints]]
## Azure Blueprint

Azure Blueprint は、ポリシー、ロール、ARM テンプレートなどをまとめて定義し、環境を標準化するためのサービスである。主に **大規模なガバナンス管理** や **複数サブスクリプションの環境標準化** に利用される。しかし今回の要件は単一のリソースグループへのデプロイ制限であるため、Blueprint を利用するのは過剰な構成となる。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Policy]]
## Azure Policy

Azure Policy はリソースの構成や設定が組織のルールに従っているかを評価するガバナンス機能である。例えば、特定のリージョンのみ許可する、特定の SKU のみ許可する、といった制御が可能である。しかし Azure Policy は **誰がデプロイできるか** を制御する仕組みではないため、アクセス制御の目的には適さない。

---

## Resource Lock

Resource Lock はリソースの削除や変更を防ぐための機能である。これは主に誤操作による削除防止を目的としているため、デプロイ範囲の制御には利用できない。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Managed Identity]]
## Managed Identity

Managed Identity は Azure サービスが他のサービスに認証するための ID 管理機能であり、アクセス制御の範囲を定義するものではない。そのため、ユーザーのデプロイ権限を制御する用途には適さない。

---

# 7 推奨アーキテクチャ

最もシンプルで効果的な構成は次の通りである。

```text
Developer Team
       │
       ▼
Custom RBAC Role
       │
       ▼
Specific Resource Group
       │
       ▼
ARM Template Deployment
```

この構成では、開発者チームは指定されたリソースグループ内でのみ ARM テンプレートを実行できる。

---

# 8 メリット

この構成を採用することでいくつかの重要なメリットが得られる。

まず、開発者のデプロイ範囲を特定のリソースグループに限定できるため、誤った場所にリソースを作成するリスクを防ぐことができる。また、RBAC を利用することで **最小権限の原則** を適用でき、セキュリティを強化することができる。

さらに、ARM テンプレートを利用することでインフラ構成をコードとして管理できるため、環境の再現性が高まり、CI/CD パイプラインとの統合も容易になる。

---

# 9 まとめ

今回の要件は次の通りである。

- 開発者が Azure にリソースをデプロイできる
    
- ただし特定のリソースグループのみに限定する
    
- デプロイは ARM テンプレートを利用する
    

これらの要件を満たす最適な Azure 機能は **Custom RBAC Role（カスタム RBAC ロール）** である。

RBAC を利用することで、特定のリソースグループに対して必要最低限のデプロイ権限を付与できるため、安全で管理しやすい Azure 環境を実現できる。