# Topic-05 アプリ実行基盤とコンピュート

## 学習ゴール

PaaS 実行基盤、サーバーレス、VM、専有ホストやバッチ処理までを並べて、どの実行基盤をどの条件で選ぶかを理解する。

## この Topic の全体像

App Service、Functions、VM、Batch、Dedicated Host などの実行基盤選択と運用設計を扱う。

対象ドキュメント数: 20 件

## 第1章 学習マップ

### 1.1 学習順序

1. App Service 実行基盤: Web アプリの公開、認証、HTTPS、デプロイ、構成管理を先に固める。
2. Functions とイベント実行: 短い処理をイベント駆動で動かす設計を整理する。
3. VM と SQL on VM 設計: IaaS VM の性能、高可用性、バックアップ、DR をまとめて理解する。
4. Batch、専有ホスト、移行先計算基盤: GPU/HPC、Dedicated Host、移行先の計算基盤選定を扱う。

### 1.2 セクション対応表

- App Service 実行基盤: 5 件 / [[Azure App Service HTTPS 構成設計.md]] / [[Azure App Service と Microsoft Entra ID を利用した認証・認可アーキテクチャ.md]] / [[Azure App Service のゼロダウンタイムデプロイ（Deployment Slots）.md]] / [[Azure App Service 構成管理アーキテクチャ設計.md]] / [[Azure App Service 高可用性設計.md]]
- Functions とイベント実行: 2 件 / [[Azure Functions のホスティングプランとスケーリング（Consumption Plan）.md]] / [[Azure FunctionsでService Busメッセージをコールドスタートなしで処理する設計.md]]
- VM と SQL on VM 設計: 9 件 / [[Azure SQL Server 高可用性アーキテクチャ.md]] / [[Azure VM 上で SQL Server を高性能かつ低コストで運用する設計.md]] / [[Azure VM 上の SQL Server におけるディスクキャッシュ設定設計.md]] / [[Azure 仮想マシンのディザスタリカバリ設計（Azure Site Recovery）.md]] / [[Azure 仮想マシンのバックアップ戦略（RPO と不変バックアップ）.md]] / [[Azure 仮想マシンの高可用性とパフォーマンスアーキテクチャ.md]] / [[VMware vSphere 仮想マシンを Azure へ移行するアーキテクチャ.md]] / [[可用性ゾーン障害に耐える Azure VM 配置設計（Dedicated Host.md]] / [[ミッションクリティカルなAzure仮想マシンの高可用性設計.md]]
- Batch、専有ホスト、移行先計算基盤: 4 件 / [[Azure Batch におけるコスト最適化と HPC ワークロード設計.md]] / [[Azure Batch による GPU バッチワークロードのコスト最適化.md]] / [[Azure Dedicated Host による物理的分離インフラ設計.md]] / [[Azure での多層アプリケーション設計（Web層とデータベース層の最適化）.md]]

## 第2章 基礎概念と構成要素

### 2.1 App Service 実行基盤

Web アプリの公開、認証、HTTPS、デプロイ、構成管理を先に固める。

主な出典: [[Azure App Service HTTPS 構成設計.md]] / [[Azure App Service と Microsoft Entra ID を利用した認証・認可アーキテクチャ.md]] / [[Azure App Service のゼロダウンタイムデプロイ（Deployment Slots）.md]] / [[Azure App Service 構成管理アーキテクチャ設計.md]] / [[Azure App Service 高可用性設計.md]]

主要論点: Azure App Service Managed Certificate / 全体アーキテクチャ / カスタムドメイン設定 / HTTPS の有効化 / SSL証明書の自動更新 / 証明書管理フロー / メリット / ― App Service Authentication（Easy Auth）と Conditional Access によるセキュアな Web アプリ設計 ― / Azure App Service Authentication（Easy Auth） / Microsoft Entra ID によるユーザー認証

### 2.2 Functions とイベント実行

短い処理をイベント駆動で動かす設計を整理する。

主な出典: [[Azure Functions のホスティングプランとスケーリング（Consumption Plan）.md]] / [[Azure FunctionsでService Busメッセージをコールドスタートなしで処理する設計.md]]

主要論点: Azure Functions / Azure Functions のホスティングプラン / Consumption Plan / アーキテクチャまたは設計のポイント / Azure Functions Premium Plan / Azure Functions App Service Plan / Azure Functions Dedicated Plan / Azure Functions トリガー / コールドスタート回避の方法 / A. 消費プラン + Service Bus Trigger

### 2.3 VM と SQL on VM 設計

IaaS VM の性能、高可用性、バックアップ、DR をまとめて理解する。

主な出典: [[Azure SQL Server 高可用性アーキテクチャ.md]] / [[Azure VM 上で SQL Server を高性能かつ低コストで運用する設計.md]] / [[Azure VM 上の SQL Server におけるディスクキャッシュ設定設計.md]] / [[Azure 仮想マシンのディザスタリカバリ設計（Azure Site Recovery）.md]] / [[Azure 仮想マシンのバックアップ戦略（RPO と不変バックアップ）.md]] / [[Azure 仮想マシンの高可用性とパフォーマンスアーキテクチャ.md]] / [[VMware vSphere 仮想マシンを Azure へ移行するアーキテクチャ.md]] / [[可用性ゾーン障害に耐える Azure VM 配置設計（Dedicated Host.md]] / [[ミッションクリティカルなAzure仮想マシンの高可用性設計.md]]

主要論点: SQL Server on VM の高可用性 / フェイルオーバー / Always On AG の特徴 / Azure SQL Database の高可用性 / フェイルオーバーグループ構成 / フェイルオーバーグループの特徴 / フェイルオーバーグループの重要ポイント / セカンダリサーバーは別リージョン / 読み取り専用 / 自動フェイルオーバー

### 2.4 Batch、専有ホスト、移行先計算基盤

GPU/HPC、Dedicated Host、移行先の計算基盤選定を扱う。

主な出典: [[Azure Batch におけるコスト最適化と HPC ワークロード設計.md]] / [[Azure Batch による GPU バッチワークロードのコスト最適化.md]] / [[Azure Dedicated Host による物理的分離インフラ設計.md]] / [[Azure での多層アプリケーション設計（Web層とデータベース層の最適化）.md]]

主要論点: Azure Batch の基本アーキテクチャ / Azure Batch プールタイプ / Batch Service Pool / User Subscription Pool / Azure Hybrid Benefit / Azure Batch VM 種類 / Low Priority VM / Dedicated VM / MPI ジョブの特徴 / Dev タスクの特徴

## 第3章 設計判断の軸

### 3.1 App Service 実行基盤

- そのため今回のシステムでは、以下のセキュリティ要件を満たす構成を採用する必要がある。 ([[Azure App Service HTTPS 構成設計.md]])
- これらの要件を満たす最適な Azure の機能が **App Service Managed Certificate（App Service 管理証明書）**である。 ([[Azure App Service HTTPS 構成設計.md]])
- 通常、SSL 証明書は認証局（CA）から購入し、サーバーにインストールし、有効期限が近づくたびに更新作業を行う必要がある。しかし Managed Certificate を利用すると、証明書の発行、インストール、更新をすべて Azure が自動的に行うため、運用負荷を大幅に削減することができる。 ([[Azure App Service HTTPS 構成設計.md]])
- ユーザーはブラウザからカスタムドメインにアクセスする。DNS はそのドメインを Azure App Service に解決し、通信は HTTPS（TLS）によって暗号化される。SSL 証明書は Azure が管理しているため、運用担当者が証明書を手動で管理する必要はない。 ([[Azure App Service HTTPS 構成設計.md]])
- Azure App Service のデフォルトドメインは通常 `*.azurewebsites.net` であるが、企業の Web サイトでは独自ドメインを使用するのが一般的である。そのため、まず Azure App Service にカスタムドメインを追加する必要がある。 ([[Azure App Service HTTPS 構成設計.md]])
- この設定により、ユーザーが `www.example.com` にアクセスすると Azure App Service にルーティングされる。 ([[Azure App Service HTTPS 構成設計.md]])

### 3.2 Functions とイベント実行

- まず、キューに大量のメッセージが追加された場合には、自動的にスケールアウトして複数のインスタンスでメッセージ処理を行う必要があります。これにより、大量のメッセージでも迅速に処理できます。 ([[Azure Functions のホスティングプランとスケーリング（Consumption Plan）.md]])
- 一方で、キューが空の状態では処理が不要になるため、実行環境は **ゼロインスタンスまでスケールダウン**できる必要があります。これにより、無駄なコストを削減できます。 ([[Azure Functions のホスティングプランとスケーリング（Consumption Plan）.md]])
- このようなイベント駆動型で変動の大きいワークロードに適した Azure Functions のホスティングプランを選択する必要があります。 ([[Azure Functions のホスティングプランとスケーリング（Consumption Plan）.md]])
- この問題では、Azure Functions のホスティングプランに関していくつかの重要な要件があります。これらを整理すると、適切なプランを選択しやすくなります。 ([[Azure Functions のホスティングプランとスケーリング（Consumption Plan）.md]])
- まず、スケーリングに関する要件があります。キューのメッセージ量に応じて関数のインスタンス数を自動的に増減させる必要があります。 ([[Azure Functions のホスティングプランとスケーリング（Consumption Plan）.md]])
- 次に、コスト効率に関する要件があります。メッセージが存在しないときには処理を行わないため、インスタンスをゼロまでスケールダウンできる必要があります。 ([[Azure Functions のホスティングプランとスケーリング（Consumption Plan）.md]])

### 3.3 VM と SQL on VM 設計

- 以下はあなたのドキュメントに **フェイルオーバーグループ（Azure SQL Database）** の内容を整理して組み込んだ **完全版**です。 ([[Azure SQL Server 高可用性アーキテクチャ.md]])
- 重要なのは **SQL Server on VM と Azure SQL Database の高可用性方式が違う**ことなので、その構造が分かるように整理しています。 ([[Azure SQL Server 高可用性アーキテクチャ.md]])
- AzureでSQLデータベースの高可用性を設計する場合、 ([[Azure SQL Server 高可用性アーキテクチャ.md]])
- 企業のデータベースには次の要件がある。 ([[Azure SQL Server 高可用性アーキテクチャ.md]])
- 企業が Azure 上で **SQL Server を IaaS（Azure Virtual Machine）として運用する場合**、次のような要件が発生することが多い。 ([[Azure VM 上で SQL Server を高性能かつ低コストで運用する設計.md]])
- 今回の問題では、SQL Server を Azure VM 上にデプロイする際に以下の条件を満たす必要がある。 ([[Azure VM 上で SQL Server を高性能かつ低コストで運用する設計.md]])

### 3.4 Batch、専有ホスト、移行先計算基盤

- 今回のシナリオでは Azure Batch を使用して **2 種類のワークロード**を実行する必要がある。 ([[Azure Batch におけるコスト最適化と HPC ワークロード設計.md]])
- この要件に基づいて **Batch プール構成**を選択する必要がある。 ([[Azure Batch におけるコスト最適化と HPC ワークロード設計.md]])
- Dev 環境で AHB を使う必要性は低く ([[Azure Batch におけるコスト最適化と HPC ワークロード設計.md]])
- 機械学習（Machine Learning）のワークロードでは、GPU を使用した高性能な計算が必要になる場合があります。特にディープラーニングのトレーニングや推論処理では、GPU による並列処理が大きなパフォーマンス向上をもたらします。 ([[Azure Batch による GPU バッチワークロードのコスト最適化.md]])
- しかし GPU 仮想マシンは Azure の中でも非常に高価なコンピューティングリソースです。そのため、GPU を必要とするワークロードでは **必要な時間だけ GPU を使用するアーキテクチャ**が重要になります。 ([[Azure Batch による GPU バッチワークロードのコスト最適化.md]])
- 問題の要件を整理すると、次のポイントが重要になります。 ([[Azure Batch による GPU バッチワークロードのコスト最適化.md]])

## 第4章 ユースケースで理解する

### 4.1 App Service 実行基盤のユースケース

- Azure App Service HTTPS 構成設計 （カスタムドメイン + SSL自動更新）: ある組織では、新しい Web アプリケーションを **Azure App Service** 上に構築し、インターネットに公開する予定である。Azure App Service は PaaS 型の Web アプリケーション実行環境であり、インフラ管理を Azure に任せながらアプリケーションのデプロイと運用を行うことができる。そのため、... 出典: [[Azure App Service HTTPS 構成設計.md]]
- Azure App Service と Microsoft Entra ID を利用した認証・認可アーキテクチャ: クラウド環境で Web アプリケーションを公開する場合、ユーザー認証とアクセス制御はアプリケーションのセキュリティ設計において最も重要な要素の一つである。特に企業向けアプリケーションでは、ユーザーが組織の ID 管理システムで認証され、所属グループやロールに応じてアクセス権限が制御される仕組みが必要になる。 出典: [[Azure App Service と Microsoft Entra ID を利用した認証・認可アーキテクチャ.md]]
- Azure App Service のゼロダウンタイムデプロイ（Deployment Slots）: クラウドアプリケーションでは、サービスを停止せずに新しいバージョンをリリースすることが重要です。ユーザーが利用している Web アプリケーションを更新する際に、アプリケーションを停止してしまうとサービスの可用性が低下し、ユーザー体験にも影響を与えてしまいます。 出典: [[Azure App Service のゼロダウンタイムデプロイ（Deployment Slots）.md]]
- Azure App Service 構成管理アーキテクチャ設計: 企業がクラウド環境で多数の Web アプリケーションを運用する場合、**アプリケーションの構成管理と変更履歴管理**が非常に重要になる。今回のシナリオでは、Azure サブスクリプション内に **50 個の Azure App Service インスタンス**を展開し、それぞれでコードベースの Web アプリケーションをホストする予定であ... 出典: [[Azure App Service 構成管理アーキテクチャ設計.md]]
- Azure App Service 高可用性設計 （リージョン障害対応 + 自動トラフィックフェイルオーバー）: ある組織では **Azure App Service** を使用してミッションクリティカルな Web アプリケーションを運用している。ミッションクリティカルなシステムでは、サービス停止がビジネスに重大な影響を与えるため、障害発生時でもサービスを継続できる高可用性アーキテクチャが求められる。 出典: [[Azure App Service 高可用性設計.md]]

### 4.2 Functions とイベント実行のユースケース

- Azure Functions のホスティングプランとスケーリング（Consumption Plan）: ある企業では、Azure Storage Queue に格納されたメッセージを処理する Azure Functions をデプロイしています。キューにはバックエンド処理のための作業項目が格納されており、関数はそれらのメッセージを取得して処理します。 出典: [[Azure Functions のホスティングプランとスケーリング（Consumption Plan）.md]]
- Azure FunctionsでService Busメッセージをコールドスタートなしで処理する設計: あるシステムでは、Azure Service Bus のキューに送信されたメッセージを Azure Functions で処理する必要があります。 この処理はイベント駆動型で自動的に実行される必要があります。 出典: [[Azure FunctionsでService Busメッセージをコールドスタートなしで処理する設計.md]]

### 4.3 VM と SQL on VM 設計のユースケース

- Azure SQL 高可用性アーキテクチャ （SQL Server on Azure VM / Azure SQL Database）: AzureでSQLデータベースの高可用性を設計する場合、 **利用しているSQLの種類によって採用する技術が異なる。** 出典: [[Azure SQL Server 高可用性アーキテクチャ.md]]
- Azure VM 上で SQL Server を高性能かつ低コストで運用する設計 （20,000 IOPS + SR-IOV を満たす VM シリーズとディスク選択）: 企業が Azure 上で **SQL Server を IaaS（Azure Virtual Machine）として運用する場合**、次のような要件が発生することが多い。 出典: [[Azure VM 上で SQL Server を高性能かつ低コストで運用する設計.md]]
- Azure VM 上の SQL Server におけるディスクキャッシュ設定設計 （P40 Managed Disk を使用したデータディスクとログディスクの最適化）: 企業では Azure 仮想マシン（Azure VM）上で SQL Server を実行し、データベースワークロードを処理するケースが多い。 出典: [[Azure VM 上の SQL Server におけるディスクキャッシュ設定設計.md]]
- Azure 仮想マシンのディザスタリカバリ設計（Azure Site Recovery）: ある企業では、Azure 仮想マシン上でアプリケーションを運用しています。このアプリケーションはビジネスに重要な役割を持っており、障害が発生した場合でもできるだけ早くサービスを復旧できる仕組みが必要です。 出典: [[Azure 仮想マシンのディザスタリカバリ設計（Azure Site Recovery）.md]]
- Azure 仮想マシンのバックアップ戦略（RPO と不変バックアップ）: ある企業では、ビジネスクリティカルなアプリケーションを Azure 仮想マシン上で運用しています。このアプリケーションは企業の重要業務を支えているため、障害発生時には迅速にデータを復旧できるバックアップ戦略が必要です。 出典: [[Azure 仮想マシンのバックアップ戦略（RPO と不変バックアップ）.md]]
- Azure 仮想マシンの高可用性とパフォーマンスアーキテクチャ （VM Scale Sets / Availability Zones / Traffic Manager）: 企業が Azure 上でアプリケーションをホストする場合、単一の仮想マシン (VM) に依存した構成では、障害や負荷増加に対して脆弱になる可能性がある。そのため、Azure では複数の VM を組み合わせて **高可用性と安定したパフォーマンスを確保するアーキテクチャ**を採用することが推奨されている。 出典: [[Azure 仮想マシンの高可用性とパフォーマンスアーキテクチャ.md]]
- VMware vSphere 仮想マシンを Azure へ移行するアーキテクチャ: 企業がオンプレミス環境からクラウドへ移行する際、最も一般的なシナリオの一つが **VMware 仮想マシンの Azure への移行**である。Azure は VMware 環境を Azure 仮想マシンとして実行できるようにするための複数の移行ツールを提供しており、その中心となるサービスが **Azure Migrate** である。 出典: [[VMware vSphere 仮想マシンを Azure へ移行するアーキテクチャ.md]]
- 可用性ゾーン障害に耐える Azure VM 配置設計（Dedicated Host）: ある企業では、既存のアプリケーションを Azure 仮想マシンへ移行しました。このアプリケーションはビジネス上非常に重要であり、高い可用性が求められています。そのため、インフラストラクチャの設計では単一障害点を排除し、データセンター障害やゾーン障害が発生してもアプリケーションが継続して稼働する必要があります。 出典: [[可用性ゾーン障害に耐える Azure VM 配置設計（Dedicated Host.md]]
- ミッションクリティカルなAzure仮想マシンの高可用性設計: ある企業は、ミッションクリティカルなアプリケーションを **Azure Virtual Machines (VM)** 上で実行しています。このアプリケーションはビジネスにとって非常に重要であり、サービス停止が許されないシステムです。 出典: [[ミッションクリティカルなAzure仮想マシンの高可用性設計.md]]

### 4.4 Batch、専有ホスト、移行先計算基盤のユースケース

- Azure Batch におけるコスト最適化と HPC ワークロード設計 （開発タスクと MPI 本番ジョブのプール構成）: Azure Batch は、大規模な並列処理や HPC（High Performance Computing）ワークロードを実行するためのマネージドサービスである。 出典: [[Azure Batch におけるコスト最適化と HPC ワークロード設計.md]]
- Azure Batch による GPU バッチワークロードのコスト最適化: 機械学習（Machine Learning）のワークロードでは、GPU を使用した高性能な計算が必要になる場合があります。特にディープラーニングのトレーニングや推論処理では、GPU による並列処理が大きなパフォーマンス向上をもたらします。 出典: [[Azure Batch による GPU バッチワークロードのコスト最適化.md]]
- Azure Dedicated Host による物理的分離インフラ設計: ある企業が、**機密性の高い財務データを処理する新しいアプリケーション**を Azure 上に導入しようとしています。 このアプリケーションは厳格なセキュリティ要件を満たす必要があります。 出典: [[Azure Dedicated Host による物理的分離インフラ設計.md]]
- Azure での多層アプリケーション設計（Web層とデータベース層の最適化）: ある企業が、既存の **多層アプリケーション（Multi-tier Application）** を Azure に移行しようとしています。 出典: [[Azure での多層アプリケーション設計（Web層とデータベース層の最適化）.md]]

## 第5章 学習チェックポイント

- まず App Service 実行基盤 → Functions とイベント実行 → VM と SQL on VM 設計 → Batch、専有ホスト、移行先計算基盤 の順で読むと、基礎から応用まで流れを追いやすい。
- 第2章でサービスや構成要素の位置づけを理解し、第3章で制約条件を確認してから第4章のユースケースを読む。
- 同じサービスでも、要件によって選定理由が変わるため、出典リンク先の問題設定を必ず確認する。
- 類似ケースが複数ある場合は、第4章のユースケースを比較して判断軸を揃える。

## 関連用語

- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Key Vault]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure App Service]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Front Door]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure DNS]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Microsoft Entra ID]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Conditional Access]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Multi-Factor Authentication (MFA)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure API Management]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Deployment Slots]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Container Registry]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Availability Zone]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Backup]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#RTO]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Batch]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Kubernetes Service (AKS)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Dedicated Host]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Availability Set]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Functions]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Storage Account]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Network (VNet)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Service Bus]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Load Balancer]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]

## 出典ドキュメント

- [[Azure App Service HTTPS 構成設計.md]]
- [[Azure App Service と Microsoft Entra ID を利用した認証・認可アーキテクチャ.md]]
- [[Azure App Service のゼロダウンタイムデプロイ（Deployment Slots）.md]]
- [[Azure App Service 構成管理アーキテクチャ設計.md]]
- [[Azure App Service 高可用性設計.md]]
- [[Azure Batch におけるコスト最適化と HPC ワークロード設計.md]]
- [[Azure Batch による GPU バッチワークロードのコスト最適化.md]]
- [[Azure Dedicated Host による物理的分離インフラ設計.md]]
- [[Azure Functions のホスティングプランとスケーリング（Consumption Plan）.md]]
- [[Azure FunctionsでService Busメッセージをコールドスタートなしで処理する設計.md]]
- [[Azure SQL Server 高可用性アーキテクチャ.md]]
- [[Azure VM 上で SQL Server を高性能かつ低コストで運用する設計.md]]
- [[Azure VM 上の SQL Server におけるディスクキャッシュ設定設計.md]]
- [[Azure での多層アプリケーション設計（Web層とデータベース層の最適化）.md]]
- [[Azure 仮想マシンのディザスタリカバリ設計（Azure Site Recovery）.md]]
- [[Azure 仮想マシンのバックアップ戦略（RPO と不変バックアップ）.md]]
- [[Azure 仮想マシンの高可用性とパフォーマンスアーキテクチャ.md]]
- [[VMware vSphere 仮想マシンを Azure へ移行するアーキテクチャ.md]]
- [[可用性ゾーン障害に耐える Azure VM 配置設計（Dedicated Host.md]]
- [[ミッションクリティカルなAzure仮想マシンの高可用性設計.md]]
