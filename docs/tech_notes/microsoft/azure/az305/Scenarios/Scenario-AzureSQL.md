# Scenario: Azure SQL

## シナリオ一覧

- Business Critical で高可用性を確保する
- Elastic Pool でマルチテナント DB を集約する
- Always Encrypted と TDE を使い分ける
- SQL Server 互換性を優先して Managed Instance か VM を選ぶ
- Auto Failover Group でリージョン障害に備える
- SQL Database の読み取り負荷を Redis で緩和する
- 既存データベースを Azure へ移行する際の選択肢を整理する

## business-critical-で高可用性を確保する

シナリオ  
ミッションクリティカルな業務 DB をゾーン障害込みで高可用化する。

構成  
Azure SQL Database (Business Critical)  
↓  
Zone-redundant / HA

ポイント  
- 可用性と性能を同時に重視するときの基本選択肢

関連リソース  
Azure SQL Database

出典  
- [[Sources/Azure SQL Database のサービス層設計（Business Critical）.md]]
- [[Sources/Azure SQL Database におけるミッションクリティカルアプリケーションの高可用性設計.md]]

## elastic-pool-でマルチテナント-db-を集約する

シナリオ  
多数テナントの DB を 1 つのプールで弾力的に運用する。

構成  
Tenant Databases  
↓  
Elastic Pool

ポイント  
- 負荷のピークがずれる SaaS で有効
- 個別課金と集約運用のバランスを取りやすい

関連リソース  
Azure SQL Database / Elastic Pool

出典  
- [[Sources/Azure SQL Database Elastic Pool による複数データベースの最適化.md]]
- [[Sources/マルチテナントアプリケーションにおけるAzure SQL Database Elastic Poolの設計.md]]

## always-encrypted-と-tde-を使い分ける

シナリオ  
保存時暗号化とクライアント側秘匿を分けて設計する。

構成  
Client  
↓  
Always Encrypted / TDE  
↓  
Azure SQL Database

ポイント  
- TDE は保存時暗号化
- Always Encrypted は DBA からも秘匿したい列向け

関連リソース  
Azure SQL Database

出典  
- [[Sources/Azure SQL Database データ保護・暗号化機能整理.md]]
- [[Sources/Azure SQL Database の機密データ保護（Always Encrypted）.md]]

## sql-server-互換性を優先して-managed-instance-か-vm-を選ぶ

シナリオ  
既存 SQL Server 機能をどこまで残すかで移行先を決める。

構成  
Existing SQL Server  
↓  
Managed Instance or SQL on VM

ポイント  
- インスタンス機能が必要なら Managed Instance
- OS 制御や完全互換が必要なら SQL on VM

関連リソース  
Azure SQL Managed Instance / Azure Virtual Machines

出典  
- [[Sources/Azure SQL Managed Instance によるクロスデータベーストランザクション対応の移行.md]]
- [[Sources/Azure SQL ServerワークロードをAzureへ移行する際の最適なデータベースサービス.md]]

## auto-failover-group-でリージョン障害に備える

シナリオ  
地域障害時もアプリを継続させるために SQL Database をリージョン冗長化する。

構成  
Primary Region SQL  
↔  
Auto Failover Group  
↔  
Secondary Region SQL

ポイント  
- 接続先切替を単純化できる
- 読み取りセカンダリの使い方も設計対象

関連リソース  
Azure SQL Database / Auto Failover Group

出典  
- [[Sources/Azure SQL Database の自動フェールオーバー（Auto Failover Group）.md]]
- [[Sources/Azure SQL Database のマルチリージョン高可用性設計（Auto Failover Group）.md]]

## sql-database-の読み取り負荷を-redis-で緩和する

シナリオ  
読み取り集中を Redis キャッシュで緩和し、DB の遅延を抑える。

構成  
App  
↓  
Redis Cache  
↓  
Azure SQL Database

ポイント  
- セッション共有とキャッシュを分けて考える
- キャッシュ失効戦略まで設計する

関連リソース  
Azure Cache for Redis / Azure SQL Database

出典  
- [[Sources/Azure SQL Database の読み取り遅延に対する最適化と Azure Cache for Redis の活用.md]]
- [[Sources/Azure Cache for Redis を用いたマルチリージョンWebアプリケーション設計.md]]

## 既存データベースをazureへ移行する際の選択肢を整理する

シナリオ  
オンプレミス SQL Server から Azure のどの DB サービスへ移行するかを整理する。

構成  
On-prem SQL Server  
↓  
SQL Database / Managed Instance / SQL on VM

ポイント  
- 互換性、管理負荷、サイズ、ダウンタイムで判断する

関連リソース  
Azure SQL Database / Azure SQL Managed Instance / Azure Virtual Machines

出典  
- [[Sources/Azure SQL 移行サービス整理.md]]
- [[Sources/SQL Server から Azure へのデータ移行ツールの選択.md]]
