[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Managed Instance]]
## SQL Server → Azure SQL Managed Instance 移行方式ガイド

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Database Migration Service]]
### （Azure Database Migration Service と Azure Data Studio を含む統合ドキュメント）

---

# 1 概要

企業がオンプレミスの SQL Server を Azure へ移行する際、最も一般的な移行先の一つが **Azure SQL Managed Instance** である。  
Azure SQL Managed Instance は SQL Server と高い互換性を持つ PaaS データベースサービスであり、既存アプリケーションを大きく変更せずにクラウドへ移行できる。

今回のシナリオでは次の条件が与えられている。

- オンプレミス SQL Server 2008
    
- 単一データベース（約 50GB）
    
- 移行先：Azure SQL Managed Instance
    
- **サービス停止を最小限にする必要がある**
    

この条件では

**Azure Data Studio を使用したオンライン移行**

が最も適した方法となる。

一方、前の問題では

- 約 50 データベース
    
- オフライン移行
    
- 管理オーバーヘッド最小
    

という条件だったため

**Azure Database Migration Service**

が最適解であった。

このドキュメントでは両者を含めて **Azure SQL 移行方式全体を整理**する。

---

# 2 Azure SQL 移行アーキテクチャ

オンプレミス SQL Server を Azure に移行する典型構成は次の通りである。

```text
On-Premises Datacenter
----------------------

SQL Server
     │
     ▼
Migration Tool
     │
     ▼
Azure SQL Managed Instance
```

移行ツールは複数存在し、要件に応じて選択する。

---

# 3 移行方式の種類

SQL Server のクラウド移行には主に **2種類の移行方式**がある。

|移行方式|特徴|
|---|---|
|Offline Migration|移行中アプリ停止|
|Online Migration|ダウンタイム最小|

---

## 3.1 Offline Migration（オフライン移行）

オフライン移行では、データベースを停止してからコピーする。

```text
Application
    │
Stop application
    │
Copy database
    │
Start application
```

特徴

- 構成がシンプル
    
- 移行速度が速い
    
- ダウンタイムが発生
    

適しているケース

- 小規模DB
    
- 業務停止が許容される
    

---

## 3.2 Online Migration（オンライン移行）

オンライン移行では、データを同期しながら移行する。

```text
Source SQL Server
        │
Initial Data Copy
        │
Continuous Sync
        │
Cutover
        │
Azure SQL
```

特徴

- ダウンタイムが非常に短い
    
- 移行が複雑
    
- 同期プロセスが必要
    

適しているケース

- 本番環境
    
- 高可用システム
    
- 大規模サービス
    

---

# 4 Azure Database Migration Service（DMS）

Azure Database Migration Service は Azure が提供する **フルマネージド移行サービス**である。

主な用途

- SQL Server → Azure SQL
    
- SQL Server → Managed Instance
    
- Oracle → Azure
    
- MySQL → Azure
    

---

## 4.1 DMS のアーキテクチャ

```text
On-Prem SQL Server
       │
       ▼
Azure Database Migration Service
       │
       ▼
Azure SQL Managed Instance
```

DMS は

- データベース検出
    
- スキーマ移行
    
- データコピー
    

を自動化する。

---

## 4.2 DMS が適するケース

DMS は次のようなケースに最適。

```text
Large SQL Server
      │
      ├ DB1
      ├ DB2
      ├ DB3
      └ DB50
```

複数DBを一括移行できる。

メリット

- 管理オーバーヘッド最小
    
- Azure Portal 管理
    
- バッチ移行
    

---

# 5 Azure Data Studio 移行

Azure Data Studio は Microsoft が提供するデータベース開発ツールであり、**SQL Server から Azure SQL へのオンライン移行機能**を持つ。

拡張機能

```
Azure SQL Migration Extension
```

を利用する。

---

## 5.1 Azure Data Studio 移行アーキテクチャ

```text
SQL Server
   │
   ▼
Azure Data Studio
Migration Extension
   │
   ▼
Azure SQL Managed Instance
```

---

## 5.2 Azure Data Studio の特徴

Azure Data Studio の移行機能は

- オンライン移行
    
- データ同期
    
- カットオーバー
    

をサポートする。

構造

```text
Source Database
       │
Initial copy
       │
Continuous replication
       │
Cutover
       │
Target Database
```

これにより

**ダウンタイムを最小化できる。**

---

# 6 他ツールとの違い

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Migrate]]
## Azure Migrate

Azure Migrate は

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
- VM移行
    
- インフラ評価
    

のツール。

```text
Server Migration
```

用途であり

**データベース移行専用ではない。**

---

## SQL Server Management Studio（SSMS）

SSMS は

- DBA管理ツール
    
- バックアップ
    
- 手動移行
    

が可能。

しかし

```text
自動移行機能なし
```

管理負荷が大きい。

---

## WANdisco LiveData

WANdisco は

- 商用レプリケーションツール
    
- 大規模データ同期
    

しかし

- Azure標準ツールではない
    
- コスト高
    

---

# 7 移行ツール比較

|ツール|特徴|
|---|---|
|Azure Database Migration Service|複数DB移行|
|Azure Data Studio|オンライン移行|
|DMA|互換性チェック|
|SSMA|他DB → SQL|

---

# 8 典型的な Azure 移行アーキテクチャ

```text
On-Prem Datacenter
------------------

SQL Server
   │
   ▼
Migration Tool
(DMS / Azure Data Studio)
   │
   ▼
Azure SQL Managed Instance
   │
   ▼
Applications
```

---

# 9 Azure Architect 設計指針

Azure Architect が移行ツールを選択する際の基準

|条件|推奨ツール|
|---|---|
|多DB移行|DMS|
|ダウンタイム最小|Azure Data Studio|
|互換性チェック|DMA|
|Oracle移行|SSMA|

---

# 10 試験ポイント（AZ-305）

AZ-305 試験では次の判断が重要。

### DB数が多い

```
Azure Database Migration Service
```

### ダウンタイム最小

```
Azure Data Studio
```

---

# 11 まとめ

SQL Server を Azure SQL Managed Instance に移行する方法には複数の選択肢がある。

今回の2つの問題を整理すると

|条件|推奨|
|---|---|
|50DB オフライン移行|Azure Database Migration Service|
|単一DB ダウンタイム最小|Azure Data Studio|

アーキテクチャ

```text
On-Prem SQL Server
       │
       ▼
Migration Tool
(DMS / Azure Data Studio)
       │
       ▼
Azure SQL Managed Instance
```

Azure Data Studio はオンライン移行に適しており、  
Azure Database Migration Service は大規模移行に適したツールである。