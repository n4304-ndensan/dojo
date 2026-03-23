[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
# Azure VM 上で SQL Server を高性能かつ低コストで運用する設計

（20,000 IOPS + SR-IOV を満たす VM シリーズとディスク選択）

---

# 1 背景（シナリオ）

企業が Azure 上で **SQL Server を IaaS（Azure Virtual Machine）として運用する場合**、次のような要件が発生することが多い。

今回の問題では、SQL Server を Azure VM 上にデプロイする際に以下の条件を満たす必要がある。

### 要件

1. **20,000 IOPS のディスク性能**
    
2. **SR-IOV（Accelerated Networking）サポート**
    
3. **コストを最小限に抑える**
    
4. SQL Server ワークロードに適した VM を選択
    

この要件を満たすためには

- **VM シリーズ**
    
- **ディスクタイプ**
    

の両方を適切に選択する必要がある。

---

# 2 Azure VM での SQL Server アーキテクチャ

Azure IaaS で SQL Server を運用する場合、典型的な構成は次のようになる。

```
Azure Virtual Machine
      │
      ├ OS Disk
      │
      ├ Data Disk (Database files)
      │
      └ Log Disk (Transaction logs)
```

SQL Server のパフォーマンスは主に以下に依存する。

|要素|影響|
|---|---|
|VM CPU / Memory|クエリ処理性能|
|Disk IOPS|データ読み書き|
|Disk Throughput|大量データ処理|
|Network|レプリケーションや HA|

今回の問題では **ディスク IOPS とネットワーク性能**が重要になる。

---

# 3 IOPS（Input Output Operations Per Second）

IOPS はストレージ性能を表す最も重要な指標の一つである。

SQL Server では次の処理が頻繁に行われる。

- データページ読み込み
    
- トランザクションログ書き込み
    
- インデックス操作
    

そのため

```
IOPS不足
↓
クエリ遅延
↓
DBパフォーマンス低下
```

が発生する。

今回の要件では

```
20,000 IOPS
```

が必要となる。

---

# 4 SR-IOV（Single Root I/O Virtualization）

SR-IOV は Azure の **Accelerated Networking** 技術の基盤となる。

目的

```
ネットワーク仮想化オーバーヘッド削減
```

通常の仮想 NIC では

```
VM
↓
Hypervisor
↓
Network
```

という経路になる。

SR-IOV を使用すると

```
VM
↓
直接 NIC
```

となる。

結果

|効果|説明|
|---|---|
|低レイテンシ|高速通信|
|CPU削減|仮想化オーバーヘッド減|
|高スループット|大規模処理対応|

---

# 5 Azure VM シリーズ概要

Azure VM は用途別にシリーズが分かれている。

|シリーズ|用途|
|---|---|
|D シリーズ|汎用|
|E シリーズ|メモリ最適化|
|N シリーズ|GPU|
|L シリーズ|ストレージ最適化|

SQL Server は一般的に

```
メモリ最適化
```

VM が推奨される。

---

# 6 ESv4 シリーズ

ESv4 シリーズは

```
Eシリーズ（メモリ最適化）
```

VM の第4世代である。

特徴

|特徴|説明|
|---|---|
|高メモリ|SQL Server 向き|
|高IO性能|Premium Disk 対応|
|Accelerated Networking|SR-IOV対応|

そのため

```
SQL Server ワークロード
```

に適している。

---

# 7 他の VM シリーズの問題

### DSv3

DSv3 は汎用 VM である。

問題

- IOPS 制限が低い
    
- SQL Server 高負荷に最適ではない
    

---

### NCv3

NCv3 は

```
GPU コンピューティング
```

用である。

用途

- AI
    
- CUDA
    
- HPC
    

SQL Server 用ではないため

```
コストが高すぎる
```

---

### NVv4

NVv4 は

```
GPU + グラフィック
```

用途。

例

- VDI
    
- GPUレンダリング
    

SQL Server 用ではない。

---

# 8 Azure ディスクタイプ

Azure VM のディスクは次の種類がある。

|ディスク|特徴|
|---|---|
|Standard HDD|低コスト|
|Standard SSD|中性能|
|Premium SSD|高性能|
|Ultra Disk|最高性能|

---

# 9 Premium SSD

Premium SSD は Azure VM の一般的な高性能ディスクである。

特徴

|特徴|説明|
|---|---|
|高IOPS|最大20k以上|
|低レイテンシ|DB向け|
|コスト|Ultraより安い|

SQL Server ワークロードでは

```
Premium SSD
```

がよく使用される。

---

# 10 Ultra Disk

Ultra Disk は

```
最高性能ストレージ
```

である。

特徴

|特徴|説明|
|---|---|
|IOPS|160,000|
|低レイテンシ|非常に高速|
|コスト|非常に高い|

問題

```
今回の要件
↓
20,000 IOPS
```

なので

```
Ultra Diskは過剰性能
```

となる。

---

# 11 Standard SSD

Standard SSD は

```
一般用途
```

ディスクである。

問題

```
IOPS不足
```

SQL Server には適さない。

---

# 12 最適構成

要件

|条件|必要|
|---|---|
|20,000 IOPS|Premium SSD|
|SR-IOV|Accelerated Networking|
|SQL Server 最適|Eシリーズ|

そのため

```
VM: ESv4
Disk: Premium SSD
```

が最適となる。

---

# 13 最終アーキテクチャ

```
Azure VM (ESv4)
      │
      ├ Premium SSD (Data)
      │
      ├ Premium SSD (Log)
      │
      └ Accelerated Networking (SR-IOV)
```

---

# 14 他の選択肢が不適切な理由

|選択肢|問題|
|---|---|
|A DSv3|SQL Server に最適でない|
|B NCv3|GPU用途で高コスト|
|C NVv4 + Standard SSD|IOPS不足|
|D ESv4 + Premium SSD|最適|

---

# 15 最終回答

```
D
VM: ESv4
Disk: Premium SSD
```

---

# 16 まとめ

SQL Server を Azure VM で高性能に運用する場合、次の 3 点が重要である。

|要素|推奨|
|---|---|
|VMシリーズ|Eシリーズ|
|ディスク|Premium SSD|
|ネットワーク|Accelerated Networking|

今回の要件

```
20,000 IOPS
SR-IOV
低コスト
```

を満たす最適構成は

```
ESv4 + Premium SSD
```

となる。