[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Blob Storage]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Factory]]
# Azure Data Factory を用いたオンプレミスファイルサーバーから Azure Blob Storage へのデータ移行設計

（Self-hosted Integration Runtime を利用した安全なデータコピー）

---

# 1 背景

企業ではオンプレミス環境に Windows ファイルサーバーを保有しており、そこに保存されている大量のデータを Azure クラウドに移行する必要がある。今回のシナリオでは、**約 2 TB のデータ**を Azure Blob Storage にコピーする必要がある。

しかし、オンプレミスのファイルサーバーは企業のセキュリティポリシーにより **インターネットに直接公開されていない**。つまり、Azure のサービスが直接ファイルサーバーにアクセスすることはできない。サーバーは企業の **ファイアウォールの背後（内部ネットワーク）**に配置されている。

このような環境では、クラウドサービスが内部ネットワークのデータソースに直接接続することができないため、**安全なブリッジ（ゲートウェイ）**を利用する必要がある。

この問題を解決するために Azure では **Azure Data Factory（ADF）** と **Self-hosted Integration Runtime（SHIR）** を組み合わせて利用することが推奨されている。

---

# 2 Azure Data Factory とは

Azure Data Factory は Microsoft が提供する **クラウドベースのデータ統合サービス**であり、さまざまなデータソース間でのデータ移動やデータ変換を自動化することができる。

主な機能は以下の通りである。

|機能|説明|
|---|---|
|データコピー|異なるデータストア間でデータを移動|
|データ変換|データフローを使用した ETL/ELT 処理|
|ワークフロー管理|パイプラインによるデータ処理のオーケストレーション|
|スケジュール実行|トリガーによる定期処理|

今回のケースでは **Copy Activity（コピーアクティビティ）**を利用してオンプレミスのファイルを Azure Blob Storage にコピーする。

---

# 3 問題のポイント

今回の設計で最も重要なポイントは次の点である。

### ① データソースはオンプレミス

Windows ファイルサーバーは

```
企業ネットワーク内部
```

に存在する。

---

### ② ファイアウォールの背後にある

Azure Data Factory から直接接続することはできない。

つまり

```
Azure → On-premises
```

の直接通信ができない。

---

### ③ インターネット公開されていない

企業ネットワークでは

```
Inbound 接続
```

がブロックされている。

---

# 4 Self-hosted Integration Runtime（SHIR）

この問題を解決するために Azure Data Factory では **Self-hosted Integration Runtime（SHIR）** を使用する。

SHIR はオンプレミス環境にインストールするエージェントであり、Azure Data Factory とオンプレミスデータソースの間の **安全な通信ブリッジ**として機能する。

特徴

|特徴|説明|
|---|---|
|オンプレミス接続|内部ネットワークのデータにアクセス可能|
|アウトバウンド通信|ファイアウォール設定を簡素化|
|安全な通信|HTTPS 通信を使用|
|データコピー処理|ADF パイプラインのコピー処理を実行|

---

# 5 SHIR の通信モデル

重要なポイントは **通信方向**である。

SHIR は Azure に対して **アウトバウンド接続**を確立する。

通信フローは次のようになる。

```
On-premises Server
        │
        ▼
Self-hosted Integration Runtime
        │
        ▼
Azure Data Factory
        │
        ▼
Azure Blob Storage
```

このモデルでは

- インバウンドポートを開ける必要がない
    
- ファイアウォールの変更が最小限
    

というメリットがある。

---

# 6 Azure Data Factory の構成要素

ADF パイプラインは複数の構成要素から成り立っている。

|コンポーネント|役割|
|---|---|
|Linked Service|データソース接続定義|
|Dataset|データ構造定義|
|Pipeline|処理のワークフロー|
|Activity|実際の処理（Copyなど）|

---

# 7 Linked Service

Linked Service は

```
接続情報
```

を定義する。

今回のシナリオでは 2つ必要になる。

### オンプレミスファイルサーバー

接続

```
Windows File Share
+
Self-hosted Integration Runtime
```

---

### Azure Blob Storage

接続

```
Azure Blob Storage
```

---

# 8 Dataset

Dataset は

```
実際のデータ構造
```

を定義する。

例

|Dataset|説明|
|---|---|
|Source Dataset|オンプレミスフォルダ|
|Sink Dataset|Blob コンテナ|

---

# 9 Pipeline

Pipeline は

```
データ処理のワークフロー
```

を定義する。

今回の処理は

```
Copy Activity
```

である。

データフロー

```
On-premises File Server
        │
        ▼
Copy Activity
        │
        ▼
Azure Blob Storage
```

---

# 10 正しい実行手順

オンプレミスのデータを Azure Blob Storage にコピーする正しい順序は次の通りである。

### Step 1

Self-hosted Integration Runtime をインストールする。

オンプレミスネットワークにある Windows サーバーに SHIR をインストールする。

---

### Step 2

Linked Service を作成する。

次の接続を定義する。

- On-premises File Share
    
- Azure Blob Storage
    

---

### Step 3

Pipeline を作成する。

Copy Activity を使用して

```
File Server → Blob Storage
```

のコピー処理を定義する。

---

# 11 他の選択肢が不適切な理由

### B

Azure File Sync

Azure File Sync は

```
Azure Files
```

との同期サービスであり、Blob Storage コピーには適さない。

---

### C

File Server Resource Manager + SSIS

これはオンプレミス ETL ソリューションであり

```
Azure Data Factory
```

とは関係がない。

---

### D

Azure Integration Runtime

Azure Integration Runtime は

```
クラウド内データ
```

専用であり、オンプレミスのデータソースにはアクセスできない。

---

# 12 データ転送のアーキテクチャ

最終的な構成は次のようになる。

```
On-premises File Server
        │
        ▼
Self-hosted Integration Runtime
        │
        ▼
Azure Data Factory Pipeline
        │
        ▼
Copy Activity
        │
        ▼
Azure Blob Storage
```

---

# 13 大容量データ転送のポイント

今回のデータ量は **2 TB** である。

大容量コピーでは次の設定が重要になる。

|項目|説明|
|---|---|
|Parallel Copy|並列コピー|
|Retry Policy|再試行設定|
|Compression|圧縮|
|Incremental Copy|増分コピー|

これにより転送パフォーマンスを最適化できる。

---

# 14 最終回答

正解

```
A
Self-hosted Integration Runtime をインストールする
→ Pipeline を作成する
→ Linked Service を定義する
```

---

# 15 まとめ

今回の設計ポイントは以下の通りである。

|要件|解決方法|
|---|---|
|オンプレミス接続|Self-hosted Integration Runtime|
|データ移行|Azure Data Factory|
|コピー処理|Copy Activity|
|保存先|Azure Blob Storage|

つまり今回の最適な構成は

```
Self-hosted Integration Runtime
+
Azure Data Factory Pipeline
+
Azure Blob Storage
```

である。

この方法は **オンプレミスデータを Azure に移行する際の標準的なアーキテクチャ**として Microsoft が推奨している。