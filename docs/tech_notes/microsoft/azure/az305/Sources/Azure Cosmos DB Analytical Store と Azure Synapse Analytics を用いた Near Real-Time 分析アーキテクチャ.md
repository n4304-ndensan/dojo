[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Cosmos DB]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Synapse Analytics]]
## Azure Cosmos DB Analytical Store と Azure Synapse Analytics を用いた Near Real-Time 分析アーキテクチャ

### ― Apache Spark Pool を利用したデータ処理設計 ―

---

# 1 概要

Azure Cosmos DB はグローバル分散型 NoSQL データベースサービスであり、トランザクション処理（OLTP）と分析処理（OLAP）の両方に対応できるように設計されている。Azure Cosmos DB には **Analytical Store（分析ストア）** という機能があり、運用データを分析用に最適化された形式で保存することができる。

今回のシナリオでは次の構成が与えられている。

- Azure Cosmos DB アカウント **account1**
    
- コンテナ **Contained**
    
- **Analytical Store が有効**
    
- Azure Synapse Analytics ワークスペース **Workspace1**
    

さらに次の要件がある。

- Cosmos DB に保存されたデータを **near real-time（NRT）で処理**
    
- 処理結果を **Synapse の Data Warehouse に書き込む**
    
- 処理は **Synapse ワークスペース内のランタイムエンジンを使用**
    
- **データ移動を最小限に抑える**
    

これらの条件を満たす最適な Synapse プールタイプは

**Apache Spark Pool**

である。

---

# 2 背景

従来、NoSQL データベースに格納されたデータを分析するためには、次のような ETL パイプラインが必要だった。

```text
Application
     │
     ▼
Operational Database
     │
     ▼
ETL Pipeline
     │
     ▼
Data Warehouse
```

この構成にはいくつかの問題がある。

- ETL 処理による **データコピー**
    
- データ同期の **遅延**
    
- データパイプラインの **管理負荷**
    

これを改善するために Microsoft は **HTAP（Hybrid Transactional Analytical Processing）** を実現する仕組みとして Cosmos DB Analytical Store を提供している。

---

# 3 Cosmos DB Analytical Store

Analytical Store は Cosmos DB のトランザクションデータを分析向け形式で保存するストレージ層である。

構造は次のようになる。

```text
Application
     │
     ▼
Cosmos DB
     │
     ├ Transactional Store
     │
     └ Analytical Store
```

特徴

- OLTP ワークロードに影響を与えない
    
- 列指向フォーマット
    
- 分析クエリに最適化
    

---

# 4 Azure Synapse Link

Cosmos DB Analytical Store は **Azure Synapse Link** によって Synapse と統合される。

Azure Synapse Link を使用すると

- ETL不要
    
- データコピー不要
    
- ほぼリアルタイム分析
    

が可能になる。

構造

```text
Cosmos DB
     │
     ▼
Analytical Store
     │
     ▼
Synapse Link
     │
     ▼
Azure Synapse Analytics
```

Synapse は Analytical Store に直接アクセスできる。

---

# 5 Synapse のプールタイプ

Azure Synapse Analytics には複数の処理エンジンが存在する。

|プールタイプ|用途|
|---|---|
|Serverless SQL|データレイク分析|
|Dedicated SQL|データウェアハウス|
|Apache Spark|データ処理|
|Data Explorer|ログ分析|

---

# 6 Apache Spark Pool

Apache Spark Pool は Synapse の分散処理エンジンであり、大規模データ処理や ETL ワークロードに適している。

Spark Pool の特徴

- 分散処理
    
- 大規模データ分析
    
- 機械学習
    
- ETL 処理
    

構造

```text
Data Source
     │
     ▼
Apache Spark
     │
     ▼
Processed Data
```

---

# 7 今回のアーキテクチャ

今回のシナリオでは次のデータフローが想定される。

```text
Application
     │
     ▼
Cosmos DB Container
(Contained)
     │
     ▼
Analytical Store
     │
     ▼
Azure Synapse Link
     │
     ▼
Apache Spark Pool
     │
     ▼
Synapse Data Warehouse
```

この構成では

- ETL パイプライン不要
    
- データコピー不要
    
- near real-time 処理
    

が実現できる。

---

# 8 なぜ Apache Spark Pool が最適なのか

今回の要件を整理すると次のようになる。

|要件|説明|
|---|---|
|NRT処理|分散処理が必要|
|データ移動最小|Synapse Link|
|Synapseランタイム|Synapse Pool|

Apache Spark は

- Synapse 内で実行可能
    
- Cosmos DB Analytical Store を直接処理可能
    
- ETL 処理を実装可能
    

したがって最適な選択となる。

---

# 9 他のプールタイプが不適な理由

## Serverless SQL Pool

用途

- Data Lake クエリ
    
- ad-hoc SQL
    

問題

- ETL 処理ができない
    
- Spark のような分散処理ができない
    

---

## Dedicated SQL Pool

用途

- データウェアハウス
    

問題

- データ処理エンジンではない
    
- データロード先として使う
    

---

## Data Explorer Pool

用途

- ログ分析
    
- 時系列分析
    

問題

- Cosmos DB Analytical Store との統合用途ではない
    

---

# 10 完全アーキテクチャ

今回の推奨構成

```text
Application
     │
     ▼
Azure Cosmos DB
(Container: Contained)
     │
     ▼
Analytical Store
     │
     ▼
Azure Synapse Link
     │
     ▼
Synapse Workspace
     │
     ▼
Apache Spark Pool
     │
     ▼
Dedicated SQL Pool
(Data Warehouse)
```

---

# 11 ユースケース

この構成は次のようなケースに適している。

### リアルタイム分析

```text
IoT Data
   │
   ▼
Cosmos DB
   │
   ▼
Spark Processing
   │
   ▼
Data Warehouse
```

---

### eコマース分析

```text
Orders
   │
   ▼
Cosmos DB
   │
   ▼
Synapse Spark
   │
   ▼
BI Dashboard
```

---

### ビッグデータ処理

```text
Operational DB
   │
   ▼
Analytical Store
   │
   ▼
Spark Analytics
```

---

# 12 設計指針

Azure Architect が Synapse プールを選択する基準

|処理タイプ|推奨|
|---|---|
|ETL / データ処理|Spark|
|Data Warehouse|Dedicated SQL|
|Data Lake Query|Serverless SQL|
|ログ分析|Data Explorer|

---

# 13 まとめ

今回の要件

- Cosmos DB Analytical Store
    
- near real-time 処理
    
- Synapse ワークスペース内ランタイム
    
- データ移動最小
    

これを満たす Synapse プールタイプは

**Apache Spark Pool**

である。

アーキテクチャ

```text
Cosmos DB
   │
   ▼
Analytical Store
   │
   ▼
Synapse Link
   │
   ▼
Apache Spark Pool
   │
   ▼
Data Warehouse
```

Apache Spark Pool は Synapse 内での分散データ処理を可能にし、Cosmos DB Analytical Store と連携することで **ETL不要の near real-time 分析アーキテクチャ**を実現できる。