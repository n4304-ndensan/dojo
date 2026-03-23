# Topic-11 移行とハイブリッドアーキテクチャ

## 学習ゴール

ハイブリッド接続、ファイル/データ移送、DB 移行、バックアップの順に見て、オンプレミスから Azure への移行道筋を整理する。

## この Topic の全体像

オンプレミス連携、移行方式、移送サービス、ハイブリッド構成をまとめる。

対象ドキュメント数: 9 件

## 第1章 学習マップ

### 1.1 学習順序

1. ハイブリッド接続: まずオンプレミスと Azure をどう接続するかを理解する。
2. ファイル共有とデータ移送: 次にファイル共有や Data Box などのデータ移送パターンを見る。
3. データベース移行: SQL 系移行の方式、互換性、ダウンタイム最小化を整理する。
4. ハイブリッド保護運用: 最後にオンプレミスを含む保護・運用を確認する。

### 1.2 セクション対応表

- ハイブリッド接続: 3 件 / [[Azure ハイブリッド接続アーキテクチャ設計ドキュメント.md]] / [[Azure ハイブリッド接続設計.md]] / [[ExpressRoute と VPN を組み合わせたハイブリッドネットワーク接続設計.md]]
- ファイル共有とデータ移送: 2 件 / [[Azure App Service とオンプレミスファイルサーバーのハイブリッドファイル共有設計.md]] / [[Azure 大容量データ移行サービスまとめ.md]]
- データベース移行: 3 件 / [[Azure SQL 移行サービス整理.md]] / [[SQL Server から Azure へのデータ移行ツールの選択.md]] / [[SQL Server から Azure へのデータ移行ツールの選択2.md]]
- ハイブリッド保護運用: 1 件 / [[Azure Backup とハイブリッドバックアップアーキテクチャ.md]]

## 第2章 基礎概念と構成要素

### 2.1 ハイブリッド接続

まずオンプレミスと Azure をどう接続するかを理解する。

主な出典: [[Azure ハイブリッド接続アーキテクチャ設計ドキュメント.md]] / [[Azure ハイブリッド接続設計.md]] / [[ExpressRoute と VPN を組み合わせたハイブリッドネットワーク接続設計.md]]

主要論点: ハイブリッド接続とは / Azure PaaSサービス / Azure VPN Gateway / 問題 / Azure ExpressRoute / ExpressRoute接続タイプ / Private Peering / Microsoft Peering / Azure Private Link / 重要なポイント

### 2.2 ファイル共有とデータ移送

次にファイル共有や Data Box などのデータ移送パターンを見る。

主な出典: [[Azure App Service とオンプレミスファイルサーバーのハイブリッドファイル共有設計.md]] / [[Azure 大容量データ移行サービスまとめ.md]]

主要論点: Azure Files / Azure File Sync / アーキテクチャまたは設計のポイント / Azure Blob Storage / Azure Data Box Gateway / Azure Import/Export Service / Azure Data Box Disk（Azure Data Box Disk） / Azure Import/Export Service（Azure インポート / エクスポート サービス） / Azure Data Box（Azure Data Box） / Azure Data Box Heavy（Azure Data Box Heavy）

### 2.3 データベース移行

SQL 系移行の方式、互換性、ダウンタイム最小化を整理する。

主な出典: [[Azure SQL 移行サービス整理.md]] / [[SQL Server から Azure へのデータ移行ツールの選択.md]] / [[SQL Server から Azure へのデータ移行ツールの選択2.md]]

主要論点: Azure SQL Database（Azure SQL データベース） / Azure SQL Database Hyperscale（Azure SQL データベース ハイパースケール） / Azure SQL Managed Instance（Azure SQL マネージド インスタンス） / SQL Server on Azure Virtual Machines（Azure 仮想マシン上の SQL Server） / Azure Synapse Analytics（Azure Synapse 分析） / SQL サービス選択の基本ルール / 試験用の覚え方 / Data Migration Assistant（DMA） / Azure Cosmos DB データ移行ツール / 技術的な仕組み

### 2.4 ハイブリッド保護運用

最後にオンプレミスを含む保護・運用を確認する。

主な出典: [[Azure Backup とハイブリッドバックアップアーキテクチャ.md]]

主要論点: サービスの仕組み / Windows Admin Center との統合 / 主要機能 / Azure Backup / Recovery Services Vault / バックアップポリシー / セキュリティ / 暗号化 / Soft Delete / RBAC

## 第3章 設計判断の軸

### 3.1 ハイブリッド接続

- 企業システムでは、オンプレミス環境とクラウド環境を接続する **ハイブリッドクラウドアーキテクチャ** が一般的です。 ([[Azure ハイブリッド接続アーキテクチャ設計ドキュメント.md]])
- Azureではオンプレミス環境とAzureを接続するために複数のネットワークサービスが提供されています。 ([[Azure ハイブリッド接続アーキテクチャ設計ドキュメント.md]])
- これらのサービスは **接続経路・セキュリティレベル・用途** が異なります。 ([[Azure ハイブリッド接続アーキテクチャ設計ドキュメント.md]])
- そのため、要件に応じて適切なサービスを選択する必要があります。 ([[Azure ハイブリッド接続アーキテクチャ設計ドキュメント.md]])
- 本ドキュメントでは、Azureのハイブリッド接続サービスを体系的に説明し、 ([[Azure ハイブリッド接続アーキテクチャ設計ドキュメント.md]])
- 特に **ExpressRoute を利用したプライベート接続** の設計について解説します。 ([[Azure ハイブリッド接続アーキテクチャ設計ドキュメント.md]])

### 3.2 ファイル共有とデータ移送

- しかし、企業の運用環境ではファイルの保存場所に関する追加要件があります。アプリケーションが保存したファイルは、オンプレミス環境に存在するファイルサーバーにも複製される必要があります。さらに、オンプレミスのユーザーはそのファイルに対して **SMB（Server Message Block）プロトコルを使用してアクセスできる必要**があります。 ([[Azure App Service とオンプレミスファイルサーバーのハイブリッドファイル共有設計.md]])
- この要件は、クラウドアプリケーションとオンプレミスのファイル共有を統合する **ハイブリッドストレージアーキテクチャ**を必要とします。また、企業は運用管理の負担を最小限に抑えたいと考えているため、継続的な同期処理は可能な限り自動化されたサービスによって実現する必要があります。 ([[Azure App Service とオンプレミスファイルサーバーのハイブリッドファイル共有設計.md]])
- この問題を理解するためには、アーキテクチャ設計に影響する要件を整理することが重要です。問題文から読み取れる条件を整理すると、主にファイルアクセス方式、データ同期、運用管理の3つの観点が重要になります。 ([[Azure App Service とオンプレミスファイルサーバーのハイブリッドファイル共有設計.md]])
- まず、ファイルアクセス方式に関する要件があります。オンプレミスのユーザーは既存のファイルサーバーと同じ方法でファイルにアクセスする必要があります。 ([[Azure App Service とオンプレミスファイルサーバーのハイブリッドファイル共有設計.md]])
- 次に、データ配置に関する要件があります。アプリケーションは Azure App Service 上で動作しているため、クラウド側にファイルの保存場所が必要になります。 ([[Azure App Service とオンプレミスファイルサーバーのハイブリッドファイル共有設計.md]])
- さらに、オンプレミス環境との同期が必要です。 ([[Azure App Service とオンプレミスファイルサーバーのハイブリッドファイル共有設計.md]])

### 3.3 データベース移行

- これらは **互換性・管理負荷・用途**が異なるため、移行要件に応じて選択します。 ([[Azure SQL 移行サービス整理.md]])
- OS管理、パッチ適用、バックアップ、可用性構成などはすべて Azure が自動管理します。 ([[Azure SQL 移行サービス整理.md]])
- そのため、既存 SQL Server をそのまま移行する場合は **再設計が必要になることがあります。** ([[Azure SQL 移行サービス整理.md]])
- ストレージとコンピュートを分離したアーキテクチャを採用し、非常に大きなデータベースを扱うことができます。 ([[Azure SQL 移行サービス整理.md]])
- しかし、次のような管理作業はユーザーが実施する必要があります。 ([[Azure SQL 移行サービス整理.md]])
- オンプレ SQL Server と完全互換が必要 ([[Azure SQL 移行サービス整理.md]])

### 3.4 ハイブリッド保護運用

- 企業がオンプレミスの **Windows Server ファイルサーバー**を運用している場合、ハードウェア障害や災害に備えてバックアップを取得する必要がある。 ([[Azure Backup とハイブリッドバックアップアーキテクチャ.md]])
- オンプレミスバックアップでは **Azure Backup + Recovery Services Vault** が利用される。 ([[Azure Backup とハイブリッドバックアップアーキテクチャ.md]])
- Azure Backupはフルマネージドバックアップサービスである。 ([[Azure Backup とハイブリッドバックアップアーキテクチャ.md]])
- Recovery Services Vaultはバックアップデータの管理リソースである。 ([[Azure Backup とハイブリッドバックアップアーキテクチャ.md]])
- バックアップのスケジュールを設定する。 ([[Azure Backup とハイブリッドバックアップアーキテクチャ.md]])
- データは転送時・保存時に暗号化される。 ([[Azure Backup とハイブリッドバックアップアーキテクチャ.md]])

## 第4章 ユースケースで理解する

### 4.1 ハイブリッド接続のユースケース

- Azure ハイブリッド接続アーキテクチャ設計ドキュメント （ExpressRoute / Private Link / VPN / Service Endpoint）: 企業システムでは、オンプレミス環境とクラウド環境を接続する **ハイブリッドクラウドアーキテクチャ** が一般的です。 出典: [[Azure ハイブリッド接続アーキテクチャ設計ドキュメント.md]]
- Azure ハイブリッド接続設計 （オンプレミスネットワーク → Azure VNet）: ある組織では、オンプレミスのネットワークと Azure 仮想ネットワーク (VNet) を接続する **ハイブリッド接続アーキテクチャ**を設計する必要がある。 出典: [[Azure ハイブリッド接続設計.md]]
- ExpressRoute と VPN を組み合わせたハイブリッドネットワーク接続設計: ある企業では、オンプレミスデータセンターと Azure クラウド環境を接続するハイブリッドネットワークアーキテクチャを設計しています。企業のワークロードは Azure 上の仮想ネットワークで稼働しており、オンプレミスのシステムと頻繁に通信する必要があります。 出典: [[ExpressRoute と VPN を組み合わせたハイブリッドネットワーク接続設計.md]]

### 4.2 ファイル共有とデータ移送のユースケース

- Azure App Service とオンプレミスファイルサーバーのハイブリッドファイル共有設計: ある企業では、.NET Web アプリケーションである **App2** を Azure App Service 上で運用しています。このアプリケーションは、ユーザーがアップロードしたファイルや生成されたデータファイルを保存する機能を持っています。 出典: [[Azure App Service とオンプレミスファイルサーバーのハイブリッドファイル共有設計.md]]
- Azure 大容量データ移行サービスまとめ（物理データ転送）: オンプレミス環境から Azure に大量のデータを移行する場合、ネットワーク帯域が不足していたり回線の信頼性が低いケースがあります。このような場合は、インターネット経由の転送ではなく **物理デバイスを使用してデータを Azure に配送する方法**が利用されます。 出典: [[Azure 大容量データ移行サービスまとめ.md]]

### 4.3 データベース移行のユースケース

- Azure SQL 移行サービス整理（試験・実務ミニガイド）: オンプレミス SQL Server を Azure に移行する場合、主に次の4つのサービスが候補になります。 出典: [[Azure SQL 移行サービス整理.md]]
- SQL Server から Azure へのデータ移行ツールの選択: 企業がクラウド移行を進める際、オンプレミスのデータベースを Azure に移行するケースは非常に多い。特に既存の SQL Server 環境をクラウドに移行する場合、データベースのサイズ、移行対象、移行先のサービスなどに応じて適切なツールを選択する必要がある。Azure にはさまざまなデータ移行ツールが存在しており、それぞれ用途や対象とな... 出典: [[SQL Server から Azure へのデータ移行ツールの選択.md]]
- SQL Server から Azure へのデータ移行ツールの選択: 企業がクラウド移行を進める際、オンプレミスのデータベースを Azure に移行するケースは非常に多い。特に既存の SQL Server 環境をクラウドに移行する場合、データベースのサイズ、移行対象、移行先のサービスなどに応じて適切なツールを選択する必要がある。Azure にはさまざまなデータ移行ツールが存在しており、それぞれ用途や対象とな... 出典: [[SQL Server から Azure へのデータ移行ツールの選択2.md]]

### 4.4 ハイブリッド保護運用のユースケース

- Azure Backup とハイブリッドバックアップアーキテクチャ （オンプレミス Windows Server バックアップ）: 企業がオンプレミスの **Windows Server ファイルサーバー**を運用している場合、ハードウェア障害や災害に備えてバックアップを取得する必要がある。 出典: [[Azure Backup とハイブリッドバックアップアーキテクチャ.md]]

## 第5章 学習チェックポイント

- まず ハイブリッド接続 → ファイル共有とデータ移送 → データベース移行 → ハイブリッド保護運用 の順で読むと、基礎から応用まで流れを追いやすい。
- 第2章でサービスや構成要素の位置づけを理解し、第3章で制約条件を確認してから第4章のユースケースを読む。
- 同じサービスでも、要件によって選定理由が変わるため、出典リンク先の問題設定を必ず確認する。
- 重複文書がある場合は `同一内容` 表記のある出典もあわせて確認する。

## 関連用語

- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure App Service]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Blob Storage]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Files]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Box]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Role-Based Access Control (RBAC)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Storage Account]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Backup]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Recovery Services Vault]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Site Recovery]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Monitor]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Managed Instance]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Synapse Analytics]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Key Vault]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Network (VNet)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Private Endpoint]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Private Link]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure ExpressRoute]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Cosmos DB]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual WAN]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Bastion]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Factory]]

## 出典ドキュメント

- [[Azure App Service とオンプレミスファイルサーバーのハイブリッドファイル共有設計.md]]
- [[Azure Backup とハイブリッドバックアップアーキテクチャ.md]]
- [[Azure SQL 移行サービス整理.md]]
- [[Azure ハイブリッド接続アーキテクチャ設計ドキュメント.md]]
- [[Azure ハイブリッド接続設計.md]]
- [[Azure 大容量データ移行サービスまとめ.md]]
- [[ExpressRoute と VPN を組み合わせたハイブリッドネットワーク接続設計.md]]
- [[SQL Server から Azure へのデータ移行ツールの選択.md]]
- [[SQL Server から Azure へのデータ移行ツールの選択2.md]]
