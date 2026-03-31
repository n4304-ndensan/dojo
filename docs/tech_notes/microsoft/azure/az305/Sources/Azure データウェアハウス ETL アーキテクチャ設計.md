# Azure データウェアハウス ETL アーキテクチャ設計

（Azure SQL + Oracle 統合）

---

# 1 背景

ある企業では、複数のデータソースからデータを統合する **クラウド型データウェアハウス**を設計している。  
このデータウェアハウスでは、以下のような複数のシステムからデータを収集する必要がある。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
- Azure SQL Database
    
- オンプレミス Oracle Database
    

これらの異なるデータソースを統合し、分析基盤へロードするためには **ETL（Extract / Transform / Load）パイプライン**が必要になる。

---

# 2 要件

このデータ統合ソリューションには、以下の機能要件がある。

### データ統合

- Azure SQL Database からデータ取得
    
- オンプレミス Oracle Database からデータ取得
    

### データ変換

- 複雑な変換処理
    
- データクレンジング
    
- 集約処理
    

### データモデリング

- Slowly Changing Dimensions（SCD）対応
    
    - Type1
        
    - Type2
        

### 運用要件

- スケーラブルなETLパイプライン
    
- クラウドネイティブなデータ統合
    
- GUIベースの開発
    
- パイプラインオーケストレーション
    

これらの要件を満たす Azure サービスとして最も適しているのは

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Factory]]
**Azure Data Factory（Data Flow）**

である。

---

# 3 Azure Data Factory

Azure Data Factory（ADF）は、Azure の **クラウドネイティブ ETL / ELT サービス**であり、データ統合およびデータパイプラインのオーケストレーションを提供する。

ADFは次のような機能を提供する。

|機能|説明|
|---|---|
|データ統合|多様なデータソースとの接続|
|パイプライン管理|ETLワークフロー管理|
|データ変換|Mapping Data Flow|
|スケーリング|自動スケーリング|
|監視|パイプライン監視|

ADFは、オンプレミスとクラウドの両方のデータソースを統合できるため、ハイブリッドデータ統合の中心サービスとして使用される。

---

# 4 Data Flow（データ変換）

Azure Data Factory では **Mapping Data Flow** を使用することで、大規模データの変換処理をGUIベースで設計できる。

Data Flow の特徴

- コード不要のデータ変換
    
- Spark ベースの処理
    
- スケーラブルな実行
    
- 複雑な変換ロジック
    

Data Flow では以下のような処理を実装できる。

- フィルタリング
    
- ジョイン
    
- 集約
    
- データクレンジング
    
- データ派生列
    

---

# 5 Slowly Changing Dimensions（SCD）

データウェアハウスでは、ディメンションテーブルの履歴管理が必要になる。  
ADF Data Flow は **SCD Type1 / Type2** をサポートしている。

### SCD Type1

履歴を保持せず、既存レコードを更新する。

```text
顧客名変更

旧
顧客ID 1001
名前 田中

新
顧客ID 1001
名前 田中太郎
```

### SCD Type2

履歴を保持する。

```text
顧客履歴

顧客ID 名前        有効開始日  有効終了日
1001    田中        2023       2024
1001    田中太郎    2024       NULL
```

ADF Data Flow は SCD ロジックを簡単に実装できるコンポーネントを提供している。

---

# 6 アーキテクチャ

ETL パイプライン構成は次のようになる。

```text
Source Systems
   │
   ├─ Azure SQL Database
   │
   ├─ Oracle Database (On-Prem)
   │
   ▼
Azure Data Factory
   │
   ├─ Data Pipeline
   │
   ├─ Mapping Data Flow
   │
   ▼
Data Warehouse
```

ADF は ETL パイプラインの **中央オーケストレーションレイヤー**として機能する。

---

# 7 データ処理フロー

ETL 処理は次の流れで実行される。

```text
Extract
   │
   ▼
Azure SQL / Oracle
   │
   ▼
Transform
   │
   ▼
ADF Data Flow
   │
   ▼
Load
   │
   ▼
Data Warehouse
```

ADF パイプラインは、スケジュールやトリガーによって実行される。

---

# 8 他の選択肢の評価

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Databricks]]
### Azure Databricks

Databricks は Apache Spark ベースのデータ処理プラットフォームであり、大規模データ処理や機械学習に適している。しかし、通常は **ETL処理エンジン**として使用され、パイプラインのオーケストレーションは別のサービスが必要になることが多い。

そのため、ETLパイプラインの中心サービスとしては ADF の方が適している。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Synapse Analytics]]
### Azure Synapse Pipelines

Synapse Pipelines は ADF と似た機能を持つが、主に **Synapse Analytics 環境内のデータ処理**を目的としている。Synapse を中心としたアーキテクチャでは有効だが、一般的なデータ統合 ETL では ADF の方が広く使用される。

---

### SQL Server Integration Services（SSIS）

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
SSIS は従来の ETL ツールであり、Azure VM 上で実行することもできる。しかし、この方法では次のような問題がある。

- VM 管理が必要
    
- スケーリングが困難
    
- クラウドネイティブではない
    

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Logic Apps]]
### Logic Apps

Logic Apps はワークフロー自動化ツールであり、大規模データ変換やETL処理には適していない。

---

# 9 メリット

Azure Data Factory を ETL 中心サービスとして使用することで、次のメリットが得られる。

### 高い接続性

Azure SQL や Oracle など多くのデータソースに接続できる。

### スケーラビリティ

Spark ベースの Data Flow により大規模データ処理が可能。

### GUIベース開発

コードを書かずに ETL パイプラインを設計できる。

### クラウドネイティブ

サーバーレスでインフラ管理が不要。

---

# 10 まとめ

今回の要件

- Azure SQL と Oracle の統合
    
- 複雑な変換処理
    
- Slowly Changing Dimensions
    
- ETLパイプライン管理
    

これらを満たす Azure サービスは

**Azure Data Factory（Data Flow）**

である。

Azure Data Factory は、データウェアハウス向けの ETL パイプラインの中心となるクラウドネイティブなデータ統合サービスであり、複雑な変換処理や SCD を含むデータ統合ワークロードを効率的に実装できる。