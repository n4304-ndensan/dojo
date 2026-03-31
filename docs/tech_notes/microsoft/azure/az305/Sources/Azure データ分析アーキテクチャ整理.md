
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Cosmos DB]]
# Azure データ分析アーキテクチャ整理（Cosmos DB / Synapse / Data Pipeline）

Azure では、データ処理の用途によってサービスが明確に分かれています。  
特に試験や実務では **OLTP（運用データ処理）と Analytics（分析処理）を分離する設計**が重要になります。

この分野では主に次のサービスを理解する必要があります。

- Azure Cosmos DB
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Synapse Analytics]]
- Azure Synapse Analytics
- Azure Synapse Link
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Factory]]
- Azure Data Factory
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Event Hubs]]
- Azure Event Hub
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Stream Analytics]]
- Azure Stream Analytics
- Azure Data Lake Storage

これらは **データの生成 → 収集 → 保存 → 分析**という流れの中で役割が分かれています。

---

# Azure データ処理全体アーキテクチャ

Azure のデータ分析基盤は、次のような構成で設計されることが多いです。

```

データ生成  
IoT / Web / アプリ  
↓  
Event Hub / IoT Hub  
↓  
Stream Analytics  
↓  
Cosmos DB / Data Lake  
↓  
Synapse Analytics  
↓  
Power BI / AI / 分析

```

この構造のポイントは次の通りです。

- **リアルタイム処理** → Event Hub / Stream Analytics
- **運用データ保存** → Cosmos DB
- **分析データ保存** → Data Lake
- **分析処理** → Synapse Analytics

---

# Azure Cosmos DB

Azure Cosmos DB は **グローバル分散型 NoSQL データベース**です。

特徴

- 低レイテンシ
- 高スループット
- マルチリージョン
- 自動スケーリング

Cosmos DB は **OLTP（オンライン取引処理）**に最適化されています。

つまり次のような用途です。

- IoT テレメトリ
- Webアプリデータ
- リアルタイムユーザーデータ
- セッションデータ

Cosmos DB の内部構造は次の2種類のストアがあります。

```

Transaction Store  
Analytical Store

```

### Transaction Store

通常のアプリケーションが使用するデータストアです。

特徴

- 高速読み書き
- 低レイテンシ
- OLTP最適化

### Analytical Store

分析用のストアです。

特徴

- カラム指向ストレージ
- 分析クエリ最適化
- Synapseから直接アクセス可能

---

# Azure Synapse Analytics

Azure Synapse Analytics は **データ分析プラットフォーム**です。

主な機能

- SQL Data Warehouse
- Spark 分析
- Data Integration
- BI 分析

Synapse では次のような分析ができます。

- 大規模SQL分析
- データウェアハウス
- 機械学習
- BIレポート

Synapse は **OLAP（オンライン分析処理）**に最適化されています。

つまり

```

Cosmos DB → OLTP  
Synapse → OLAP

```

です。

---

# Azure Synapse Link for Cosmos DB

Synapse Link は、Cosmos DB のデータを **分析用にSynapseと接続する機能**です。

通常、分析を行うにはデータを別のシステムにコピーする必要があります。

例

```

Cosmos DB → Data Lake → Synapse

```

しかし Synapse Link を使うと

```

Cosmos DB  
↓  
Analytical Store  
↓  
Synapse

```

という **データコピーなしの分析**が可能になります。

特徴

- ほぼリアルタイム分析
- OLTPに影響なし
- ETL不要
- Synapseから直接クエリ可能

そのため次の問題では **Synapse Link が正解になります。**

条件

- Cosmos DB データ
- Synapse 分析
- OLTPに影響なし

---

# Azure Data Factory

Azure Data Factory は **データ統合（ETL / ELT）サービス**です。

主な役割

- データコピー
- データ変換
- データパイプライン

例

```

Cosmos DB  
↓  
Data Factory  
↓  
Data Lake

```

特徴

- バッチ処理
- スケジュール実行
- ETLパイプライン

用途

- データウェアハウス構築
- データ統合
- データ移行

ただし Data Factory は

```

リアルタイム分析

```

には向いていません。

---

# Azure Cosmos DB Change Feed

Change Feed は **Cosmos DB のデータ変更イベントのログ**です。

取得できる情報

- データ追加
- データ更新
- データ削除

主な用途

- イベント駆動アーキテクチャ
- リアルタイム処理
- マイクロサービス連携

例

```

Cosmos DB  
↓  
Change Feed  
↓  
Azure Functions

```

ただし Change Feed は

```

分析クエリ

```

のための仕組みではありません。

---

# Azure Event Hub

Event Hub は **大規模イベントストリーミングサービス**です。

主な用途

- IoT データ
- ログ収集
- テレメトリ

特徴

- 高スループット
- ストリーミング処理
- Kafka互換

例

```

IoT Device  
↓  
Event Hub

```

---

# Azure Stream Analytics

Stream Analytics は **リアルタイムストリーム処理サービス**です。

イベントデータをリアルタイムに分析できます。

例

```

Event Hub  
↓  
Stream Analytics  
↓  
Cosmos DB

```

処理例

- フィルタリング
- 集計
- 異常検知

---

# Azure Data Lake Storage

Azure Data Lake は **大規模分析データ保存ストレージ**です。

特徴

- ペタバイト級データ
- 分析最適化
- Hadoop互換

用途

- データウェアハウス
- AI / ML
- BI

---

# Azure データサービス役割整理

|サービス|役割|
|---|---|
Cosmos DB | OLTPデータベース |
Synapse Analytics | 分析エンジン |
Synapse Link | Cosmos DB分析接続 |
Data Factory | データ統合 |
Change Feed | データ変更イベント |
Event Hub | イベントストリーム |
Stream Analytics | リアルタイム分析 |
Data Lake | 分析データストレージ |

---

# 試験での判断パターン

Azure試験では次の判断が頻出です。

### Cosmos DB データを分析

→ **Synapse Link**

---

### データコピー / ETL

→ **Data Factory**

---

### リアルタイムイベント

→ **Event Hub**

---

### ストリーム分析

→ **Stream Analytics**

---

### データ変更イベント

→ **Change Feed**

---

# まとめ

Azure のデータ分析アーキテクチャでは、  
**OLTP（運用処理）と OLAP（分析処理）を分離することが基本設計です。**

Cosmos DB は運用データベースとして使用され、  
Synapse Analytics は分析エンジンとして利用されます。

Synapse Link を使用することで、Cosmos DB のデータを  
**OLTP ワークロードに影響を与えずにリアルタイム分析**することができます。

この構成は Azure のリアルタイムデータ分析基盤で  
最も一般的なパターンの一つです。
