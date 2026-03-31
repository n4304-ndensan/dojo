# Topic-07 データベースとデータ保護

## 学習ゴール

Azure SQL を軸に、保護、性能、復旧、NoSQL、キャッシュ、移行までを段階的に理解し、データストア選定の判断材料を作る。

## この Topic の全体像

Azure SQL、Managed Instance、Cosmos DB、Redis などのデータ ストア設計と保護機能をまとめる。

対象ドキュメント数: 16 件

## 第1章 学習マップ

### 1.1 学習順序

1. Azure SQL 基礎設計: まずサービス層、購入モデル、展開方式を把握する。
2. データ保護と暗号化: 次に TDE や Always Encrypted など保護機構の違いを見る。
3. 性能、可用性、復旧: 性能最適化、PITR、フェイルオーバー、整合性を判断軸で読む。
4. NoSQL、キャッシュ、移行: Cosmos DB、Redis、Managed Instance、SQL 移行をつなげて理解する。

### 1.2 セクション対応表

- Azure SQL 基礎設計: 4 件 / [[Azure SQL Database のサービス層設計（Business Critical）.md]] / [[Azure SQL Database のサービス層設計ガイド.md]] / [[Azure SQL Database の購入モデルと展開設計.md]] / [[Azure SQL Database 購入モデル設計ドキュメント.md]]
- データ保護と暗号化: 1 件 / [[Azure SQL Database データ保護・暗号化機能整理.md]]
- 性能、可用性、復旧: 3 件 / [[Azure Cosmos DB における一貫性モデルとマルチリージョン書き込み設計.md]] / [[Azure SQL Database のクエリパフォーマンス自動改善.md]] / [[Azure SQL Database のポイントインタイム復元（PITR）.md]]
- NoSQL、キャッシュ、移行: 8 件 / [[Azure Cache for Redis を用いたマルチリージョンWebアプリケーション設計.md]] / [[Azure Cosmos DB Analytical Store と Azure Synapse Analytics を用いた Near Real-Time 分析アーキテクチャ.md]] / [[Azure Cosmos DB API ドキュメント.md]] / [[Azure データ分析アーキテクチャ整理.md]] / [[Azure への OLTP データベース移行アーキテクチャ設計.md]] / [[SQL Server → Azure SQL Managed Instance 移行方式ガイド.md]] / [[SQL Server Always On データベース移行設計ドキュメント.md]] / [[大規模 SQL Server データベースを最小ダウンタイムで Azure に移行する設計.md]]

## 第2章 基礎概念と構成要素

### 2.1 Azure SQL 基礎設計

まずサービス層、購入モデル、展開方式を把握する。

主な出典: [[Azure SQL Database のサービス層設計（Business Critical）.md]] / [[Azure SQL Database のサービス層設計ガイド.md]] / [[Azure SQL Database の購入モデルと展開設計.md]] / [[Azure SQL Database 購入モデル設計ドキュメント.md]]

主要論点: Azure SQL Database のサービス層 / General Purpose / Hyperscale / Business Critical / アーキテクチャまたは設計のポイント / Standard / ― サーバーレスコンピューティングモデル（General Purpose Serverless）― / Serverless コンピューティングモデル / 自動スケーリング / 自動一時停止（Auto Pause）

### 2.2 データ保護と暗号化

次に TDE や Always Encrypted など保護機構の違いを見る。

主な出典: [[Azure SQL Database データ保護・暗号化機能整理.md]]

主要論点: Always Encrypted（常に暗号化） / Dynamic Data Masking（動的データマスキング） / Transparent Data Encryption（TDE） / Row-Level Security（行レベルセキュリティ） / Column-Level Encryption（列レベル暗号化） / Azure SQL セキュリティ機能の役割比較 / 試験での判断ポイント

### 2.3 性能、可用性、復旧

性能最適化、PITR、フェイルオーバー、整合性を判断軸で読む。

主な出典: [[Azure Cosmos DB における一貫性モデルとマルチリージョン書き込み設計.md]] / [[Azure SQL Database のクエリパフォーマンス自動改善.md]] / [[Azure SQL Database のポイントインタイム復元（PITR）.md]]

主要論点: Azure Cosmos DB の一貫性レベル / Cosmos DB におけるマルチリージョン書き込み / クエリパフォーマンス低下の主な原因 / Query Plan Regression / 統計情報の変化 / パラメータスニッフィング / Automatic Tuning / Force Plan（強制プラン） / Query Plan Regression の自動修正 / Elastic Pool との関係

### 2.4 NoSQL、キャッシュ、移行

Cosmos DB、Redis、Managed Instance、SQL 移行をつなげて理解する。

主な出典: [[Azure Cache for Redis を用いたマルチリージョンWebアプリケーション設計.md]] / [[Azure Cosmos DB Analytical Store と Azure Synapse Analytics を用いた Near Real-Time 分析アーキテクチャ.md]] / [[Azure Cosmos DB API ドキュメント.md]] / [[Azure データ分析アーキテクチャ整理.md]] / [[Azure への OLTP データベース移行アーキテクチャ設計.md]] / [[SQL Server → Azure SQL Managed Instance 移行方式ガイド.md]] / [[SQL Server Always On データベース移行設計ドキュメント.md]] / [[大規模 SQL Server データベースを最小ダウンタイムで Azure に移行する設計.md]]

主要論点: Azure Cache for Redis / マルチリージョンアーキテクチャ / Geo-Replication / データ永続化（Persistence） / RDB Persistence / AOF Persistence / Premium Tier の特徴 / セッション継続性 / Active-Active Geo Replication / Redis Cluster

## 第3章 設計判断の軸

### 3.1 Azure SQL 基礎設計

- ある企業では、新しいアプリケーションのために Azure SQL Database を使用する計画を立てています。このアプリケーションはビジネスにとって非常に重要なシステムであり、データベースには高い可用性と高性能が求められています。 ([[Azure SQL Database のサービス層設計（Business Critical）.md]])
- 特に重要な要件として、Azure の **可用性ゾーン（Availability Zone）全体が障害になった場合でも、データベースは利用可能であり続ける必要があります。** ([[Azure SQL Database のサービス層設計（Business Critical）.md]])
- Azure の可用性ゾーンは、同じリージョン内でも物理的に分離されたデータセンター群です。これにより、電源障害やネットワーク障害などが発生した場合でも、他のゾーンが影響を受けないように設計されています。 ([[Azure SQL Database のサービス層設計（Business Critical）.md]])
- しかし、アプリケーションがミッションクリティカルな場合には、1つのゾーンが完全に停止してもデータベースが継続して稼働するような構成が必要になります。 ([[Azure SQL Database のサービス層設計（Business Critical）.md]])
- このような要件を満たす Azure SQL Database のサービス層を選択する必要があります。 ([[Azure SQL Database のサービス層設計（Business Critical）.md]])
- この問題では、Azure SQL Database のサービス層を選択する際に重要な要件があります。これらの要件を整理すると、最適な選択肢が明確になります。 ([[Azure SQL Database のサービス層設計（Business Critical）.md]])

### 3.2 データ保護と暗号化

- Azure SQL Database では、機密データを保護するために複数のセキュリティ機能が提供されています。 ([[Azure SQL Database データ保護・暗号化機能整理.md]])
- これらの機能は **保護する対象や目的が異なる**ため、要件に応じて適切な機能を選択する必要があります。 ([[Azure SQL Database データ保護・暗号化機能整理.md]])
- Always Encrypted は、**データベース管理者や開発者が機密データの平文を見ることができないようにする暗号化機能**です。 ([[Azure SQL Database データ保護・暗号化機能整理.md]])
- この仕組みでは、データは **クライアントアプリケーション側で暗号化されてからデータベースに送信されます**。 ([[Azure SQL Database データ保護・暗号化機能整理.md]])
- そのため、データベースには常に暗号化されたデータのみが保存されます。 ([[Azure SQL Database データ保護・暗号化機能整理.md]])
- 開発者が SQL クエリを実行しても、取得できるのは暗号化された値のみになります。 ([[Azure SQL Database データ保護・暗号化機能整理.md]])

### 3.3 性能、可用性、復旧

- グローバルに展開されるクラウドアプリケーションでは、複数の地域にユーザーが存在するため、データベースを複数リージョンに分散配置することが一般的である。Azure Cosmos DB はこのような用途のために設計された分散データベースであり、世界中のリージョンにレプリカを配置して低遅延アクセスを提供できる。 ([[Azure Cosmos DB における一貫性モデルとマルチリージョン書き込み設計.md]])
- しかし、グローバル分散システムでは常に **「一貫性（Consistency）」と「遅延（Latency）」のトレードオフ**が存在する。データの整合性を厳密に保証しようとすると、リージョン間の同期が必要になり、その結果として読み取りや書き込みの遅延が増加する。一方、遅延を最小化するためにローカル書き込みを許可すると、データの整合性を弱める必要がある。 ([[Azure Cosmos DB における一貫性モデルとマルチリージョン書き込み設計.md]])
- この問題の設計要件を整理すると、次の三つの重要なポイントが見えてくる。 ([[Azure Cosmos DB における一貫性モデルとマルチリージョン書き込み設計.md]])
- 第一に、すべてのリージョンにおいて **強い一貫性（Strong Consistency）** を保証する必要がある。これは、あるクライアントが書き込みを行った後、どのリージョンから読み取っても必ずその書き込み結果が反映されている状態を意味する。つまり、データの読み取りが常に最新状態であることを保証する必要がある。 ([[Azure Cosmos DB における一貫性モデルとマルチリージョン書き込み設計.md]])
- 第二に、読み取りおよび書き込みの遅延を最小限に抑える必要がある。グローバル分散環境では、リージョン間通信が発生すると遅延が増えるため、アーキテクチャ設計によってできる限りその影響を小さくする必要がある。 ([[Azure Cosmos DB における一貫性モデルとマルチリージョン書き込み設計.md]])
- 第三に、Cosmos DB の設計上の制約として **強い一貫性とマルチリージョン書き込みは両立しない**という点を理解する必要がある。つまり、すべてのリージョンで強い一貫性を保証したい場合、書き込みリージョンは単一に限定する必要がある。 ([[Azure Cosmos DB における一貫性モデルとマルチリージョン書き込み設計.md]])

### 3.4 NoSQL、キャッシュ、移行

- Webアプリケーションでは、ユーザーのログイン情報やショッピングカートなどを保持するために **セッション状態（Session State）** を保存する必要がある。 ([[Azure Cache for Redis を用いたマルチリージョンWebアプリケーション設計.md]])
- 高可用性を確保するため、多くのクラウドシステムでは **マルチリージョン構成**が採用される。 ([[Azure Cache for Redis を用いたマルチリージョンWebアプリケーション設計.md]])
- Redis は基本的にメモリデータベースであるため、ノード障害時にデータが失われる可能性がある。 ([[Azure Cache for Redis を用いたマルチリージョンWebアプリケーション設計.md]])
- 永続化を有効にすることで、ノード障害時でもデータを復元できる。 ([[Azure Cache for Redis を用いたマルチリージョンWebアプリケーション設計.md]])
- 今回の要件を満たすのは **Premium Tier のみ**である。 ([[Azure Cache for Redis を用いたマルチリージョンWebアプリケーション設計.md]])
- ユーザーは再ログインする必要がない。 ([[Azure Cache for Redis を用いたマルチリージョンWebアプリケーション設計.md]])

## 第4章 ユースケースで理解する

### 4.1 Azure SQL 基礎設計のユースケース

- Azure SQL Database のサービス層設計（Business Critical）: ある企業では、新しいアプリケーションのために Azure SQL Database を使用する計画を立てています。このアプリケーションはビジネスにとって非常に重要なシステムであり、データベースには高い可用性と高性能が求められています。 出典: [[Azure SQL Database のサービス層設計（Business Critical）.md]]
- Azure SQL Database のサービス層設計ガイド: Azure SQL Database は、Microsoft Azure が提供する **フルマネージドのリレーショナルデータベースサービス（PaaS）**である。 Microsoft SQL Server エンジンをベースにしており、ユーザーはデータベースの管理やバックアップ、パッチ適用などの運用作業をほぼ意識することなく利用できる。 出典: [[Azure SQL Database のサービス層設計ガイド.md]]
- Azure SQL Database の購入モデルと展開設計: 企業がクラウド環境で SaaS アプリケーションやデータサービスを運用する場合、複数のデータベースを効率的に管理する必要がある。 出典: [[Azure SQL Database の購入モデルと展開設計.md]]
- Azure SQL Database 購入モデル設計ドキュメント （vCore / DTU / Serverless / Elastic Pool）: Azure SQL Databaseは **PaaS型リレーショナルデータベースサービス**であり、アプリケーションの要件に応じて複数の購入モデル（Pricing Model）が提供されています。 出典: [[Azure SQL Database 購入モデル設計ドキュメント.md]]

### 4.2 データ保護と暗号化のユースケース

- Azure SQL Database データ保護・暗号化機能整理: Azure SQL Database では、機密データを保護するために複数のセキュリティ機能が提供されています。 これらの機能は **保護する対象や目的が異なる**ため、要件に応じて適切な機能を選択する必要があります。 出典: [[Azure SQL Database データ保護・暗号化機能整理.md]]

### 4.3 性能、可用性、復旧のユースケース

- Azure Cosmos DB における一貫性モデルとマルチリージョン書き込み設計: グローバルに展開されるクラウドアプリケーションでは、複数の地域にユーザーが存在するため、データベースを複数リージョンに分散配置することが一般的である。Azure Cosmos DB はこのような用途のために設計された分散データベースであり、世界中のリージョンにレプリカを配置して低遅延アクセスを提供できる。 出典: [[Azure Cosmos DB における一貫性モデルとマルチリージョン書き込み設計.md]]
- Azure SQL Database のクエリパフォーマンス自動改善 （Automatic Tuning – Force Plan）: 企業が運用する Azure SQL Database では、営業時間中に次のような問題が発生することがある。 出典: [[Azure SQL Database のクエリパフォーマンス自動改善.md]]
- Azure SQL Database のポイントインタイム復元（PITR）: アプリケーション開発では、データベースのスキーマ変更が頻繁に発生します。特に開発段階では、テーブル構造の変更、インデックスの追加、データ型の変更などが何度も行われます。 出典: [[Azure SQL Database のポイントインタイム復元（PITR）.md]]

### 4.4 NoSQL、キャッシュ、移行のユースケース

- Azure Cache for Redis を用いたマルチリージョンWebアプリケーション設計 （Premium Tier + Geo-Replication + Data Persistence）: Webアプリケーションでは、ユーザーのログイン情報やショッピングカートなどを保持するために **セッション状態（Session State）** を保存する必要がある。 出典: [[Azure Cache for Redis を用いたマルチリージョンWebアプリケーション設計.md]]
- Azure Cosmos DB Analytical Store と Azure Synapse Analytics を用いた Near Real-Time 分析アーキテクチャ: Azure Cosmos DB はグローバル分散型 NoSQL データベースサービスであり、トランザクション処理（OLTP）と分析処理（OLAP）の両方に対応できるように設計されている。Azure Cosmos DB には **Analytical Store（分析ストア）** という機能があり、運用データを分析用に最適化された形式で保存... 出典: [[Azure Cosmos DB Analytical Store と Azure Synapse Analytics を用いた Near Real-Time 分析アーキテクチャ.md]]
- Azure Cosmos DB API ドキュメント （SQL API / MongoDB API / Cassandra API / Gremlin API / Table API）: Azure Cosmos DBは **グローバル分散型NoSQLデータベースサービス**です。 低レイテンシ、高可用性、水平スケーリングを特徴とし、クラウドアプリケーションや大規模データ処理に使用されます。 出典: [[Azure Cosmos DB API ドキュメント.md]]
- Azure データ分析アーキテクチャ整理（Cosmos DB / Synapse / Data Pipeline）: Azure では、データ処理の用途によってサービスが明確に分かれています。 特に試験や実務では **OLTP（運用データ処理）と Analytics（分析処理）を分離する設計**が重要になります。 出典: [[Azure データ分析アーキテクチャ整理.md]]
- Azure への OLTP データベース移行アーキテクチャ設計 （Azure SQL Database Hyperscale）: 企業ではオンプレミス環境で稼働している **OLTP（Online Transaction Processing）データベース**を Azure に移行する計画を進めている。OLTP データベースは、日常業務のトランザクション処理を担う重要なシステムであり、以下の特徴を持つ。 出典: [[Azure への OLTP データベース移行アーキテクチャ設計.md]]
- SQL Server → Azure SQL Managed Instance 移行方式ガイド: 企業がオンプレミスの SQL Server を Azure へ移行する際、最も一般的な移行先の一つが **Azure SQL Managed Instance** である。 Azure SQL Managed Instance は SQL Server と高い互換性を持つ PaaS データベースサービスであり、既存アプリケーションを大きく... 出典: [[SQL Server → Azure SQL Managed Instance 移行方式ガイド.md]]
- SQL Server Always On データベース移行設計ドキュメント （SQL Server 2017 → SQL Server 2019 on Azure Linux VM）: 企業ではオンプレミス環境で **SQL Server 2017** を使用しており、可用性確保のために **Always On Availability Group (AG)** を利用した高可用性構成でデータベースを運用している。 出典: [[SQL Server Always On データベース移行設計ドキュメント.md]]
- 大規模 SQL Server データベースを最小ダウンタイムで Azure に移行する設計: ある企業では、オンプレミス環境で運用されている **SQL Server データベース**を Azure に移行する計画を立てています。このデータベースは企業の主要な業務システムを支えており、データ量は約 **4TB** に達しています。また、このデータベースは日々大量のトランザクションが発生するため、トランザクションログの増加速度が非常... 出典: [[大規模 SQL Server データベースを最小ダウンタイムで Azure に移行する設計.md]]

## 第5章 学習チェックポイント

- まず Azure SQL 基礎設計 → データ保護と暗号化 → 性能、可用性、復旧 → NoSQL、キャッシュ、移行 の順で読むと、基礎から応用まで流れを追いやすい。
- 第2章でサービスや構成要素の位置づけを理解し、第3章で制約条件を確認してから第4章のユースケースを読む。
- 同じサービスでも、要件によって選定理由が変わるため、出典リンク先の問題設定を必ず確認する。
- 類似ケースが複数ある場合は、第4章のユースケースを比較して判断軸を揃える。

## 関連用語

- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Network (VNet)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Storage Account]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Cache for Redis]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Cosmos DB]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Synapse Analytics]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Transparent Data Encryption (TDE)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Always Encrypted]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Availability Zone]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure App Service]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Point-in-Time Restore (PITR)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Managed Instance]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Functions]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Event Hubs]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Factory]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Stream Analytics]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Power BI]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure IoT Hub]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Migrate]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Database Migration Service]]

## 出典ドキュメント

- [[Azure Cache for Redis を用いたマルチリージョンWebアプリケーション設計.md]]
- [[Azure Cosmos DB Analytical Store と Azure Synapse Analytics を用いた Near Real-Time 分析アーキテクチャ.md]]
- [[Azure Cosmos DB API ドキュメント.md]]
- [[Azure Cosmos DB における一貫性モデルとマルチリージョン書き込み設計.md]]
- [[Azure SQL Database データ保護・暗号化機能整理.md]]
- [[Azure SQL Database のクエリパフォーマンス自動改善.md]]
- [[Azure SQL Database のサービス層設計（Business Critical）.md]]
- [[Azure SQL Database のサービス層設計ガイド.md]]
- [[Azure SQL Database のポイントインタイム復元（PITR）.md]]
- [[Azure SQL Database の購入モデルと展開設計.md]]
- [[Azure SQL Database 購入モデル設計ドキュメント.md]]
- [[Azure データ分析アーキテクチャ整理.md]]
- [[Azure への OLTP データベース移行アーキテクチャ設計.md]]
- [[SQL Server → Azure SQL Managed Instance 移行方式ガイド.md]]
- [[SQL Server Always On データベース移行設計ドキュメント.md]]
- [[大規模 SQL Server データベースを最小ダウンタイムで Azure に移行する設計.md]]
