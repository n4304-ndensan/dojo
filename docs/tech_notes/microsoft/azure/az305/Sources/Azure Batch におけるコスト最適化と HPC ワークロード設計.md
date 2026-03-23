[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Batch]]
# Azure Batch におけるコスト最適化と HPC ワークロード設計

（開発タスクと MPI 本番ジョブのプール構成）

---

# 1 背景（シナリオ）

Azure Batch は、大規模な並列処理や HPC（High Performance Computing）ワークロードを実行するためのマネージドサービスである。

Batch は主に次の用途で利用される。

- 大規模なデータ処理
    
- HPC シミュレーション
    
- レンダリング処理
    
- 機械学習トレーニング
    

今回のシナリオでは Azure Batch を使用して **2 種類のワークロード**を実行する必要がある。

|ワークロード|特徴|
|---|---|
|Dev タスク|短時間・頻繁に実行|
|MPI 本番ジョブ|長時間・高信頼性|

さらに次の要件がある。

### 要件

1. コストを最小化する
    
2. 可能な限り **Azure Hybrid Benefit（AHB）** を利用する
    
3. MPI 本番ジョブの安定性を確保する
    

この要件に基づいて **Batch プール構成**を選択する必要がある。

---

# 2 Azure Batch の基本アーキテクチャ

Azure Batch の処理モデルは次のようになる。

```text
Batch Account
     │
     ▼
Pool
     │
     ▼
Compute Nodes
     │
     ▼
Tasks / Jobs
```

主な構成要素

|コンポーネント|説明|
|---|---|
|Batch Account|Batch サービス管理|
|Pool|計算ノード集合|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
|Compute Node|実行 VM|
|Job|タスクの集合|
|Task|実行処理|

---

# 3 Azure Batch プールタイプ

Azure Batch では **2 種類のプール構成**が存在する。

|プールタイプ|説明|
|---|---|
|Batch Service Pool|Batch 管理 VM|
|User Subscription Pool|ユーザーサブスクリプション VM|

---

# 4 Batch Service Pool

Batch Service Pool は Azure Batch が VM を管理する方式である。

特徴

|特徴|説明|
|---|---|
|簡単セットアップ|Batch が VM 作成|
|管理負荷|低|
|Azure Hybrid Benefit|利用不可|

つまり

```text
管理は簡単
↓
ライセンス最適化はできない
```

---

# 5 User Subscription Pool

User Subscription Pool は VM を **ユーザーサブスクリプション内に作成する方式**である。

特徴

|特徴|説明|
|---|---|
|VM 管理|ユーザー|
|AHB|利用可能|
|柔軟性|高|

つまり

```text
Azure Hybrid Benefit
```

を使用できる。

---

# 6 Azure Hybrid Benefit

Azure Hybrid Benefit は

```text
既存の Windows / SQL ライセンス
↓
Azure で再利用
```

できる仕組みである。

これにより

```text
VM ライセンスコスト削減
```

が可能になる。

---

# 7 Azure Batch VM 種類

Batch では VM 種類も重要である。

|VM タイプ|説明|
|---|---|
|Dedicated VM|常に確保|
|Low Priority VM|余剰リソース利用|

---

# 8 Low Priority VM

Low Priority VM（現在は Spot VM と呼ばれる）は

```text
Azure の余剰リソース
```

を利用する VM である。

特徴

|特徴|説明|
|---|---|
|コスト|非常に安い|
|可用性|保証なし|
|中断|発生する可能性|

つまり

```text
安いが不安定
```

である。

---

# 9 Dedicated VM

Dedicated VM は通常の VM である。

特徴

|特徴|説明|
|---|---|
|可用性|保証|
|安定性|高|
|コスト|高|

MPI ワークロードでは

```text
Dedicated VM
```

が推奨される。

---

# 10 MPI ジョブの特徴

MPI（Message Passing Interface）は HPC 並列処理で利用される。

特徴

|特徴|説明|
|---|---|
|長時間ジョブ|数時間〜数日|
|同期処理|ノード連携|
|中断|致命的|

つまり

```text
ノード中断 = ジョブ失敗
```

となる。

---

# 11 Dev タスクの特徴

Dev タスクは

- 短時間
    
- テスト用途
    
- 再実行可能
    

つまり

```text
中断しても問題ない
```

ケースが多い。

---

# 12 最適構成

要件を整理する。

|ワークロード|必要条件|
|---|---|
|Dev|低コスト|
|Production MPI|高信頼性|
|全体|Azure Hybrid Benefit|

---

# 13 Dev プール

Dev タスクは

```text
中断可能
```

なので

```text
Low Priority VM
```

を使用する。

さらに管理負荷を減らすため

```text
Batch Service Pool
```

を使用する。

---

# 14 Production プール

MPI 本番ジョブは

```text
安定性
```

が必要である。

そのため

```text
Dedicated VM
```

を使用する。

さらに

```text
Azure Hybrid Benefit
```

を利用するため

```text
User Subscription Pool
```

を使用する。

---

# 15 最終構成

```text
Azure Batch

Dev Pool
 ├ Batch Service Pool
 └ Low Priority VM

Production Pool
 ├ User Subscription Pool
 └ Dedicated VM
```

---

# 16 他の選択肢が不適切な理由

### A

Dev → User Subscription

Dev 環境で AHB を使う必要性は低く

管理負荷が増える。

---

### C

Production → Low Priority

MPI ジョブでは

```text
VM 中断
```

が発生すると

```text
ジョブ失敗
```

になる。

---

### D

Production → Low Priority

本番 HPC ワークロードには適さない。

---

# 17 最終回答

正解

```text
B

開発: Batch Service + Low Priority VM  
本番: User Subscription + Dedicated VM
```

---

# 18 まとめ

今回の問題の重要ポイントは **Azure Batch のコスト最適化設計**である。

|ワークロード|プール|VM|
|---|---|---|
|Dev|Batch Service|Low Priority|
|Production|User Subscription|Dedicated|

これにより

- Dev 環境 → **低コスト**
    
- Production → **高信頼性 + AHB**
    

を両立できる。

これは **Azure Batch HPC ワークロードの標準設計パターン**である。