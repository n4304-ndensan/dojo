# Topic-02-Governance

## 学習ゴール

[[Azure用語集.md#Management Group]] から [[Azure用語集.md#Azure Policy]] までを一続きで理解し、Azure 全体に同じルールをどう配布するかを説明できるようにする。

## このTopicの全体像

この Topic では、管理階層、RBAC、Policy、標準化テンプレート、タグとコスト配賦をまとめる。  
判断軸は「どのスコープに適用するか」「拒否するか、監査するか、自動修復するか」「誰にどこまで委任するか」。

---

# 第1章 学習マップ

## 1.1 学習順序

1. 管理グループ、サブスクリプション、リソースグループの階層を理解する。
2. RBAC で権限委任の粒度を決める。
3. Azure Policy で構成ガードレールを適用する。
4. ARM/Blueprints/タグで標準化と展開運用に落とす。

## 1.2 Azureリソース一覧

- Management Group
- Subscription / Resource Group
- Azure RBAC
- Azure Policy
- ARM Template / deployment control
- Cost Management / Tags

---

# 第2章 Azureリソース解説

## Resource: Management Group and Resource Hierarchy

### 概要

[[Azure用語集.md#Management Group]] は複数サブスクリプションに共通ルールを適用するための上位スコープ。

### できること

- 複数サブスクリプションの集約
- Policy / RBAC の継承
- 部門単位、環境単位の運用分割

### 技術仕様

- ルート管理グループから下位へ継承する。
- サブスクリプションは管理グループ配下で整理する。
- リソースグループはデプロイとライフサイクルの最小単位になる。

### SDK / API

- ARM / Azure Resource Manager API
- Azure CLI / PowerShell

### 他サービスとの比較

- Management Group vs Resource Group: 前者は組織階層、後者はデプロイ単位。
- Subscription vs Resource Group: 請求・分離境界か、運用単位かで役割が違う。

### どのようなときに使うか

- 部門やリージョン単位でルールを統一したいとき
- 複数サブスクリプションに同一 Policy を流したいとき

### 関連シナリオ

- [[Scenarios/Scenario-AzurePolicy.md#管理グループでリージョン制限を適用する]]
- [[Scenarios/Scenario-AzurePolicy.md#タグと標準構成を全サブスクリプションへ展開する]]

### 主な出典

- [[Sources/Topic-02.md]]
- [[Sources/Azure ガバナンス設計における Root Management Group の役割.md]]
- [[Sources/Azure リソースグループの役割と利点.md]]
- [[Sources/Azureで複数サブスクリプションを管理するためのガバナンス設計（Tags と Management Groups）.md]]

## Resource: Azure RBAC

### 概要

[[Azure用語集.md#Azure RBAC]] は Azure リソースに対する操作権限をスコープ付きで割り当てる仕組み。

### できること

- 組み込みロールの割り当て
- カスタムロール作成
- リソースグループ単位の権限委任
- Managed Identity への権限付与

### 技術仕様

- スコープは管理グループ、サブスクリプション、リソースグループ、個別リソース。
- 組み込みロールで広すぎる場合はカスタムロールを使う。
- 最小権限が基本で、Owner は例外的に扱う。

### SDK / API

- Azure CLI `az role assignment`
- ARM / Microsoft.Authorization API

### 他サービスとの比較

- RBAC vs Access Policy: Azure の多くの新しい設計では RBAC を優先する。
- RBAC vs Policy: RBAC は誰が操作できるか、Policy は何を許すか。

### どのようなときに使うか

- 開発チームに限定操作だけ許可したいとき
- ネットワーク管理だけを特定部門へ委任したいとき
- Managed Identity に対象リソースの権限を与えたいとき

### 関連シナリオ

- [[Scenarios/Scenario-AzurePolicy.md#特定リソースグループだけへデプロイを許可する]]

### 主な出典

- [[Sources/Topic-02.md]]
- [[Sources/Azure ARM テンプレートデプロイ制御設計.md]]
- [[Sources/Azure RBAC とスコープ階層によるリソース作成権限の判断.md]]
- [[Sources/管理グループ階層で仮想ネットワーク管理を委任する RBAC 設計.md]]

## Resource: Azure Policy

### 概要

[[Azure用語集.md#Azure Policy]] は構成ルールを継続的に評価し、監査・拒否・自動修復へつなげるガードレール。

### できること

- リージョン制限
- タグ強制
- 暗号化強制
- AKS ガバナンス
- `DeployIfNotExists` による修復

### 技術仕様

- 代表的な効果は `Audit`、`Deny`、`Append`、`DeployIfNotExists`。
- 管理グループへ適用するとサブスクリプション横断で効く。
- SQL TDE やタグ補完は自動修復パターンが使える。

### SDK / API

- Azure Policy definitions / assignments
- ARM / Bicep / CLI

### 他サービスとの比較

- Policy vs ARM Template: Policy は継続監視、ARM は宣言的デプロイ。
- Policy vs Blueprint: 標準構成パッケージという観点では似るが、現場では Policy と IaC の組み合わせで考えると整理しやすい。

### どのようなときに使うか

- 組織ルール違反の構成を防ぎたいとき
- 手で直さずに修復まで自動化したいとき
- AKS や SQL のセキュリティ基準を一括適用したいとき

### 関連シナリオ

- [[Scenarios/Scenario-AzurePolicy.md#管理グループでリージョン制限を適用する]]
- [[Scenarios/Scenario-AzurePolicy.md#sql-database-の-tde-を自動修復付きで強制する]]
- [[Scenarios/Scenario-AzurePolicy.md#aks-ワークロードへセキュリティ基準を強制する]]

### 主な出典

- [[Sources/Topic-02.md]]
- [[Sources/Azure Policy を管理グループで適用する理由（リージョン制限ポリシー）.md]]
- [[Sources/Azure Policy による Azure SQL Database TDE 強制設計.md]]
- [[Sources/AKS コンテナセキュリティガバナンス設計.md]]

## Resource: Standardization and Cost Control

### 概要

テンプレート化、タグ、コスト配賦を通じて、Azure 環境を「同じ形で増やせる」状態にする運用レイヤ。

### できること

- 標準構成テンプレートの再利用
- タグによる原価配賦
- 複数サブスクリプションへの共通展開

### 技術仕様

- ARM / IaC は構成の再現性を担保する。
- タグは Cost Management、運用フィルタ、所有者明確化に使う。
- 標準構成は管理グループ単位で配布すると運用がぶれにくい。

### SDK / API

- ARM Template / Bicep
- Azure DevOps / CI/CD
- Cost Management API

### 他サービスとの比較

- 手動デプロイ vs IaC: サブスクリプション数が増えるほど IaC の差が大きい。
- タグ手入力 vs Policy 補完: 組織運用では自動補完の方がぶれが少ない。

### どのようなときに使うか

- 部門横断で同じ初期構成を配りたいとき
- 課金責任をタグで明確にしたいとき

### 関連シナリオ

- [[Scenarios/Scenario-AzurePolicy.md#タグと標準構成を全サブスクリプションへ展開する]]

### 主な出典

- [[Sources/Topic-02.md]]
- [[Sources/Azure Blueprints による標準化デプロイ設計.md]]
- [[Sources/Azure Blueprints を利用した大規模 Azure 環境のガバナンス設計.md]]
- [[Sources/Azure のコスト管理とコスト配分（Cost Management + タグ）.md]]

---

# 第3章 設計判断ガイド

## 3.1 スコープを選ぶとき

- 組織横断なら管理グループ。
- 1 つのアプリや環境単位ならリソースグループ。
- リソース単位の細粒度委任は RBAC。

## 3.2 強制方法を選ぶとき

- 違反を止めるなら `Deny`。
- まず可視化したいなら `Audit`。
- 作成時に足りない設定を補うなら `Append`。
- 構成不足を後追い修復したいなら `DeployIfNotExists`。

## 3.3 標準化を続けるとき

- IaC と Policy を分けて考える。
- IaC は「望ましい構成を作る」。
- Policy は「望ましくない構成を残さない」。

---

# 第4章 関連シナリオ

- [[Scenarios/Scenario-AzurePolicy.md]]

