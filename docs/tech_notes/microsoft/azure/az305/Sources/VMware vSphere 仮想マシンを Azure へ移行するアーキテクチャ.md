## VMware vSphere 仮想マシンを Azure へ移行するアーキテクチャ

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Migrate]]
### ― Azure Migrate Project を利用した大規模 VM 移行設計 ―

---

# 1 概要

企業がオンプレミス環境からクラウドへ移行する際、最も一般的なシナリオの一つが **VMware 仮想マシンの Azure への移行**である。Azure は VMware 環境を Azure 仮想マシンとして実行できるようにするための複数の移行ツールを提供しており、その中心となるサービスが **Azure Migrate** である。

今回のシナリオでは次の環境が存在する。

オンプレミス環境

- データセンター **Site1**
    
- **VMware vSphere クラスター Cluster1**
    
- 約 **100 台の仮想マシン**
    
- VMware **vCenter で管理**
    

Azure 環境

- Azure サブスクリプション **Sub1**
    

要件

- Cluster1 の仮想マシンを **Azure に移行**
    
- Azure で VM の実行をサポートするためのリソースを事前に準備
    
- **管理オーバーヘッドを最小化**
    

この条件を満たす最初のアクションは

**Azure Migrate Project を作成する**

である。

---

# 2 背景

オンプレミスの VMware 仮想マシンを Azure へ移行する場合、単純に VM をコピーするだけでは移行できない。Azure 上で VM を実行するためには次の処理が必要になる。

- 仮想マシンの検出
    
- インフラ評価
    
- サイズ推定
    
- 依存関係分析
    
- 移行計画
    

これらを管理するための統合サービスが **Azure Migrate** である。

Azure Migrate はクラウド移行の中心サービスとして、次の機能を提供する。

- サーバー検出
    
- 移行評価
    
- 移行オーケストレーション
    
- 移行進行状況の管理
    

---

# 3 Azure Migrate の仕組み

Azure Migrate は複数の移行ツールを統合したサービスであり、移行作業を **Azure Portal から一元管理**できる。

基本構造

```text
On-Premises Datacenter
----------------------

VMware vSphere
      │
      ▼
Azure Migrate Appliance
      │
      ▼
Azure Migrate Project
      │
      ▼
Azure Virtual Machines
```

Azure Migrate の中心にあるのが **Azure Migrate Project** であり、このプロジェクトが移行管理のコンテナとして機能する。

---

# 4 Azure Migrate Project

Azure Migrate Project は移行プロセスを管理するための **論理的な管理リソース**である。

役割

- 移行対象サーバーの管理
    
- 移行評価の実行
    
- 移行ツールの統合
    
- 進行状況の監視
    

構造

```text
Azure Migrate Project
        │
        ├ Discovery
        ├ Assessment
        └ Migration
```

つまり、Azure Migrate Project を作成しない限り

- VM 検出
    
- 評価
    
- 移行
    

を管理することができない。

そのため **最初のステップは Azure Migrate Project 作成**となる。

---

# 5 VMware 仮想マシン移行の流れ

Azure Migrate を使った VMware VM 移行は次の順序で行う。

### Step 1

Azure Migrate Project 作成

```text
Azure Subscription
        │
        ▼
Azure Migrate Project
```

---

### Step 2

Azure Migrate Appliance デプロイ

```text
VMware Environment
        │
        ▼
Azure Migrate Appliance
```

Appliance は VMware 環境を検出する。

---

### Step 3

サーバー検出

```text
vCenter
   │
   ▼
VM discovery
```

---

### Step 4

移行評価

Azure は次の情報を評価する。

- CPU
    
- RAM
    
- Disk
    
- ネットワーク
    
- 依存関係
    

---

### Step 5

Azure VM へ移行

```text
VMware VM
      │
      ▼
Replication
      │
      ▼
Azure VM
```

---

# 6 Azure Migrate Appliance

Azure Migrate Appliance はオンプレミスに配置する仮想アプライアンスである。

役割

- VMware VM の検出
    
- パフォーマンス収集
    
- 移行評価
    

構造

```text
VMware vCenter
      │
      ▼
Azure Migrate Appliance
      │
      ▼
Azure Migrate Project
```

しかし、このアプライアンスは **Azure Migrate Project が存在して初めて接続できる**。

したがって

- 先に Project 作成
    
- 次に Appliance
    

という順序になる。

---

# 7 Azure VMware Solution が不適な理由

Azure VMware Solution は VMware 環境を Azure 上でそのまま実行するサービスである。

構造

```text
Azure Datacenter
      │
      ▼
VMware Cloud
```

しかしこのサービスは

- VMware 管理ツールをそのまま使用
    
- 専用インフラ
    
- 高コスト
    

という特徴があり、今回の要件である

**管理オーバーヘッド最小**

には適さない。

---

# 8 Azure VMware Solution Host

AVS Host は Azure VMware Solution のインフラ構成要素であり、VMware SDDC を構成するためのリソースである。

用途

```text
VMware Environment in Azure
```

しかし今回の目的は

**VMware VM を Azure VM に移行する**

ことであり、AVS は必要ない。

---

# 9 推奨アーキテクチャ

今回のシナリオの推奨構成

```text
On-Prem Datacenter
------------------

VMware Cluster (Cluster1)
        │
        ▼
Azure Migrate Appliance
        │
        ▼
Azure Migrate Project
        │
        ▼
Azure Virtual Machines
        │
        ▼
Azure Virtual Network
```

この構成では

- VMware VM を検出
    
- Azure VM サイズを評価
    
- Azure へ移行
    

が自動化される。

---

# 10 Azure Architect 設計指針

VMware VM を Azure に移行する際の基本フロー

|ステップ|操作|
|---|---|
|1|Azure Migrate Project 作成|
|2|Azure Migrate Appliance デプロイ|
|3|VM 検出|
|4|移行評価|
|5|Azure VM 移行|

---

# 11 試験ポイント（AZ-305）

AZ-305 試験では次の判断が重要。

VMware VM を Azure VM に移行する場合

```text
Azure Migrate
```

最初に作成するリソース

```text
Azure Migrate Project
```

---

# 12 まとめ

今回の要件

- VMware クラスター
    
- 約 100 VM
    
- Azure へ移行
    
- 管理オーバーヘッド最小
    

この条件を満たす最初のアクションは

**Azure Migrate Project を作成する**

である。

アーキテクチャ

```text
VMware vSphere
      │
      ▼
Azure Migrate Appliance
      │
      ▼
Azure Migrate Project
      │
      ▼
Azure Virtual Machines
```

Azure Migrate Project は Azure 移行の管理コンテナとして機能し、VMware 仮想マシンの検出、評価、移行を一元的に管理できる。