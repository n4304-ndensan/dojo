---
分類: Database
tags:
  - cloud/azure
  - cloud/azure/sql-managed-instance
  - cloud/azure/sql-virtual-machine
  - cloud/architecture/database-migration
  - cloud/architecture/sql-server
  - database/relational
  - database/cross-database-query
  - exam/azure
---

# SQL Server データベース間クエリを維持した Azure 移行

## 1. 背景（シナリオ）

ある企業では、オンプレミス環境で稼働しているアプリケーションが複数の **SQL Server データベース**と連携しています。

このアプリケーションでは、各データベースに対して **完全修飾名（Fully Qualified Name）**を使用してアクセスしています。例えば次のような形式です。

Server.Database.Schema.Table

または

Database.Schema.Table

この構成では、複数のデータベース間でクエリを実行する **クロスデータベースクエリ**が使用されています。

企業はこのアプリケーションを Azure に移行する予定ですが、移行後も **データベース間クエリ機能を維持することが重要な要件**となっています。

## 2. 要件整理

問題文から読み取れる重要なポイントを整理すると次の通りです。

まず、既存アプリケーションは SQL Server データベースを使用しています。そのため、移行先のデータベースサービスは SQL Server と高い互換性を持つ必要があります。

次に、アプリケーションは複数のデータベースを参照するクロスデータベースクエリを使用しています。

さらに、アプリケーションコードを大きく変更せずに移行することが望ましい状況です。

これらの条件を整理すると、次のようになります。

SQL Server 互換性  
クロスデータベースクエリのサポート  
既存アプリケーションとの互換性  
最小限のアプリケーション変更  

この条件を満たす Azure サービスを選択する必要があります。

## 3. 技術の基本概念

SQL Server 環境では、同一インスタンス内に複数のデータベースを配置し、別のデータベースのテーブルを直接参照することができます。

例えば次のようなクエリです。

SELECT * FROM SalesDB.dbo.Orders

このような **クロスデータベースクエリ**は SQL Server の標準機能です。

Azure に移行する際には、この機能を維持できるサービスを選択する必要があります。

Azure には SQL Server ワークロードを実行できる複数のサービスがありますが、それぞれ互換性が異なります。

## 4. アーキテクチャまたは設計のポイント

このシナリオでは、SQL Server のネイティブ機能を維持できるサービスを選択する必要があります。

そのため、次の2つのサービスが適しています。

Azure SQL Managed Instance  
Azure Virtual Machines 上の SQL Server  

Azure SQL Managed Instance は SQL Server とほぼ完全互換の PaaS サービスです。

一方、Azure VM 上の SQL Server は IaaS 環境であり、オンプレミスと同じ SQL Server インスタンスを実行できます。

どちらも **同一インスタンス内の複数データベース参照**をサポートしているため、既存のクロスデータベースクエリがそのまま動作します。

## 5. 設計判断（なぜこの構成になるか）

Azure SQL Managed Instance は SQL Server との高い互換性を提供するマネージドサービスです。

このサービスでは次のような利点があります。

まず、クロスデータベースクエリがネイティブにサポートされています。

次に、バックアップやパッチ管理などの運用作業が Azure によって自動化されます。

さらに、オンプレミス SQL Server との互換性が高いため、アプリケーション変更を最小限に抑えられます。

一方、Azure Virtual Machines 上の SQL Server は完全な SQL Server 環境を提供するため、すべての SQL Server 機能を利用できます。

このため、既存アプリケーションをそのまま移行することが可能です。

## 6. 他の選択肢が誤りな理由

### B Azure Database for PostgreSQL

PostgreSQL は SQL Server とは異なるデータベースエンジンです。

T-SQL や SQL Server 固有の機能をサポートしていないため、既存アプリケーションの変更が必要になります。

### D Azure Cosmos DB

Azure Cosmos DB は NoSQL データベースサービスです。

リレーショナルデータベースや SQL Server の機能とは互換性がないため、データモデルやアプリケーションの大幅な変更が必要になります。

## 7. 最終回答

A. **Azure SQL Managed Instance**  
C. **SQL Server on Azure Virtual Machines**

## 8. まとめ

SQL Server データベースを Azure に移行する際に、クロスデータベースクエリを維持する必要がある場合は、SQL Server 互換性の高いサービスを選択する必要があります。

Azure SQL Managed Instance は PaaS 型で高い互換性を提供し、Azure VM 上の SQL Server は完全な SQL Server 環境を提供します。

試験では **「クロスデータベースクエリ」「完全修飾名」「SQL Server 移行」** といったキーワードが出てきた場合、この2つが正解になるケースが多いです。