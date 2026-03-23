[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Synapse Analytics]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Share]]
## Azure Synapse Pipelines と Azure Data Share を組み合わせたデータ統合・共有アーキテクチャ

### 1 概要

本シナリオでは、ApexCore Consulting が Azure 上のデータ分析基盤を構築し、パートナー企業 IrondClad Inc. とデータ連携を行う。  
このアーキテクチャでは、2つの主要な機能が必要になる。

1. **パートナー環境からデータを取り込み、変換して Data Lake に保存する**
    
2. **分析結果のデータスナップショットをパートナーに共有する**
    

この2つの要件を満たすために次の Azure サービスを組み合わせる。

- **Azure Synapse Pipelines**
    
    - データ取り込み・変換（ETL）
        
- **Azure Data Share**
    
    - 組織間データ共有
        

この組み合わせにより、データの統合・分析・安全な共有を実現できる。

---

# 2 背景

ApexCore の環境には次のリソースがある。

|リソース|用途|
|---|---|
|Azure Synapse Analytics (apexcoreworkspace1)|データ分析|
|Azure Data Lake Storage (apexcorelake1)|データ保存|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
|Azure SQL Database (apexcoresql1)|製品データ|

現在のデータフロー

```text
Azure SQL Database
        │
        ▼
Data Lake Storage
        │
        ▼
Synapse Analytics
```

一方、パートナー企業 Ironclad は次の環境を持つ。

|リソース|用途|
|---|---|
|IroncladVM1|SQL Server 2019|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Storage Account]]
|ironcladsa1|Azure Storage|

この環境から研究データを ApexCore 側の Data Lake に取り込み、分析する必要がある。

---

# 3 サービスの仕組み

## 3.1 Synapse Pipelines（データ統合）

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Factory]]
Synapse Pipelines は Azure Data Factory と同じエンジンを利用する ETL オーケストレーション機能である。

役割

- データコピー
    
- データ変換
    
- データワークフロー管理
    

データ取り込みの流れ

```text
IroncladVM1 (SQL Server)
        │
        ▼
Synapse Pipeline
        │
        │ データ変換
        ▼
Azure Data Lake Storage
        │
        ▼
Synapse Analytics
```

このパイプラインは以下を実行する。

- SQL Server からデータ取得
    
- ApexCore 形式へ変換
    
- Data Lake に保存
    

---

## 3.2 Azure Data Share（データ共有）

Azure Data Share は、Azure サブスクリプションや組織間で安全にデータを共有するサービスである。

特徴

- データスナップショット共有
    
- アクセス制御
    
- 自動更新
    

共有の仕組み

```text
ApexCore
Data Lake / Synapse
        │
        ▼
Azure Data Share
        │
        ▼
Ironclad Storage
```

この仕組みにより

- ApexCore の分析データ
    
- Synapse の結果
    

を Ironclad に安全に提供できる。

---

# 4 主要機能

## 4.1 Synapse Pipelines

### Copy Activity

SQL Server など多くのソースからデータをコピーできる。

```text
SQL Server → Data Lake
```

### Mapping Data Flow

データ形式変換

例

```text
CSV → Parquet
JSON → Structured Table
```

### スケジュール実行

```text
Daily
Hourly
Event Trigger
```

---

## 4.2 Azure Data Share

### データスナップショット共有

```text
Producer
   │
   ▼
Snapshot
   │
   ▼
Consumer
```

### アクセス制御

共有対象を制限できる。

- サブスクリプション
    
- ストレージ
    
- パートナー
    

### 自動同期

定期的にデータ更新可能。

---

# 5 関連Azureサービス

|サービス|役割|
|---|---|
|Azure Synapse Analytics|データ分析|
|Synapse Pipelines|データ統合|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Lake Storage Gen2]]
|Azure Data Lake Storage Gen2|データ保存|
|Azure Data Share|組織間共有|
|SQL Server|データソース|

統合構成

```text
Ironclad SQL Server
       │
       ▼
Synapse Pipelines
       │
       ▼
Data Lake Storage
       │
       ▼
Synapse Analytics
       │
       ▼
Azure Data Share
       │
       ▼
Ironclad Storage
```

---

# 6 アーキテクチャ

全体アーキテクチャ

```text
Ironclad Subscription
---------------------

IroncladVM1
(SQL Server)
       │
       ▼
Synapse Pipeline
       │
       │ Transform
       ▼

ApexCore Subscription
---------------------

Data Lake Storage
(apexcorelake1)
       │
       ▼
Synapse Analytics
(apexcoreworkspace1)
       │
       ▼
Azure Data Share
       │
       ▼
Ironclad Storage (ironcladsa1)
```

この構成により

- データ統合
    
- データ分析
    
- パートナー共有
    

を実現できる。

---

# 7 他サービスとの違い

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Box]]
### Azure Data Box Gateway

用途

- PB級データ移行
    
- オフライン転送
    

ETL処理には不向き。

---

### Azure Synapse Pipelines

用途

- データ取り込み
    
- ETL変換
    
- データ統合
    

---

### Azure Data Share

用途

- 組織間データ共有
    
- スナップショット提供
    

---

# 8 ユースケース

### パートナー企業とのデータ共有

```text
Enterprise Data Lake
       │
       ▼
Azure Data Share
       │
       ▼
Partner Storage
```

### データ統合基盤

```text
Data Sources
      │
      ▼
Synapse Pipelines
      │
      ▼
Data Lake
```

### 分析基盤

```text
Data Lake
      │
      ▼
Synapse Analytics
      │
      ▼
Power BI
```

---

# 9 設計指針

Azureデータ基盤の設計では次の役割分担が一般的。

|サービス|役割|
|---|---|
|Synapse Pipelines|ETL|
|Data Lake|データ保存|
|Synapse|SQL分析|
|Data Share|組織間共有|

覚え方

```text
Move → Store → Analyze → Share
```

---

# 10 まとめ

本シナリオでは2つの要件がある。

1. **Ironclad の研究データを取り込み変換する**
    
2. **分析データのスナップショットを共有する**
    

推奨アーキテクチャ

```text
SQL Server
   │
   ▼
Synapse Pipelines
   │
   ▼
Data Lake Storage
   │
   ▼
Synapse Analytics
   │
   ▼
Azure Data Share
```

この構成により

- データ統合
    
- データ変換
    
- 分析
    
- 安全な共有
    

を効率的に実現できる。