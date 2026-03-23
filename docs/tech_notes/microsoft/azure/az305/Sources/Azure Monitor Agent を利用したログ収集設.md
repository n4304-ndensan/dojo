[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Monitor]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Monitor Agent]]
## Azure Monitor Agent を利用したログ収集設計

### ― Data Collection Rule (DCR) と Data Collection Endpoint (DCE) を用いたログ収集アーキテクチャ ―

---

# 1 背景と問題のコンテキスト

Azure 環境では、仮想マシンやアプリケーションから生成されるログを一元的に収集し、監視・分析を行うために **Azure Monitor** が利用される。Azure Monitor はクラウドおよびオンプレミスのリソースからテレメトリデータを収集し、ログ分析やアラート、可観測性（Observability）を提供する統合監視サービスである。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Log Analytics]]
今回のシナリオでは、Azure サブスクリプションに **WS1 という名前の Log Analytics Workspace** が存在し、パブリックエンドポイントからアクセス可能な構成となっている。このワークスペースに対して、複数リージョンに配置された Windows 仮想マシンからログを収集する必要がある。

対象となる仮想マシンは次の通りである。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
|リージョン|VM数|構成|
|---|---|---|
|Central US|20|Windows Server Core|
|East US|50|Windows Server + IIS Infrastructure|
|West US|50|Windows Server + IIS Application|

これらの VM からログを収集する際には、Azure Monitor の **Azure Monitor Agent（AMA）** を使用する。

しかし、単純にすべてのログを収集すると不要なログデータが大量に取り込まれ、Log Analytics のコストが増加してしまう。そのため今回の要件では

**取り込まれるログデータ量を最小限にする**

という条件が設定されている。

---

# 2 Azure Monitor Agent（AMA）

Azure Monitor Agent は Azure Monitor の新しいログ収集エージェントであり、従来の Log Analytics Agent（MMA）に代わるものとして設計されている。

AMA の特徴は次の通りである。

- Azure Monitor 用の統合エージェント
    
- Windows と Linux の両方に対応
    
- Data Collection Rule による柔軟なログ収集設定
    
- スケーラブルな監視アーキテクチャ
    

AMA の基本構造は次のようになる。

```text
Virtual Machine
      │
      ▼
Azure Monitor Agent
      │
      ▼
Data Collection Rule
      │
      ▼
Log Analytics Workspace
```

ここで重要なのが **Data Collection Rule（DCR）** である。

---

# 3 Data Collection Rule（DCR）

Data Collection Rule は、Azure Monitor Agent がどのログを収集するかを定義するルールである。DCR によって以下を制御できる。

- 収集するログの種類
    
- 収集対象リソース
    
- 送信先の Log Analytics Workspace
    
- フィルタリング条件
    

つまり、DCR はログ収集の **ポリシー定義** として機能する。

DCR の基本構造は次の通り。

```text
Data Collection Rule
 ├ Data Sources
 │   ├ Windows Event Logs
 │   └ IIS Logs
 │
 ├ Resource Scope
 │   └ Target Virtual Machines
 │
 └ Destination
     └ Log Analytics Workspace
```

---

# 4 各リージョンのログ要件

今回のシナリオでは、リージョンごとに収集するログが異なる。

|リージョン|収集ログ|
|---|---|
|Central US|Windows Event Logs|
|East US|Windows Event Logs + IIS Logs|
|West US|IIS Logs|

さらに

**ログ取り込み量を最小化**

する必要があるため、不要なログを収集する設計は避ける必要がある。

---

# 5 DCR 設計の考え方

もし 1 つの DCR を使用すると、次の問題が発生する。

```text
DCR
 ├ Windows Event Logs
 └ IIS Logs
```

この場合

- Central US VM でも IIS ログ収集
    
- West US VM でも Windows Event Logs 収集
    

が発生してしまう。

これは要件

**ログ取り込み量最小化**

に違反する。

---

# 6 最小 DCR 数

リージョンごとにログ収集要件が異なるため、DCR を分ける必要がある。

最小構成は次の通り。

### DCR1

Central US

```text
Collect
 └ Windows Event Logs
```

---

### DCR2

East US

```text
Collect
 ├ Windows Event Logs
 └ IIS Logs
```

---

### DCR3

West US

```text
Collect
 └ IIS Logs
```

したがって必要な DCR の最小数は

**3**

となる。

---

# 7 Data Collection Endpoint（DCE）

Data Collection Endpoint は、Azure Monitor Agent がログを送信するための **エンドポイントリソース**である。

DCE は次の目的で使用される。

- Azure Monitor Agent の接続先
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Private Link]]
- Private Link 対応
    
- データ取り込み経路の管理
    

構造は次の通り。

```text
Virtual Machine
      │
      ▼
Azure Monitor Agent
      │
      ▼
Data Collection Endpoint
      │
      ▼
Log Analytics Workspace
```

---

# 8 DCE の設計

今回のシナリオでは

- WS1 は **Public Endpoint**
    
- Private Link は使用しない
    

そのため、複数 DCE を作成する必要はない。

すべての VM が同じエンドポイントを利用できる。

したがって **DCE の最小数は**

**1**

となる。

---

# 9 完成アーキテクチャ

最終的なログ収集構成は次の通り。

```text
Central US VMs
    │
    ▼
DCR1 (Windows Event Logs)
    │
    ▼

East US VMs
    │
    ▼
DCR2 (Windows + IIS Logs)
    │
    ▼

West US VMs
    │
    ▼
DCR3 (IIS Logs)
    │
    ▼

Azure Monitor Agent
    │
    ▼
Data Collection Endpoint
    │
    ▼
Log Analytics Workspace (WS1)
```

---

# 10 まとめ

今回のシナリオの最適設計は次の通り。

|項目|最小数|
|---|---|
|Data Collection Rule (DCR)|3|
|Data Collection Endpoint (DCE)|1|

理由

- リージョンごとに収集ログが異なる
    
- 不要ログ収集を防ぐ必要がある
    
- WS1 はパブリックエンドポイント
    

この設計により

- ログ取り込み量を最小化
    
- Azure Monitor Agent の管理簡素化
    
- Azure Monitor の効率的な運用
    

を実現できる。