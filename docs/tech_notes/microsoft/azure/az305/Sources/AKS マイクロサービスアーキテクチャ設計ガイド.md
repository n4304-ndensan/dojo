[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Kubernetes Service (AKS)]]
## AKS マイクロサービスアーキテクチャ設計ガイド

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Dapr]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Istio]]
### ― Dapr と Istio を利用した分散アプリケーション機能の実装 ―

---

# 1 背景と問題のコンテキスト

クラウドネイティブアプリケーションでは、従来のモノリシックアーキテクチャではなく **マイクロサービスアーキテクチャ** が主流となっている。マイクロサービスではアプリケーションを小さなサービス単位に分割し、それぞれを独立したコンテナとして実行する。これにより、スケーラビリティや開発効率が向上する。

Azure 環境でこのようなコンテナ化されたアプリケーションを実行する場合、通常は **Azure Kubernetes Service（AKS）** が利用される。AKS は Kubernetes クラスターをマネージドサービスとして提供し、コンテナのオーケストレーションやスケーリングを簡単に実装できる。

しかし、マイクロサービスアーキテクチャでは次のような分散システム特有の課題が発生する。

- サービス間のメッセージング
    
- 非同期通信
    
- イベント処理
    
- トラフィック制御
    
- バージョン別トラフィック分割
    

今回の問題では、AKS 上のマイクロサービスアプリケーションに対して次の2つの機能を実装する必要がある。

1. **Pub/Sub メッセージング**
    
2. **トラフィックルーティングおよびトラフィック分割**
    

さらに重要な要件として

**運用および管理上のオーバーヘッドを最小限にする**

という条件がある。

これらの要件を満たすために適切な技術は次の通りである。

|機能|推奨技術|
|---|---|
|Pub/Subメッセージング|Dapr|
|トラフィックルーティング|Istio|

---

# 2 AKS におけるマイクロサービス構成

AKS 上でのマイクロサービスは通常、次のような構造になる。

```text
Client
   │
   ▼
Ingress / Gateway
   │
   ▼
Microservice A
Microservice B
Microservice C
```

各サービスは Kubernetes Pod として実行され、HTTP や gRPC、イベントメッセージなどで通信する。

しかし、サービス数が増えると次の問題が発生する。

- サービス間通信が複雑になる
    
- メッセージング機構を個別実装する必要がある
    
- トラフィック制御が難しくなる
    

これらを解決するために **クラウドネイティブのミドルウェア**が利用される。

---

# 3 Dapr（Distributed Application Runtime）

Dapr は **分散アプリケーション開発を簡素化するためのランタイム**である。  
Kubernetes やコンテナ環境で動作し、マイクロサービスに必要な共通機能を提供する。

主な機能は次の通り。

- Pub/Sub メッセージング
    
- 状態管理
    
- サービス呼び出し
    
- シークレット管理
    
- イベント駆動処理
    

Dapr の特徴は **サイドカーアーキテクチャ**である。  
各アプリケーションコンテナの横に Dapr サイドカーが配置される。

```text
Pod
 ├ Application Container
 └ Dapr Sidecar
```

アプリケーションは Dapr API を利用するだけで、複雑な分散システム機能を利用できる。

---

# 4 Dapr による Pub/Sub メッセージング

マイクロサービスでは、同期通信（HTTP）だけでなく **非同期メッセージング** が重要になる。

例えば注文処理システムでは

```text
Order Service
      │
      ▼
Payment Service
      │
      ▼
Shipping Service
```

のようなイベント駆動型処理が必要になる。

Dapr を使用すると Pub/Sub メッセージングを簡単に実装できる。

```text
Publisher Service
       │
       ▼
Dapr Pub/Sub
       │
       ▼
Subscriber Service
```

Dapr は内部で次のようなメッセージング基盤と接続できる。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Service Bus]]
- Azure Service Bus
    
- Kafka
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Cache for Redis]]
- Redis
    
- RabbitMQ
    

開発者はメッセージング基盤を意識せず、Dapr API を呼び出すだけでよい。

この仕組みにより

- アプリケーションコードの簡素化
    
- メッセージング基盤の抽象化
    
- 運用負荷の削減
    

が実現される。

そのため **Pub/Sub メッセージング機能を実装する場合は Dapr が推奨される**。

---

# 5 Istio（Service Mesh）

Istio は Kubernetes 向けの **Service Mesh（サービスメッシュ）** である。  
Service Mesh はマイクロサービス間の通信を制御するためのインフラ層である。

Istio は次の機能を提供する。

- トラフィックルーティング
    
- トラフィック分割
    
- セキュリティ（mTLS）
    
- 可観測性
    
- リトライ制御
    

Istio も Dapr と同様に **サイドカー方式**を使用する。

```text
Pod
 ├ Application Container
 └ Envoy Proxy (Sidecar)
```

Envoy プロキシがサービス間通信を制御する。

---

# 6 Istio によるトラフィックルーティング

Istio を使用すると、サービスへのトラフィックルーティングを細かく制御できる。

例えば、複数バージョンのサービスがある場合

```text
Service v1
Service v2
```

Istio を使えば

```text
90% → v1
10% → v2
```

のようなトラフィック分割が可能になる。

```text
Client
   │
   ▼
Istio Gateway
   │
   ├ 90% → Service v1
   └ 10% → Service v2
```

これは **カナリアリリース** や **ブルーグリーンデプロイ**に使用される。

---

# 7 なぜ他の選択肢が不適か

### Flux

Flux は Kubernetes の **GitOps ツール**である。

役割

- Kubernetes リソースの Git 管理
    
- 自動デプロイ
    

構造

```text
Git Repository
      │
      ▼
Flux
      │
      ▼
Kubernetes Cluster
```

Flux は **デプロイ管理ツール**であり

- Pub/Sub
    
- トラフィックルーティング
    

などの機能は提供しない。

---

### Dapr

Dapr は

- Pub/Sub
    
- 状態管理
    
- イベント処理
    

を提供するが

**トラフィック分割やサービスメッシュ機能は提供しない**。

---

# 8 完成アーキテクチャ

最終構成は次のようになる。

```text
Client
   │
   ▼
Istio Gateway
   │
   ▼
Microservices (AKS)
 ├ Service A
 ├ Service B
 └ Service C
      │
      ▼
Dapr Pub/Sub
      │
      ▼
Message Broker
```

役割

|コンポーネント|役割|
|---|---|
|AKS|コンテナ実行基盤|
|Dapr|Pub/Sub メッセージング|
|Istio|トラフィック制御|
|Service Bus/Kafka|メッセージブローカー|

---

# 9 マイクロサービス機能の整理

AKS 上でよく利用される機能は次の通り。

|機能|ツール|
|---|---|
|Pub/Sub メッセージング|Dapr|
|状態管理|Dapr|
|サービス間通信|Dapr|
|トラフィックルーティング|Istio|
|トラフィック分割|Istio|
|デプロイ管理|Flux|

---

# 10 まとめ

今回の問題では AKS 上のマイクロサービスに対して次の機能を実装する必要がある。

1. Pub/Sub メッセージング
    
2. トラフィックルーティングと分割
    

最適な技術は次の通り。

|機能|推奨ソリューション|
|---|---|
|Pub/Sub|Dapr|
|トラフィックルーティング|Istio|

この構成により

- 分散システム機能の簡素化
    
- Kubernetes ネイティブな通信管理
    
- 運用オーバーヘッドの削減
    

を実現できる。