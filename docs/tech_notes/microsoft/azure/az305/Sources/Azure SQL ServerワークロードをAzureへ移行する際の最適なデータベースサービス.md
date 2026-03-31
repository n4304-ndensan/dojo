---
分類: Cloud
tags:
  - cloud/azure
  - cloud/azure/sql-managed-instance
  - cloud/azure/sql-database
  - cloud/azure/cosmosdb
  - cloud/azure/postgresql
  - cloud/architecture/high-availability
  - cloud/architecture/scalability
  - security/encryption
  - database/relational
  - database/sql-server
  - cloud/migration
  - exam/azure/database
---

# Azure SQL ServerワークロードをAzureへ移行する際の最適なデータベースサービス

## 1. 背景（シナリオ）

ある企業が、オンプレミス環境で稼働している **SQL Server データベース**を **Azure へ移行**する計画を立てています。  
この移行では、既存のアプリケーションを大きく変更することなく、クラウドの利点を活用することが求められています。

特に以下の点が重要視されています。

- 複雑なSQLクエリを高速に実行できること
- システム停止を最小化する高可用性
- データレジデンシー（データの保存地域）への準拠
- ワークロードに応じたスケーリング
- 保存時および通信時の暗号化などの高度なセキュリティ

このような条件を満たしながら **SQL Server ワークロードをAzureに移行する際に最適なサービス**を選択する必要があります。

---

## 2. 要件整理

この問題では、クラウド移行において満たすべき要件が複数提示されています。まず、それらを整理することが重要です。

本シナリオで求められている要件は次の通りです。

- 高可用性
- 複雑なSQLクエリへの低遅延アクセス
- SQL Server との高い互換性
- データレジデンシー対応
- スケーリング機能
- 保存時・通信時の暗号化
- ネットワークレベルのセキュリティ

これらの条件から、この問題は単なるデータベース選択ではなく **「SQL Server移行に最適なAzureマネージドサービス」** を問う問題であると理解できます。

---

## 3. 技術の基本概念

Azureには複数のデータベースサービスが存在し、それぞれ目的が異なります。SQL Server移行の観点から、主要なサービスを整理します。

### Azure SQL Database

Azure SQL Database は PaaS 型のリレーショナルデータベースサービスです。  
単一データベース単位で提供されるため、完全なSQL Server機能との互換性が必要な場合には制限があります。

主な特徴として以下があります。

- 自動バックアップ
- 自動パッチ適用
- 高可用性
- Hyperscale などの高いスケーラビリティ

ただし **SQL Serverインスタンスレベル機能の互換性が完全ではない**場合があります。

---

### Azure SQL Managed Instance

Azure SQL Managed Instance は **SQL Serverとの互換性を重視したPaaSサービス**です。  
オンプレミスSQL Serverの移行を最もスムーズに行えるよう設計されています。

主な特徴として以下が挙げられます。

- SQL Serverとの高い互換性
- インスタンスレベル機能のサポート
- 組み込み高可用性
- 仮想ネットワーク内への配置
- TDEなどの高度なセキュリティ機能

そのため **SQL Serverリフト＆シフト移行の第一候補**となります。

---

### Azure Cosmos DB

Azure Cosmos DB は **NoSQLデータベース**であり、グローバル分散や低レイテンシを特徴とします。

主に次のような用途に適しています。

- グローバルアプリケーション
- 大規模分散データ
- JSONドキュメントやグラフデータ

ただし **SQL Serverワークロードの移行には適していません**。

---

### Azure Database for PostgreSQL

PostgreSQLはSQL Serverとは異なるデータベースエンジンです。

そのため以下の問題が発生します。

- SQL構文の互換性問題
- アプリケーション変更が必要
- ストアドプロシージャの再設計

このため **SQL Server移行には通常選択されません**。

---

## 4. アーキテクチャまたは設計のポイント

この問題の設計ポイントは「SQL Serverワークロードを最小変更でクラウドへ移行すること」です。  
そのためには **SQL Server互換性・高可用性・セキュリティのバランス**が重要になります。

Azure SQL Managed Instance はこれらを満たすために設計されています。

具体的には次の特徴があります。

- 組み込み高可用性（99.99% SLA）
- SQL Serverとの高い互換性
- 仮想ネットワーク統合
- TDEによる保存時暗号化
- TLSによる通信暗号化
- vCoreベースのスケーリング

この構成により、オンプレミス環境とほぼ同等の運用モデルをクラウドで実現できます。

---

## 5. 設計判断（なぜこの構成になるか）

本シナリオでは **既存SQL Serverワークロードの移行**が前提となっています。  
そのため、最も重要な要素は「互換性」と「移行の容易さ」です。

Azure SQL Managed Instance が最適である理由は次の通りです。

まず、高可用性の観点では **サービス内で冗長構成が自動的に提供される**ため、システム停止のリスクを大幅に減らすことができます。

次に、SQL Serverとの互換性が非常に高いため、次のような既存資産をそのまま利用できます。

- SQLクエリ
- ストアドプロシージャ
- SQL Server Agent
- CLR
- Linked Server

さらにセキュリティ面では以下の機能が標準で利用できます。

- TDE（保存時暗号化）
- TLS（通信暗号化）
- VNet統合
- Privateアクセス

これにより **企業向けのセキュリティ要件にも対応可能**です。

---

## 6. 他の選択肢が誤りな理由

この問題では、他の選択肢がなぜ適さないかを理解することも重要です。

### A Azure SQL Database Hyperscale

Azure SQL Database Hyperscale はスケーラブルなデータベースですが、次の問題があります。

- SQL Serverインスタンス機能が完全互換ではない
- 一部のSQL Server機能が利用できない

そのため **既存SQL Serverアプリケーションの移行には制約があります。**

---

### B Azure Database for PostgreSQL

PostgreSQLはSQL Serverとは異なるデータベースエンジンです。

その結果として次の問題が発生します。

- SQL構文の違い
- アプリケーション修正が必要
- 移行コスト増加

したがって **SQL Server移行には不適切です。**

---

### C Azure Cosmos DB

Cosmos DBはNoSQLデータベースであり、用途が大きく異なります。

主な違いは以下です。

- SQL Serverワークロードではない
- RDBMSではない
- トランザクション構造が異なる

そのため **SQL Serverベースの複雑なクエリには適していません。**

---

## 7. 最終回答

D. **Azure SQL Managed Instance**

---

## 8. まとめ

SQL ServerワークロードをAzureへ移行する場合、最も重要なのは **互換性・可用性・セキュリティ・移行の容易さ**です。

Azure SQL Managed Instance はこれらをすべて満たすサービスであり、特に次の特徴が重要です。

- SQL Serverとの高い互換性
- 組み込み高可用性
- VNet統合によるセキュリティ
- TDE / TLSによる暗号化
- クラウドスケーリング

そのため **SQL Serverのクラウド移行シナリオでは最も推奨されるAzureデータベースサービス**となります。