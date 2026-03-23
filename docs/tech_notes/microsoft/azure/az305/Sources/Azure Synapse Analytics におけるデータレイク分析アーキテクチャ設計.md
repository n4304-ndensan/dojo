[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Synapse Analytics]]
# Azure Synapse Analytics におけるデータレイク分析アーキテクチャ設計

（Dedicated SQL Pool + Serverless Spark Pool）

---

# 1 背景

企業は大量のデータを蓄積・分析するために **Azure Synapse Analytics** を利用してデータ分析基盤を構築している。Azure Synapse は、データウェアハウス機能とビッグデータ分析機能を統合したサービスであり、SQL 分析と Spark 分析の両方を同一プラットフォームで実行できる。

今回のシナリオでは次の要件がある。

- データは **Azure Data Lake** に保存されている
    
- Data Lake のデータを **ハッシュ分散テーブル**へ取り込む必要がある
    
- **Delta Lake 形式のデータをクエリ・更新**する必要がある
    

この要件は **データウェアハウス処理とデータレイク処理の両方を含むハイブリッド分析アーキテクチャ**である。

---

# 2 Azure Synapse のコンピューティングエンジン

Azure Synapse には複数のコンピューティングエンジンが存在する。

|プールタイプ|用途|
|---|---|
|Dedicated SQL Pool|データウェアハウス|
|Serverless SQL Pool|オンデマンド SQL クエリ|
|Spark Pool|ビッグデータ処理|

今回の問題では

- SQL DWH 処理
    
- Data Lake 処理
    

の両方が必要になる。

---

# 3 ハッシュ分散テーブルとは

Azure Synapse Dedicated SQL Pool では、大規模データを効率的に処理するために **分散テーブル**が利用される。

分散方式には以下がある。

|方式|説明|
|---|---|
|Hash distribution|指定列で分散|
|Round Robin|ランダム分散|
|Replicated|全ノードコピー|

この中で **Hash distribution** は最も一般的な方式である。

理由

- JOIN パフォーマンス向上
    
- データスキャン高速化
    
- 大規模ファクトテーブル向け
    

例

```sql
CREATE TABLE Sales
(
    SalesId INT,
    CustomerId INT,
    Amount DECIMAL
)
WITH
(
    DISTRIBUTION = HASH(CustomerId)
)
```

このようなテーブルは **Dedicated SQL Pool でのみ利用可能**である。

---

# 4 Dedicated SQL Pool の役割

Dedicated SQL Pool は Synapse の **MPP（Massively Parallel Processing）型データウェアハウスエンジン**である。

特徴

- 大規模データロード
    
- 分散クエリ
    
- SQL ベース分析
    
- 高速 JOIN 処理
    

Data Lake からデータをロードする際は

```text
COPY INTO
PolyBase
CTAS
```

などを使用してハッシュ分散テーブルへ取り込む。

---

# 5 Delta Lake とは

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Databricks]]
Delta Lake は Databricks が開発した **データレイク上のトランザクションレイヤー**である。

特徴

|機能|説明|
|---|---|
|ACID トランザクション|データ整合性|
|タイムトラベル|過去データ参照|
|スキーマ進化|スキーマ変更対応|
|高速更新|MERGE/UPDATE|

Delta Lake は **Apache Spark を前提としたフォーマット**である。

---

# 6 Spark が必要な理由

Delta Lake の操作は Spark ランタイムによって提供される。

Spark で可能な操作

```python
df.write.format("delta").save()
```

```python
spark.sql("MERGE INTO table ...")
```

このような操作は **SQL Engine ではなく Spark Engine**によって実行される。

そのため

```text
Delta Lake
↓
Spark Pool
```

が必須になる。

---

# 7 Serverless Spark Pool

Synapse では Spark を次の方法で利用できる。

- Dedicated Spark Pool
    
- Serverless Spark Pool
    

Serverless Spark Pool の特徴

- クラスター管理不要
    
- 必要な時だけ起動
    
- コスト削減
    
- Delta Lake ネイティブ対応
    

Delta Lake の操作

- UPDATE
    
- DELETE
    
- MERGE
    
- INSERT
    

は Spark を利用するのが一般的である。

---

# 8 推奨アーキテクチャ

今回の要件を満たす構成は次のようになる。

```text
Data Sources
     │
     ▼
Azure Data Lake
     │
     ├───────────────┐
     │               │
     ▼               ▼
Dedicated SQL Pool   Serverless Spark Pool
(Hash distributed)   (Delta Lake)
     │               │
     ▼               ▼
Data Warehouse      Data Lake Analytics
```

役割分担

|コンポーネント|役割|
|---|---|
|Dedicated SQL Pool|データウェアハウスロード|
|Serverless Spark Pool|Delta Lake クエリ・更新|

---

# 9 ワークフロー

実際のデータ処理フローは次のようになる。

### ① Data Lake にデータ保存

```text
IoT / Logs / Apps
      │
      ▼
Azure Data Lake
```

---

### ② Dedicated SQL Pool に取り込み

```text
Data Lake
     │
     ▼
Hash Distributed Tables
```

利用例

```sql
COPY INTO Sales
FROM 'https://datalake/...'
```

---

### ③ Delta Lake を Spark で処理

```text
Delta Lake Tables
      │
      ▼
Spark Pool
```

Spark により

- 更新
    
- トランザクション
    
- ETL
    

を実行する。

---

# 10 他の選択肢が不適切な理由

## Serverless SQL Pool

Serverless SQL Pool は **読み取り専用クエリエンジン**である。

制限

- Delta Lake 更新不可
    
- ACID トランザクション不可
    

---

## Spark だけで取り込み

Spark で Dedicated SQL Pool のハッシュ分散テーブルを管理するのは非効率である。

理由

- SQL DW 専用設計
    
- 分散ロード最適化
    

---

## SQL Pool だけ

SQL Pool は Delta Lake の ACID 機能を提供しない。

---

# 11 最終回答

正解

**C**

```text
取り込み: Dedicated SQL Pool
Delta Lake: Serverless Spark Pool
```

---

# 12 まとめ

Azure Synapse の分析基盤では **複数のコンピューティングエンジンを組み合わせる設計**が重要である。

今回の最適構成

|用途|サービス|
|---|---|
|データウェアハウス|Dedicated SQL Pool|
|Delta Lake 分析|Serverless Spark Pool|

この構成により

- 高速データロード
    
- 大規模分散クエリ
    
- Delta Lake トランザクション
    

を同時に実現できる。