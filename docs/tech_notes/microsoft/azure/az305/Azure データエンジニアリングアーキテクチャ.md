# Azure データエンジニアリングアーキテクチャ

（Data Lake + Synapse + Databricks + Data Factory）

## 1 概要

企業がオンプレミスの **Microsoft SQL Server** に保存しているトランザクションデータをクラウドへ移行し、分析基盤を構築する場合、Azureでは **モダンデータプラットフォームアーキテクチャ** を採用する。

この問題の要件は次の4つである。

1. オンプレミス SQL Server → Azure データウェアハウスへ移行
    
2. 夜間スケジュールでデータ転送
    
3. Spark クラスターでデータ検査（Scala / R / Python）
    
4. 様々なソースのデータを保存するデータレイク
    

これらを満たす典型的な Azure アーキテクチャは以下になる。

```text
On-Prem SQL Server
        │
        ▼
Azure Data Factory
        │
        ▼
Data Lake Storage Gen2
        │
        ▼
Azure Synapse Analytics
        │
        ▼
Azure Databricks
```

この構成により、データの取り込み、保存、分析、処理を統合した **クラウドデータプラットフォーム** を構築できる。

---

# 2 背景

企業では次のようなデータ活用ニーズがある。

- トランザクションデータの分析
    
- ビッグデータ処理
    
- 機械学習
    
- データサイエンス
    

従来のオンプレミスデータウェアハウスでは次の問題がある。

- スケーラビリティ不足
    
- インフラ管理コスト
    
- データ統合の難しさ
    

クラウドでは

**データレイク + データウェアハウス**

を組み合わせたアーキテクチャが主流となっている。

---

# 3 モダンデータアーキテクチャ

Azureの分析基盤は一般的に次の構造になる。

```text
Data Sources
   │
   ▼
Data Ingestion
   │
   ▼
Data Lake
   │
   ▼
Data Processing
   │
   ▼
Data Warehouse
   │
   ▼
Analytics
```

Azureサービスとの対応

| 層               | サービス                    |
| --------------- | ----------------------- |
| Data Ingestion  | Azure Data Factory      |
| Data Lake       | Data Lake Storage Gen2  |
| Data Processing | Azure Databricks        |
| Data Warehouse  | Azure Synapse Analytics |

---

# 4 Azure Synapse Analytics

## 概要

**Azure Synapse Analytics** は、Azureのデータウェアハウスサービスである。

旧名称

```
Azure SQL Data Warehouse
```

特徴

- 大規模データ分析
    
- 分散クエリエンジン
    
- ビッグデータ統合
    

基本構造

```text
Data Lake
   │
   ▼
Synapse Analytics
   │
   ▼
BI / Analytics
```

---

## 主な機能

|機能|説明|
|---|---|
|MPP (Massively Parallel Processing)|分散処理|
|SQL Analytics|SQLベース分析|
|Spark Analytics|Spark分析|
|Data Integration|ETL|

---

## 利用用途

- データウェアハウス
    
- BI分析
    
- ビッグデータ分析
    

---

# 5 Azure Data Factory

## 概要

**Azure Data Factory (ADF)** はデータ統合サービスである。

役割

- データ移動
    
- データ変換
    
- パイプライン管理
    

この問題の要件

```
夜間にスケジュールされたデータ転送
```

はADFで実現できる。

構成

```text
SQL Server
   │
   ▼
Data Factory Pipeline
   │
   ▼
Azure Synapse
```

---

# 6 Azure Data Lake Storage Gen2

## 概要

Data Lake Storage Gen2 は

**ビッグデータ保存用ストレージ**

である。

特徴

- 大規模データ保存
    
- 階層型名前空間
    
- Hadoop互換
    

構成

```text
Data Sources
   │
   ▼
Data Lake Storage
```

用途

- Raw Data保存
    
- ETL処理
    
- データ統合
    

---

# 7 Azure Databricks

## 概要

Azure Databricksは

**Apache Sparkベースの分析プラットフォーム**

である。

特徴

- 分散データ処理
    
- Notebook
    
- ML処理
    

対応言語

|言語|対応|
|---|---|
|Python|○|
|Scala|○|
|R|○|
|SQL|○|

構成

```text
Data Lake
   │
   ▼
Databricks Spark Cluster
   │
   ▼
Data Analysis
```

---

# 8 アーキテクチャ

問題の要件を満たす構成

```text
On-Prem SQL Server
       │
       ▼
Azure Data Factory
       │
       ▼
Data Lake Storage Gen2
       │
       ▼
Azure Synapse Analytics
       │
       ▼
Azure Databricks
```

機能

|コンポーネント|役割|
|---|---|
|Data Factory|データ転送|
|Data Lake|データ保存|
|Synapse|データウェアハウス|
|Databricks|Spark分析|

---

# 9 問題シナリオの整理

要件とAzureサービスの対応

|要件|Azureサービス|
|---|---|
|SQL Server → Azure移行|Data Factory|
|夜間スケジュール|Data Factory|
|Sparkノートブック|Databricks|
|データレイク|Data Lake Storage|
|データウェアハウス|Synapse Analytics|

---

# 10 不正解の選択肢

### Azure Data Factory

役割

```
ETL / Data Pipeline
```

データウェアハウスではない。

---

### Azure Databricks

役割

```
Spark Data Processing
```

ストレージ / DWHではない。

---

### Data Lake Storage

役割

```
Raw Data Storage
```

分析用データウェアハウスではない。

---

# 11 設計指針

アーキテクトは次の構造でデータ基盤を設計する。

```text
Data Ingestion
      │
      ▼
Data Lake
      │
      ▼
Data Processing
      │
      ▼
Data Warehouse
```

Azureサービス

|層|サービス|
|---|---|
|Ingestion|Data Factory|
|Storage|Data Lake|
|Processing|Databricks|
|Warehouse|Synapse|

---

