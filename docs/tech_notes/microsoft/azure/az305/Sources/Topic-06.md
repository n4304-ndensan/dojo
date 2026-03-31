# Topic-06 ストレージ、バックアップ、可用性

## 学習ゴール

ストレージの種類、保持ポリシー、バックアップ、DR、共有ストレージの順に読み、データ継続性を整理する。

## この Topic の全体像

ストレージ、バックアップ、DR、高可用性、データ保持を整理する。

対象ドキュメント数: 13 件

## 第1章 学習マップ

### 1.1 学習順序

1. ストレージ基礎と保持: Blob、Files、Data Lake、NetApp Files など保存先の性質を先に比較する。
2. バックアップと復旧: Backup と Vault を中心に保護と復元の運用を確認する。
3. 高可用性と DR: ゾーン障害やリージョン障害への備えを判断軸で整理する。
4. 共有ファイルとハイブリッド保持: ファイル共有やオンプレミス連携を含む保持パターンを見る。

### 1.2 セクション対応表

- ストレージ基礎と保持: 6 件 / [[Azure Blob Storage における WORM（Write Once Read Many）データ保持.md]] / [[Azure Data Factory を用いたオンプレミスファイルサーバーから Azure Blob Storage へのデータ移行設計.md]] / [[Azure Storage 設計ドキュメント.md]] / [[Azure 大容量データ転送設計.md]] / [[Event Hubs Capture を利用したコールドパス分析アーキテクチャ設計.md]] / [[Linux VM 向けの高性能共有ファイルシステム（Azure NetApp Files）.md]]
- バックアップと復旧: 1 件 / [[Azure Recovery Services Vault 技術ドキュメント.md]]
- 高可用性と DR: 5 件 / [[Azure SQL Database 設計ドキュメント.md]] / [[Azure 高可用性とディザスタリカバリー設計ドキュメント.md]] / [[DotNETアプリケーションのAzure移行設計.md]] / [[RTO・RPO 要件に基づく Azure 災害復旧（DR）ソリューションの選択.md]] / [[マルチリージョン Azure アプリ設計.md]]
- 共有ファイルとハイブリッド保持: 1 件 / [[オンプレミス NAS から Azure Data Lake Storage Gen2 へのデータ移行設計.md]]

## 第2章 基礎概念と構成要素

### 2.1 ストレージ基礎と保持

Blob、Files、Data Lake、NetApp Files など保存先の性質を先に比較する。

主な出典: [[Azure Blob Storage における WORM（Write Once Read Many）データ保持.md]] / [[Azure Data Factory を用いたオンプレミスファイルサーバーから Azure Blob Storage へのデータ移行設計.md]] / [[Azure Storage 設計ドキュメント.md]] / [[Azure 大容量データ転送設計.md]] / [[Event Hubs Capture を利用したコールドパス分析アーキテクチャ設計.md]] / [[Linux VM 向けの高性能共有ファイルシステム（Azure NetApp Files）.md]]

主要論点: WORM（Write Once Read Many） / Azure Blob Storage の Immutable Storage / アーキテクチャまたは設計のポイント / Blob Soft Delete / Azure Storage Encryption / Blob Versioning / Azure Data Factory とは / 問題のポイント / データソースはオンプレミス / ファイアウォールの背後にある

### 2.2 バックアップと復旧

Backup と Vault を中心に保護と復元の運用を確認する。

主な出典: [[Azure Recovery Services Vault 技術ドキュメント.md]]

主要論点: サービスの仕組み / 主要機能 / Azure VM Backup / バックアップポリシー / セキュリティ機能 / Soft Delete / Multi-User Authorization / Immutable Backup / 可用性 / 関連Azureサービス

### 2.3 高可用性と DR

ゾーン障害やリージョン障害への備えを判断軸で整理する。

主な出典: [[Azure SQL Database 設計ドキュメント.md]] / [[Azure 高可用性とディザスタリカバリー設計ドキュメント.md]] / [[DotNETアプリケーションのAzure移行設計.md]] / [[RTO・RPO 要件に基づく Azure 災害復旧（DR）ソリューションの選択.md]] / [[マルチリージョン Azure アプリ設計.md]]

主要論点: データセンター障害時の可用性 / PII データの暗号化 / 自動スケーリング / コスト最小化 / Azure SQL Database のサービス階層 / General Purpose / Business Critical / Hyperscale / Always Encrypted / Serverless モデル

### 2.4 共有ファイルとハイブリッド保持

ファイル共有やオンプレミス連携を含む保持パターンを見る。

主な出典: [[オンプレミス NAS から Azure Data Lake Storage Gen2 へのデータ移行設計.md]]

主要論点: ― AzCopy を利用した大規模ファイル転送アーキテクチャ ― / データ転送のアーキテクチャ / AzCopy によるデータ転送の仕組み / Azure File Sync / Robocopy / Azure Storage Mover / 完成アーキテクチャ / AzCopy のメリット

## 第3章 設計判断の軸

### 3.1 ストレージ基礎と保持

- ある企業では、法規制や監査要件を満たすためにデータアーカイブソリューションを設計しています。保存対象となるデータには財務記録やログデータなどが含まれており、これらのデータは一定期間の保存が義務付けられています。 ([[Azure Blob Storage における WORM（Write Once Read Many）データ保持.md]])
- 規制要件では、データが保存された後に変更または削除されないことを保証する必要があります。具体的には、データは **WORM（Write Once Read Many）状態**で保存され、5年間保持されなければなりません。 ([[Azure Blob Storage における WORM（Write Once Read Many）データ保持.md]])
- WORMとは、一度書き込まれたデータが変更されることなく、読み取りのみ可能な状態で保存される仕組みを指します。この仕組みは、監査ログ、金融取引履歴、医療データなどの分野でよく使用されます。 ([[Azure Blob Storage における WORM（Write Once Read Many）データ保持.md]])
- Azure 上でこのようなアーカイブソリューションを構築する場合、データの削除や変更を防ぐストレージ機能を選択する必要があります。 ([[Azure Blob Storage における WORM（Write Once Read Many）データ保持.md]])
- この問題では、データアーカイブに関するいくつかの重要な要件があります。まずはシステム設計に影響する条件を整理する必要があります。 ([[Azure Blob Storage における WORM（Write Once Read Many）データ保持.md]])
- 最初の要件は保存期間です。企業は規制要件を満たすためにデータを **5年間保存**する必要があります。 ([[Azure Blob Storage における WORM（Write Once Read Many）データ保持.md]])

### 3.2 バックアップと復旧

- Recovery Services Vaultは、バックアップデータの保存場所であり、同時にバックアップや復旧操作の管理ポイントとして機能する。 ([[Azure Recovery Services Vault 技術ドキュメント.md]])
- Azure VM、SQL Server、Azure Files、オンプレミスサーバーなど様々なリソースを保護することができる。 ([[Azure Recovery Services Vault 技術ドキュメント.md]])
- 従来のオンプレミスでは、以下のような構成でバックアップを管理していた。 ([[Azure Recovery Services Vault 技術ドキュメント.md]])
- というクラウドネイティブなバックアップアーキテクチャが提供されている。 ([[Azure Recovery Services Vault 技術ドキュメント.md]])
- Recovery Services Vaultは、バックアップと復旧操作を統合管理するリソースである。 ([[Azure Recovery Services Vault 技術ドキュメント.md]])
- 2. バックアップポリシーに従ってスナップショットが作成される ([[Azure Recovery Services Vault 技術ドキュメント.md]])

### 3.3 高可用性と DR

- このような情報は法規制やセキュリティポリシーの対象となるため、**適切な暗号化とアクセス制御が必要**となる。 ([[Azure SQL Database 設計ドキュメント.md]])
- また、給与計算などの処理は特定の時間帯に集中することが多く、通常時と比較してデータベースの負荷が急激に増加する場合がある。そのため、インフラ設計では **自動スケーリング機能**を活用することで、必要な時だけリソースを増やし、コストを抑えることが望ましい。 ([[Azure SQL Database 設計ドキュメント.md]])
- さらに、このシステムは企業の業務システムであるため、**データセンター障害時にもサービスが利用可能であること**が求められる。 ([[Azure SQL Database 設計ドキュメント.md]])
- この問題では、次の4つの要件を満たす必要がある。 ([[Azure SQL Database 設計ドキュメント.md]])
- Azure データセンターが停止した場合でも、サービスを継続できる必要がある。 ([[Azure SQL Database 設計ドキュメント.md]])
- この要件は **Geo レプリケーションや Failover Group** を利用することで実現できる。 ([[Azure SQL Database 設計ドキュメント.md]])

### 3.4 共有ファイルとハイブリッド保持

- 今回の要件は、オンプレミスの NAS デバイスからこの storage1 へ JSON ファイルを効率的に転送する方法を設計することである。データ量は 1TB と比較的大きいため、単純なコピーではなく **Azure に最適化されたデータ転送ツール**を利用する必要がある。 ([[オンプレミス NAS から Azure Data Lake Storage Gen2 へのデータ移行設計.md]])
- この要件を満たす最適なツールが **AzCopy** である。 ([[オンプレミス NAS から Azure Data Lake Storage Gen2 へのデータ移行設計.md]])
- このような条件では、Azure が提供する **専用のデータ転送ツール**を使用することが推奨される。 ([[オンプレミス NAS から Azure Data Lake Storage Gen2 へのデータ移行設計.md]])
- この構成では、NAS にアクセスできるサーバーまたはクライアントマシンで AzCopy を実行し、Azure ストレージへ直接アップロードする。 ([[オンプレミス NAS から Azure Data Lake Storage Gen2 へのデータ移行設計.md]])
- この要件を満たす最適なソリューションは ([[オンプレミス NAS から Azure Data Lake Storage Gen2 へのデータ移行設計.md]])

## 第4章 ユースケースで理解する

### 4.1 ストレージ基礎と保持のユースケース

- Azure Blob Storage における WORM（Write Once Read Many）データ保持: ある企業では、法規制や監査要件を満たすためにデータアーカイブソリューションを設計しています。保存対象となるデータには財務記録やログデータなどが含まれており、これらのデータは一定期間の保存が義務付けられています。 出典: [[Azure Blob Storage における WORM（Write Once Read Many）データ保持.md]]
- Azure Data Factory を用いたオンプレミスファイルサーバーから Azure Blob Storage へのデータ移行設計 （Self-hosted Integration Runtime を利用した安全なデータコピー）: 企業ではオンプレミス環境に Windows ファイルサーバーを保有しており、そこに保存されている大量のデータを Azure クラウドに移行する必要がある。今回のシナリオでは、**約 2 TB のデータ**を Azure Blob Storage にコピーする必要がある。 出典: [[Azure Data Factory を用いたオンプレミスファイルサーバーから Azure Blob Storage へのデータ移行設計.md]]
- Azure Storage 設計ドキュメント （性能・コスト・耐障害性・データ保護を考慮したストレージアーキテクチャ）: Azure Storage は、Microsoft Azure が提供するクラウドストレージサービスであり、大規模データを高可用性・高耐久性で保存するための基盤を提供する。オンプレミスのファイルサーバーやアプリケーションのデータをクラウドへ移行する場合、Azure Storage は非常に重要な役割を果たす。 出典: [[Azure Storage 設計ドキュメント.md]]
- Azure 大容量データ転送設計 （オンプレミス → Azure Blob Storage 50TB 移行）: ある組織では、オンプレミス環境で運用しているファイルサーバーのデータを **Azure Blob Storage** に移行する必要がある。クラウド移行プロジェクトでは、既存の大量データを安全かつ迅速にクラウドへ転送することが重要な課題となる。 出典: [[Azure 大容量データ転送設計.md]]
- Event Hubs Capture を利用したコールドパス分析アーキテクチャ設計 （Azure Data Lake Storage Gen2 + Parquet）: 企業では IoT デバイス、アプリケーションログ、トランザクションイベントなどをリアルタイムに収集するために **Azure Event Hubs** を利用することが多い。Event Hubs は Azure の高スループットなイベントストリーミングサービスであり、数百万件/秒規模のイベントを受信することが可能である。 出典: [[Event Hubs Capture を利用したコールドパス分析アーキテクチャ設計.md]]
- Linux VM 向けの高性能共有ファイルシステム（Azure NetApp Files）: ある企業では、Azure 上で稼働するアプリケーションを設計しています。このアプリケーションは複数の Linux 仮想マシン上で実行され、同じデータセットを共有する必要があります。 出典: [[Linux VM 向けの高性能共有ファイルシステム（Azure NetApp Files）.md]]

### 4.2 バックアップと復旧のユースケース

- Azure Recovery Services Vault 技術ドキュメント: **Recovery Services Vault（リカバリーサービスボールト）** は、Azureにおけるバックアップおよび災害復旧を管理するための中核サービスである。 出典: [[Azure Recovery Services Vault 技術ドキュメント.md]]

### 4.3 高可用性と DRのユースケース

- Azure SQL Database 設計ドキュメント （PII データ保護・可用性・自動スケーリング・コスト最適化）: 企業では、従業員情報を管理する **Web アプリケーション**を構築している。 このアプリケーションでは、従業員の個人情報（PII : Personally Identifiable Information）を **Azure SQL Database** に保存する予定である。 出典: [[Azure SQL Database 設計ドキュメント.md]]
- Azure 高可用性とディザスタリカバリー設計ドキュメント: クラウドシステムでは、障害や災害が発生してもサービスを継続するために **高可用性（High Availability: HA）** と **ディザスタリカバリー（Disaster Recovery: DR）** の設計が必要です。 出典: [[Azure 高可用性とディザスタリカバリー設計ドキュメント.md]]
- .NETアプリケーションのAzure移行設計 （Windowsサービス依存アプリ + マルチリージョン高可用性）: ある組織が、オンプレミス環境で稼働している **複雑な.NETアプリケーション**を Azure に移行する計画を立てている。このアプリケーションは長年運用されてきたレガシー構成であり、以下の特徴を持っている。 出典: [[DotNETアプリケーションのAzure移行設計.md]]
- RTO・RPO 要件に基づく Azure 災害復旧（DR）ソリューションの選択: 企業がオンプレミスで運用しているシステムをクラウドと連携させる際には、災害発生時にどのようにシステムを復旧するかという **ディザスタリカバリー（Disaster Recovery: DR）戦略** を設計する必要がある。DR の設計では、どれだけ早くサービスを復旧できるか、どれだけデータ損失を許容できるかといった指標が重要になる。これら... 出典: [[RTO・RPO 要件に基づく Azure 災害復旧（DR）ソリューションの選択.md]]
- マルチリージョン Azure アプリ設計: 企業は可用性と災害対策（DR）を目的として、クラウドアプリケーションを **複数リージョンに展開する構成**を採用することが多い。 出典: [[マルチリージョン Azure アプリ設計.md]]

### 4.4 共有ファイルとハイブリッド保持のユースケース

- オンプレミス NAS から Azure Data Lake Storage Gen2 へのデータ移行設計: 企業がデータ分析基盤やクラウドネイティブアプリケーションを導入する際、オンプレミスに保存されている大量のデータをクラウドストレージへ移行する必要があるケースが多い。今回のシナリオでは、オンプレミス環境に **Network Attached Storage（NAS）** が存在し、その中に約 **1TB の JSON ファイル**を含むフ... 出典: [[オンプレミス NAS から Azure Data Lake Storage Gen2 へのデータ移行設計.md]]

## 第5章 学習チェックポイント

- まず ストレージ基礎と保持 → バックアップと復旧 → 高可用性と DR → 共有ファイルとハイブリッド保持 の順で読むと、基礎から応用まで流れを追いやすい。
- 第2章でサービスや構成要素の位置づけを理解し、第3章で制約条件を確認してから第4章のユースケースを読む。
- 同じサービスでも、要件によって選定理由が変わるため、出典リンク先の問題設定を必ず確認する。
- 類似ケースが複数ある場合は、第4章のユースケースを比較して判断軸を揃える。

## 関連用語

- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Storage Account]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Blob Storage]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#WORM]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Files]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Factory]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Multi-Factor Authentication (MFA)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Role-Based Access Control (RBAC)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Backup]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Recovery Services Vault]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Site Recovery]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Monitor]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Transparent Data Encryption (TDE)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Always Encrypted]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Failover Group]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Availability Set]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Availability Zone]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Kubernetes Service (AKS)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Front Door]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure DNS]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#RTO]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#RPO]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Box]]

## 出典ドキュメント

- [[Azure Blob Storage における WORM（Write Once Read Many）データ保持.md]]
- [[Azure Data Factory を用いたオンプレミスファイルサーバーから Azure Blob Storage へのデータ移行設計.md]]
- [[Azure Recovery Services Vault 技術ドキュメント.md]]
- [[Azure SQL Database 設計ドキュメント.md]]
- [[Azure Storage 設計ドキュメント.md]]
- [[Azure 高可用性とディザスタリカバリー設計ドキュメント.md]]
- [[Azure 大容量データ転送設計.md]]
- [[DotNETアプリケーションのAzure移行設計.md]]
- [[Event Hubs Capture を利用したコールドパス分析アーキテクチャ設計.md]]
- [[Linux VM 向けの高性能共有ファイルシステム（Azure NetApp Files）.md]]
- [[RTO・RPO 要件に基づく Azure 災害復旧（DR）ソリューションの選択.md]]
- [[オンプレミス NAS から Azure Data Lake Storage Gen2 へのデータ移行設計.md]]
- [[マルチリージョン Azure アプリ設計.md]]
