# Topic-06-Database

## 学習ゴール

[[Azure用語集.md#Azure SQL Database]]、[[Azure用語集.md#Azure SQL Managed Instance]]、[[Azure用語集.md#Azure Cosmos DB]] を中心に、データストアの選定軸を整理する。

## このTopicの全体像

この Topic では、SQL Database、Managed Instance、SQL on VM、Cosmos DB、Redis、OSS DB を扱う。  
判断軸は「互換性」「可用性」「整合性」「運用負荷」「スケールモデル」。

---

# 第1章 学習マップ

## 1.1 学習順序

1. Azure SQL Database の基本層と保護機能を理解する。
2. Managed Instance と SQL on VM の互換性差を整理する。
3. Cosmos DB で NoSQL とグローバル分散を補う。
4. Redis と OSS DB で周辺リソースを押さえる。

## 1.2 Azureリソース一覧

- Azure SQL Database
- Azure SQL Managed Instance
- SQL Server on Azure VM
- Azure Cosmos DB
- Azure Cache for Redis
- Azure Database for MySQL / PostgreSQL

---

# 第2章 Azureリソース解説

## Resource: Azure SQL Database

### 概要

[[Azure用語集.md#Azure SQL Database]] は Azure の標準 PaaS RDBMS。

### できること

- Single Database / Elastic Pool
- Serverless / Provisioned
- TDE / Always Encrypted / RLS / DDM
- PITR
- Auto Failover Group

### 技術仕様

- Service tier は General Purpose、Business Critical、Hyperscale を中心に選ぶ。
- Elastic Pool は複数 DB の負荷平準化に向く。
- 暗号化は TDE、機密列保護は Always Encrypted、表示制御は DDM と目的を分ける。

### SDK / API

- T-SQL
- Azure SQL SDK / REST API
- Azure CLI / ARM

### 他サービスとの比較

- SQL Database vs Managed Instance: 互換性優先なら MI、運用簡素化優先なら SQL Database。
- SQL Database vs SQL on VM: OS と SQL インスタンス制御が必要なら VM。

### どのようなときに使うか

- 新規業務アプリの標準 DB
- マルチテナント SaaS の Elastic Pool
- 可用性と自動バックアップを PaaS に寄せたいとき

### 関連シナリオ

- [[Scenarios/Scenario-AzureSQL.md#business-critical-で高可用性を確保する]]
- [[Scenarios/Scenario-AzureSQL.md#elastic-pool-でマルチテナント-db-を集約する]]
- [[Scenarios/Scenario-AzureSQL.md#always-encrypted-と-tde-を使い分ける]]

### 主な出典

- [[Sources/Topic-07.md]]
- [[Sources/Azure SQL Database のサービス層設計ガイド.md]]
- [[Sources/Azure SQL Database の購入モデルと展開設計.md]]
- [[Sources/Azure SQL Database データ保護・暗号化機能整理.md]]

## Resource: Azure SQL Managed Instance and SQL on VM

### 概要

[[Azure用語集.md#Azure SQL Managed Instance]] は SQL Server 互換性を高く保つ PaaS。SQL on VM はさらに IaaS 寄りの選択肢。

### できること

- 既存 SQL Server 互換性の高い移行
- インスタンス レベル機能の保持
- クロス DB トランザクション
- SQL on VM による完全制御

### 技術仕様

- Managed Instance は VNet 内に配置し、ネットワーク要件を持つ。
- SQL on VM は OS、パッチ、バックアップ、可用性を自分で設計する。
- Always On、FCI、ディスク キャッシュ設計が SQL on VM の論点になる。

### SDK / API

- T-SQL
- SQL Server tooling
- ARM / CLI

### 他サービスとの比較

- Managed Instance vs SQL Database: 互換性の深さで分ける。
- SQL on VM vs Managed Instance: OS 管理まで必要かで分ける。

### どのようなときに使うか

- 既存 SQL Server 資産を大きく変えずに移行したいとき
- インスタンス機能や SQL Agent 的な互換性を重視するとき

### 関連シナリオ

- [[Scenarios/Scenario-AzureSQL.md#sql-server-互換性を優先して-managed-instance-か-vm-を選ぶ]]
- [[Scenarios/Scenario-AzureSQL.md#auto-failover-group-でリージョン障害に備える]]

### 主な出典

- [[Sources/Topic-07.md]]
- [[Sources/Azure SQL Managed Instance 接続設計ドキュメント.md]]
- [[Sources/Azure VM 上の SQL Server フェイルオーバークラスターインスタンス（FCI）による高可用性構成.md]]
- [[Sources/Azure SQL ServerワークロードをAzureへ移行する際の最適なデータベースサービス.md]]

## Resource: Azure Cosmos DB

### 概要

[[Azure用語集.md#Azure Cosmos DB]] はグローバル分散型 NoSQL データベース。

### できること

- マルチリージョン複製
- 複数 API
- 整合性レベル選択
- Synapse Link / Analytical Store

### 技術仕様

- Strong consistency とマルチリージョン書き込みはトレードオフになる。
- グローバル低遅延読み取りに強い。
- Functions、Event Hubs、Change Feed と相性がよい。

### SDK / API

- SQL API SDK
- Cosmos REST API
- Change Feed / Synapse Link

### 他サービスとの比較

- Cosmos DB vs SQL Database: スキーマ固定 RDB か、グローバル NoSQL か。
- Cosmos DB vs Storage: クエリ可能な分散 DB が要るなら Cosmos DB。

### どのようなときに使うか

- グローバル分散アプリ
- 低レイテンシの JSON ドキュメント保存
- イベント処理結果の格納

### 関連シナリオ

- [[Scenarios/Scenario-CosmosDB.md#マルチリージョン読み取りを前提に整合性を設計する]]
- [[Scenarios/Scenario-CosmosDB.md#event-hubs--functions-の処理結果を-cosmos-db-へ保存する]]

### 主な出典

- [[Sources/Topic-07.md]]
- [[Sources/Azure Cosmos DB API ドキュメント.md]]
- [[Sources/Azure Cosmos DB における一貫性モデルとマルチリージョン書き込み設計.md]]
- [[Sources/Azure Cosmos DB Analytical Store と Azure Synapse Analytics を用いた Near Real-Time 分析アーキテクチャ.md]]

## Resource: Azure Cache for Redis

### 概要

[[Azure用語集.md#Azure Cache for Redis]] は読み取り高速化とセッション共有のためのインメモリ キャッシュ。

### できること

- セッション共有
- キャッシュ層
- Geo-replication
- 永続化

### 技術仕様

- Premium 以上で高度なレプリケーションや永続化を使いやすい。
- SQL Database の読み取り負荷軽減やマルチリージョン セッション保持に使う。

### SDK / API

- Redis protocol
- Redis client libraries

### 他サービスとの比較

- Redis vs Cosmos DB: キャッシュか、永続データストアか。

### どのようなときに使うか

- 読み取りボトルネックを緩和したいとき
- セッション状態を共有したいとき

### 関連シナリオ

- [[Scenarios/Scenario-AzureSQL.md#sql-database-の読み取り負荷を-redis-で緩和する]]

### 主な出典

- [[Sources/Topic-07.md]]
- [[Sources/Azure Cache for Redis を用いたマルチリージョンWebアプリケーション設計.md]]
- [[Sources/Azure SQL Database の読み取り遅延に対する最適化と Azure Cache for Redis の活用.md]]

## Resource: MySQL and PostgreSQL Flexible Server

### 概要

オープンソース系 DB をマネージドで使いたいときの選択肢。

### できること

- マネージド OSS DB 提供
- 可用性選択
- コンピュート層選択
- 移行支援

### 技術仕様

- Flexible Server で可用性やサイズを選ぶ。
- Database Migration Service と組み合わせて移行する。

### SDK / API

- 各 DB 標準ドライバ
- Azure 管理 API

### 他サービスとの比較

- SQL 系互換が必要なら Azure SQL。
- OSS 互換性を優先するなら Flexible Server。

### どのようなときに使うか

- MySQL / PostgreSQL ベースの既存アプリ移行

### 関連シナリオ

- [[Scenarios/Scenario-AzureSQL.md#既存データベースをazureへ移行する際の選択肢を整理する]]

### 主な出典

- [[Sources/Azure Database for MySQL Flexible Server の高可用性設計.md]]
- [[Sources/Azure Database Migration Service と PostgreSQL Flexible Server を使用した移行.md]]

---

# 第3章 設計判断ガイド

## 3.1 互換性を優先するとき

- 新規アプリなら SQL Database。
- SQL Server 互換を深く求めるなら Managed Instance。
- OS 制御まで必要なら SQL on VM。

## 3.2 グローバル分散を優先するとき

- NoSQL と低遅延多リージョンなら Cosmos DB。
- 強整合性要件が厳しいほど書き込みリージョン設計が重要になる。

## 3.3 保護機能を選ぶとき

- 保存時暗号化は TDE。
- クライアント側秘匿まで必要なら Always Encrypted。
- 行や表示の制御は RLS / DDM。

---

# 第4章 関連シナリオ

- [[Scenarios/Scenario-AzureSQL.md]]
- [[Scenarios/Scenario-CosmosDB.md]]

