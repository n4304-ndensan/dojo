[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Blueprints]]
# Azure Blueprints による標準化デプロイ設計

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Management Group]]
（Management Group・Blueprint Definition・Assignment）

---

# 1 背景

ある組織では、4つの部門がそれぞれ独自の **Azure サブスクリプション**を持っている。  
組織ではクラウド環境の標準化とガバナンスを維持するために **Azure Blueprints** を使用して共通のインフラ構成を自動展開する計画である。

各部門のサブスクリプションに対して、次のリソースを同じ構成でデプロイする必要がある。

### 展開対象リソース

- Resource Group
    
- Web App
    
- Custom Role
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Cosmos DB]]
- Azure Cosmos DB
    

これらを **すべての部門で同一構成として標準化**する必要がある。

---

# 2 Azure Blueprints とは

Azure Blueprints は、Azure 環境の **ガバナンスと標準化されたデプロイ**を実現するサービスである。

Blueprint では次のような構成要素をまとめて定義できる。

|アーティファクト|説明|
|---|---|
|Resource Groups|リソースグループ作成|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Resource Manager (ARM)]]
|ARM Templates|リソースデプロイ|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Policy]]
|Policy Assignment|Azure Policy|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Role-Based Access Control (RBAC)]]
|Role Assignment|RBACロール|

Blueprint を使用することで、複数のサブスクリプションに対して **同一の構成を再利用可能なテンプレートとして展開**できる。

---

# 3 Azure ガバナンス階層

Azure ではリソース管理の階層構造が存在する。

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
Resources
```

Blueprint は **Management Group または Subscription** に対して適用できる。

---

# 4 Blueprint の構成要素

Azure Blueprints には3つの重要な概念がある。

### 1 Blueprint Definition

Blueprint Definition は **テンプレート本体**である。  
どのリソースをデプロイするかを定義する。

例

```text
Blueprint Definition
 ├ Resource Group
 ├ Web App
 ├ Cosmos DB
 └ Custom Role
```

---

### 2 Blueprint Assignment

Blueprint Assignment は **Blueprint をどこに適用するか**を指定する。

```text
Blueprint Definition
        │
        ▼
Blueprint Assignment
        │
        ▼
Subscription
```

サブスクリプションごとに Assignment が必要になる。

---

### 3 Management Group

複数のサブスクリプションをまとめて管理するための論理グループ。

---

# 5 今回の要件

今回の環境

|部門|サブスクリプション|
|---|---|
|部門A|Subscription A|
|部門B|Subscription B|
|部門C|Subscription C|
|部門D|Subscription D|

合計

```text
4 Subscriptions
```

すべて同じ構成をデプロイする。

---

# 6 必要な Management Group

4つのサブスクリプションを **1つの管理グループにまとめる**ことができる。

```text
Management Group
   ├ Subscription A
   ├ Subscription B
   ├ Subscription C
   └ Subscription D
```

したがって必要な管理グループ数は

```
1
```

---

# 7 必要な Blueprint Definition

すべての部門で **同じ構成**をデプロイするため、Blueprint Definition は **1つだけ**で十分。

```text
Blueprint Definition
 ├ Resource Group
 ├ Web App
 ├ Cosmos DB
 └ Custom Role
```

必要数

```
1
```

---

# 8 必要な Blueprint Assignment

Blueprint Assignment は **サブスクリプション単位で行う**。

今回

```
4 Subscriptions
```

なので

```
4 Assignments
```

になる。

---

# 9 最終構成

まとめると

|項目|必要数|
|---|---|
|Management Group|1|
|Blueprint Definition|1|
|Blueprint Assignment|4|

---

# 10 アーキテクチャ図

```text
Tenant
   │
   ▼
Management Group (1)
   │
   ├ Subscription A
   │      │
   │      └ Blueprint Assignment
   │
   ├ Subscription B
   │      │
   │      └ Blueprint Assignment
   │
   ├ Subscription C
   │      │
   │      └ Blueprint Assignment
   │
   └ Subscription D
          │
          └ Blueprint Assignment

Blueprint Definition (1)
```

---

# 11 他の選択肢が不適切な理由

### 4 Blueprint Definitions

Blueprint Definition は再利用できるため、サブスクリプションごとに作成する必要はない。

---

### 4 Management Groups

すべてのサブスクリプションを1つの管理グループにまとめられるため不要。

---

### 2 Management Groups

特に分割要件がないため不要。

---

# 12 試験ポイント

Azure 試験では次のルールを覚えると解きやすい。

### Blueprint Definition

```text
テンプレートの数
```

---

### Blueprint Assignment

```text
適用対象の数
```

---

### Management Group

```text
必要最小数
```

---

# 13 まとめ

今回の要件

- 4つのサブスクリプション
    
- 同じ構成を展開
    
- Azure Blueprints を使用
    

必要構成

|項目|数|
|---|---|
|Management Group|1|
|Blueprint Definition|1|
|Blueprint Assignment|4|

したがって正解は

**A**

```
1 Management Group
1 Blueprint Definition
4 Blueprint Assignments
```