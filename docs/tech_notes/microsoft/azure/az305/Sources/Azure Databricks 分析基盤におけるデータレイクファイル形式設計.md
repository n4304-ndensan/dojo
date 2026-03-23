[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Databricks]]
# Azure Databricks 分析基盤におけるデータレイクファイル形式設計

（Partitioned Parquet）

---

# 1 背景

データエンジニアリングチームは、Azure Databricks を中心とした分析プラットフォームを構築している。複数のデータソース（業務システム、ログ、IoTデータ、外部APIなど）から毎日大量のデータが取り込まれ、データサイエンティストやアナリストがそれを利用して分析や機械学習を行う。

今回のシナリオでは、次のようなデータ処理要件がある。

- 毎日 **約500GBの新しいデータ** を取り込む
    
- Databricks 上で処理し、データレイクに保存する
    
- データサイエンティストが **インタラクティブにクエリ** を実行できる
    
- スキーマの整合性を維持する
    
- 高速な列スキャンを実現する
    
- フィルタリングの **Predicate Pushdown** をサポートする
    

これらの要件を満たすデータレイクのファイル形式として最適なのは

**Partitioned Parquet**

である。

---

# 2 データレイクにおけるファイル形式の重要性

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Lake Storage Gen2]]
データレイクでは、データは通常オブジェクトストレージ（Azure Data Lake Storage Gen2など）にファイルとして保存される。そのため、使用するファイル形式はクエリ性能やデータ管理に大きく影響する。

特に分析基盤では、次の要素が重要になる。

|要件|説明|
|---|---|
|スキーマの取り締まり|データ型や構造の一貫性|
|カラムスキャン効率|必要な列のみ読み込む|
|Predicate Pushdown|ストレージレベルでフィルタリング|
|圧縮効率|ストレージ削減|
|分散処理対応|Spark / Databricks|

Parquet はこれらの要件を満たす設計になっている。

---

# 3 Parquetとは

Parquet は Apache Hadoop エコシステムで開発された **列指向ストレージフォーマット（Columnar Format）**である。

通常の行指向フォーマットとは異なり、列単位でデータを保存する。

行指向

```text
Row1: colA colB colC
Row2: colA colB colC
Row3: colA colB colC
```

列指向（Parquet）

```text
Column A: A1 A2 A3
Column B: B1 B2 B3
Column C: C1 C2 C3
```

この構造は分析クエリに非常に適している。

---

# 4 効率的なカラムスキャン

分析クエリでは、通常すべての列を読み込むわけではない。

例

```sql
SELECT user_id, purchase_amount
FROM transactions
WHERE purchase_date = '2025-01-01'
```

この場合

- user_id
    
- purchase_amount
    
- purchase_date
    

のみが必要になる。

Parquet の場合

```text
Read columns:
user_id
purchase_amount
purchase_date
```

不要な列は読み込まない。

CSVの場合

```text
Read entire row
```

そのため、大規模データでは Parquet の方が圧倒的に高速になる。

---

# 5 Predicate Pushdown

Parquet は **Predicate Pushdown** をサポートする。

これは、クエリエンジンがストレージレベルでフィルタリングを行う仕組みである。

例

```sql
SELECT *
FROM transactions
WHERE country = 'Japan'
```

Parquet の場合

```text
Storage Layer
    │
    ▼
Filter rows where country = Japan
```

不要なデータを読み込まないため

- I/O削減
    
- クエリ高速化
    

が実現される。

---

# 6 スキーマの取り締まり

Parquet はスキーマ付きフォーマットである。

例

```text
transactions schema

user_id: INT
purchase_amount: DOUBLE
purchase_date: TIMESTAMP
```

これにより

- データ型の保証
    
- データ品質の維持
    
- 分析処理の安定性
    

が実現できる。

CSV や JSON の場合

```text
Data types ambiguous
```

型の不整合が発生しやすい。

---

# 7 パーティション分割

500GB/日のデータを効率的に管理するためには **パーティション分割** が重要である。

例

```text
transactions/
    year=2025/
        month=01/
            day=01/
            day=02/
```

クエリ

```sql
SELECT *
FROM transactions
WHERE day = '2025-01-01'
```

この場合

```text
Scan only partition
day=01
```

500GB 全体を読み込む必要がない。

これを **Partition Pruning** と呼ぶ。

---

# 8 Databricksとの統合

Azure Databricks は Apache Spark ベースのデータ処理エンジンであり、Parquet に対して高度な最適化が行われている。

Spark の最適化

- Vectorized Reader
    
- Predicate Pushdown
    
- Column Pruning
    
- Catalyst Optimizer
    

構造

```text
Azure Data Lake Storage
        │
        ▼
Parquet Files
        │
        ▼
Azure Databricks (Spark)
        │
        ▼
Interactive Queries
```

この組み合わせは大規模分析基盤の標準構成である。

---

# 9 他のフォーマットとの比較

## CSV

メリット

- シンプル
    
- 人間が読みやすい
    

問題

- スキーマなし
    
- 列スキャン不可
    
- Pushdownなし
    

大規模分析には不向き。

---

## JSON

メリット

- 半構造化データ対応
    

問題

- 行指向
    
- パースコスト高
    
- クエリ性能低
    

---

## Avro

メリット

- スキーマサポート
    
- 圧縮効率
    

問題

- 行指向フォーマット
    
- 列スキャン効率が低い
    

---

# 10 推奨アーキテクチャ

Databricks データレイク構成

```text
Data Sources
     │
     ▼
Azure Databricks ETL
     │
     ▼
Azure Data Lake Storage
     │
     ▼
Partitioned Parquet Files
     │
     ▼
Interactive Analytics
```

この構成により

- 大規模データ処理
    
- 高速クエリ
    
- スケーラブル分析
    

が可能になる。

---

# 11 まとめ

今回の要件

- 500GB/日のデータ処理
    
- Databricks分析
    
- スキーマ管理
    
- 高速カラムスキャン
    
- Predicate Pushdown
    

これらを満たす最適なファイル形式は

**Partitioned Parquet**

である。

Parquet は

- 列指向ストレージ
    
- 高圧縮
    
- 高速クエリ
    
- Spark最適化
    

という特徴を持ち、大規模データレイクおよび Databricks 分析基盤における標準的なファイル形式として広く採用されている。