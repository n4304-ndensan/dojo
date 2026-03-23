[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Factory]]
# Azure Data Factory 大容量データコピー設計

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Synapse Analytics]]
（オンプレミス SQL Server → Azure Synapse Analytics）

---

# 1 背景

ある企業では、オンプレミス環境に存在する **SQL Server データベース**から  
クラウド上の **Azure Synapse Analytics** にデータをコピーする必要がある。

このデータ移行は **Azure Data Factory (ADF)** を利用して実装される。

しかし次の課題が存在する。

- データ量が非常に大きい
    
- ETL 処理時間を短縮する必要がある
    
- コピー処理のスループットを最大化する必要がある
    

このシナリオでは

**Self-hosted Integration Runtime + Parallel Copy**

を使用するのが最適である。

---

# 2 Azure Data Factory 概要

Azure Data Factory は **データ統合サービス**であり、ETL / ELT パイプラインを構築できる。

### ADFの基本構成

```text
Pipeline
   │
   ├ Activities
   │
   └ Data Movement
```

主な用途

- データコピー
    
- ETL
    
- データ統合
    
- ワークフロー管理
    

---

# 3 Integration Runtime (IR)

Integration Runtime は **データ移動と処理の実行エンジン**である。

ADFには3種類の IR がある。

|IRタイプ|用途|
|---|---|
|Azure IR|クラウドデータソース|
|Self-hosted IR|オンプレミスデータ|
|Azure-SSIS IR|SSISパッケージ|

---

# 4 Self-hosted Integration Runtime

オンプレミス環境にあるデータソースにアクセスするためには

**Self-hosted Integration Runtime (SHIR)** を使用する。

### SHIRの役割

```text
Azure Data Factory
        │
        ▼
Self-hosted Integration Runtime
        │
        ▼
On-prem SQL Server
```

SHIRは

- オンプレミスサーバー
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
- VM
    
- 社内ネットワーク
    

などにインストールされる。

---

# 5 データコピーの問題

大量データコピーでは次の問題が発生する。

### 逐次コピー

```text
Copy Thread
   │
   ▼
Data Transfer
```

問題

- スループット低い
    
- 転送時間長い
    

---

# 6 Parallel Copy

Parallel Copy は複数スレッドでデータをコピーする。

### 並列処理

```text
Copy Threads
   │
   ├ Thread1
   ├ Thread2
   ├ Thread3
   └ Thread4
```

メリット

- 高速データ転送
    
- CPU利用効率
    
- ネットワーク活用
    

---

# 7 Parallel Copy アーキテクチャ

```text
On-prem SQL Server
       │
       ▼
Self-hosted IR
       │
       ├ Parallel Worker 1
       ├ Parallel Worker 2
       ├ Parallel Worker 3
       └ Parallel Worker 4
       │
       ▼
Azure Synapse Analytics
```

これにより

- 同時読み込み
    
- 同時書き込み
    

が可能になる。

---

# 8 Azure Synapse Analytics

Synapse は **分散型データウェアハウス**である。

特徴

|特徴|説明|
|---|---|
|MPP|Massively Parallel Processing|
|分散テーブル|大規模分析|
|PolyBase|高速ロード|

Parallel Copy は Synapse の **MPP構造**と相性が良い。

---

# 9 パフォーマンス最適化

Parallel Copy のチューニング要素

### 並列度

```text
parallelCopies
```

### データ分割

例

- ID range
    
- Date range
    
- Partition
    

---

### SHIR性能

重要な要素

|リソース|影響|
|---|---|
|CPU|スレッド処理|
|Memory|バッファ|
|Network|転送速度|

---

# 10 他の選択肢の評価

## Azure IR

問題

- オンプレミスSQLに直接アクセス不可
    

---

## Azure-SSIS IR

用途

- SSISパッケージ実行
    

問題

- 単純コピーには過剰
    

---

## Sequential Copy

問題

- 単一スレッド
    
- 非効率
    

---

# 11 推奨アーキテクチャ

```text
Azure Data Factory
        │
        ▼
Pipeline
        │
        ▼
Self-hosted IR
        │
        ▼
On-prem SQL Server
        │
        ▼
Parallel Copy
        │
        ▼
Azure Synapse Analytics
```

---

# 12 実際のデータフロー

```text
SQL Server Tables
        │
        ▼
ADF Copy Activity
        │
        ▼
Self-hosted IR
        │
        ├ Worker1
        ├ Worker2
        ├ Worker3
        └ Worker4
        │
        ▼
Azure Synapse DW
```

---

# 13 メリット

### 高速データ移動

Parallel Copy

---

### セキュア接続

オンプレミス → Azure

---

### スケーラビリティ

Synapse MPP

---

# 14 まとめ

今回の要件

- オンプレミスSQL Server
    
- 大容量データコピー
    
- 高速転送
    

最適な構成は

**Self-hosted Integration Runtime + Parallel Copy**

である。

この構成により

- 高スループット
    
- 安全な接続
    
- 大規模データ移動
    

を実現できる。