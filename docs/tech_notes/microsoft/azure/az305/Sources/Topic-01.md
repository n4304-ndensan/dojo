# Topic-01 ID とアクセス管理

## 学習ゴール

ID 基盤からアプリ認証、ワークロード ID、ハイブリッド ID までを順に見て、誰がどの方法で認証され、どこで権限を絞るかを説明できる状態を目指す。

## この Topic の全体像

Microsoft Entra ID を中心に、ユーザー認証、外部アクセス、権限委任、ワークロード ID、ID ガバナンスを横断整理する。

対象ドキュメント数: 28 件

## 第1章 学習マップ

### 1.1 学習順序

1. ID 基盤とユーザー認証: まずは人のサインイン、SSO、外部ユーザー、ガバナンスの入口を押さえる。
2. アプリ認証と API アクセス: 次にアプリ登録、OAuth、委任アクセス、API ロールの関係を固める。
3. ワークロード ID と秘密情報管理: 人ではなく Azure リソースや AKS ワークロードがどう認証するかを整理する。
4. ハイブリッド ID と運用統制: 最後に AD 連携、MFA、管理アクセス保護、テナント運用をつなげる。

### 1.2 セクション対応表

- ID 基盤とユーザー認証: 9 件 / [[Azure Entra ID Governance（アクセスガバナンス）体系整理.md]] / [[Azure Entra ID におけるシングルサインオン方式.md]] / [[Azure Entra ID における外部ユーザーアクセス管理（B2B とアクセスレビュー）.md]] / [[Azure Entra ID 外部ユーザーアクセス.md]] / [[Azure Identity Governance ドキュメント.md]] / [[Azure 認証戦略設計ドキュメント.md]] / [[Microsoft Entra ID ハイブリッド認証方式ドキュメント.md]] / [[Microsoft Entra ID を利用した Azure Web アプリケーション認証アーキテクチャ.md]] / [[外部ユーザーアクセスの定期監査（Azure AD Access Reviews）.md]]
- アプリ認証と API アクセス: 3 件 / [[Azure AD を利用した ASP.NET Core アプリのユーザー委任アクセス設計.md]] / [[Azure Entra ID（Azure AD）アプリ登録と API ロールをトークンに含める設計.md]] / [[SaaS アプリにおける Azure AD と OAuth 2.0 を利用した認証・認可設計.md]]
- ワークロード ID と秘密情報管理: 9 件 / [[AKS から Azure SQL Database に安全に接続する認証方式.md]] / [[AKS における Azure リソース認証アーキテクチャ.md]] / [[Azure App Service で Key Vault の秘密を利用する設計.md]] / [[Azure Key Vault セキュアアクセス設計.md]] / [[Azure Key Vault と Managed Identity を使用したシークレットアクセス制御.md]] / [[Azure Key Vault を用いた API キー管理アーキテクチャ.md]] / [[Azure Key Vault 技術ドキュメント.md]] / [[Azure VM 上の ASP.NET Core アプリから Key Vault を安全に利用する設計.md]] / [[Azure ワークロード用 ID.md]]
- ハイブリッド ID と運用統制: 7 件 / [[Azure AD Connect によるハイブリッドID同期設計.md]] / [[Azure App Service 認証セキュリティ設計.md]] / [[Azure サブスクリプションのテナント変更とアクセス管理.md]] / [[Azure 仮想マシンをインターネットから安全に管理する設計.md]] / [[Azure 管理アクセスを MFA で保護する方法（Conditional Access）.md]] / [[Workday と Microsoft Entra ID を利用したユーザープロビジョニング設計ドキュメント.md]] / [[ハイブリッド ID 環境でオンプレミス障害に耐える認証方式.md]]

## 第2章 基礎概念と構成要素

### 2.1 ID 基盤とユーザー認証

まずは人のサインイン、SSO、外部ユーザー、ガバナンスの入口を押さえる。

主な出典: [[Azure Entra ID Governance（アクセスガバナンス）体系整理.md]] / [[Azure Entra ID におけるシングルサインオン方式.md]] / [[Azure Entra ID における外部ユーザーアクセス管理（B2B とアクセスレビュー）.md]] / [[Azure Entra ID 外部ユーザーアクセス.md]] / [[Azure Identity Governance ドキュメント.md]] / [[Azure 認証戦略設計ドキュメント.md]] / [[Microsoft Entra ID ハイブリッド認証方式ドキュメント.md]] / [[Microsoft Entra ID を利用した Azure Web アプリケーション認証アーキテクチャ.md]] / [[外部ユーザーアクセスの定期監査（Azure AD Access Reviews）.md]]

主要論点: Entitlement Management / Access Reviews / Privileged Identity Management（PIM） / Conditional Access / Identity Protection / 試験での判断のポイント / SAMLベースのSSO / OpenID Connect / パスワードベースSSO / リンクベースSSO

### 2.2 アプリ認証と API アクセス

次にアプリ登録、OAuth、委任アクセス、API ロールの関係を固める。

主な出典: [[Azure AD を利用した ASP.NET Core アプリのユーザー委任アクセス設計.md]] / [[Azure Entra ID（Azure AD）アプリ登録と API ロールをトークンに含める設計.md]] / [[SaaS アプリにおける Azure AD と OAuth 2.0 を利用した認証・認可設計.md]]

主要論点: ユーザーは Azure AD でサインインする / ユーザーのカレンダーにアクセスする / 最小権限（Least Privilege） / 管理作業を最小化する / Azure AD におけるアプリ認証 / Microsoft Graph カレンダーアクセス / Azure AD の権限モデル / Delegated Permissions / Application Permissions / Delegated Permissions が適切な理由

### 2.3 ワークロード ID と秘密情報管理

人ではなく Azure リソースや AKS ワークロードがどう認証するかを整理する。

主な出典: [[AKS から Azure SQL Database に安全に接続する認証方式.md]] / [[AKS における Azure リソース認証アーキテクチャ.md]] / [[Azure App Service で Key Vault の秘密を利用する設計.md]] / [[Azure Key Vault セキュアアクセス設計.md]] / [[Azure Key Vault と Managed Identity を使用したシークレットアクセス制御.md]] / [[Azure Key Vault を用いた API キー管理アーキテクチャ.md]] / [[Azure Key Vault 技術ドキュメント.md]] / [[Azure VM 上の ASP.NET Core アプリから Key Vault を安全に利用する設計.md]] / [[Azure ワークロード用 ID.md]]

主要論点: Azure AD Managed Identity の仕組み / 技術的な構成 / Azure Workload Identity / Azure Workload Identity の仕組み / Kubernetes Service Account / Federated Identity Credential / Managed Identity / RBAC と最小権限 / 最小権限の設計 / AKS 認証アーキテクチャ

### 2.4 ハイブリッド ID と運用統制

最後に AD 連携、MFA、管理アクセス保護、テナント運用をつなげる。

主な出典: [[Azure AD Connect によるハイブリッドID同期設計.md]] / [[Azure App Service 認証セキュリティ設計.md]] / [[Azure サブスクリプションのテナント変更とアクセス管理.md]] / [[Azure 仮想マシンをインターネットから安全に管理する設計.md]] / [[Azure 管理アクセスを MFA で保護する方法（Conditional Access）.md]] / [[Workday と Microsoft Entra ID を利用したユーザープロビジョニング設計ドキュメント.md]] / [[ハイブリッド ID 環境でオンプレミス障害に耐える認証方式.md]]

主要論点: Azure AD Connect / パスワードハッシュ同期 / フィルタリングの必要性 / Azure AD Connect のフィルタリング方式 / Domain Filtering / OU Filtering / Group-Based Filtering / グループベースフィルタリング / OU除外の実現 / Domain / OU Filtering

## 第3章 設計判断の軸

### 3.1 ID 基盤とユーザー認証

- Identity Governance は、ユーザーアクセスの **付与・管理・監査・削除**までのライフサイクルを管理する機能群です。 ([[Azure Entra ID Governance（アクセスガバナンス）体系整理.md]])
- 企業では、プロジェクトチーム、外部パートナー、管理者権限など様々なアクセスが存在するため、 ([[Azure Entra ID Governance（アクセスガバナンス）体系整理.md]])
- Azure Entra ID では主に次の機能を組み合わせてアクセスガバナンスを実現します。 ([[Azure Entra ID Governance（アクセスガバナンス）体系整理.md]])
- Entitlement Management は、Azure Entra ID の **アクセスライフサイクル管理機能**です。 ([[Azure Entra ID Governance（アクセスガバナンス）体系整理.md]])
- ユーザーがアプリケーションやグループなどのリソースへアクセスする際の **申請・承認・期限管理・レビュー**を一元的に管理できます。 ([[Azure Entra ID Governance（アクセスガバナンス）体系整理.md]])
- この機能では **Access Package（アクセスパッケージ）**という仕組みを使用します。 ([[Azure Entra ID Governance（アクセスガバナンス）体系整理.md]])

### 3.2 アプリ認証と API アクセス

- また、これらのアプリケーションは **Microsoft Graph API** を利用してユーザーのカレンダー情報にアクセスする。 ([[Azure AD を利用した ASP.NET Core アプリのユーザー委任アクセス設計.md]])
- 問題文から読み取れる重要な要件は次の通りである。 ([[Azure AD を利用した ASP.NET Core アプリのユーザー委任アクセス設計.md]])
- アプリケーションはユーザー認証を Azure AD に委任する必要がある。 ([[Azure AD を利用した ASP.NET Core アプリのユーザー委任アクセス設計.md]])
- ユーザーのカレンダー情報にアクセスする必要がある。 ([[Azure AD を利用した ASP.NET Core アプリのユーザー委任アクセス設計.md]])
- を簡単に管理できる設計が必要となる。 ([[Azure AD を利用した ASP.NET Core アプリのユーザー委任アクセス設計.md]])
- この App Registration を通じて、アプリケーションは Azure AD からトークンを取得し、Graph API にアクセスできる。 ([[Azure AD を利用した ASP.NET Core アプリのユーザー委任アクセス設計.md]])

### 3.3 ワークロード ID と秘密情報管理

- この問題は、AKS 上のアプリケーションが Azure SQL Database に接続する際に、**認証情報をコードに保存せずに安全にアクセスする方法**を理解しているかどうかを問うものである。 ([[AKS から Azure SQL Database に安全に接続する認証方式.md]])
- この要件を満たすためには、アプリケーションが実行時に Azure AD からトークンを取得し、そのトークンを使って Azure SQL Database に接続する仕組みが必要になる。つまり、静的な資格情報ではなく **動的なトークンベース認証**を利用することが求められている。 ([[AKS から Azure SQL Database に安全に接続する認証方式.md]])
- この方式では、アプリケーションは Azure AD から短期間有効なトークンを取得し、そのトークンを使ってデータベースに接続する。パスワードを保存する必要がなく、トークンは一定時間で失効するためセキュリティリスクも低減される。 ([[AKS から Azure SQL Database に安全に接続する認証方式.md]])
- AKS から Azure SQL Database に接続する際の構成を整理すると、次のような流れになる。 ([[AKS から Azure SQL Database に安全に接続する認証方式.md]])
- ポッドには Azure AD の管理された ID が関連付けられている。アプリケーションはこの ID を利用して Azure AD からアクセストークンを取得する。そのトークンを利用して Azure SQL Database に対して認証を行う。 ([[AKS から Azure SQL Database に安全に接続する認証方式.md]])
- この方式では、接続文字列にパスワードを含める必要がなく、Azure AD による認証とアクセス制御を利用することができる。 ([[AKS から Azure SQL Database に安全に接続する認証方式.md]])

### 3.4 ハイブリッド ID と運用統制

- しかし、大規模な Active Directory 環境では、すべてのユーザーやグループを同期する必要はない。特定の組織単位（OU）やグループのみを同期対象とし、それ以外のオブジェクトは除外する必要がある。 ([[Azure AD Connect によるハイブリッドID同期設計.md]])
- 今回のシナリオでは、以下の要件を満たすハイブリッドIDソリューションを設計する必要がある。 ([[Azure AD Connect によるハイブリッドID同期設計.md]])
- グループベースフィルタリングを利用すると、OU構造を変更する必要がない。 ([[Azure AD Connect によるハイブリッドID同期設計.md]])
- セキュリティポリシーとして、組織は以下のアクセス制御要件を定義している。 ([[Azure App Service 認証セキュリティ設計.md]])
- この要件を満たすには、次の2つの Azure 機能を組み合わせる必要がある。 ([[Azure App Service 認証セキュリティ設計.md]])
- Azure AD 条件付きアクセスポリシー** ([[Azure App Service 認証セキュリティ設計.md]])

## 第4章 ユースケースで理解する

### 4.1 ID 基盤とユーザー認証のユースケース

- Azure Entra ID Governance（アクセスガバナンス）体系整理: Azure Entra ID（旧 Azure Active Directory）では、ユーザーの認証だけでなく、 **誰がどのリソースにアクセスできるかを管理する仕組み**が重要になります。 この領域は **Identity Governance（アイデンティティガバナンス）**と呼ばれます。 出典: [[Azure Entra ID Governance（アクセスガバナンス）体系整理.md]]
- Azure Entra ID におけるシングルサインオン方式: Azure Entra ID（旧 Azure Active Directory）では、ユーザーが一度の認証で複数のアプリケーションにアクセスできる **シングルサインオン（SSO）** を提供しています。SSO を実現する方法はいくつかあり、アプリケーションの種類や変更可否、既存の認証方式によって適切な方式を選択する必要があります。 出典: [[Azure Entra ID におけるシングルサインオン方式.md]]
- Azure Entra ID における外部ユーザーアクセス管理（B2B とアクセスレビュー）: あるグローバル企業では、複数の子会社を含む組織構造を持っています。現在、この企業では **Azure Entra ID（旧 Azure Active Directory）** を中心としたアイデンティティ管理基盤を設計しています。 出典: [[Azure Entra ID における外部ユーザーアクセス管理（B2B とアクセスレビュー）.md]]
- Azure Entra ID 外部ユーザーアクセス（試験・実務ミニ整理）: Azure AD（現在は **Azure Entra ID**）では、他の会社のユーザーが自社のアプリやリソースにアクセスするための仕組みがいくつか用意されています。Azureの認証・ID管理に関する試験では、外部ユーザーのアクセス方法として次の仕組みの違いを理解していることが重要です。 出典: [[Azure Entra ID 外部ユーザーアクセス.md]]
- Azure Identity Governance ドキュメント （PIM / Access Reviews / Least Privilege / JIT Access）: 企業のセキュリティガイドラインでは、管理者権限や特権アクセスを適切に管理することが重要です。特にクラウド環境では、過剰な権限が重大なセキュリティリスクになるため、次の原則が広く採用されています。 出典: [[Azure Identity Governance ドキュメント.md]]
- Azure 認証戦略設計ドキュメント （社内ユーザー + 外部パートナー / SSO / リスクベース条件付きアクセス）: ある組織では、新しいアプリケーションを構築しており、以下のユーザーが利用する予定である。 出典: [[Azure 認証戦略設計ドキュメント.md]]
- Microsoft Entra ID ハイブリッド認証アーキテクチャ （PHS / PTA / Seamless SSO / AD FS）: 企業がクラウドサービスを導入する際、多くの場合すでに **オンプレミスの Active Directory (AD)** を運用している。 このような環境では、既存のユーザー管理を維持しながらクラウドサービスを利用するために **Microsoft Entra ID（旧 Azure Active Directory）との統合**が必要にな... 出典: [[Microsoft Entra ID ハイブリッド認証方式ドキュメント.md]]
- Microsoft Entra ID を利用した Azure Web アプリケーション認証アーキテクチャ （Application Registration / SSO / Device-based Access）: 企業が Azure 上に Web アプリケーションを公開する場合、ユーザー認証をどのように実装するかは非常に重要な設計ポイントである。特に企業アプリケーションでは、単純なログイン機能だけではなく、次のようなセキュリティ要件が求められることが多い。 出典: [[Microsoft Entra ID を利用した Azure Web アプリケーション認証アーキテクチャ.md]]
- 外部ユーザーアクセスの定期監査（Azure AD Access Reviews）: 多くの企業では、Azure Entra ID（旧 Azure AD）を使用して外部ユーザーとコラボレーションを行っています。例えば、パートナー企業、ベンダー、コンサルタント、委託開発者などが Azure テナントにゲストユーザーとして招待されることがあります。 出典: [[外部ユーザーアクセスの定期監査（Azure AD Access Reviews）.md]]

### 4.2 アプリ認証と API アクセスのユースケース

- Azure AD を利用した ASP.NET Core アプリのユーザー委任アクセス設計 (App Registration + Delegated Permissions): 企業では **ASP.NET Core アプリケーション**を Azure 上の仮想マシン環境で運用している。 本シナリオでは、以下のような構成のアプリケーションが存在する。 出典: [[Azure AD を利用した ASP.NET Core アプリのユーザー委任アクセス設計.md]]
- Azure Entra ID（Azure AD）アプリ登録と API ロールをトークンに含める設計: Azure Entra ID（旧 Azure Active Directory）では、アプリケーション同士が安全に通信するための仕組みとして **アプリ登録（App Registration）** が使用される。特に API を公開するアプリケーションと、その API を呼び出すクライアントアプリケーションが存在する場合、API 側で定義... 出典: [[Azure Entra ID（Azure AD）アプリ登録と API ロールをトークンに含める設計.md]]
- SaaS アプリにおける Azure AD と OAuth 2.0 を利用した認証・認可設計 （Web アプリと Web API のトークン発行と認可の役割）: クラウドアプリケーション、特に **SaaS（Software as a Service）アプリケーション**では、ユーザー認証と API アクセス制御を安全に実装する必要がある。 出典: [[SaaS アプリにおける Azure AD と OAuth 2.0 を利用した認証・認可設計.md]]

### 4.3 ワークロード ID と秘密情報管理のユースケース

- AKS から Azure SQL Database に安全に接続する認証方式: クラウドネイティブアプリケーションでは、コンテナ化されたワークロードがデータベースなどの外部サービスへアクセスする必要がある。今回のシナリオでは、Azure Kubernetes Service（AKS）クラスターで動作するアプリケーションが **Azure SQL Database** に接続する必要がある。ただし重要な制約として、**... 出典: [[AKS から Azure SQL Database に安全に接続する認証方式.md]]
- AKS における Azure リソース認証アーキテクチャ （Azure Workload Identity / RBAC / 最小権限）: 近年、多くの企業がモノリシックアプリケーションから **マイクロサービスアーキテクチャ** へ移行している。 この移行では、アプリケーションは複数の独立したサービスに分割され、それぞれが **コンテナとしてデプロイ**される。 出典: [[AKS における Azure リソース認証アーキテクチャ.md]]
- Azure App Service で Key Vault の秘密を利用する設計 （Key Vault Reference + Managed Identity）: クラウド環境でアプリケーションを運用する場合、次のような **機密情報（Secrets）**を安全に管理する必要がある。 出典: [[Azure App Service で Key Vault の秘密を利用する設計.md]]
- Azure Key Vault セキュアアクセス設計 （PIM + Managed Identity）: ある組織では、**Azure Key Vault** を利用してアプリケーションのシークレット（APIキー、接続文字列、証明書など）を管理する Web アプリケーションを開発している。 出典: [[Azure Key Vault セキュアアクセス設計.md]]
- Azure Key Vault と Managed Identity を使用したシークレットアクセス制御: ある Azure アプリケーションでは、アプリケーションの設定情報や接続文字列などの機密情報を安全に管理するために **Azure Key Vault** を使用しています。このアプリケーションは Azure 上で実行されており、Key Vault からシークレットを取得して利用する必要があります。 出典: [[Azure Key Vault と Managed Identity を使用したシークレットアクセス制御.md]]
- Azure Key Vault を用いた API キー管理アーキテクチャ （Secret + Managed Identity）: 企業は Ubuntu 仮想マシン上で動作するアプリケーションを開発しており、そのアプリケーションは **サードパーティのメールサービス API** を利用する必要がある。メールサービスを利用するには **API キー**による認証が必要である。 出典: [[Azure Key Vault を用いた API キー管理アーキテクチャ.md]]
- Azure Key Vault 技術ドキュメント （Secrets / Keys / Certificates / ARM統合）: **Azure Key Vault** は、クラウドアプリケーションで使用する **機密情報を安全に保存・管理するサービス**である。 出典: [[Azure Key Vault 技術ドキュメント.md]]
- Azure VM 上の ASP.NET Core アプリから Key Vault を安全に利用する設計 （Managed Identity + OAuth2 Client Credentials Flow）: 企業では **ASP.NET Core アプリケーション**を Azure 環境で運用している。 今回のシナリオでは、アプリケーションは **Windows Server 2022 Azure Edition の仮想マシン (VM)** 上にデプロイされている。 出典: [[Azure VM 上の ASP.NET Core アプリから Key Vault を安全に利用する設計.md]]
- Azure ワークロード用 ID（実務・試験コンパクト整理）: Azureでアプリケーションが Azure SQL、Storage、Key Vault などへアクセスする場合、主に次の4種類のIDを使います。 出典: [[Azure ワークロード用 ID.md]]

### 4.4 ハイブリッド ID と運用統制のユースケース

- Azure AD Connect によるハイブリッドID同期設計 （グループベースのフィルタリング）: 多くの企業では、オンプレミスの **Active Directory（AD）** とクラウドの **Azure Active Directory（Azure AD / Microsoft Entra ID）** を併用する **ハイブリッドID環境**を採用している。 出典: [[Azure AD Connect によるハイブリッドID同期設計.md]]
- Azure App Service 認証セキュリティ設計 （Azure AD + 条件付きアクセス）: ある企業では、**Azure App Service** 上に多層 Web アプリケーションを展開している。 このアプリケーションは **Azure Active Directory（Azure AD / Microsoft Entra ID）** を利用してユーザー認証を行う構成になっている。 出典: [[Azure App Service 認証セキュリティ設計.md]]
- Azure サブスクリプションのテナント変更とアクセス管理 （Azure AD テナントアソシエーション変更時のロール設計）: 企業が Azure を大規模に利用している場合、複数の **Azure Active Directory（Azure AD / Microsoft Entra ID）テナント**と **多数のサブスクリプション**を管理する必要がある。 出典: [[Azure サブスクリプションのテナント変更とアクセス管理.md]]
- Azure 仮想マシンをインターネットから安全に管理する設計 （Azure Bastion + Conditional Access + Azure MFA）: 企業では Azure 上の仮想マシン（VM）を運用しており、インターネット経由でこれらの VM を管理する必要がある。 一般的に VM 管理には次のプロトコルが使用される。 出典: [[Azure 仮想マシンをインターネットから安全に管理する設計.md]]
- Azure 管理アクセスを MFA で保護する方法（Conditional Access）: ある企業では、Azure 環境のセキュリティを強化するために Azure ポータルへの管理アクセスを保護する必要があります。Azure ポータルはクラウドリソースの管理を行う重要なインターフェースであり、ここへのアクセスが侵害されると、環境全体のセキュリティに重大な影響を与える可能性があります。 出典: [[Azure 管理アクセスを MFA で保護する方法（Conditional Access）.md]]
- Workday と Microsoft Entra ID を利用したユーザープロビジョニング設計ドキュメント: 本シナリオでは、組織のユーザー管理を **クラウドベースのHRシステム（Workday）** を起点として自動化することが目的である。 現在は従業員（employees）および請負業者（contractors）のオンボーディングが手動で行われているが、これを自動プロビジョニングへ移行する。 出典: [[Workday と Microsoft Entra ID を利用したユーザープロビジョニング設計ドキュメント.md]]
- ハイブリッド ID 環境でオンプレミス障害に耐える認証方式: ある企業では、オンプレミスの Active Directory と Azure AD（現在の Microsoft Entra ID）を統合した **ハイブリッド ID 環境**を構築しています。この環境では Azure AD Connect を使用して、オンプレミスのユーザーアカウントをクラウドに同期しています。 出典: [[ハイブリッド ID 環境でオンプレミス障害に耐える認証方式.md]]

## 第5章 学習チェックポイント

- まず ID 基盤とユーザー認証 → アプリ認証と API アクセス → ワークロード ID と秘密情報管理 → ハイブリッド ID と運用統制 の順で読むと、基礎から応用まで流れを追いやすい。
- 第2章でサービスや構成要素の位置づけを理解し、第3章で制約条件を確認してから第4章のユースケースを読む。
- 同じサービスでも、要件によって選定理由が変わるため、出典リンク先の問題設定を必ず確認する。
- 類似ケースが複数ある場合は、第4章のユースケースを比較して判断軸を揃える。

## 関連用語

- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Microsoft Entra ID]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Managed Identity]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Workload Identity]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Key Vault]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Kubernetes Service (AKS)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Storage Account]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Role-Based Access Control (RBAC)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Management Group]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Container Registry]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Cosmos DB]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Cache for Redis]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Service Bus]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure AD Connect]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#App Registration]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure App Service]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Conditional Access]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Multi-Factor Authentication (MFA)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Access Reviews]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Identity Governance]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Resource Manager (ARM)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Network (VNet)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Private Endpoint]]

## 出典ドキュメント

- [[AKS から Azure SQL Database に安全に接続する認証方式.md]]
- [[AKS における Azure リソース認証アーキテクチャ.md]]
- [[Azure AD Connect によるハイブリッドID同期設計.md]]
- [[Azure AD を利用した ASP.NET Core アプリのユーザー委任アクセス設計.md]]
- [[Azure App Service で Key Vault の秘密を利用する設計.md]]
- [[Azure App Service 認証セキュリティ設計.md]]
- [[Azure Entra ID Governance（アクセスガバナンス）体系整理.md]]
- [[Azure Entra ID におけるシングルサインオン方式.md]]
- [[Azure Entra ID における外部ユーザーアクセス管理（B2B とアクセスレビュー）.md]]
- [[Azure Entra ID 外部ユーザーアクセス.md]]
- [[Azure Entra ID（Azure AD）アプリ登録と API ロールをトークンに含める設計.md]]
- [[Azure Identity Governance ドキュメント.md]]
- [[Azure Key Vault セキュアアクセス設計.md]]
- [[Azure Key Vault と Managed Identity を使用したシークレットアクセス制御.md]]
- [[Azure Key Vault を用いた API キー管理アーキテクチャ.md]]
- [[Azure Key Vault 技術ドキュメント.md]]
- [[Azure VM 上の ASP.NET Core アプリから Key Vault を安全に利用する設計.md]]
- [[Azure サブスクリプションのテナント変更とアクセス管理.md]]
- [[Azure ワークロード用 ID.md]]
- [[Azure 仮想マシンをインターネットから安全に管理する設計.md]]
- [[Azure 管理アクセスを MFA で保護する方法（Conditional Access）.md]]
- [[Azure 認証戦略設計ドキュメント.md]]
- [[Microsoft Entra ID ハイブリッド認証方式ドキュメント.md]]
- [[Microsoft Entra ID を利用した Azure Web アプリケーション認証アーキテクチャ.md]]
- [[SaaS アプリにおける Azure AD と OAuth 2.0 を利用した認証・認可設計.md]]
- [[Workday と Microsoft Entra ID を利用したユーザープロビジョニング設計ドキュメント.md]]
- [[ハイブリッド ID 環境でオンプレミス障害に耐える認証方式.md]]
- [[外部ユーザーアクセスの定期監査（Azure AD Access Reviews）.md]]
