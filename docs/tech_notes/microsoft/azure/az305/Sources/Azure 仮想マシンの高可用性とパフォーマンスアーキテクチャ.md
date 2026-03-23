# Azure 仮想マシンの高可用性とパフォーマンスアーキテクチャ

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Availability Zone]]
（VM Scale Sets / Availability Zones / Traffic Manager）

---

# 1 概要

企業が Azure 上でアプリケーションをホストする場合、単一の仮想マシン (VM) に依存した構成では、障害や負荷増加に対して脆弱になる可能性がある。そのため、Azure では複数の VM を組み合わせて **高可用性と安定したパフォーマンスを確保するアーキテクチャ**を採用することが推奨されている。

今回のシナリオでは、企業が Azure の仮想マシン上で複数のアプリケーションをホストする計画を立てている。各アプリケーションは異なる要件を持つが、共通して次の2つの要件が提示されている。

1. 一連の仮想マシンで **信頼性のあるパフォーマンスを維持すること**
    
2. データセンター障害が発生しても **アプリケーションの運用を継続できること**
    

この問題は、Azure Solution Architect 試験でよく出題される **「パフォーマンス」と「可用性」の違いを理解しているか** を問う典型的な問題である。

今回問われているのは **最初の要件**、つまり

**「VMで信頼性のあるパフォーマンスを維持する方法」**

である。

この要件に最も適した Azure サービスは

**Azure Virtual Machine Scale Sets (VMSS)**

である。

---

# 2 背景

クラウド環境では、アプリケーションのトラフィックは常に一定とは限らない。時間帯やイベント、ユーザー数の変化によって負荷は大きく変動する。

例えばゲーム業界のアプリケーションでは次のような状況が起こる。

- 新しいイベント開始時にアクセス急増
    
- メンテナンス後の同時接続増加
    
- 大規模アップデート時のトラフィック急増
    

単一の VM 構成では、こうした負荷に対応できない。

単一VM構成

```
Users
  │
  ▼
Single VM
  │
  ▼
Application
```

この構成には次の問題がある。

- スケールできない
    
- VM障害でサービス停止
    
- パフォーマンス低下
    

そのため Azure では

**複数 VM を利用したスケーラブルな構成**

が推奨される。

---

# 3 Azure Virtual Machine Scale Sets

## 概要

Azure Virtual Machine Scale Sets (VMSS) は

**同一構成の仮想マシンを複数台自動的に展開・管理するサービス**

である。

VMSS を使用すると

- VM の自動スケーリング
    
- 負荷分散
    
- 高可用性
    

を実現できる。

基本構成

```
Users
   │
   ▼
Load Balancer
   │
   ├ VM Instance
   ├ VM Instance
   ├ VM Instance
   └ VM Instance
```

---

# 4 VM Scale Sets の特徴

VM Scale Sets には次の重要な機能がある。

### 自動スケーリング

トラフィック量に応じて VM 数を増減する。

例

```
CPU 使用率 > 70%
      │
      ▼
VM 追加
```

逆に

```
CPU 使用率 < 30%
      │
      ▼
VM 削除
```

これにより

**常に適切なパフォーマンスを維持**

できる。

---

### ロードバランシング

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Load Balancer]]
VMSS は Azure Load Balancer と統合されており、トラフィックを複数の VM に分散する。

```
Users
  │
  ▼
Azure Load Balancer
  │
  ├ VM1
  ├ VM2
  └ VM3
```

この構成により

- 単一 VM の負荷集中を防ぐ
    
- レスポンス速度を安定化
    

できる。

---

### 自動インスタンス管理

VMSS は次の操作を自動化する。

- VM 作成
    
- VM 更新
    
- VM 修復
    

そのため運用コストが低い。

---

# 5 可用性とパフォーマンスの違い

Azure アーキテクチャでは、次の2つの概念が区別される。

|概念|意味|
|---|---|
|Performance|処理性能|
|Availability|障害耐性|

今回の問題は

**パフォーマンス**

に関する質問である。

VMSS は

**パフォーマンスとスケーラビリティ**

を提供する。

---

# 6 他の選択肢との違い

## Azure Availability Zones

Availability Zones は

**データセンター障害に対する耐性**

を提供する。

構造

```
Region
 │
 ├ Zone 1
 ├ Zone 2
 └ Zone 3
```

用途

- 災害耐性
    
- 高可用性
    

しかし

**パフォーマンス維持が主目的ではない**

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Application Gateway]]
## Azure Application Gateway

Application Gateway は

**Web アプリケーション用ロードバランサー**

である。

用途

- HTTP/HTTPS トラフィック管理
    
- Web Application Firewall (WAF)
    

構造

```
Users
 │
 ▼
Application Gateway
 │
 ▼
Web Servers
```

VM パフォーマンス管理とは直接関係しない。

---

## Azure Traffic Manager

Traffic Manager は

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure DNS]]
**DNS ベースのグローバルロードバランサー**

である。

構造

```
Users
 │
 ▼
Traffic Manager
 │
 ├ Region A
 └ Region B
```

用途

- リージョン間トラフィック制御
    
- 災害復旧
    

VM のパフォーマンスとは関係しない。

---

# 7 典型的なアーキテクチャ

Azure VM アプリケーション構成

```
Users
 │
 ▼
Azure Load Balancer
 │
 ▼
Virtual Machine Scale Set
 │
 ├ VM1
 ├ VM2
 ├ VM3
 └ VM4
```

この構成により

- トラフィック分散
    
- 自動スケーリング
    
- 安定したパフォーマンス
    

が実現される。

---

# 8 Azure 高可用性構成

完全な構成では

VMSS + Availability Zones

が組み合わされる。

```
Users
 │
 ▼
Load Balancer
 │
 ▼
VM Scale Set
 │
 ├ Zone1 VM
 ├ Zone2 VM
 └ Zone3 VM
```

これにより

- パフォーマンス
    
- 可用性
    
- 災害耐性
    

を同時に確保できる。

---

# 9 試験問題のポイント

問題では

**最初の要件**

が問われている。

```
信頼性のあるパフォーマンス
```

これは

**スケーリング**

を意味する。

そのため答えは

**Azure Virtual Machine Scale Sets**

となる。

---

# 10 まとめ

Azure で仮想マシンベースのアプリケーションを運用する場合、パフォーマンスと可用性を確保するために複数のサービスが存在する。

|サービス|役割|
|---|---|
|VM Scale Sets|スケーリング / パフォーマンス|
|Availability Zones|データセンター障害対策|
|Traffic Manager|リージョン間ルーティング|
|Application Gateway|Webトラフィック制御|

今回の問題では

**仮想マシンで信頼性のあるパフォーマンスを維持する**

ことが目的であるため

**Azure Virtual Machine Scale Sets**

が最適なソリューションとなる。