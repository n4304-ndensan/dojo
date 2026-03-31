# Azure への OLTP データベース移行アーキテクチャ設計

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
（Azure SQL Database Hyperscale）

---

# 1 背景

企業ではオンプレミス環境で稼働している **OLTP（Online Transaction Processing）データベース**を Azure に移行する計画を進めている。OLTP データベースは、日常業務のトランザクション処理を担う重要なシステムであり、以下の特徴を持つ。

- 高頻度の読み書き処理
    
- 多数の同時接続
    
- 低レイテンシ要求
    
- 高い可用性
    

今回の移行にあたり、クラウド環境でも同様の性能・可用性・拡張性を維持する必要がある。

---

# 2 要件整理

今回の問題では以下の要件が提示されている。

### ① OLTP ワークロード最適化

トランザクション処理に最適化されたデータベースが必要。

---

### ② スケールアップ / スケールダウン

ワークロードに応じて **コンピューティングリソースを柔軟に変更**できる必要がある。

---

### ③ ジオ冗長バックアップ

リージョン障害に備えて **Geo-redundant backups** が必要。

---

### ④ 最大 75TB のデータ処理

データベースは非常に大きくなる可能性があり、最大 **75TB** の容量に対応する必要がある。

---

# 3 Azure における SQL データベースサービス

Azure には複数の SQL データベースサービスが存在する。

|サービス|特徴|
|---|---|
|Azure SQL Database|完全 PaaS|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Managed Instance]]
|Azure SQL Managed Instance|SQL Server 互換 PaaS|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
|SQL Server on Azure VM|IaaS|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Synapse Analytics]]
|Azure Synapse Analytics|分析データベース|

今回の要件は **大規模 OLTP** であるため、分析データベースである Synapse は対象外となる。

---

# 4 Azure SQL Database Hyperscale

Azure SQL Database Hyperscale は、大規模 OLTP ワークロード向けに設計されたサービス階層である。

特徴

|機能|説明|
|---|---|
|最大容量|最大 100TB|
|高速スケール|数分でスケール変更|
|分離ストレージ|コンピュートとストレージ独立|
|自動バックアップ|Geo-redundant backups|
|高可用性|複数レプリカ|

今回の **75TB 要件**を満たすのは Hyperscale のみである。

---

# 5 Hyperscale アーキテクチャ

Hyperscale は従来の SQL Database とは異なるアーキテクチャを採用している。

```text
Client Applications
        │
        ▼
Compute Nodes
        │
        ▼
Log Service
        │
        ▼
Page Servers
        │
        ▼
Azure Storage
```

特徴

- コンピュートとストレージを分離
    
- スケールアウト可能
    
- 高速データアクセス
    

---

# 6 スケーリング機能

Hyperscale では **コンピュートとストレージを独立してスケール**できる。

### コンピュートスケール

```text
Low workload
     ↓
Scale down compute
```

```text
High workload
     ↓
Scale up compute
```

---

### ストレージスケール

データ容量は **自動的に拡張**される。

```text
Initial database
   1TB
     ↓
10TB
     ↓
75TB
```

---

# 7 ジオ冗長バックアップ

Azure SQL Database では自動バックアップが提供される。

バックアップの種類

|タイプ|説明|
|---|---|
|Local redundant backup|同一リージョン|
|Geo-redundant backup|別リージョン|

Hyperscale では **geo-redundant backup** を利用できるため、リージョン障害時にもデータを復元できる。

---

# 8 OLTP ワークロードへの最適化

Hyperscale は OLTP ワークロードに適した設計となっている。

特徴

- 高同時接続数
    
- 高速トランザクション処理
    
- 大容量データ対応
    
- スケールアウト読み取り
    

また **read scale-out** 機能により、読み取り専用レプリカを追加できる。

---

# 9 他の選択肢が不適切な理由

## Azure SQL Managed Instance (Business Critical)

Managed Instance は SQL Server 互換性が高いが、データ容量制限がある。

最大容量

```text
約 16TB
```

そのため **75TB 要件を満たさない**。

---

## SQL Server on Azure VM

IaaS の場合

- OS 管理必要
    
- パッチ管理
    
- バックアップ管理
    
- DR 設計
    

などの運用負荷が増える。

---

## Azure Synapse Analytics

Synapse は **OLAP（分析処理）**向けのデータベースであり、OLTP ワークロードには適していない。

---

## Azure SQL Database General Purpose

General Purpose は最大容量が小さく、大規模 OLTP には向かない。

---

# 10 推奨アーキテクチャ

今回の最適構成

```text
Applications
     │
     ▼
Azure SQL Database
(Hyperscale Tier)
     │
     ├─ Compute scaling
     ├─ Auto backups
     ├─ Geo-redundant backup
     └─ 100TB storage
```

---

# 11 最終回答

正解

**B**

```text
Azure SQL Database
Hyperscale
```

---

# 12 まとめ

今回の要件

|要件|解決策|
|---|---|
|75TB データ|Hyperscale|
|OLTP 最適化|Azure SQL Database|
|スケールアップ/ダウン|Compute scaling|
|ジオバックアップ|Geo-redundant backup|

そのため最適なサービスは

```text
Azure SQL Database
Hyperscale Tier
```

となる。

Hyperscale は Azure の **大規模 OLTP システム移行の標準アーキテクチャ**として設計されている。