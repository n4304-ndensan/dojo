[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Private Link]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Monitor]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Monitor Private Link Scope (AMPLS)]]
## Azure Monitor Private Link Scope（AMPLS）設計ガイド

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Log Analytics]]
### ― Log Analytics Workspace へのログ取り込みを Microsoft バックボーンに閉じる設計 ―

---

# 1 概要

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Private Endpoint]]
このシナリオは、Azure Monitor / Log Analytics へのログ通信を **パブリック経由ではなく Microsoft のバックボーンネットワークだけで流したい**、という要件に対して、**Azure Monitor Private Link Scope（AMPLS）** と **Private Endpoint** をどう配置するかを問う問題である。Azure Monitor Private Link の実装手順としては、まず **AMPLS を作成し、そこに Log Analytics Workspace を接続し、その AMPLS を Private Endpoint に接続する**という構成になる。 ([Microsoft Learn](https://learn.microsoft.com/en-us/azure/azure-monitor/fundamentals/private-link-configure?utm_source=chatgpt.com "Configure private link for Azure Monitor"))

この問題では、答えが2段階に分かれている。

- **AMPLS の最小数** → **1**
    
- **Private Endpoint の最小数** → **2**
    

この2つは似ているようで意味が違う。  
**AMPLS は論理的な管理コンテナ**であり、**Private Endpoint は実際に各ネットワークに生えるプライベートIP付きの入口**である。したがって、Workspace が1つなら AMPLS も1つで足りるが、接続元ネットワークの到達性によっては Private Endpoint は複数必要になる。 ([Microsoft Learn](https://learn.microsoft.com/en-us/azure/azure-monitor/fundamentals/private-link-configure?utm_source=chatgpt.com "Configure private link for Azure Monitor"))

---

# 2 背景

今回の環境には次の要素がある。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure DNS]]
- **Private DNS zone**: `apexcore.com`
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Network (VNet)]]
- **VNet1**
    
    - `apexcore.com` にリンク
        
    - VNet2 とピアリング
        
- **VNet2**
    
    - `apexcore.com` にリンク
        
    - VNet1 とピアリング
        
- **VNet3**
    
    - `apexcore.com` にリンク
        
    - VNet1 / VNet2 と接続なし
        
- **Workspace1**
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
    - 3つの VNet 上の VM から収集したログを保存する Log Analytics Workspace
        

さらに要件は次の2つである。

1. VM から Workspace1 に送るログデータは **Microsoft backbone network** のみを通ること
    
2. **管理オーバーヘッドを最小化**すること
    

ここで重要なのは、普通に Log Analytics Workspace を使うだけでは、監視通信は Azure Monitor の公開エンドポイントに向かう設計になりうるという点である。Azure Monitor Private Link を使うことで、Azure Monitor への通信を **Private Endpoint 経由**に切り替えられる。Azure Private Link は、VNet 内のプライベートIPを使って PaaS サービスへ接続し、通信は Azure バックボーン上で完結する。 ([Microsoft Learn](https://learn.microsoft.com/en-us/azure/azure-monitor/fundamentals/private-link-security?utm_source=chatgpt.com "Use Azure Private Link to connect networks to Azure Monitor"))

---

# 3 まず AMPLS とは何か

AMPLS は **Azure Monitor Private Link Scope** の略で、Azure Monitor の Private Link 構成を束ねるための **論理スコープ**である。Microsoft Learn でも、Azure Monitor Private Link の構成は以下の3ステップで説明されている。

1. AMPLS を作成する
    
2. Azure Monitor リソース（例: Log Analytics Workspace）を AMPLS に接続する
    
3. AMPLS を Private Endpoint に接続する ([Microsoft Learn](https://learn.microsoft.com/en-us/azure/azure-monitor/fundamentals/private-link-configure?utm_source=chatgpt.com "Configure private link for Azure Monitor"))
    

つまり、AMPLS 自体はネットワークインターフェースではない。  
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Application Insights]]
実際に VNet の中にプライベートIPを持って存在するのは Private Endpoint 側である。AMPLS は「どの Workspace や Application Insights を Private Link 対象にするか」をまとめる管理単位だと理解するとよい。 ([Microsoft Learn](https://learn.microsoft.com/en-us/azure/azure-monitor/fundamentals/private-link-configure?utm_source=chatgpt.com "Configure private link for Azure Monitor"))

構造としてはこうなる。

```text
Virtual Machines
      │
      ▼
Private Endpoint
      │
      ▼
AMPLS
      │
      ▼
Log Analytics Workspace
```

ここから分かる重要ポイントは、**Workspace1 が1つなら、基本的に AMPLS も1つでよい**ということだ。複数 VNet があっても、同じ監視対象に対して別々の AMPLS を増やす必要はない。むしろ管理オーバーヘッドが増える。 ([Microsoft Learn](https://learn.microsoft.com/en-us/azure/azure-monitor/fundamentals/private-link-configure?utm_source=chatgpt.com "Configure private link for Azure Monitor"))

---

# 4 なぜ AMPLS は 1つでよいのか

今回の要件では、全VMが最終的にアクセスしたい先は **Workspace1** である。Workspace1 を Private Link 化するために、Workspace1 を1つの AMPLS に関連付ければよい。

イメージは次のようになる。

```text
Workspace1
   │
   ▼
AMPLS1
```

これに対して VNet 側から接続してくる。

```text
VNet1 / VNet2 / VNet3
        │
        ▼
Private Endpoint(s)
        │
        ▼
AMPLS1
        │
        ▼
Workspace1
```

つまり **1 Workspace = 1 AMPLS** で整理できる。  
問題文の「管理上の努力を最小化」という要件にも一致する。もし VNet ごとに AMPLS を分けると、

- AMPLS の作成数が増える
    
- 接続設定が増える
    
- ポリシーやアクセスモード管理が分散する
    

ため、明らかに管理コストが増す。したがって **AMPLS は 1つ**が最小構成である。 ([Microsoft Learn](https://learn.microsoft.com/en-us/azure/azure-monitor/fundamentals/private-link-configure?utm_source=chatgpt.com "Configure private link for Azure Monitor"))

---

# 5 ではなぜ Private Endpoint は 1つでは足りないのか

ここがこの問題の本質である。

Private Endpoint は Azure Private Link の **実体**であり、**特定の VNet / Subnet にプライベートIPを作るネットワークインターフェース**である。Microsoft Learn でも、Private Endpoint は「VNet 内のプライベートIPアドレスを使用するネットワークインターフェース」と説明されている。 ([Microsoft Learn](https://learn.microsoft.com/en-us/azure/private-link/private-endpoint-overview?utm_source=chatgpt.com "What is a private endpoint? - Azure Private Link"))

さらに Azure Private Link service の到達性として、Private Endpoint は

- 同じ VNet
    
- **リージョンピアリングされた VNet**
    
- グローバルピアリングされた VNet
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure ExpressRoute]]
- VPN / ExpressRoute 経由のオンプレ
    

から到達可能とされている。 ([Microsoft Learn](https://learn.microsoft.com/en-us/azure/private-link/private-link-service-overview?utm_source=chatgpt.com "What is Azure Private Link service?"))

つまり今回の構成では、

- **VNet1 と VNet2 はピアリング済み**
    
- **VNet3 は孤立**
    

であるため、VNet1 か VNet2 のどちらか一方に置いた Private Endpoint は、もう片方のピアVNetからも利用できるが、VNet3 からは利用できない。

図にするとこうなる。

```text
VNet1 ───── VNet2
  │
  │  (peeringあり)
  │
 Private Endpoint #1
  │
  ▼
 AMPLS1
  │
  ▼
 Workspace1

VNet3  (peeringなし)
  │
 Private Endpoint #2
  │
  ▼
 AMPLS1
  │
  ▼
 Workspace1
```

したがって **Private Endpoint は最小で2つ必要**になる。  
1つでは VNet3 をカバーできず、3つだと VNet1/VNet2 間で共有可能な分まで重複してしまう。 ([Microsoft Learn](https://learn.microsoft.com/en-us/azure/private-link/private-endpoint-overview?utm_source=chatgpt.com "What is a private endpoint? - Azure Private Link"))

---

# 6 この問題の答えを整理するとどうなるか

この問題は実は2つの設問に分かれている。

## 6.1 AMPLS の最小数

答えは **1**。

理由は、Workspace1 が1つであり、その Workspace を1つの AMPLS に接続すれば、論理的な管理スコープとして十分だからである。複数 VNet があることは、AMPLS の数を増やす理由にはならない。 ([Microsoft Learn](https://learn.microsoft.com/en-us/azure/azure-monitor/fundamentals/private-link-configure?utm_source=chatgpt.com "Configure private link for Azure Monitor"))

## 6.2 Private Endpoint の最小数

答えは **2**。

理由は、Private Endpoint は VNet 内に生えるため、到達可能性はネットワーク接続性に依存するからである。VNet1 と VNet2 はピアリングされているので1つの Private Endpoint を共有可能だが、VNet3 は独立しているため別の Private Endpoint が必要になる。 ([Microsoft Learn](https://learn.microsoft.com/en-us/azure/private-link/private-endpoint-overview?utm_source=chatgpt.com "What is a private endpoint? - Azure Private Link"))

---

# 7 ありがちな誤解

## 7.1 「VNet が3つあるなら Private Endpoint も3つ必要では？」

必ずしもそうではない。Private Endpoint は **同じ VNet だけでなく、ピアリングされた VNet からも到達可能**である。したがって、VNet1 と VNet2 がピアリングされている今回の構成では、2つの VNet に対して1つの Private Endpoint で足りる。 ([Microsoft Learn](https://learn.microsoft.com/en-us/azure/private-link/private-link-service-overview?utm_source=chatgpt.com "What is Azure Private Link service?"))

## 7.2 「Private Endpoint が2つ必要なら AMPLS も2つ必要では？」

これも違う。AMPLS は **監視リソースの管理スコープ**であり、Private Endpoint は **接続元ネットワーク側の入口**である。1つの AMPLS に対して複数の Private Endpoint をぶら下げる設計は自然であり、むしろ標準的である。 ([Microsoft Learn](https://learn.microsoft.com/en-us/azure/azure-monitor/fundamentals/private-link-configure?utm_source=chatgpt.com "Configure private link for Azure Monitor"))

## 7.3 「Private DNS zone が3つの VNet にリンクされていることは何を意味するのか」

Private DNS zone のリンクは、Private Endpoint の名前解決を各 VNet で可能にするために重要である。Azure Monitor Private Link は DNS 依存性があり、Microsoft Learn でも Azure Monitor Private Link Scope は DNS private link zones に依存していると説明されている。 ([Microsoft Learn](https://learn.microsoft.com/en-us/azure/azure-monitor/data-collection/data-collection-endpoint-overview?utm_source=chatgpt.com "Data collection endpoints in Azure Monitor"))

---

# 8 アーキテクチャ全体像

この問題の最小構成を、後で見返しても分かる形で整理すると次のようになる。

```text
                       +----------------------+
                       |   Log Analytics      |
                       |     Workspace1       |
                       +----------+-----------+
                                  |
                                  v
                       +----------------------+
                       |        AMPLS1        |
                       | Azure Monitor Private|
                       |     Link Scope       |
                       +----+------------+----+
                            |            |
                            |            |
                            v            v
                +----------------+   +----------------+
                | Private EP #1  |   | Private EP #2  |
                | in VNet1 or 2  |   |    in VNet3    |
                +--------+-------+   +--------+-------+
                         |                    |
                         |                    |
              +----------+------+             |
              |                 |             |
              v                 v             v
           +------+         +------+      +------+
           |VNet1 |<------->|VNet2 |      |VNet3 |
           +------+  peered +------+      +------+
```

要点は次の3つだけである。

1. **Workspace1 に対して AMPLS は1つ**
    
2. **VNet1 / VNet2 用に Private Endpoint を1つ**
    
3. **孤立した VNet3 用に Private Endpoint をもう1つ**
    

---

# 9 設計指針

この種の問題は、次の順番で考えると解きやすい。

まず、監視対象の Workspace がいくつあるかを見る。  
Workspace が1つなら、まず **AMPLS も1つ**で考える。

次に、接続元ネットワークの到達性を見る。  
同一VNetか、ピアリング済みか、孤立しているかを確認する。

- 同じ到達圏にあるネットワーク群 → Private Endpoint を共有できる可能性が高い
    
- 到達できない独立ネットワーク → 別 Private Endpoint が必要
    

この問題では

- VNet1 と VNet2 は同じ到達圏
    
- VNet3 は別到達圏
    

なので、**Private Endpoint は 2つ**になる。

---

# 10 まとめ

このシナリオの答えを最短で覚えるなら、次のように整理するとよい。

```text
Workspace が1つ → AMPLS は1つ
到達圏が2つ    → Private Endpoint は2つ
```

今回の正解は次のとおり。

- **Azure Monitor Private Link Scope (AMPLS) の最小数**: **1**
    
- **Private Endpoint の最小数**: **2**
    

理由は、AMPLS は監視リソースをまとめる論理スコープであり、Workspace1 に対して1つあれば十分だからである。一方、Private Endpoint はネットワーク到達性に依存し、VNet1/VNet2 は共有可能だが、VNet3 は孤立しているため別途必要になる。 ([Microsoft Learn](https://learn.microsoft.com/en-us/azure/azure-monitor/fundamentals/private-link-configure?utm_source=chatgpt.com "Configure private link for Azure Monitor"))

この整理ができると、AZ-305 の Azure Monitor / Private Link / ネットワーク分離系の問題はかなり解きやすくなる。