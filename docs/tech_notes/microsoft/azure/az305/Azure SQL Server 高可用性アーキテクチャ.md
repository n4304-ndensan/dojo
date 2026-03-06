以下はあなたのドキュメントに **フェイルオーバーグループ（Azure SQL Database）** の内容を整理して組み込んだ **完全版**です。  
重要なのは **SQL Server on VM と Azure SQL Database の高可用性方式が違う**ことなので、その構造が分かるように整理しています。

---

# Azure SQL 高可用性アーキテクチャ

（SQL Server on Azure VM / Azure SQL Database）

---

# 1 概要

AzureでSQLデータベースの高可用性を設計する場合、  
**利用しているSQLの種類によって採用する技術が異なる。**

主な2つの構成

|環境|高可用性ソリューション|
|---|---|
|SQL Server on Azure VM|Always On Availability Groups|
|Azure SQL Database|Auto Failover Group|

つまり

```text
SQL Server on VM → Always On AG
Azure SQL Database → Failover Group
```

となる。

---

# 2 背景

企業のデータベースには次の要件がある。

- 高可用性（High Availability）
    
- 災害復旧（Disaster Recovery）
    
- データ保護（Backup）
    

障害原因

- VM障害
    
- OS障害
    
- SQL Server障害
    
- リージョン障害
    

単一構成

```text
Application
    │
    ▼
SQL Server
```

問題

- 単一障害点
    
- サービス停止
    
- データ損失
    

そのため

```text
レプリケーション + フェイルオーバー
```

の構成が必要になる。

---

# 3 SQL Server on VM の高可用性

（Always On Availability Groups）

## 概要

Always On Availability Groups（AG）は

**SQL Serverの高可用性機能**

である。

機能

- データレプリケーション
    
- 自動フェイルオーバー
    
- 複数レプリカ
    

構成

```text
Primary SQL Server VM
        │
        ▼
Always On Availability Group
        │
        ▼
Secondary SQL Server VM
```

---

## フェイルオーバー

障害発生時

```text
Primary Failure
      │
      ▼
Automatic Failover
      │
      ▼
Secondary becomes Primary
```

メリット

- ダウンタイム最小化
    
- データ同期
    
- 高可用性
    

---

## Always On AG の特徴

|機能|説明|
|---|---|
|自動フェイルオーバー|プライマリ障害時|
|データ同期|同期レプリケーション|
|複数レプリカ|最大8|
|読み取りスケール|セカンダリで読み取り|

---

# 4 Azure SQL Database の高可用性

（Auto Failover Group）

Azure SQL Databaseでは

**Auto Failover Group**

を使用する。

構成

```text
Primary SQL Server
      │
      ▼
Replication
      │
      ▼
Secondary SQL Server
```

特徴

- 複数データベースをグループ化
    
- 自動フェイルオーバー
    
- リージョン間レプリケーション
    

---

## フェイルオーバーグループ構成

```text
Application
     │
     ▼
Failover Group Endpoint
     │
     ├ Primary Server
     └ Secondary Server
```

アプリケーションは

**1つの接続エンドポイント**

を使用する。

---

# 5 フェイルオーバーグループの特徴

|機能|説明|
|---|---|
|自動フェイルオーバー|プライマリ障害時|
|複数DB管理|グループ単位|
|読み取りエンドポイント|セカンダリ読み取り|
|リージョンDR|対応|

---

# 6 フェイルオーバーグループの重要ポイント

試験でよく問われるポイント

### ① セカンダリサーバーは別リージョン

理由

リージョン障害対策

```text
Primary Region
      │
      ▼
Secondary Region
```

---

### ② 読み取り専用

セカンダリは

```text
Read Only
```

である。

書き込みは

```text
Primary only
```

---

### ③ 自動フェイルオーバー

```text
Primary Failure
      │
      ▼
Secondary promoted
```

---

### ④ 最大5秒のデータ損失

レプリケーションは

**非同期**

のため

```text
最大5秒のデータロス
```

が発生する可能性がある。

---

# 7 他の選択肢との違い

## Long-term Backup Retention

```text
Backup Storage
```

用途

- コンプライアンス
    
- 長期保存
    

高可用性ではない。

---

## Active Geo Replication

```text
Primary DB
      │
      ▼
Secondary DB
```

用途

- 災害復旧
    
- 手動フェイルオーバー
    

Failover Groupの方が

- 管理が簡単
    
- グループ単位
    

---

# 8 アーキテクチャ

## SQL Server on VM

```text
Application
   │
   ▼
Load Balancer
   │
   ├ SQL VM 1 (Primary)
   └ SQL VM 2 (Secondary)
        │
        ▼
Always On AG
```

---

## Azure SQL Database

```text
Application
   │
   ▼
Failover Group Endpoint
   │
   ├ Primary SQL Server
   └ Secondary SQL Server
```

---

# 9 設計指針

Azure SQL高可用性は

**SQLの種類で決まる。**

|環境|HAソリューション|
|---|---|
|SQL Server on VM|Always On AG|
|Azure SQL Database|Failover Group|
|Azure SQL DB DR|Geo Replication|

---

# 10 まとめ

Azure SQLの高可用性設計

|環境|方式|
|---|---|
|SQL Server on VM|Always On Availability Groups|
|Azure SQL Database|Auto Failover Group|

フェイルオーバーグループの特徴

- セカンダリは別リージョン
    
- 読み取り専用
    
- 自動フェイルオーバー
    
- 最大5秒のデータロス
    

---

もし希望あれば、試験で頻出の

**Azure SQL 高可用性まとめ（これ1枚で解ける版）**

を作ります。  
実はこの分野 **出題パターンがほぼ固定**です。