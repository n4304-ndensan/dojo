[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Kubernetes Service (AKS)]]
# Azure Kubernetes Service (AKS) スケーリング設計

（Windows + Linux コンテナ / バーストワークロード）

---

# 1 背景

ある組織では **Azure Kubernetes Service (AKS)** を利用してコンテナ基盤を構築する予定である。  
このクラスターは次の特徴を持つ。

- **Windows コンテナ**
    
- **Linux コンテナ**
    

両方を同一クラスターで実行する **ハイブリッド AKS クラスター** である。

また、アプリケーションのトラフィックは一定ではなく、

- 短時間で急激に負荷が増える
    
- バースト的なワークロードが発生する
    

という特性を持つ。

---

# 2 要件

今回の設計では以下の要件を満たす必要がある。

### 技術要件

1. **迅速なスケーリング**
    
2. **バースト負荷への対応**
    
3. **ノードプロビジョニング時間の最小化**
    
4. **Windows + Linux コンテナの混在**
    

---

# 3 Kubernetesのスケーリングレイヤ

Kubernetes には複数のスケーリング方式が存在する。

|スケーリング|説明|
|---|---|
|Pod スケーリング|Pod 数を増減|
|Node スケーリング|ノード数を増減|
|Event スケーリング|外部イベントでスケール|

---

# 4 主なスケーリング機能

## Horizontal Pod Autoscaler (HPA)

Pod数を増減する。

### 例

CPU 使用率が 80% を超えると Pod を増やす。

```text
Pods
 │
 ├ Pod1
 ├ Pod2
 └ Pod3
```

しかし HPA は **ノードを追加しない**。

ノード容量が足りない場合

```text
Pending Pods
```

が発生する。

---

## Cluster Autoscaler

ノード数を増減する。

```text
Cluster
 │
 ├ Node1
 ├ Node2
 └ Node3
```

負荷が増えると

```text
Cluster
 │
 ├ Node1
 ├ Node2
 ├ Node3
 ├ Node4
 └ Node5
```

問題

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
- **VM起動に数分かかる**
    

---

## KEDA

イベント駆動スケーリング。

例

- Queue length
    
- Kafka events
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Service Bus]]
- Service Bus
    

しかし

- Podスケーリングのみ
    
- ノード問題は解決しない
    

---

# 5 バースト問題

通常のAKSスケーリングには問題がある。

### ノード起動時間

VM起動

```text
2〜5分
```

バーストワークロードでは

```text
秒単位スケール
```

が必要になる。

---

# 6 Virtual Nodes

この問題を解決するのが

**AKS Virtual Nodes**

である。

Virtual Node は

```text
Azure Container Instances (ACI)
```

を利用して Pod を実行する。

---

# 7 Virtual Node アーキテクチャ

通常の AKS

```text
AKS Cluster
 │
 ├ Node VM
 ├ Node VM
 └ Node VM
```

Virtual Node 有効

```text
AKS Cluster
 │
 ├ Node VM
 ├ Node VM
 ├ Node VM
 │
 └ Virtual Node (ACI)
```

Podは

- VMノード
    
- ACIノード
    

どちらにもスケジュールされる。

---

# 8 ACIバーストスケーリング

負荷増加時

```text
Pods
 │
 ├ VM Node
 ├ VM Node
 └ VM Node
```

バースト発生

```text
Pods
 │
 ├ VM Node
 ├ VM Node
 ├ VM Node
 └ ACI Virtual Node
```

ACIは

- VM起動不要
    
- 数秒で起動
    

---

# 9 スケールフロー

```text
Traffic Spike
      │
      ▼
Pods requested
      │
      ▼
AKS Scheduler
      │
      ├ Node pool
      └ Virtual Node (ACI)
```

ACIが即座にコンテナを実行する。

---

# 10 Windows + Linux コンテナ

AKS は複数ノードプールを持つ。

```text
AKS
 │
 ├ Linux Node Pool
 │
 └ Windows Node Pool
```

Virtual Node も利用可能である。

---

# 11 アーキテクチャ例

```text
Users
 │
 ▼
Application Gateway
 │
 ▼
AKS Cluster
 │
 ├ Linux Node Pool
 │
 ├ Windows Node Pool
 │
 └ Virtual Node (ACI)
```

通常負荷

- Node Pool
    

バースト

- ACI
    

---

# 12 Virtual Node のメリット

### 超高速スケール

VM起動不要

---

### コスト最適化

通常

```text
少数ノード
```

ピーク

```text
ACI利用
```

---

### シンプル設計

Cluster Autoscaler不要でもバースト対応可能

---

# 13 他の選択肢の評価

## Cluster Autoscaler

ノード追加

問題

- VM起動遅い
    

---

## Horizontal Pod Autoscaler

Podスケール

問題

- ノード不足
    

---

## Manual Scaling

問題

- 手動
    
- 自動化不可
    

---

## KEDA

イベントスケール

問題

- ノード追加不可
    

---

# 14 推奨アーキテクチャ

```text
Users
 │
 ▼
Load Balancer
 │
 ▼
AKS Cluster
 │
 ├ Linux Node Pool
 │
 ├ Windows Node Pool
 │
 └ Virtual Node (ACI Burst)
```

---

# 15 まとめ

今回の要件

- Windows + Linux コンテナ
    
- バーストワークロード
    
- 高速スケール
    

最適な機能は

**AKS Virtual Nodes + Azure Container Instances**

である。

これにより

- VM起動待ちを回避
    
- 秒単位スケール
    
- コスト効率
    

を実現できる。