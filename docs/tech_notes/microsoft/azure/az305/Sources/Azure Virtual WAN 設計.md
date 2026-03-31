[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual WAN]]
# Azure Virtual WAN 設計

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure ExpressRoute]]
## グローバル拠点（北米・ヨーロッパ・アジア）を ExpressRoute + VPN で接続するアーキテクチャ

---

# 1 背景（シナリオ）

企業が世界中に拠点を持つ場合、各拠点のネットワークを **クラウド経由で安全かつ高速に接続する必要**がある。

今回のシナリオでは、企業が次の地域にオフィスを持っている。

|地域|Azureリージョン|
|---|---|
|北米|East US|
|ヨーロッパ|West Europe|
|アジア|Southeast Asia|

この企業は Azure をグローバルネットワークのハブとして利用するため、次のネットワーク構成を実装する必要がある。

### ネットワーク要件

1. **ExpressRoute 回路を接続**
    
2. **サイト間 VPN をサポート**
    
3. **遅延を最小化**
    
4. **コストを最小化**
    

このようなグローバルネットワーク構成では、Azure の **Virtual WAN (vWAN)** を利用するのが一般的である。

---

# 2 Azure Virtual WAN とは

Azure Virtual WAN は、Microsoft が提供する **クラウドベースのグローバルネットワークサービス**である。

Virtual WAN を利用すると、以下の接続を統合できる。

|接続タイプ|説明|
|---|---|
|Site-to-Site VPN|拠点 VPN|
|Point-to-Site VPN|ユーザー接続|
|ExpressRoute|専用線|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Network (VNet)]]
|VNet 接続|Azure 内接続|

つまり Virtual WAN は

```text
企業拠点
    │
    ▼
Azure Virtual WAN
    │
    ▼
Azure VNet / 他拠点
```

という **グローバルネットワークハブ**として機能する。

---

# 3 Virtual WAN のアーキテクチャ

Virtual WAN は次のコンポーネントで構成される。

```text
Virtual WAN
     │
     ▼
Virtual Hub
     │
 ┌───┼──────────────┐
 │   │              │
VPN  ExpressRoute   VNet
```

### Virtual Hub

Virtual Hub は

```text
Azureリージョン内のネットワークハブ
```

である。

Virtual Hub には以下が接続される。

- ExpressRoute
    
- VPN Gateway
    
- Azure VNet
    

---

# 4 Virtual Hub の配置

Virtual Hub は **Azure リージョン単位で配置**される。

つまり

```text
1リージョン
↓
1 Virtual Hub
```

となる。

---

# 5 今回のネットワーク構成

今回の企業には **3つの Azure リージョン**がある。

|リージョン|地域|
|---|---|
|East US|北米|
|West Europe|ヨーロッパ|
|Southeast Asia|アジア|

さらに、それぞれのリージョンに

```text
ExpressRoute 回路
```

が存在する。

---

# 6 ExpressRoute と Virtual Hub

ExpressRoute 回路は **リージョン単位で接続される**。

つまり

```text
ExpressRoute
↓
Virtual Hub
↓
Virtual WAN
```

となる。

そのため

```text
ExpressRoute 回路があるリージョン
↓
Virtual Hub 必須
```

である。

---

# 7 必要なハブ数

今回のリージョン

```text
East US
West Europe
Southeast Asia
```

つまり

```text
3リージョン
↓
3ハブ
```

が必要になる。

---

# 8 Virtual WAN SKU

Virtual WAN には 2 種類の SKU がある。

|SKU|特徴|
|---|---|
|Basic|基本機能|
|Standard|フル機能|

---

# 9 Basic SKU

Basic SKU の特徴

|機能|可否|
|---|---|
|Site-to-Site VPN|可能|
|Point-to-Site VPN|可能|
|ExpressRoute|不可|

つまり

```text
ExpressRoute
↓
Basicでは利用不可
```

である。

---

# 10 Standard SKU

Standard SKU の特徴

|機能|可否|
|---|---|
|Site-to-Site VPN|可能|
|ExpressRoute|可能|
|VNet 接続|可能|
|Routing|高度|

つまり

```text
ExpressRoute + VPN
```

の両方を利用する場合

```text
Standard SKU
```

が必須となる。

---

# 11 遅延を最小化する設計

ネットワーク遅延を最小化するためには

```text
トラフィックを地域内で処理
```

することが重要である。

つまり

```text
East US 拠点
↓
East US Hub
```

という構成にすることで

```text
リージョン間通信
```

を減らせる。

---

# 12 コスト最適化

もし

```text
4つ目のハブ
```

を追加すると

```text
不要なハブコスト
```

が発生する。

今回の要件では

```text
3リージョン
```

なので

```text
3ハブ
```

で十分である。

---

# 13 他の選択肢が不適切な理由

### B

4ハブ + Basic

問題

```text
Basic
↓
ExpressRoute不可
```

---

### C

3ハブ + Basic

ハブ数は正しいが

```text
Basic SKU
↓
ExpressRoute不可
```

---

### D

4ハブ + Standard

機能は満たすが

```text
不要なハブ
↓
コスト増加
```

---

# 14 最適アーキテクチャ

```text
Azure Virtual WAN
      │
      │
 ┌────┼──────────────┐
 │    │              │
Hub1  Hub2           Hub3
EastUS WestEurope    SoutheastAsia
 │      │              │
ER      ER             ER
 │      │              │
VPN     VPN            VPN
```

この構成により

- 各リージョンでローカル接続
    
- ExpressRoute + VPN 統合
    
- 低遅延ネットワーク
    

を実現できる。

---

# 15 最終回答

正解

```text
A
3つのハブ
Standard SKU
```

---

# 16 まとめ

今回の問題のポイントは **Azure Virtual WAN の設計原則**である。

重要ポイント

|設計要素|内容|
|---|---|
|Virtual Hub|リージョン単位|
|ExpressRoute|Standard SKU 必須|
|ハブ数|リージョン数|

今回の環境

|リージョン|必要Hub|
|---|---|
|East US|1|
|West Europe|1|
|Southeast Asia|1|

つまり

```text
3 Hub
+
Standard SKU
```

が最適となる。

---

もし希望があれば、次に **Azure ネットワーク問題を解くときの思考アルゴリズム（試験で一瞬で解く方法）**も作れます。  
これを覚えると **AZ-104 / AZ-305 / SC-100 系の問題がかなり速く解けるようになります。**