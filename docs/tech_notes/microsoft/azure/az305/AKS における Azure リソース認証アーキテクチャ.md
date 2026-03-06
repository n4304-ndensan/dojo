# AKS における Azure リソース認証アーキテクチャ

（Azure Workload Identity / RBAC / 最小権限）

---

# 1 概要

近年、多くの企業がモノリシックアプリケーションから **マイクロサービスアーキテクチャ** へ移行している。  
この移行では、アプリケーションは複数の独立したサービスに分割され、それぞれが **コンテナとしてデプロイ**される。

Azure 環境では、このようなコンテナベースのアプリケーションを実行するための主要なプラットフォームとして **Azure Kubernetes Service (AKS)** が提供されている。

典型的なマイクロサービス構成では、アプリケーションの各サービスは次のような Azure リソースと連携する。

- Azure Key Vault（秘密情報管理）
    
- Azure Cosmos DB（データベース）
    
- Azure Cache for Redis（キャッシュ）
    
- Azure Storage
    
- Azure Service Bus
    

そのため、Kubernetes 上で動作する **各 Pod が Azure リソースへ安全にアクセスできる認証方式** を設計することが重要となる。

今回のシナリオでは、ゲーム業界の企業が次の技術を使用してマイクロサービス環境を構築している。

- Docker
    
- Azure Container Registry
    
- Azure Kubernetes Service (AKS)
    

しかし、次の課題に直面している。

- 各 Pod が Azure Active Directory（Entra ID）を利用して認証する必要がある
    
- Key Vault / Cosmos / Redis などへ RBAC を使ってアクセスする必要がある
    
- 各 Pod の操作ログを取得する必要がある
    
- **最小権限の原則（Least Privilege）を守る必要がある**
    

これらの要件を満たすために最適なソリューションは

**Azure Workload Identity**

を利用するアーキテクチャである。

---

# 2 背景

Kubernetes 環境では、アプリケーションは **Pod** という単位で実行される。

基本構造

```
AKS Cluster
   │
   ├ Pod A
   ├ Pod B
   └ Pod C
```

各 Pod はマイクロサービスとして独立しており、次のような Azure リソースにアクセスする。

```
Pod
 │
 ├ Key Vault
 ├ Cosmos DB
 └ Redis Cache
```

従来、このようなアクセスは **サービスプリンシパル（Service Principal）** を使用して実装されることが多かった。

```
Pod
 │
 ▼
Service Principal
 │
 ▼
Azure Resource
```

しかし、この方法にはいくつかの問題がある。

### 問題1

資格情報（クライアントシークレット）を管理する必要がある

### 問題2

資格情報が Kubernetes Secret に保存される

### 問題3

漏洩リスクがある

### 問題4

最小権限を維持するのが難しい

この問題を解決するために Azure は **Managed Identity と Workload Identity** を提供している。

---

# 3 Azure Workload Identity

## 概要

Azure Workload Identity は、Kubernetes Pod が **Microsoft Entra ID の管理された ID（Managed Identity）を使用して Azure リソースにアクセスできるようにする仕組み**である。

この方式では

- シークレット不要
    
- トークンベース認証
    
- RBAC統合
    

が実現される。

構造

```
Pod
 │
 ▼
Kubernetes Service Account
 │
 ▼
Azure Workload Identity
 │
 ▼
Microsoft Entra ID
 │
 ▼
Azure Resource
```

Pod は Azure AD トークンを取得し、そのトークンを使って Azure リソースにアクセスする。

---

# 4 Azure Workload Identity の仕組み

Azure Workload Identity は次のコンポーネントで構成される。

### Kubernetes Service Account

Pod に関連付けられる認証エンティティ

### Federated Identity Credential

Kubernetes Service Account と Entra ID を関連付ける

### Managed Identity

Azure リソースへアクセスする ID

構造

```
Pod
 │
 ▼
Service Account
 │
 ▼
Federated Identity
 │
 ▼
Managed Identity
 │
 ▼
Azure Resource
```

この構造により、Pod は Azure リソースへ安全にアクセスできる。

---

# 5 RBAC と最小権限

Azure では **Role-Based Access Control (RBAC)** を使用してリソースアクセスを制御する。

RBAC は次の概念で構成される。

|要素|説明|
|---|---|
|Security Principal|ユーザー / ID|
|Role|権限の集合|
|Scope|適用範囲|

RBAC のスコープ

```
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

最小権限の原則では

**必要なリソースに対してのみ最小の権限を付与する**

ことが重要である。

---

# 6 最小権限の設計

今回のシナリオでは、Pod は次のリソースへアクセスする。

- Key Vault
    
- Cosmos DB
    
- Redis
    

そのため RBAC は

**リソースグループレベル**

で設定するのが適切である。

```
Resource Group
   │
   ├ Key Vault
   ├ Cosmos DB
   └ Redis
```

RBAC

```
Managed Identity
      │
      ▼
Reader Role
      │
      ▼
Resource Group
```

これにより

- 過剰な権限付与を防止
    
- セキュリティ強化
    

が可能になる。

---

# 7 AKS 認証アーキテクチャ

最適な構成は次の通りである。

```
AKS Cluster
     │
     ▼
Pod
     │
     ▼
Kubernetes Service Account
     │
     ▼
Azure Workload Identity
     │
     ▼
Managed Identity
     │
     ▼
Azure Resource Group
     │
     ├ Key Vault
     ├ Cosmos DB
     └ Redis
```

この構成により

- Pod ごとの認証
    
- RBAC 制御
    
- セキュアアクセス
    

が実現される。

---

# 8 不正解の選択肢

### Service Principal + Subscription Role

この方法には問題がある。

```
Subscription
   │
   ▼
Contributor
```

これは

- 権限が大きすぎる
    
- 最小権限に違反
    

となる。

---

### Node Managed Identity

ノード単位の ID を使用すると

```
Node
 │
 ├ Pod A
 ├ Pod B
 └ Pod C
```

すべての Pod が同じ権限を共有してしまう。

これも最小権限に違反する。

---

### AKS レベル RBAC

AKS レベルの RBAC は

```
Cluster
```

単位の権限管理であり、  
Azure リソースの RBAC とは別の概念である。

---

# 9 試験問題の要件整理

問題の要件

- Pod が Entra ID 認証
    
- Azure リソースへ RBAC アクセス
    
- 詳細ログ
    
- 最小権限
    

この条件から

**Azure Workload Identity**

を使用する必要がある。

さらに RBAC は

```
Resource Group Level
```

で設定する必要がある。

---

# 10 正解アーキテクチャ

```
AKS Pod
   │
   ▼
Azure Workload Identity
   │
   ▼
Managed Identity
   │
   ▼
RBAC (Resource Group)
   │
   ├ Key Vault
   ├ Cosmos DB
   └ Redis
```

---

# 11 まとめ

AKS のマイクロサービス環境では、Pod が Azure リソースへアクセスする際の認証方式が重要となる。

従来の Service Principal 方式では

- シークレット管理
    
- セキュリティリスク
    

が存在する。

そのため現在は

**Azure Workload Identity**

を使用する設計が推奨される。

この方式により

- Pod 単位認証
    
- Managed Identity
    
- RBAC
    
- 最小権限
    

が実現される。

今回のシナリオでは

**Azure Workload Identity を Pod ごとに有効化し、  
Key Vault / Cosmos / Redis が配置されたリソースグループレベルで RBAC 権限を付与する**

ことが最適なソリューションとなる。