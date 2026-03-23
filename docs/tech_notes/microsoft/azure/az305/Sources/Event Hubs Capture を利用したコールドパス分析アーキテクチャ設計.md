[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Event Hubs]]
# Event Hubs Capture を利用したコールドパス分析アーキテクチャ設計

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Lake Storage Gen2]]
（Azure Data Lake Storage Gen2 + Parquet）

---

# 1 背景

企業では IoT デバイス、アプリケーションログ、トランザクションイベントなどをリアルタイムに収集するために **Azure Event Hubs** を利用することが多い。Event Hubs は Azure の高スループットなイベントストリーミングサービスであり、数百万件/秒規模のイベントを受信することが可能である。

今回のシナリオでは、以下の要件がある。

- 毎日 **50,000 件のイベント**が Event Hubs に送信される
    
- Event Hubs Capture を使用して **コールドパス処理**を行う
    
- 保存されたデータは **レポートシステム（BI・分析ツール）**によって利用される
    

この要件では、単にイベントを保存するだけではなく、**後続の分析・レポート処理を効率よく行えるデータ保存方式**を選択することが重要になる。

---

# 2 コールドパス処理とは

ストリーミングデータ処理では一般的に **Hot Path** と **Cold Path** の 2 種類の処理が存在する。

## Hot Path

リアルタイム分析を行う処理

例

- アラート生成
    
- リアルタイムダッシュボード
    
- 即時異常検知
    

典型サービス

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Stream Analytics]]
- Azure Stream Analytics
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Functions]]
- Azure Functions
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Databricks]]
- Azure Databricks
    

---

## Cold Path

蓄積されたデータを後から分析する処理

例

- バッチ分析
    
- BI レポート
    
- データウェアハウス処理
    
- 機械学習トレーニング
    

典型サービス

- Azure Data Lake
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Synapse Analytics]]
- Azure Synapse
    
- Azure Databricks
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Power BI]]
- Power BI
    

今回の要件は **コールドパス分析**であるため、**分析に最適なストレージとフォーマット**を選択する必要がある。

---

# 3 Event Hubs Capture

Event Hubs Capture は Event Hubs の機能の一つであり、イベントストリームを **自動的にストレージへ保存**できる。

保存先として利用できるのは主に次のストレージである。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Blob Storage]]
- Azure Blob Storage
    
- Azure Data Lake Storage Gen2
    

この機能の利点は次の通り。

- ストリームデータを自動保存
    
- バッチ分析用データを生成
    
- ストリーミングと分析を分離
    

---

# 4 Azure Data Lake Storage Gen2

コールドパス分析では **Azure Data Lake Storage Gen2 (ADLS Gen2)** が最も一般的なストレージである。

ADLS Gen2 は以下の特徴を持つ。

### ビッグデータ分析向け設計

大量データの保存と分析処理に最適化されている。

---

### 階層型ネームスペース

ディレクトリ構造を持つため、データを効率的に整理できる。

例

```
/events
   /2026
      /03
         /01
         /02
```

---

### 高スケーラビリティ

ペタバイト級データの保存が可能。

---

### 分析エンジンとの統合

次のサービスとネイティブ統合されている。

- Azure Synapse Analytics
    
- Azure Databricks
    
- Spark
    
- Power BI
    

---

# 5 データ形式の選択

ストレージに保存するデータ形式は分析性能に大きく影響する。

代表的なフォーマットは次の通り。

|形式|特徴|
|---|---|
|JSON|読みやすいがサイズが大きい|
|Avro|行指向フォーマット|
|Parquet|列指向フォーマット|

---

# 6 Parquet の特徴

Parquet はビッグデータ分析で最も一般的に利用されるフォーマットである。

主な特徴は以下。

### 列指向フォーマット

データを列単位で保存する。

例

```
Column1: A A A A
Column2: B B B B
Column3: C C C C
```

これにより、分析クエリが必要な列だけ読み込むことが可能。

---

### 高圧縮

列単位で圧縮されるため、ストレージ容量を削減できる。

---

### クエリ性能向上

BI ツールや SQL エンジンが高速に処理できる。

---

### Predicate Pushdown

クエリ条件により不要なデータ読み込みを回避できる。

---

# 7 Avro との違い

Avro は行指向フォーマットであり、ストリーミングには適しているが分析用途では Parquet の方が有利である。

|比較項目|Avro|Parquet|
|---|---|---|
|構造|行指向|列指向|
|ストリーミング|適している|普通|
|分析クエリ|普通|非常に高速|
|圧縮率|中程度|高い|

コールドパス分析では **Parquet が標準的な選択**である。

---

# 8 JSON が不適切な理由

JSON は開発者には扱いやすいフォーマットだが、大規模分析では問題がある。

問題点

- データサイズが大きい
    
- クエリが遅い
    
- 圧縮効率が低い
    
- BI クエリエンジンに不利
    

そのため大規模分析用途では一般的に使用されない。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Files]]
# 9 Azure Files が不適切な理由

Azure Files は SMB/NFS のファイル共有サービスであり、次の用途に向いている。

- アプリケーション共有ストレージ
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
- VM ファイル共有
    
- Lift & Shift
    

しかしビッグデータ分析には適していない。

理由

- 分析エンジンとの統合が弱い
    
- パーティション管理不可
    
- 大規模データ処理に非最適
    

---

# 10 推奨アーキテクチャ

最適な構成は次の通り。

```
Event Producers
      │
      ▼
Azure Event Hubs
      │
      ▼
Event Hubs Capture
      │
      ▼
Azure Data Lake Storage Gen2
      │
      ▼
Parquet Files
      │
      ▼
Analytics / Reporting
   ├─ Azure Synapse
   ├─ Databricks
   └─ Power BI
```

この構成により

- 高速分析
    
- ストレージ効率
    
- BI 互換性
    

が実現できる。

---

# 11 最終回答

正解

**C**

```
Azure Data Lake Storage Gen2 + Parquet
```

---

# 12 まとめ

Event Hubs Capture を利用したコールドパス分析では、**データレイク型ストレージと分析最適フォーマットの組み合わせ**が重要である。

最適な組み合わせは次の通り。

- ストレージ：Azure Data Lake Storage Gen2
    
- フォーマット：Parquet
    

この構成は

- BI 分析
    
- データウェアハウス
    
- 大規模バッチ処理
    

に最適化されており、Azure のビッグデータアーキテクチャで広く採用されている設計パターンである。