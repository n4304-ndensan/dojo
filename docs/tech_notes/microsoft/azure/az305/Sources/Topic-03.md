# Topic-03 ネットワークとプライベート接続

## 学習ゴール

入口、到達性、閉域接続、名前解決、保護境界の順に見て、通信経路をどう設計するかを体系的に把握する。

## この Topic の全体像

VNet、Private Endpoint、グローバル負荷分散、名前解決、到達性制御を扱う。

対象ドキュメント数: 15 件

## 第1章 学習マップ

### 1.1 学習順序

1. 入口設計と負荷分散: 最初に Ingress、Application Gateway、Front Door、Load Balancer の役割差を押さえる。
2. VNet とハイブリッド接続: 次に VNet、ExpressRoute、Virtual WAN で到達性をどう作るかを見る。
3. Private Endpoint と DNS: その上で PaaS への閉域接続と DNS 名前解決を整理する。
4. ネットワークセキュリティ: 最後に NSG、Firewall、公開制御の観点から保護境界を確認する。

### 1.2 セクション対応表

- 入口設計と負荷分散: 6 件 / [[AKS Ingress Controller 設計ドキュメント.md]] / [[Azure Standard Internal Load Balancer によるアプリケーション層のロードバランシング.md]] / [[Azure グローバルルーティングおよびWeb配信サービス設計ドキュメント.md]] / [[グローバル負荷分散アーキテクチャ.md]] / [[マルチリージョン Web アプリケーションのグローバル負荷分散（Azure Front Door）.md]] / [[マルチリージョン Web アプリケーションのグローバル負荷分散（Azure Front Door）2.md]]
- VNet とハイブリッド接続: 3 件 / [[Azure API Management の外部モードと同一 VNet 内バックエンド接続の判断.md]] / [[Azure ExpressRoute Global Reach を用いたマルチサイトネットワーク設計.md]] / [[Azure Virtual WAN 設計.md]]
- Private Endpoint と DNS: 5 件 / [[Azure PaaS サービスへのプライベート接続設計（Private Endpoint）.md]] / [[Azure Private Endpoint を使用した SQL Database の DNS 名前解決設計.md]] / [[Azure SQL Managed Instance 接続設計ドキュメント.md]] / [[Azure Storage アカウントのネットワーク制御（Private Endpoint と Storage Firewall）.md]] / [[Azure Web App から VNet リソースへ安全にアクセスする方法（Private Endpoint）.md]]
- ネットワークセキュリティ: 1 件 / [[Azure 仮想マシン向けネットワークセキュリティ設計.md]]

## 第2章 基礎概念と構成要素

### 2.1 入口設計と負荷分散

最初に Ingress、Application Gateway、Front Door、Load Balancer の役割差を押さえる。

主な出典: [[AKS Ingress Controller 設計ドキュメント.md]] / [[Azure Standard Internal Load Balancer によるアプリケーション層のロードバランシング.md]] / [[Azure グローバルルーティングおよびWeb配信サービス設計ドキュメント.md]] / [[グローバル負荷分散アーキテクチャ.md]] / [[マルチリージョン Web アプリケーションのグローバル負荷分散（Azure Front Door）.md]] / [[マルチリージョン Web アプリケーションのグローバル負荷分散（Azure Front Door）2.md]]

主要論点: KubernetesにおけるIngress / Ingress構造 / Ingress Controllerとは / マイクロサービスアーキテクチャとIngress / TLS Termination / Path-based Routing / WebSocket / Azure Application Gateway Ingress Controller (AGIC) / Application Gatewayとは / AGICの特徴

### 2.2 VNet とハイブリッド接続

次に VNet、ExpressRoute、Virtual WAN で到達性をどう作るかを見る。

主な出典: [[Azure API Management の外部モードと同一 VNet 内バックエンド接続の判断.md]] / [[Azure ExpressRoute Global Reach を用いたマルチサイトネットワーク設計.md]] / [[Azure Virtual WAN 設計.md]]

主要論点: 関連 Azure サービスの説明 / 技術的な仕組み / ネットワーク課題 / BGP の役割 / ExpressRoute + BGP の動作 / フェールオーバーの仕組み / BGP を使うメリット / 動的ルーティング / 自動フェールオーバー / 最適経路選択

### 2.3 Private Endpoint と DNS

その上で PaaS への閉域接続と DNS 名前解決を整理する。

主な出典: [[Azure PaaS サービスへのプライベート接続設計（Private Endpoint）.md]] / [[Azure Private Endpoint を使用した SQL Database の DNS 名前解決設計.md]] / [[Azure SQL Managed Instance 接続設計ドキュメント.md]] / [[Azure Storage アカウントのネットワーク制御（Private Endpoint と Storage Firewall）.md]] / [[Azure Web App から VNet リソースへ安全にアクセスする方法（Private Endpoint）.md]]

主要論点: Azure Private Endpoint / Private Link の特徴 / アーキテクチャまたは設計のポイント / A サービスエンドポイントポリシー / C 強制トンネル付き VPN / D ネットワークセキュリティグループ / Private Endpoint の基本 / Private Endpoint を利用した SQL Database の DNS / 問題のポイント / SQL Database は Private Endpoint

### 2.4 ネットワークセキュリティ

最後に NSG、Firewall、公開制御の観点から保護境界を確認する。

主な出典: [[Azure 仮想マシン向けネットワークセキュリティ設計.md]]

主要論点: Azure ネットワークセキュリティの基本構造 / Network Security Group（NSG） / NSG のルール例 / Azure Firewall / Threat Intelligence / 集中型ネットワーク監視 / セキュリティアーキテクチャ / Azure DDoS Protection + ASG / Azure WAF + Bastion / Azure Front Door + JIT

## 第3章 設計判断の軸

### 3.1 入口設計と負荷分散

- 外部からのアクセスを **Ingress Controller** を通じて制御するのが一般的です。 ([[AKS Ingress Controller 設計ドキュメント.md]])
- 外部からアクセスするためには、次の方法があります。 ([[AKS Ingress Controller 設計ドキュメント.md]])
- これらを次のように公開する必要があります。 ([[AKS Ingress Controller 設計ドキュメント.md]])
- これらを満たすIngress Controllerを選択する必要があります。 ([[AKS Ingress Controller 設計ドキュメント.md]])
- HTTPS通信の暗号化処理を **ロードバランサまたはIngressで終了する仕組み**です。 ([[AKS Ingress Controller 設計ドキュメント.md]])
- Ingress Controllerは長時間接続を維持する必要があります。 ([[AKS Ingress Controller 設計ドキュメント.md]])

### 3.2 VNet とハイブリッド接続

- この問題を解くには、三つの要件をそれぞれ独立して考える必要がある。 ([[Azure API Management の外部モードと同一 VNet 内バックエンド接続の判断.md]])
- Azure API Management は、API を一元公開し、認証、レート制限、変換、監視などを提供する API ゲートウェイサービスである。ネットワーク面では、仮想ネットワークと組み合わせることで、内部バックエンドに安全に接続する構成を取れる。特に VNet インジェクションには **外部モード** と **内部モード** がある。 ([[Azure API Management の外部モードと同一 VNet 内バックエンド接続の判断.md]])
- 企業は複数のデータセンターを運用しており、オンプレミス環境と Azure クラウドを接続してハイブリッドネットワークを構築している。 ([[Azure ExpressRoute Global Reach を用いたマルチサイトネットワーク設計.md]])
- 各 Azure 仮想ネットワーク (VNet) は **ExpressRoute** を利用してオンプレミスネットワークと接続されている。また、**ExpressRoute Global Reach** によってオンプレミス拠点同士も接続されている。 ([[Azure ExpressRoute Global Reach を用いたマルチサイトネットワーク設計.md]])
- この環境では、Azure からインターネットへ出るトラフィックを **オンプレミスのインターネット出口（オンプレミスサイト）経由**で送る必要がある。 ([[Azure ExpressRoute Global Reach を用いたマルチサイトネットワーク設計.md]])
- 1. Azure はどちらのオンプレミス拠点を使うか判断する必要がある ([[Azure ExpressRoute Global Reach を用いたマルチサイトネットワーク設計.md]])

### 3.3 Private Endpoint と DNS

- しかし、この企業のネットワーク設計では、セキュリティ上の理由から重要な要件があります。まず、Azure 仮想ネットワーク（VNet）から PaaS サービスへの通信は **パブリックインターネットを経由してはならない**という要件があります。つまり、通信は Azure のバックボーンネットワーク内部で完結する必要があります。 ([[Azure PaaS サービスへのプライベート接続設計（Private Endpoint）.md]])
- さらに、もう一つ重要な要件があります。PaaS サービスは **オンプレミスネットワークからアクセスできないようにする必要**があります。企業の内部ネットワークから直接 PaaS サービスにアクセスできてしまうと、セキュリティポリシーに違反する可能性があるためです。 ([[Azure PaaS サービスへのプライベート接続設計（Private Endpoint）.md]])
- このような要件を満たすためには、Azure のネットワーク機能を正しく理解し、PaaS サービスへのアクセス経路を制御する必要があります。 ([[Azure PaaS サービスへのプライベート接続設計（Private Endpoint）.md]])
- この問題を正しく理解するためには、まずネットワーク要件を整理することが重要です。問題文には複数の条件が含まれており、それぞれが設計判断に影響します。 ([[Azure PaaS サービスへのプライベート接続設計（Private Endpoint）.md]])
- まず、Azure 仮想ネットワークから PaaS サービスへの通信に関する要件があります。この通信はセキュリティの観点からインターネットを経由してはならず、Azure の内部ネットワークを利用する必要があります。 ([[Azure PaaS サービスへのプライベート接続設計（Private Endpoint）.md]])
- 次に、アクセス制御に関する要件があります。企業はオンプレミス環境から PaaS サービスにアクセスできないようにする必要があります。 ([[Azure PaaS サービスへのプライベート接続設計（Private Endpoint）.md]])

### 3.4 ネットワークセキュリティ

- 今回のシナリオでは、Azure 仮想マシン上でインターネット向けアプリケーションをホストしている。企業は次のようなセキュリティ要件を満たすネットワーク構成を求めている。 ([[Azure 仮想マシン向けネットワークセキュリティ設計.md]])
- Azure では複数のセキュリティレイヤーを組み合わせてネットワークを保護する。 ([[Azure 仮想マシン向けネットワークセキュリティ設計.md]])
- NSG は Azure の基本的なネットワークアクセス制御機能である。サブネットまたはネットワークインターフェースに対して適用できる。 ([[Azure 仮想マシン向けネットワークセキュリティ設計.md]])
- 今回のシナリオでは、HTTPS のみ許可する必要がある。 ([[Azure 仮想マシン向けネットワークセキュリティ設計.md]])
- Bastion は管理アクセスを安全にするが、ネットワーク監視ではない。 ([[Azure 仮想マシン向けネットワークセキュリティ設計.md]])
- NSG は VM レベルのアクセス制御を提供し、Azure Firewall は集中型セキュリティと脅威インテリジェンスを提供するため、この組み合わせが最適なネットワークセキュリティアーキテクチャとなる。 ([[Azure 仮想マシン向けネットワークセキュリティ設計.md]])

## 第4章 ユースケースで理解する

### 4.1 入口設計と負荷分散のユースケース

- AKS Ingress Controller 設計ドキュメント （TLS終了・パスベースルーティング・WebSocket対応）: Azure Kubernetes Service (AKS) を使用したマイクロサービスアーキテクチャでは、 外部からのアクセスを **Ingress Controller** を通じて制御するのが一般的です。 出典: [[AKS Ingress Controller 設計ドキュメント.md]]
- 1 Azureロードバランシングは2つの視点で整理する: Azureのロードバランサーは次の **2軸** で分類すると理解しやすいです。 出典: [[Azure Standard Internal Load Balancer によるアプリケーション層のロードバランシング.md]]
- Azure グローバルルーティングおよびWeb配信サービス設計ドキュメント: Azureでは、グローバル規模のWebアプリケーションを提供するために複数のトラフィック管理サービスが提供されています。 それぞれのサービスは **動作レイヤー（DNS / L7 / キャッシュ）やスコープ（リージョン / グローバル）** が異なります。 出典: [[Azure グローバルルーティングおよびWeb配信サービス設計ドキュメント.md]]
- グローバル負荷分散アーキテクチャ （Azure Front Door + NSG Service Tag）: 複数リージョンに配置されたアプリケーションをユーザーに提供する場合、**グローバル負荷分散**と**セキュリティ制御**が重要になる。 出典: [[グローバル負荷分散アーキテクチャ.md]]
- マルチリージョン Web アプリケーションのグローバル負荷分散（Azure Front Door）: ある企業では、Web アプリケーションを Azure 上の複数リージョンにデプロイしています。このアプリケーションは世界中のユーザーが利用するため、ユーザーの地理的位置に応じて最適なリージョンへトラフィックを誘導する必要があります。 出典: [[マルチリージョン Web アプリケーションのグローバル負荷分散（Azure Front Door）.md]]
- マルチリージョン Web アプリケーションのグローバル負荷分散（Azure Front Door）: ある企業では、Web アプリケーションを Azure 上の複数リージョンにデプロイしています。このアプリケーションは世界中のユーザーが利用するため、ユーザーの地理的位置に応じて最適なリージョンへトラフィックを誘導する必要があります。 出典: [[マルチリージョン Web アプリケーションのグローバル負荷分散（Azure Front Door）2.md]]

### 4.2 VNet とハイブリッド接続のユースケース

- Azure API Management の外部モードと同一 VNet 内バックエンド接続の判断: この問題は、Azure API Management（APIM）を仮想ネットワークに接続したときに、外部利用者からのアクセス経路と、APIM から内部バックエンドへの到達性がどうなるかを理解しているかを問うものである。設計の論点は三つある。第一に、APIM を **外部モード** でデプロイした場合、インターネット上のパートナーがその A... 出典: [[Azure API Management の外部モードと同一 VNet 内バックエンド接続の判断.md]]
- Azure ExpressRoute Global Reach を用いたマルチサイトネットワーク設計 （Azure → オンプレミス インターネット出口の自動フェールオーバー）: 企業は複数のデータセンターを運用しており、オンプレミス環境と Azure クラウドを接続してハイブリッドネットワークを構築している。 出典: [[Azure ExpressRoute Global Reach を用いたマルチサイトネットワーク設計.md]]
- Azure Virtual WAN 設計: 企業が世界中に拠点を持つ場合、各拠点のネットワークを **クラウド経由で安全かつ高速に接続する必要**がある。 出典: [[Azure Virtual WAN 設計.md]]

### 4.3 Private Endpoint と DNSのユースケース

- Azure PaaS サービスへのプライベート接続設計（Private Endpoint）: ある企業では、Azure 上にアプリケーションを構築しており、そのアプリケーションは Azure の PaaS サービス（例：Azure Storage、Azure SQL Database、Azure Key Vault など）を利用する予定です。PaaS サービスは通常、パブリックエンドポイントを持ち、インターネット経由でアクセス可能... 出典: [[Azure PaaS サービスへのプライベート接続設計（Private Endpoint）.md]]
- Azure Private Endpoint を使用した SQL Database の DNS 名前解決設計 （オンプレミスから Azure Private Endpoint へアクセスする DNS 構成）: 企業では、セキュリティ要件の強化により Azure PaaS サービスを **パブリックエンドポイントではなく Private Endpoint 経由で利用するケース**が増えている。 出典: [[Azure Private Endpoint を使用した SQL Database の DNS 名前解決設計.md]]
- Azure SQL Managed Instance 接続設計ドキュメント （オンプレミスネットワークからの安全なアクセス）: 企業ではデータベースをクラウドへ移行するケースが増えており、Azure SQL Managed Instance（SQL MI）はその代表的な PaaS データベースサービスの一つである。SQL Managed Instance は SQL Server と高い互換性を持ちながら、インフラ管理を Azure 側に任せることができるため、既... 出典: [[Azure SQL Managed Instance 接続設計ドキュメント.md]]
- Azure Storage アカウントのネットワーク制御（Private Endpoint と Storage Firewall）: ある企業では、アプリケーションが Azure Storage アカウントを使用してデータを保存しています。このストレージには機密性の高いデータが含まれているため、新しいセキュリティ要件が追加されました。 出典: [[Azure Storage アカウントのネットワーク制御（Private Endpoint と Storage Firewall）.md]]
- Azure Web App から VNet リソースへ安全にアクセスする方法（Private Endpoint）: Azure App Service（Web App）は、多くの場合 PaaS サービスとしてインターネット上に公開されます。しかし実際のアプリケーションでは、Web アプリがバックエンドのリソースへアクセスする必要があります。 出典: [[Azure Web App から VNet リソースへ安全にアクセスする方法（Private Endpoint）.md]]

### 4.4 ネットワークセキュリティのユースケース

- Azure 仮想マシン向けネットワークセキュリティ設計 （NSG + Azure Firewall + Threat Intelligence）: クラウド環境で仮想マシン上にアプリケーションを公開する場合、ネットワークセキュリティ設計は非常に重要である。特にインターネット公開アプリケーションでは、不要なポートを公開すると攻撃対象となる可能性が高くなる。 出典: [[Azure 仮想マシン向けネットワークセキュリティ設計.md]]

## 第5章 学習チェックポイント

- まず 入口設計と負荷分散 → VNet とハイブリッド接続 → Private Endpoint と DNS → ネットワークセキュリティ の順で読むと、基礎から応用まで流れを追いやすい。
- 第2章でサービスや構成要素の位置づけを理解し、第3章で制約条件を確認してから第4章のユースケースを読む。
- 同じサービスでも、要件によって選定理由が変わるため、出典リンク先の問題設定を必ず確認する。
- 重複文書がある場合は `同一内容` 表記のある出典もあわせて確認する。

## 関連用語

- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Key Vault]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Kubernetes Service (AKS)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Ingress Controller]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Network (VNet)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Front Door]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Application Gateway]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Load Balancer]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Monitor]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Microsoft Entra ID]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure API Management]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure ExpressRoute]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Private Endpoint]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Private Link]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Storage Account]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure DNS]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Managed Instance]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual WAN]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure App Service]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Log Analytics]]

## 出典ドキュメント

- [[AKS Ingress Controller 設計ドキュメント.md]]
- [[Azure API Management の外部モードと同一 VNet 内バックエンド接続の判断.md]]
- [[Azure ExpressRoute Global Reach を用いたマルチサイトネットワーク設計.md]]
- [[Azure PaaS サービスへのプライベート接続設計（Private Endpoint）.md]]
- [[Azure Private Endpoint を使用した SQL Database の DNS 名前解決設計.md]]
- [[Azure SQL Managed Instance 接続設計ドキュメント.md]]
- [[Azure Standard Internal Load Balancer によるアプリケーション層のロードバランシング.md]]
- [[Azure Storage アカウントのネットワーク制御（Private Endpoint と Storage Firewall）.md]]
- [[Azure Virtual WAN 設計.md]]
- [[Azure Web App から VNet リソースへ安全にアクセスする方法（Private Endpoint）.md]]
- [[Azure グローバルルーティングおよびWeb配信サービス設計ドキュメント.md]]
- [[Azure 仮想マシン向けネットワークセキュリティ設計.md]]
- [[グローバル負荷分散アーキテクチャ.md]]
- [[マルチリージョン Web アプリケーションのグローバル負荷分散（Azure Front Door）.md]]
- [[マルチリージョン Web アプリケーションのグローバル負荷分散（Azure Front Door）2.md]]
