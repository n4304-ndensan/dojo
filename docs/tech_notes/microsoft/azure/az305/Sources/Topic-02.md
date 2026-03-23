# Topic-02 ガバナンスとリソース管理

## 学習ゴール

Azure 環境全体にルールをどうかけるか、どのスコープで権限を委任するか、標準化とコンプライアンスをどう運用に落とすかを段階的に理解する。

## この Topic の全体像

ポリシー、管理グループ、RBAC スコープ、テナントやサブスクリプション管理、標準化テンプレートをまとめる。

対象ドキュメント数: 11 件

## 第1章 学習マップ

### 1.1 学習順序

1. ポリシーと標準化: まずは Policy と Blueprints を使った標準化と制約付けを理解する。
2. 権限委任とスコープ設計: 次に RBAC と ARM による委任範囲の切り方を整理する。
3. データ保護ガバナンス: 暗号化や変更不可保持のような保護要件をどのように強制するかを見る。
4. ワークロード統制: AKS やマルチテナント環境の統制をガバナンス観点で読む。

### 1.2 セクション対応表

- ポリシーと標準化: 3 件 / [[Azure Blueprints による標準化デプロイ設計.md]] / [[Azure Blueprints を利用した大規模 Azure 環境のガバナンス設計.md]] / [[Azure Policy を管理グループで適用する理由（リージョン制限ポリシー）.md]]
- 権限委任とスコープ設計: 3 件 / [[Azure ARM テンプレートデプロイ制御設計.md]] / [[Azure RBAC とスコープ階層によるリソース作成権限の判断.md]] / [[管理グループ階層で仮想ネットワーク管理を委任する RBAC 設計.md]]
- データ保護ガバナンス: 3 件 / [[Azure Blob Storage における機密データの長期保持設計.md]] / [[Azure Policy による Azure SQL Database TDE 強制設計.md]] / [[Azure SQL Database の透過的データ暗号化（TDE）を自動修復付きで強制するガバナンス設計.md]]
- ワークロード統制: 2 件 / [[AKS コンテナセキュリティガバナンス設計.md]] / [[Azure マルチテナントデータベース設計.md]]

## 第2章 基礎概念と構成要素

### 2.1 ポリシーと標準化

まずは Policy と Blueprints を使った標準化と制約付けを理解する。

主な出典: [[Azure Blueprints による標準化デプロイ設計.md]] / [[Azure Blueprints を利用した大規模 Azure 環境のガバナンス設計.md]] / [[Azure Policy を管理グループで適用する理由（リージョン制限ポリシー）.md]]

主要論点: 展開対象リソース / Azure Blueprints とは / Azure ガバナンス階層 / Blueprint の構成要素 / Blueprint Definition / Blueprint Assignment / Management Group / 必要な Management Group / 必要な Blueprint Definition / 必要な Blueprint Assignment

### 2.2 権限委任とスコープ設計

次に RBAC と ARM による委任範囲の切り方を整理する。

主な出典: [[Azure ARM テンプレートデプロイ制御設計.md]] / [[Azure RBAC とスコープ階層によるリソース作成権限の判断.md]] / [[管理グループ階層で仮想ネットワーク管理を委任する RBAC 設計.md]]

主要論点: Azure RBAC（Role-Based Access Control） / カスタムRBACロール / アーキテクチャ / デプロイフロー / Azure Blueprint / Azure Policy / Resource Lock / Managed Identity / メリット / Azure の階層構造

### 2.3 データ保護ガバナンス

暗号化や変更不可保持のような保護要件をどのように強制するかを見る。

主な出典: [[Azure Blob Storage における機密データの長期保持設計.md]] / [[Azure Policy による Azure SQL Database TDE 強制設計.md]] / [[Azure SQL Database の透過的データ暗号化（TDE）を自動修復付きで強制するガバナンス設計.md]]

主要論点: Azure Blob Storage のアカウントタイプ / Blob Storage のアクセス層 / 各ストレージ層の特徴 / Hot Tier / Cool Tier / Archive Tier / 不変ストレージ（Immutable Storage） / 不変ポリシーの種類 / Time-based retention policy / コンテナアクセスポリシー

### 2.4 ワークロード統制

AKS やマルチテナント環境の統制をガバナンス観点で読む。

主な出典: [[AKS コンテナセキュリティガバナンス設計.md]] / [[Azure マルチテナントデータベース設計.md]]

主要論点: Azure Policy for AKS / 承認レジストリ制限 / 読み取り専用ルートファイルシステム / アーキテクチャ / デプロイフロー / メリット / マルチテナントアーキテクチャ / Azure SQL Database Elastic Pool / 自動スケーリングの仕組み

## 第3章 設計判断の軸

### 3.1 ポリシーと標準化

- 各部門のサブスクリプションに対して、次のリソースを同じ構成でデプロイする必要がある。 ([[Azure Blueprints による標準化デプロイ設計.md]])
- これらを **すべての部門で同一構成として標準化**する必要がある。 ([[Azure Blueprints による標準化デプロイ設計.md]])
- サブスクリプションごとに Assignment が必要になる。 ([[Azure Blueprints による標準化デプロイ設計.md]])
- Blueprint Definition は再利用できるため、サブスクリプションごとに作成する必要はない。 ([[Azure Blueprints による標準化デプロイ設計.md]])
- 企業の Azure 環境では、クラウド利用の拡大に伴い **多数のサブスクリプションを管理する必要がある**ケースが多い。 ([[Azure Blueprints を利用した大規模 Azure 環境のガバナンス設計.md]])
- Blueprint を 50 個管理する必要がある。 ([[Azure Blueprints を利用した大規模 Azure 環境のガバナンス設計.md]])

### 3.2 権限委任とスコープ設計

- ある組織では、Azure 環境におけるリソースのデプロイを開発者チームに許可する必要がある。しかし、クラウド環境では無制限にリソース作成を許可すると、以下のような問題が発生する可能性がある。 ([[Azure ARM テンプレートデプロイ制御設計.md]])
- この要件を満たす最も適切な Azure 機能は **Custom RBAC Role（カスタム RBAC ロール）** である。 ([[Azure ARM テンプレートデプロイ制御設計.md]])
- Azure RBAC は、Azure リソースに対するアクセス制御を管理する仕組みである。ユーザー、グループ、またはサービスプリンシパルに対して、どのリソースに対してどの操作を許可するかを定義できる。 ([[Azure ARM テンプレートデプロイ制御設計.md]])
- RBAC は **最小権限の原則（Least Privilege Principle）** に基づいて設計されており、ユーザーが必要最低限の操作のみを実行できるように制御する。これにより、誤操作や不正操作のリスクを低減することができる。 ([[Azure ARM テンプレートデプロイ制御設計.md]])
- 今回の要件では **Resource Group レベルで権限を制限する**必要がある。 ([[Azure ARM テンプレートデプロイ制御設計.md]])
- Azure には組み込みロール（Owner、Contributor、Reader など）が存在するが、これらのロールでは権限が広すぎる場合がある。そのような場合には **カスタム RBAC ロール** を作成することで、必要な操作のみを許可することができる。 ([[Azure ARM テンプレートデプロイ制御設計.md]])

### 3.3 データ保護ガバナンス

- 企業では監査ログ、契約データ、金融記録などの **機密データ**を長期間保存する必要がある。これらのデータは規制やコンプライアンス要件により、一定期間 **変更不可（WORM: Write Once Read Many）**として保持しなければならない場合が多い。 ([[Azure Blob Storage における機密データの長期保持設計.md]])
- 今回のシナリオでは以下の要件がある。 ([[Azure Blob Storage における機密データの長期保持設計.md]])
- この要件は Azure Storage の **Blob Storage 不変ポリシー（Immutable Blob Storage）**を利用することで実現できる。 ([[Azure Blob Storage における機密データの長期保持設計.md]])
- 現在 Microsoft が推奨しているのは **Standard GPv2** である。 ([[Azure Blob Storage における機密データの長期保持設計.md]])
- Azure Blob Storage には 3 種類のアクセス層が存在する。 ([[Azure Blob Storage における機密データの長期保持設計.md]])
- この構成は Azure の **コンプライアンスデータ保存設計の標準パターン**であり、金融・医療・監査ログなどの保存に広く利用されている。 ([[Azure Blob Storage における機密データの長期保持設計.md]])

### 3.4 ワークロード統制

- 今回のシステムでは、Kubernetes クラスター上で実行されるすべてのワークロードに対して、次のセキュリティ要件が定義されている。 ([[AKS コンテナセキュリティガバナンス設計.md]])
- これらの要件は、単一の Kubernetes 設定ではなく、**クラスター全体に対するガバナンスルール**として適用する必要がある。そのため、Azure のガバナンスサービスを利用してポリシーを適用する方法が適している。 ([[AKS コンテナセキュリティガバナンス設計.md]])
- この要件を満たす最適な Azure 機能は ([[AKS コンテナセキュリティガバナンス設計.md]])
- Azure Policy は、Azure 環境におけるリソース構成を評価し、組織のセキュリティ基準やコンプライアンスルールを自動的に適用するサービスである。AKS に対して Azure Policy を適用することで、Kubernetes リソースの作成時にポリシー違反を検出し、デプロイを拒否したり監査したりすることができる。 ([[AKS コンテナセキュリティガバナンス設計.md]])
- コンテナセキュリティのベストプラクティスの一つとして、コンテナのルートファイルシステムを読み取り専用にすることが推奨されている。 ([[AKS コンテナセキュリティガバナンス設計.md]])
- Pod Security Policy（PSP）は Kubernetes でポッドのセキュリティ設定を制御する機能であったが、現在は Kubernetes の新しいバージョンでは廃止されている。そのため、AKS では PSP を使用したセキュリティ管理は推奨されていない。 ([[AKS コンテナセキュリティガバナンス設計.md]])

## 第4章 ユースケースで理解する

### 4.1 ポリシーと標準化のユースケース

- Azure Blueprints による標準化デプロイ設計 （Management Group・Blueprint Definition・Assignment）: ある組織では、4つの部門がそれぞれ独自の **Azure サブスクリプション**を持っている。 組織ではクラウド環境の標準化とガバナンスを維持するために **Azure Blueprints** を使用して共通のインフラ構成を自動展開する計画である。 出典: [[Azure Blueprints による標準化デプロイ設計.md]]
- Azure Blueprints を利用した大規模 Azure 環境のガバナンス設計 （管理グループ階層を利用した Blueprint の定義と割り当て）: 企業の Azure 環境では、クラウド利用の拡大に伴い **多数のサブスクリプションを管理する必要がある**ケースが多い。 出典: [[Azure Blueprints を利用した大規模 Azure 環境のガバナンス設計.md]]
- Azure Policy を管理グループで適用する理由（リージョン制限ポリシー）: ある企業では、Azure 環境におけるガバナンスとコンプライアンスを強化するために、リソースの配置リージョンを制限することを検討しています。企業のポリシーにより、すべての Azure リソースは **West Europe（西ヨーロッパ）** または **North Europe（北ヨーロッパ）** のみで作成される必要があります。 出典: [[Azure Policy を管理グループで適用する理由（リージョン制限ポリシー）.md]]

### 4.2 権限委任とスコープ設計のユースケース

- Azure ARM テンプレートデプロイ制御設計 （特定リソースグループへの限定デプロイ）: ある組織では、Azure 環境におけるリソースのデプロイを開発者チームに許可する必要がある。しかし、クラウド環境では無制限にリソース作成を許可すると、以下のような問題が発生する可能性がある。 出典: [[Azure ARM テンプレートデプロイ制御設計.md]]
- Azure RBAC とスコープ階層によるリソース作成権限の判断: Azure では、ユーザーやグループに対してリソース操作の権限を付与するために **Azure RBAC（Role Based Access Control）** が使用される。RBAC では、ユーザーやグループに対してロールを割り当てることで、どのリソースに対してどの操作を行えるかを制御する。 出典: [[Azure RBAC とスコープ階層によるリソース作成権限の判断.md]]
- 管理グループ階層で仮想ネットワーク管理を委任する RBAC 設計: ある企業では、Enterprise Agreement（EA）に紐づく複数の Azure サブスクリプションを運用しています。これらのサブスクリプションは Azure の **管理グループ（Management Group）階層**の下で整理されており、組織全体のガバナンスを管理グループ単位で実施しています。 出典: [[管理グループ階層で仮想ネットワーク管理を委任する RBAC 設計.md]]

### 4.3 データ保護ガバナンスのユースケース

- Azure Blob Storage における機密データの長期保持設計 （Standard GPv2 + Hot Tier + Immutability Policy）: 企業では監査ログ、契約データ、金融記録などの **機密データ**を長期間保存する必要がある。これらのデータは規制やコンプライアンス要件により、一定期間 **変更不可（WORM: Write Once Read Many）**として保持しなければならない場合が多い。 出典: [[Azure Blob Storage における機密データの長期保持設計.md]]
- Azure Policy による Azure SQL Database TDE 強制設計 （DeployIfNotExists + ARMテンプレート修復）: クラウド環境では、セキュリティとコンプライアンスを維持するために、組織全体で統一されたセキュリティポリシーを適用する必要がある。特にデータベースを扱うシステムでは、保存データの暗号化が重要なセキュリティ要件となる。 出典: [[Azure Policy による Azure SQL Database TDE 強制設計.md]]
- Azure SQL Database の透過的データ暗号化（TDE）を自動修復付きで強制するガバナンス設計: ある企業では、本番環境に存在するすべての Azure SQL Database に対して、データ保護の観点から **透過的データ暗号化（Transparent Data Encryption : TDE）** を必ず有効にするポリシーを導入しようとしています。TDE はデータベースファイルを自動的に暗号化する仕組みであり、ストレージレベル... 出典: [[Azure SQL Database の透過的データ暗号化（TDE）を自動修復付きで強制するガバナンス設計.md]]

### 4.4 ワークロード統制のユースケース

- AKS コンテナセキュリティガバナンス設計 （承認レジストリ制限 + 読み取り専用ファイルシステム）: ある企業では、ミッションクリティカルな本番ワークロードを **Azure Kubernetes Service（AKS）** 上で実行している。コンテナベースのアプリケーションでは、セキュリティやコンプライアンスの観点から、コンテナイメージの信頼性と実行環境の安全性を確保することが非常に重要である。 出典: [[AKS コンテナセキュリティガバナンス設計.md]]
- Azure マルチテナントデータベース設計 （テナントごとの物理分離 + 自動スケーリング）: クラウドアプリケーションでは、複数の顧客（テナント）が同一アプリケーションを利用する **マルチテナントアーキテクチャ** が広く採用されている。SaaS（Software as a Service）モデルでは、アプリケーションロジックは共有しながらも、顧客ごとのデータを安全に管理する必要がある。 出典: [[Azure マルチテナントデータベース設計.md]]

## 第5章 学習チェックポイント

- まず ポリシーと標準化 → 権限委任とスコープ設計 → データ保護ガバナンス → ワークロード統制 の順で読むと、基礎から応用まで流れを追いやすい。
- 第2章でサービスや構成要素の位置づけを理解し、第3章で制約条件を確認してから第4章のユースケースを読む。
- 同じサービスでも、要件によって選定理由が変わるため、出典リンク先の問題設定を必ず確認する。
- 類似ケースが複数ある場合は、第4章のユースケースを比較して判断軸を揃える。

## 関連用語

- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Policy]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Kubernetes Service (AKS)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Container Registry]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Role-Based Access Control (RBAC)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Managed Identity]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Blueprints]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Management Group]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Resource Manager (ARM)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Storage Account]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Blob Storage]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#WORM]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Cosmos DB]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Transparent Data Encryption (TDE)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]

## 出典ドキュメント

- [[AKS コンテナセキュリティガバナンス設計.md]]
- [[Azure ARM テンプレートデプロイ制御設計.md]]
- [[Azure Blob Storage における機密データの長期保持設計.md]]
- [[Azure Blueprints による標準化デプロイ設計.md]]
- [[Azure Blueprints を利用した大規模 Azure 環境のガバナンス設計.md]]
- [[Azure Policy による Azure SQL Database TDE 強制設計.md]]
- [[Azure Policy を管理グループで適用する理由（リージョン制限ポリシー）.md]]
- [[Azure RBAC とスコープ階層によるリソース作成権限の判断.md]]
- [[Azure SQL Database の透過的データ暗号化（TDE）を自動修復付きで強制するガバナンス設計.md]]
- [[Azure マルチテナントデータベース設計.md]]
- [[管理グループ階層で仮想ネットワーク管理を委任する RBAC 設計.md]]
