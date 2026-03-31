---
分類: AzureSQL
tags:
  - cloud/azure
  - cloud/azure/sql-database
  - cloud/database/relational
  - cloud/database/backup
  - cloud/database/point-in-time-restore
  - cloud/devops/database-development
  - exam/azure
---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Point-in-Time Restore (PITR)]]
# Azure SQL Database のポイントインタイム復元（PITR）

## 1. 背景（シナリオ）

アプリケーション開発では、データベースのスキーマ変更が頻繁に発生します。特に開発段階では、テーブル構造の変更、インデックスの追加、データ型の変更などが何度も行われます。

このような環境では、開発者が誤った変更を行う可能性があります。例えば、次のような問題が発生することがあります。

あるスキーマ変更によってアプリケーションが正常に動作しなくなる場合があります。また、誤って重要なデータを削除してしまう可能性もあります。

そのため、開発環境では **データベースを簡単に以前の状態へ戻せる仕組み**が重要になります。

今回のシナリオでは、開発チームが使用するデータベースサービスに次の要件があります。

まず、データベースは **リレーショナルデータベース**である必要があります。

次に、開発中はスキーマ変更が頻繁に行われます。

さらに、データベース全体を **以前の時点に簡単にロールバックできる必要があります。**

そして、運用負荷を減らすために **完全に管理されたデータベースサービス**である必要があります。

このような要件を満たす Azure サービスを選択する必要があります。

---

## 2. 要件整理

問題の要件を整理すると、次のポイントが重要になります。

まず、使用するデータベースは **リレーショナルデータベース**です。

次に、スキーマ変更が頻繁に発生するため、開発者が簡単に元の状態へ戻せる必要があります。

また、データベース全体を特定の時点へ戻せる機能が必要です。

さらに、インフラの管理を行う必要がない **完全マネージドサービス**が求められています。

このような要件を満たす Azure のサービスが **Azure SQL Database の Point-in-Time Restore（PITR）**です。

---

## 3. Azure SQL Database の概要

Azure SQL Database は Microsoft が提供する **完全マネージド型のリレーショナルデータベースサービス**です。

従来の SQL Server と互換性があり、クラウド上で自動管理されます。

Azure SQL Database では次のような管理作業が自動化されています。

- バックアップ
- パッチ適用
- 高可用性
- ストレージ管理
- 障害復旧

そのため、開発者はインフラ管理ではなくアプリケーション開発に集中できます。

---

## 4. Point-in-Time Restore（PITR）

Azure SQL Database の重要な機能の一つが **Point-in-Time Restore（PITR）**です。

PITR とは、データベースを **特定の過去の時点へ復元できる機能**です。

Azure SQL Database では自動バックアップが実行されます。これには次の種類があります。

- フルバックアップ  
- 差分バックアップ  
- トランザクションログバックアップ  

これらのバックアップを組み合わせることで、データベースを **任意の時点に復元**できます。

例えば、午後 3 時にスキーマ変更を行った場合、午後 2 時 59 分の状態へ戻すことができます。

---

## 5. 開発環境におけるメリット

開発環境では、PITR は非常に便利な機能です。

例えば、開発者が誤ったスキーマ変更を実行した場合でも、すぐに以前の状態へ戻すことができます。

また、アプリケーションのテスト中にデータ破損が発生した場合でも、数分で復元できます。

さらに、PITR を使用すると、過去の状態の **データベースコピー**を作成することもできます。

このコピーを使用して、次のような用途に利用できます。

- 回帰テスト
- データ比較
- バグ再現

---

## 6. バックアップ保持期間

Azure SQL Database の PITR は、バックアップ保持期間内で利用できます。

通常、バックアップ保持期間は **7日から35日**の範囲で設定できます。

この期間内であれば、任意の時点へデータベースを復元できます。

また、長期保持が必要な場合は **Long-Term Retention（LTR）バックアップ**を利用することもできます。

---

## 7. 他の選択肢が適さない理由

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Cosmos DB]]
### Azure Cosmos DB

Cosmos DB は NoSQL データベースであり、リレーショナルデータベースではありません。

そのため、リレーショナルスキーマの管理やロールバックには適していません。

---

### Azure Database for MySQL

MySQL もマネージドサービスですが、この問題の文脈では Azure SQL Database の PITR が最も典型的な解決策になります。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
### SQL Server on Azure VM

Azure VM 上の SQL Server は IaaS サービスです。

バックアップや復元はユーザーが管理する必要があるため、完全マネージドサービスではありません。

---

## 8. 最終回答

A. Azure SQL Database with Point-in-Time Restore