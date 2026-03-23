# .NETアプリケーションのAzure移行設計

（Windowsサービス依存アプリ + マルチリージョン高可用性）

---

# 1 背景

ある組織が、オンプレミス環境で稼働している **複雑な.NETアプリケーション**を Azure に移行する計画を立てている。このアプリケーションは長年運用されてきたレガシー構成であり、以下の特徴を持っている。

- **Windowsサービスに依存している**
    
- **ローカルファイルシステムを一時ファイル保存に使用している**
    
- **大規模なアーキテクチャ変更は避けたい**
    
- **2つのAzureリージョンで高可用性を確保したい**
    

つまり今回の移行では、次の2つの要件を同時に満たす必要がある。

1. **最小限のリファクタリング（Lift & Shiftに近い移行）**
    
2. **マルチリージョン高可用性**
    

これらを総合的に満たす最適なホスティング方式は

**Azure Service Fabric**

である。

---

# 2 移行アプローチの考え方

クラウド移行では一般的に次の5つの戦略がある。

|戦略|内容|
|---|---|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
|Rehost|VM移行（Lift & Shift）|
|Replatform|少しだけ最適化|
|Refactor|クラウドネイティブ化|
|Rearchitect|大規模な再設計|
|Replace|SaaS化|

今回の条件

- Windowsサービス依存
    
- ローカルファイル依存
    
- 最小リファクタリング
    

このため **Rehost または Replatform 型の移行**が適している。

---

# 3 アプリケーションの特徴

今回の.NETアプリケーションは、次のような構造になっている可能性が高い。

```text
.NET Application
     │
     ├ Web Application
     ├ Windows Service
     └ Local File System Storage
```

問題になるポイント

1. Windowsサービス
    
2. ローカルディスク
    
3. ステートフル処理
    

PaaSサービスではこれらが制約になる。

---

# 4 Azure Service Fabric

Azure Service Fabric は Microsoft が提供する **分散アプリケーションプラットフォーム**であり、特に Windowsベースアプリケーションの移行に適している。

主な特徴

- Windowsサービスのホスティング
    
- ステートフルサービス対応
    
- 自動スケーリング
    
- 高可用性
    
- マイクロサービス管理
    

基本構造

```text
Service Fabric Cluster
       │
       ├ Node 1
       ├ Node 2
       └ Node 3
```

アプリケーションは Service Fabric の **サービス**として実行される。

---

# 5 Service Fabricのサービスモデル

Service Fabric には2つのサービスモデルがある。

### Stateless Service

状態を保持しないサービス

```text
Web API
Processing Service
```

---

### Stateful Service

状態を保持するサービス

```text
Session Service
Data Processing Service
```

Service Fabric は **Reliable Collections** を利用して状態を管理できる。

---

# 6 Windowsサービスのホスティング

Service Fabric は Windows環境上で実行されるため、既存の Windowsサービスを比較的容易に移行できる。

例

```text
Existing Windows Service
        │
        ▼
Service Fabric Service
```

大きなコード変更を必要としない。

---

# 7 ローカルファイルシステム対応

今回のアプリケーションは **一時ファイルをローカルディスクに保存**する。

Service Fabric は VM ベースのノード上で実行されるため

```text
Service Fabric Node
      │
      ▼
Local Disk
```

従来と同じようにローカルストレージを利用できる。

---

# 8 マルチリージョン高可用性

Service Fabric は複数リージョンでクラスタを構築できる。

アーキテクチャ例

```text
Region A
   │
   └ Service Fabric Cluster
         │
         ├ Node
         ├ Node
         └ Node

Region B
   │
   └ Service Fabric Cluster
```

グローバルロードバランサ

```text
Azure Traffic Manager
```

構成

```text
Users
   │
   ▼
Traffic Manager
   │
   ├ Region A Cluster
   └ Region B Cluster
```

リージョン障害時は自動フェイルオーバーする。

---

# 9 他の選択肢の評価

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure App Service]]
## Azure App Service

メリット

- PaaS
    
- 自動スケーリング
    

問題

- Windowsサービス不可
    
- ローカルファイル制限
    

今回のアプリには適さない。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Kubernetes Service (AKS)]]
## Azure Kubernetes Service (AKS)

メリット

- コンテナオーケストレーション
    
- クラウドネイティブ
    

問題

- Windowsサービスのコンテナ化が必要
    
- ストレージ設計変更
    
- 大規模リファクタリング
    

今回の要件に反する。

---

## Azure Virtual Machines

メリット

- 完全互換
    
- Lift & Shift
    

問題

- マルチリージョン管理が複雑
    
- 自動オーケストレーション不足
    

---

## Azure Container Instances

メリット

- 軽量コンテナ実行
    

問題

- スケール管理弱い
    
- 長期サービス用途に不向き
    

---

# 10 推奨アーキテクチャ

最適な構成

```text
Users
   │
   ▼
Azure Traffic Manager
   │
   ├ Region A
   │    └ Service Fabric Cluster
   │
   └ Region B
        └ Service Fabric Cluster
```

各クラスタ

```text
Service Fabric
    │
    ├ Web Service
    ├ Windows Service
    └ Stateful Service
```

この構成により

- 高可用性
    
- リージョン冗長
    
- 最小リファクタリング
    

が実現できる。

---

# 11 まとめ

今回の要件

- Windowsサービス依存
    
- ローカルファイル使用
    
- 最小リファクタリング
    
- マルチリージョンHA
    

これらを満たす最適なホスティング方式は

**Azure Service Fabric**

である。

Service Fabric は

- Windowsアプリ互換性
    
- ステートフルサービス
    
- 分散クラスタ管理
    

を提供するため、レガシー.NETアプリケーションを Azure に移行する際の強力な選択肢となる。