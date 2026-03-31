# Azure ハイブリッド接続設計

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Network (VNet)]]
（オンプレミスネットワーク → Azure VNet）

---

# 1 背景

ある組織では、オンプレミスのネットワークと Azure 仮想ネットワーク (VNet) を接続する **ハイブリッド接続アーキテクチャ**を設計する必要がある。

オンプレミスには

- 社内サーバー
    
- 社内アプリケーション
    
- 社内ユーザー
    

が存在する。

Azure 側では

- 仮想マシン
    
- アプリケーション
    
- データベース
    

などが VNet 内に配置されている。

---

# 2 要件

この接続設計では以下の条件を満たす必要がある。

### 必須要件

1. **パブリックインターネット経由で接続**
    
2. **通信は暗号化**
    
3. **オンプレミスネットワーク全体を接続**
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
4. **Azure VM にパブリック IP を付与しない**
    
5. **安全なハイブリッド接続**
    

---

# 3 推奨ソリューション

これらの要件を満たす最適なサービスは

**Azure VPN Gateway (Site-to-Site VPN)**

である。

---

# 4 Site-to-Site VPN 概要

Site-to-Site VPN は

**オンプレミスネットワーク全体**を Azure 仮想ネットワークに接続する。

通信は

- IPsec
    
- IKE
    

によって暗号化される。

---

# 5 接続アーキテクチャ

```text
On-prem Network
      │
      │ IPsec / IKE VPN Tunnel
      │
      ▼
Azure VPN Gateway
      │
      ▼
Azure Virtual Network
      │
      ├ VM
      ├ Database
      └ Application
```

オンプレミスのすべてのデバイスは Azure リソースにアクセスできる。

---

# 6 通信の流れ

```text
On-prem Server
      │
      ▼
Corporate Router / VPN Device
      │
      ▼
Encrypted Tunnel
      │
      ▼
Azure VPN Gateway
      │
      ▼
Azure VM (Private IP)
```

VM は **プライベート IP のみ**で通信する。

---

# 7 暗号化

VPN Gateway は

**IPsec/IKE トンネル**

を使用する。

### IPsec

- データ暗号化
    
- 認証
    

### IKE

- 鍵交換
    
- セッション管理
    

通信は以下のように保護される。

```text
Data
 │
 ▼
Encryption
 │
 ▼
Internet
 │
 ▼
Decryption
```

---

# 8 Azure 側の構成

VPN Gateway は VNet に配置される。

```text
Azure Virtual Network
 │
 ├ Subnet
 │
 └ Gateway Subnet
        │
        ▼
     VPN Gateway
```

Gateway Subnet は VPN Gateway 専用のサブネットである。

---

# 9 オンプレミス側の構成

オンプレミス側には VPN デバイスが必要になる。

例

- Cisco
    
- Juniper
    
- Fortinet
    
- pfSense
    

構成

```text
Corporate Network
      │
      ▼
VPN Router / Firewall
      │
      ▼
Internet
```

---

# 10 Azure VM のパブリックIP不要

Site-to-Site VPN では VM は **プライベートIPのみ**使用する。

```text
VM
 │
Private IP
 │
▼
VNet
 │
▼
VPN Gateway
```

そのため

- 攻撃面が減る
    
- セキュリティが向上する
    

---

# 11 他の選択肢の評価

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure ExpressRoute]]
## ExpressRoute

特徴

- 専用回線
    
- 高速
    

問題

- **パブリックインターネットを使わない**
    
- コスト高
    

今回の要件に合わない。

---

## Point-to-Site VPN

用途

- 個人ユーザー接続
    

例

- 開発者
    
- リモートワーカー
    

問題

- ネットワーク全体接続ではない
    

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual WAN]]
## Azure Virtual WAN

用途

- 大規模ネットワーク
    
- マルチリージョン
    

問題

- 過剰設計
    

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Bastion]]
## Azure Bastion

用途

- VM管理アクセス
    

例

- RDP
    
- SSH
    

問題

- ネットワーク接続ではない
    

---

# 12 推奨アーキテクチャ

```text
Corporate Network
       │
       ▼
VPN Router
       │
       │ IPsec VPN
       ▼
Azure VPN Gateway
       │
       ▼
Azure Virtual Network
       │
       ├ Application VM
       ├ Database VM
       └ Services
```

---

# 13 メリット

### セキュリティ

- IPsec暗号化
    
- プライベート通信
    

---

### コスト

- ExpressRouteより安価
    

---

### 容易な導入

- 標準的構成
    

---

### ハイブリッドクラウド

オンプレミスと Azure の統合

---

# 14 まとめ

今回の要件

- オンプレミスネットワーク接続
    
- パブリックインターネット経由
    
- 暗号化通信
    
- Azure VM にパブリックIP不要
    

これらを満たす最適なソリューションは

**Azure VPN Gateway (Site-to-Site VPN)**

である。

この構成により

- 安全なハイブリッド接続
    
- プライベート通信
    
- 暗号化トラフィック
    

を実現できる。