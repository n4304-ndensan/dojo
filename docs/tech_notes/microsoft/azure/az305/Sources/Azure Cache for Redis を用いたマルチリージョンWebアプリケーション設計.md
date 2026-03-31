[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Cache for Redis]]
# Azure Cache for Redis を用いたマルチリージョンWebアプリケーション設計

（Premium Tier + Geo-Replication + Data Persistence）

---

# 1 背景

Webアプリケーションでは、ユーザーのログイン情報やショッピングカートなどを保持するために **セッション状態（Session State）** を保存する必要がある。

多くのクラウドアーキテクチャでは、セッション管理のために **Redis のようなインメモリデータストア** が使用される。理由は以下の通りである。

- 高速アクセス（ミリ秒レベル）
    
- 高スループット
    
- 分散キャッシュ
    

今回のシナリオでは、Azure 上でマルチリージョンの Web アプリケーションを運用しており、セッション状態を **Azure Cache for Redis** に保存している。

しかし、次のような要件が存在する。

- Azure リージョン障害が発生してもセッションを維持する
    
- データ損失を最小化する
    
- 高パフォーマンスを維持する
    
- 災害復旧（Disaster Recovery）を考慮する
    

これらの条件を満たす最適な Redis 構成は

**Premium Tier + Geo-Replication + Data Persistence**

である。

---

# 2 Azure Cache for Redis

Azure Cache for Redis は、Redis をベースとした **マネージドキャッシュサービス**である。

主な用途

- セッション管理
    
- 分散キャッシュ
    
- リアルタイムデータ処理
    
- メッセージキュー
    

基本構造

```text
Application
     │
     ▼
Azure Cache for Redis
     │
     ▼
In-Memory Data Store
```

Redis はメモリ上でデータを管理するため非常に高速である。

---

# 3 マルチリージョンアーキテクチャ

高可用性を確保するため、多くのクラウドシステムでは **マルチリージョン構成**が採用される。

例

```text
Region A
   │
   ├ Web App
   └ Redis Cache
        │
        ▼
Geo Replication
        │
        ▼
Region B
   │
   ├ Web App
   └ Redis Cache
```

この構成では、Region A が停止しても Region B がサービスを継続できる。

---

# 4 Geo-Replication

Geo-Replication は Redis の **リージョン間レプリケーション機能**である。

動作

```text
Primary Redis
      │
      ▼
Replication
      │
      ▼
Secondary Redis
```

特徴

- 異なる Azure リージョンにレプリカを作成
    
- データ同期
    
- 災害復旧対応
    

レプリカは通常 **読み取り専用（read-only）** である。

---

# 5 データ永続化（Persistence）

Redis は基本的にメモリデータベースであるため、ノード障害時にデータが失われる可能性がある。

そのため **データ永続化（Persistence）** を有効にすることが重要である。

Azure Redis Premium では次の永続化方式が提供される。

### RDB Persistence

定期的にデータをスナップショットとして保存する。

```text
Memory
   │
   ▼
Snapshot
   │
   ▼
Azure Storage
```

---

### AOF Persistence

すべての書き込み操作をログとして保存する。

```text
Write Operation
   │
   ▼
Append Log
```

永続化を有効にすることで、ノード障害時でもデータを復元できる。

---

# 6 Premium Tier の特徴

Azure Cache for Redis には複数のサービス階層がある。

|Tier|特徴|
|---|---|
|Basic|単一ノード|
|Standard|レプリカあり|
|Premium|高可用性 + Geo機能|

Premium Tier の主な機能

- Geo-Replication
    
- Redis Cluster
    
- Data Persistence
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Network (VNet)]]
- Virtual Network Integration
    

今回の要件を満たすのは **Premium Tier のみ**である。

---

# 7 セッション継続性

ユーザーセッションは次のようなデータを含む。

- ログイン状態
    
- カート情報
    
- ユーザー設定
    

Redis に保存される例

```text
SessionID
   │
   ├ UserID
   ├ LoginToken
   └ CartData
```

Geo-Replication により、セッションデータは別リージョンにもコピーされる。

そのため

```text
Region Failure
      │
      ▼
Failover
      │
      ▼
Session Continues
```

ユーザーは再ログインする必要がない。

---

# 8 他の選択肢の評価

## Active-Active Geo Replication

Redis にはアクティブ-アクティブ構成もあるが

- レイテンシ増加
    
- コンフリクト解決
    
- 運用複雑性
    

などの問題がある。

セッション用途では通常不要である。

---

## Redis Cluster

Redis Cluster はスケーリングのための機能である。

用途

```text
Key Distribution
Across Nodes
```

しかし

- 災害復旧は提供しない
    

---

## Standard Tier

Standard Tier は

- 単一リージョン
    
- レプリカあり
    

しかし

- Geo-Replicationなし
    
- DR能力不足
    

---

# 9 推奨アーキテクチャ

最適な Redis DR 構成

```text
Region A
   │
   └ Azure Cache for Redis (Primary)
          │
          ▼
      Geo Replication
          │
          ▼
Region B
   │
   └ Azure Cache for Redis (Secondary)
```

追加構成

```text
Data Persistence → Azure Storage
```

これにより

- 高速キャッシュ
    
- 災害復旧
    
- セッション継続
    

を実現できる。

---

# 10 まとめ

今回の要件

- マルチリージョンWebアプリ
    
- Redisセッション管理
    
- リージョン障害対応
    
- データ損失最小化
    
- 高パフォーマンス
    

これらを満たす最適な構成は

**Azure Cache for Redis Premium Tier

- Geo-Replication
    
- Data Persistence**
    

である。

この構成により

- 高速セッション管理
    
- リージョン障害耐性
    
- データ保護
    

を同時に実現できる。