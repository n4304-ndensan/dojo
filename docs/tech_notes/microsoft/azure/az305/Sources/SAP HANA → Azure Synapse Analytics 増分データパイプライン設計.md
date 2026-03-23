[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Synapse Analytics]]
# SAP HANA → Azure Synapse Analytics 増分データパイプライン設計

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Factory]]
（Azure Data Factory + CDC / Watermark パターン）

---

# 1 背景

企業のデータ基盤では、オンプレミスの業務システム（ERP、SAP、CRMなど）からクラウドのデータウェアハウスへデータを定期的に取り込む必要がある。特に分析基盤として **Azure Synapse Analytics** を利用する場合、運用システムへの負荷を最小限に抑えながらデータを取り込む設計が重要となる。

今回のシナリオでは、オンプレミスの **SAP HANA データベース** から Azure Synapse Analytics へデータを取り込むパイプラインを構築する必要がある。要件は次の通りである。

- 1時間ごとに変更されたレコードを取得する
    
- ソースシステムへの影響を最小化する
    
- データウェアハウスに増分ロードする
    
- クラウドネイティブなデータ統合サービスを使用する
    

これらの要件を満たす最適な構成は

**Azure Data Factory + CDC / Watermark 列を使用した増分ロード**

である。

---

# 2 増分データロード（Incremental Load）

データウェアハウスのETL処理では、次の2つのロード方式が存在する。

### フルロード（Full Load）

すべてのデータを毎回読み込む。

```text
Source Table
 ├ Row1
 ├ Row2
 ├ Row3
 └ Row4
```

問題

- 大量データ転送
    
- ソースDB負荷
    
- 処理時間増加
    

---

### 増分ロード（Incremental Load）

変更されたデータのみ取得する。

```text
Last Load Time: 10:00

Source Table
 ├ Row1 (changed 10:30)
 ├ Row2
 ├ Row3 (changed 10:40)
 └ Row4
```

取得データ

```text
Row1
Row3
```

メリット

- データ転送量削減
    
- ソースDB負荷低減
    
- 高速処理
    

今回のシナリオでは **増分ロードが必須**である。

---

# 3 Change Data Capture（CDC）

CDC はデータベース内の変更を追跡する仕組みである。

対象となる変更

- INSERT
    
- UPDATE
    
- DELETE
    

CDCの動作

```text
Database Change
      │
      ▼
Change Log
      │
      ▼
ETL Process
```

CDCを利用すると、変更されたレコードのみを取得できる。

---

# 4 Watermark パターン

CDCが利用できない場合、一般的に **Watermark 列** を利用する。

Watermark 列の例

- LastModifiedDate
    
- UpdatedTimestamp
    
- VersionNumber
    

例

```text
Order Table
 ├ OrderID
 ├ Amount
 └ LastModifiedDate
```

ETL処理

```sql
SELECT *
FROM Orders
WHERE LastModifiedDate > LastWatermark
```

処理フロー

```text
Last Watermark
      │
      ▼
Query Changed Rows
      │
      ▼
Load to Synapse
      │
      ▼
Update Watermark
```

これにより変更データのみを取得できる。

---

# 5 Azure Data Factory

Azure Data Factory（ADF）は Azure の **クラウドETL / データ統合サービス**である。

主な機能

- データパイプライン作成
    
- スケーラブルデータコピー
    
- スケジュール実行
    
- データ変換
    

基本アーキテクチャ

```text
Data Sources
     │
     ▼
Azure Data Factory
     │
     ▼
Azure Synapse Analytics
```

ADFはオンプレミスデータソースとも統合できる。

---

# 6 SAP HANA との統合

Azure Data Factory は SAP HANA と接続できる。

接続方法

```text
On-Prem SAP HANA
      │
      ▼
Self-hosted Integration Runtime
      │
      ▼
Azure Data Factory
```

Self-hosted Integration Runtime はオンプレミスとクラウド間の安全な接続を提供する。

---

# 7 増分パイプライン構成

SAP HANA → Synapse の増分パイプラインは次のように設計する。

```text
SAP HANA
   │
   │  Incremental Query
   ▼
Azure Data Factory
   │
   │  Copy Activity
   ▼
Azure Synapse Analytics
```

処理フロー

```text
1 Retrieve last watermark
2 Query changed rows from SAP HANA
3 Copy data to staging
4 Load into Synapse table
5 Update watermark
```

スケジュール

```text
Every 1 hour
```

---

# 8 ソースシステムへの影響最小化

増分ロードはソースシステムへの影響を大幅に減らす。

理由

- フルスキャン不要
    
- インデックス利用可能
    
- 小規模データ転送
    

例

```text
Full Load
10M rows scan

Incremental Load
1000 rows scan
```

SAP のような業務システムでは特に重要である。

---

# 9 他の選択肢の評価

## SSIS + CDC

SSIS は SQL Server 向け CDC 機能が強い。

しかし

- SAP HANA ネイティブCDCが弱い
    
- オンプレミス依存
    
- クラウド運用に不向き
    

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Databricks]]
## Databricks Auto Loader

Auto Loader は

- ファイル取り込み用
    
- Cloud Storage 用
    

用途

```text
Cloud Storage → Delta Lake
```

SAP HANA の DB 取り込みには適さない。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Logic Apps]]
## Azure Logic Apps

Logic Apps は

- ワークフロー自動化
    
- イベント処理
    

用途

```text
Email → Workflow → API
```

大量データETLには適さない。

---

# 10 推奨アーキテクチャ

最適な構成

```text
On-Prem SAP HANA
        │
        ▼
Self-hosted Integration Runtime
        │
        ▼
Azure Data Factory
        │
        ▼
Azure Synapse Analytics
```

増分処理

```text
CDC / Watermark
```

---

# 11 まとめ

今回の要件

- SAP HANA から Synapse へデータ転送
    
- 1時間ごとの増分ロード
    
- ソース負荷最小化
    
- クラウドネイティブETL
    

これらを満たす最適なソリューションは

**Azure Data Factory + CDC / Watermark パターン**

である。

この構成は

- スケーラブル
    
- 低負荷
    
- 自動化可能
    

という特徴を持ち、Azure データ統合アーキテクチャにおける標準的なベストプラクティスとなっている。