# Azure 仮想マシン向けネットワークセキュリティ設計

（NSG + Azure Firewall + Threat Intelligence）

---

# 1 背景とシナリオ

クラウド環境で仮想マシン上にアプリケーションを公開する場合、ネットワークセキュリティ設計は非常に重要である。特にインターネット公開アプリケーションでは、不要なポートを公開すると攻撃対象となる可能性が高くなる。

今回のシナリオでは、Azure 仮想マシン上でインターネット向けアプリケーションをホストしている。企業は次のようなセキュリティ要件を満たすネットワーク構成を求めている。

要件は以下の通りである。

- アプリケーションは **HTTPS（443）でのみアクセス可能**
    
- **SSH（22）および RDP（3389）をインターネットからアクセス不可にする**
    
- すべての仮想ネットワークに対して **集中型のネットワーク監視** を行う
    
- **脅威検出（Threat detection）** を実装する
    

これらの要件を満たす最適な構成は

**Network Security Group（NSG） + Azure Firewall（Threat Intelligence）**

である。

---

# 2 Azure ネットワークセキュリティの基本構造

Azure では複数のセキュリティレイヤーを組み合わせてネットワークを保護する。

典型的な構成

```text
Internet
   │
   ▼
Azure Firewall
   │
   ▼
Virtual Network
   │
   ▼
Network Security Group
   │
   ▼
Virtual Machine
```

この構造では

- Azure Firewall → ネットワーク境界の防御
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
- NSG → VMレベルのアクセス制御
    

を担当する。

---

# 3 Network Security Group（NSG）

NSG は Azure の基本的なネットワークアクセス制御機能である。サブネットまたはネットワークインターフェースに対して適用できる。

NSG の主な役割は

- ポート制御
    
- プロトコル制御
    
- IP制御
    

である。

---

## NSG のルール例

今回のシナリオでは、HTTPS のみ許可する必要がある。

例

```text
Priority   Port   Action
100        443    Allow
200        22     Deny
210        3389   Deny
```

これにより

- Web アプリのみ公開
    
- 管理ポート遮断
    

が実現できる。

---

# 4 Azure Firewall

Azure Firewall は Azure の **マネージドステートフルファイアウォール**であり、仮想ネットワーク境界でトラフィックを制御する。

主な機能

- ネットワークフィルタリング
    
- アプリケーションフィルタリング
    
- DNAT / SNAT
    
- 脅威インテリジェンス
    
- ログ監視
    

構造

```text
Internet
   │
   ▼
Azure Firewall
   │
   ▼
Virtual Networks
```

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Network (VNet)]]
この構成により、複数の VNet に対して **集中型セキュリティポリシー**を適用できる。

---

# 5 Threat Intelligence

Azure Firewall には **Threat Intelligence 機能**があり、Microsoft のセキュリティデータを使用して悪意ある通信を検出する。

例

```text
Known malicious IP
Botnet servers
Command and Control servers
```

ファイアウォールはこれらの通信を自動的に

- Block
    
- Alert
    

する。

---

# 6 集中型ネットワーク監視

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Monitor]]
Azure Firewall と NSG はログを Azure Monitor に送信できる。

ログの流れ

```text
NSG Flow Logs
Azure Firewall Logs
        │
        ▼
Log Analytics
        │
        ▼
Azure Monitor
```

これにより

- ネットワークトラフィック分析
    
- 攻撃検知
    
- トラブルシューティング
    

が可能になる。

---

# 7 セキュリティアーキテクチャ

推奨構成

```text
Internet
   │
   ▼
Azure Firewall (Threat Intelligence)
   │
   ▼
Hub VNet
   │
   ▼
Spoke VNets
   │
   ▼
Network Security Groups
   │
   ▼
Virtual Machines
```

この構成は **Hub-Spoke Network Architecture** と呼ばれる。

メリット

- セキュリティの集中管理
    
- スケーラビリティ
    
- 可視性向上
    

---

# 8 他の選択肢との比較

## Azure DDoS Protection + ASG

DDoS Protection は

- 大量トラフィック攻撃対策
    

を提供する。

しかし

- ポート制御不可
    
- 脅威検知なし
    

ASG はルール管理を簡単にするが、セキュリティ機能ではない。

---

## Azure WAF + Bastion

WAF は

- SQL Injection
    
- Cross Site Scripting
    

など **Webレイヤー攻撃対策**である。

しかし

- ネットワーク制御なし
    
- VNet監視なし
    

Bastion は管理アクセスを安全にするが、ネットワーク監視ではない。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Front Door]]
## Azure Front Door + JIT

Front Door は

- グローバルロードバランサー
    

であり、ネットワークセキュリティツールではない。

JIT は

- 管理ポートの一時開放
    

機能だが、ネットワーク全体の防御にはならない。

---

# 9 推奨アーキテクチャ

最終構成

```text
Internet
   │
   ▼
Azure Firewall
   │
   ▼
Virtual Network
   │
   ▼
NSG
   │
   ▼
VM (HTTPS only)
```

この構成で実現できること

- HTTPSのみ公開
    
- 管理ポート遮断
    
- 集中監視
    
- 脅威検出
    

---

# 10 まとめ

今回の要件

- HTTPSのみ公開
    
- SSH/RDP遮断
    
- 集中型ネットワーク監視
    
- 脅威検出
    

これらを満たす Azure サービスの組み合わせは

**Network Security Groups + Azure Firewall（Threat Intelligence）**

である。

NSG は VM レベルのアクセス制御を提供し、Azure Firewall は集中型セキュリティと脅威インテリジェンスを提供するため、この組み合わせが最適なネットワークセキュリティアーキテクチャとなる。