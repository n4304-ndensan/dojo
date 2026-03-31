[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Blueprints]]
# Azure Blueprints を利用した大規模 Azure 環境のガバナンス設計

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Management Group]]
（管理グループ階層を利用した Blueprint の定義と割り当て）

---

# 1 背景（シナリオ）

企業の Azure 環境では、クラウド利用の拡大に伴い **多数のサブスクリプションを管理する必要がある**ケースが多い。

今回のシナリオでは、Azure 環境の構成は次のようになっている。

|階層|数|
|---|---|
|ルート管理グループ|1|
|子管理グループ|5|
|サブスクリプション|50|

Azure の管理階層は以下の構造を持つ。

```text
Root Management Group
│
├ Child Management Group 1
│    ├ Subscription
│    ├ Subscription
│
├ Child Management Group 2
│    ├ Subscription
│
├ Child Management Group 3
│
├ Child Management Group 4
│
└ Child Management Group 5
```

このような大規模環境では、次のような **ガバナンスの問題**が発生する。

- セキュリティポリシーの統一
    
- リソース構成の標準化
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Role-Based Access Control (RBAC)]]
- RBAC の管理
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Policy]]
- Azure Policy の適用
    

これらを効率的に管理するために **Azure Blueprints** を使用する。

---

# 2 Azure Blueprints とは

Azure Blueprints は、Azure 環境の標準構成をテンプレートとして定義する仕組みである。

Blueprint を使用することで、次のようなリソースをまとめて管理できる。

|要素|説明|
|---|---|
|Azure Policy|ガバナンスルール|
|RBAC|アクセス権|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Resource Manager (ARM)]]
|Resource Templates|ARM/Bicep リソース|
|Resource Locks|削除防止|

つまり Blueprint は

```text
クラウド環境の標準設計書
```

のような役割を持つ。

---

# 3 Blueprint の主な目的

Blueprint を使用する主な目的は次の通り。

### ① 環境標準化

企業のクラウド環境では

- セキュリティ
    
- ネットワーク
    
- RBAC
    

を統一する必要がある。

---

### ② ガバナンス強化

Blueprint は **Policy + RBAC + Resource** をまとめて適用できる。

---

### ③ 管理オーバーヘッド削減

Blueprint を使用すると

```text
一度定義
↓
複数環境へ適用
```

が可能になる。

---

# 4 Azure 管理階層（Management Hierarchy）

Azure ではリソースは階層構造で管理される。

```text
Tenant
 │
 ▼
Management Group
 │
 ▼
Subscription
 │
 ▼
Resource Group
 │
 ▼
Resource
```

この階層の特徴は

```text
上位スコープ → 下位スコープへ継承
```

である。

---

# 5 Blueprint のスコープ

Blueprint は次のスコープで定義できる。

|定義スコープ|説明|
|---|---|
|Management Group|組織全体|
|Subscription|特定サブスクリプション|

---

# 6 Blueprint の割り当て（Assignment）

Blueprint は **定義しただけでは適用されない**。

Blueprint は **Assignment（割り当て）**することで実際に適用される。

Blueprint Assignment のスコープ

|割り当て対象|説明|
|---|---|
|Management Group|複数サブスクリプション|
|Subscription|単一サブスクリプション|

---

# 7 問題の要件

今回の問題の要件は次の通り。

### 要件

- Azure 環境は **50 サブスクリプション**
    
- **管理オーバーヘッドを最小化**
    
- **Blueprint を使用してガバナンス強化**
    

つまり

```text
できるだけ少ない管理作業で
すべてのサブスクリプションに適用
```

する必要がある。

---

# 8 最適な設計

最適な方法は次の通り。

### Step 1

Blueprint を **Root Management Group で定義する**

理由

```text
最上位スコープ
```

であるため、すべての管理グループで利用できる。

---

### Step 2

Blueprint を **Child Management Group に割り当てる**

これにより

```text
Child Management Group
↓
Subscription
```

へ継承される。

---

# 9 Blueprint 継承構造

最終的な構成

```text
Root Management Group
│
│ Blueprint Definition
│
├ Child Management Group
│     │
│     │ Blueprint Assignment
│     ▼
│   Subscriptions
│
├ Child Management Group
│     │
│     ▼
│   Subscriptions
```

これにより

```text
50 サブスクリプションすべてに適用
```

される。

---

# 10 管理オーバーヘッド比較

各選択肢を比較すると次のようになる。

|方法|Blueprint数|割り当て数|管理負荷|
|---|---|---|---|
|Root MG 定義 + Child MG 割り当て|1|5|低|
|Root MG 定義 + Subscription 割り当て|1|50|中|
|Child MG 定義|5|50|高|
|Subscription 定義|50|50|非常に高|

したがって最も効率的なのは

```text
Root MG 定義
↓
Child MG 割り当て
```

である。

---

# 11 他の選択肢が不適切な理由

## A

サブスクリプションで定義

Blueprint を 50 個管理する必要がある。

管理コストが非常に高い。

---

## C

Child Management Group 定義

Blueprint を 5 個作成する必要がある。

Root MG で 1 回定義する方が効率的。

---

## D

Root MG 定義 → Subscription 割り当て

50 サブスクリプションに手動割り当てが必要。

---

# 12 最終アーキテクチャ

最適な構成

```text
Root Management Group
│
│ Blueprint Definition
│
├ Child MG1
│   └ Subscriptions
│
├ Child MG2
│   └ Subscriptions
│
├ Child MG3
│
├ Child MG4
│
└ Child MG5
```

---

# 13 最終回答

正解

```text
B

Blueprint を Root Management Group で定義  
Child Management Group に割り当てる
```

---

# 14 まとめ

この問題の重要ポイントは **Azure 管理階層の理解**である。

|設計ポイント|理由|
|---|---|
|Root MG 定義|Blueprint を 1 回だけ定義|
|Child MG 割り当て|すべてのサブスクリプションへ継承|
|管理オーバーヘッド削減|サブスクリプション単位管理を回避|

つまり最適構成は

```text
Root Management Group
↓
Child Management Group
↓
Subscriptions
```

で Blueprint を適用することである。

これは **大規模 Azure 環境における標準的なガバナンス設計**である。