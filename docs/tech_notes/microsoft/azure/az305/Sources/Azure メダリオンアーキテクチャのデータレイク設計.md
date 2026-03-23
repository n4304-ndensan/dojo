[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Medallion Architecture]]
# Azure メダリオンアーキテクチャのデータレイク設計

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Databricks]]
（Azure Databricks + Delta Lake）

---

# 1 背景とシナリオ

近年、多くの企業はデータドリブン経営を実現するために **データレイクアーキテクチャ**を採用している。特に金融・医療・政府などの規制産業では、データの整合性や監査性が非常に重要であり、単なるファイルベースのデータレイクでは要件を満たせない場合がある。

今回のシナリオでは、企業が Azure 上で **メダリオンアーキテクチャ（Medallion Architecture）** を採用したデータレイクを構築している。データエンジニアは、データパイプラインを効率的に運用しながら、次のような機能を実現する必要がある。

要求される機能は次の通りである。

- ACID トランザクション（データ整合性）
    
- スキーマ進化（Schema evolution）
    
- タイムトラベル（過去データの参照）
    
- 効率的なデータ統合（MERGE / UPSERT）
    
- 運用効率の高い処理エンジン
    

これらの要件を満たす最適な構成は

**Delta Lake フォーマット + Azure Databricks**

である。

---

# 2 メダリオンアーキテクチャとは

メダリオンアーキテクチャは、データ品質と信頼性を段階的に向上させるためのデータレイク設計パターンである。データは通常、3つのレイヤーに分けて管理される。

```text
Bronze → Silver → Gold
```

それぞれのレイヤーの役割は次の通りである。

|レイヤー|役割|
|---|---|
|Bronze|生データ（Raw data）|
|Silver|クレンジング・正規化データ|
|Gold|集計・分析用データ|

データフローは次のようになる。

```text
Source Systems
      │
      ▼
Bronze Layer
      │
      ▼
Silver Layer
      │
      ▼
Gold Layer
      │
      ▼
BI / ML / Analytics
```

この構造により、データ品質とガバナンスを段階的に強化できる。

---

# 3 Delta Lake

Delta Lake は、既存のデータレイク（Parquet ファイル）に **トランザクション機能を追加するストレージレイヤー**である。Apache Spark と統合されており、Azure Databricks の標準ストレージフォーマットとして使用される。

Delta Lake の最大の特徴は、データレイクに **データベースのような機能**を提供することである。

主要機能

- ACID トランザクション
    
- スキーマ進化
    
- タイムトラベル
    
- MERGE / UPSERT
    
- 高速クエリ
    

---

# 4 ACID トランザクション

ACID はデータベースのトランザクション特性である。

|特性|説明|
|---|---|
|Atomicity|トランザクションは全体成功または失敗|
|Consistency|データ整合性を保持|
|Isolation|同時処理の分離|
|Durability|データ永続化|

Delta Lake は **Delta Transaction Log** を利用して ACID を実現する。

```text
Delta Table
   │
   ├ Parquet files
   └ Delta Log (_delta_log)
```

これにより

- 同時書き込み
    
- データ破損防止
    

が可能になる。

---

# 5 スキーマ進化（Schema Evolution）

データレイクでは、ソースデータの構造が頻繁に変更される。

例

初期スキーマ

```text
customer_id
name
```

変更後

```text
customer_id
name
email
```

従来のデータレイクでは、この変更によりパイプラインが破損することがある。

Delta Lake は **Schema Evolution** をサポートする。

```sql
ALTER TABLE ADD COLUMN
```

これにより

- 列追加
    
- 型変更
    
- データ互換性
    

を管理できる。

---

# 6 タイムトラベル

Delta Lake は **データのバージョン履歴**を保持する。

これにより過去の状態を参照できる。

例

```sql
SELECT * FROM table VERSION AS OF 10
```

または

```sql
SELECT * FROM table TIMESTAMP AS OF '2024-01-01'
```

用途

- 監査
    
- デバッグ
    
- データ復元
    

規制環境では非常に重要な機能である。

---

# 7 MERGE / UPSERT

データパイプラインでは、次の処理が頻繁に必要になる。

- 新規データ追加
    
- 既存データ更新
    
- 重複データ削除
    

Delta Lake は SQL の MERGE 文をサポートする。

例

```sql
MERGE INTO customers
USING updates
ON customers.id = updates.id
WHEN MATCHED THEN UPDATE
WHEN NOT MATCHED THEN INSERT
```

これにより

- CDC処理
    
- データ統合
    

が効率的に実行できる。

---

# 8 Azure Databricks

Azure Databricks は Apache Spark をベースとした **ビッグデータ処理プラットフォーム**である。

主な機能

- Spark 分散処理
    
- Delta Lake ネイティブ統合
    
- Notebook 開発環境
    
- Auto-scaling クラスター
    

アーキテクチャ

```text
Azure Data Lake Storage
         │
         ▼
Azure Databricks
         │
         ▼
Delta Lake Tables
         │
         ▼
Analytics / Machine Learning
```

Databricks は Delta Lake の処理エンジンとして最適化されている。

---

# 9 他のストレージフォーマットとの比較

## Parquet

Parquet は列指向フォーマットであり、データレイクでよく使用される。

メリット

- 圧縮率が高い
    
- クエリ性能が良い
    

しかし

- ACIDなし
    
- タイムトラベルなし
    
- スキーマ進化が弱い
    

---

## ORC

ORC も列指向フォーマットである。

用途

- Hadoop / Hive
    

しかし

- ACIDトランザクションなし
    
- Delta のようなログ管理なし
    

---

## Avro

Avro は行指向フォーマットである。

メリット

- スキーマ管理
    

しかし

- ACIDなし
    
- タイムトラベルなし
    

---

# 10 推奨アーキテクチャ

Azure のメダリオンデータレイク

```text
Data Sources
     │
     ▼
Bronze (Delta Lake)
     │
     ▼
Silver (Delta Lake)
     │
     ▼
Gold (Delta Lake)
     │
     ▼
Analytics / BI
```

処理エンジン

```text
Azure Databricks
```

ストレージ

```text
Azure Data Lake Storage Gen2
```

---

# 11 まとめ

今回の要件

- ACIDトランザクション
    
- スキーマ進化
    
- タイムトラベル
    
- データ統合（MERGE）
    
- メダリオンアーキテクチャ
    

これらを満たす最適な構成は

**Delta Lake + Azure Databricks**

である。

この構成は

- データ整合性
    
- 高速分析
    
- スケーラブル処理
    
- 規制コンプライアンス
    

を同時に実現するため、Azure データレイクアーキテクチャにおいて最も一般的なベストプラクティスとなっている。