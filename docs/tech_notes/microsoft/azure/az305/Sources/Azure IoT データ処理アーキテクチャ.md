# Azure IoT データ処理アーキテクチャ

（50,000デバイス / リアルタイム異常検知 / 長期分析）

---

# 1 背景

ある IoT ソリューションでは **50,000台のデバイス**からテレメトリーデータを収集する必要がある。  
デバイスは継続的にセンサー情報を送信し、そのデータは次の目的で利用される。

- リアルタイム異常検知
    
- 運用監視
    
- バッチ分析
    
- 長期トレンド分析
    

このため、IoT データ基盤は **ストリーミング処理 + 長期ストレージ**の両方を備える必要がある。

---

# 2 要件

### 機能要件

1. 50,000デバイスからのデータ取り込み
    
2. リアルタイムで異常検出
    
3. 長期データ保存
    
4. バッチ処理によるトレンド分析
    

---

# 3 推奨サービス

要件を満たすサービスの組み合わせ

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Stream Analytics]]
**Azure Stream Analytics**

**Azure Data Lake Storage**

---

# 4 IoTデータパイプライン

典型的な IoT データ処理構成は以下のようになる。

```text
IoT Devices
     │
     ▼
Event Ingestion
     │
     ▼
Stream Processing
     │
     ▼
Long-term Storage
```

Azure では次の構成が一般的である。

```text
Devices
 │
 ▼
IoT Hub / Event Hub
 │
 ▼
Stream Analytics
 │
 ▼
Data Lake Storage
```

---

# 5 Azure Stream Analytics

Azure Stream Analytics は **リアルタイムストリーム処理サービス**である。

主な特徴

|機能|説明|
|---|---|
|ストリーム処理|リアルタイム処理|
|SQLベースクエリ|簡単な分析|
|異常検出|ストリーム分析|
|低レイテンシ|秒レベル処理|

---

### ストリーム処理の例

```sql
SELECT
    deviceId,
    AVG(temperature)
FROM telemetry
GROUP BY deviceId, TumblingWindow(minute, 1)
HAVING AVG(temperature) > 80
```

このクエリは

- 温度の平均
    
- 異常温度
    

をリアルタイムで検出する。

---

# 6 Azure Data Lake Storage

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Lake Storage Gen2]]
Azure Data Lake Storage (ADLS Gen2) は **ビッグデータ用ストレージ**である。

特徴

|機能|説明|
|---|---|
|大容量|ペタバイト級|
|低コスト|長期保存|
|Hadoop互換|分析基盤|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Databricks]]
|分析ツール連携|Synapse / Databricks|

---

### 保存データ例

```text
/data
   /raw
      /2025
      /2026
   /processed
   /analytics
```

---

# 7 データフロー

IoT データ処理の流れ

```text
Devices
 │
 ▼
Event Hub
 │
 ▼
Stream Analytics
 │
 ├ Realtime Alerts
 │
 └ Data Lake Storage
```

---

# 8 リアルタイム異常検知

Stream Analytics では

- 突発的異常
    
- センサー故障
    
- 異常値
    

を検知できる。

例

```text
Device Telemetry
 │
 ▼
Stream Analytics
 │
 ▼
Anomaly Detection
```

---

# 9 長期データ分析

Data Lake に保存されたデータは以下のツールで分析できる。

### 分析ツール

|ツール|用途|
|---|---|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Synapse Analytics]]
|Azure Synapse|DWH分析|
|Databricks|Spark分析|
|Data Factory|ETL|
|Machine Learning|AI分析|

---

### 長期トレンド分析

```text
Data Lake
 │
 ▼
Databricks
 │
 ▼
Trend Analysis
```

例

- 設備劣化
    
- 季節変動
    
- 異常パターン
    

---

# 10 他の選択肢の評価

## Azure Time Series Insights

用途

- 時系列データ可視化
    

問題

- 主ストレージではない
    

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Cosmos DB]]
## Azure Cosmos DB

用途

- NoSQLデータベース
    

問題

- 大量データ分析には不向き
    

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Event Hubs]]
## Azure Event Hubs

用途

- データ取り込み
    

問題

- 処理や保存ではない
    

---

# 11 推奨アーキテクチャ

```text
50,000 IoT Devices
        │
        ▼
Azure IoT Hub / Event Hub
        │
        ▼
Azure Stream Analytics
        │
        ├ Real-time Alerts
        │
        └ Azure Data Lake Storage
                │
                ▼
Batch Analytics
```

---

# 12 メリット

### リアルタイム処理

Stream Analytics

- 低レイテンシ
    
- SQLクエリ
    

---

### 大規模保存

Data Lake

- ペタバイトスケール
    
- 低コスト
    

---

### 分析基盤

- Synapse
    
- Databricks
    
- ML
    

---

# 13 まとめ

今回の要件

- 50,000 IoTデバイス
    
- リアルタイム異常検知
    
- 長期トレンド分析
    

最適な組み合わせは

**Azure Stream Analytics**  
**Azure Data Lake Storage**

この構成により

- リアルタイム処理
    
- 大規模ストレージ
    
- バッチ分析
    

を同時に実現できる。