[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Machine Learning]]
# Azure Machine Learning によるリアルタイム不正検出モデルのデプロイ設計

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Managed Online Endpoint]]
（Managed Online Endpoint + GPU 推論）

---

# 1 背景とシナリオ

金融取引やオンライン決済システムでは、不正検出（Fraud Detection）のために機械学習モデルがリアルタイムで利用されることが多い。例えばクレジットカード取引では、ユーザーが決済を行った瞬間にその取引が不正であるかどうかを判定する必要がある。このような用途では、モデルの推論処理が数百ミリ秒以内に完了する必要があり、非常に低いレイテンシが求められる。

今回のシナリオでは、企業が不正検出のためのリアルタイム機械学習モデルを Azure 上に導入している。モデルは Docker コンテナとしてパッケージ化されており、以下の要件を満たす必要がある。

- GPU を利用した高速推論（GPU acceleration）
    
- リクエスト数に応じた自動スケーリング
    
- コンテナベースのモデルデプロイ
    
- インフラ管理のオーバーヘッドを最小化
    

これらの条件を満たす Azure Machine Learning のコンピューティングターゲットとして最適なのが **Azure Machine Learning Managed Online Endpoint（GPU 推論付き）** である。

---

# 2 Azure Machine Learning の推論アーキテクチャ

Azure Machine Learning では、モデルの推論（Inference）を提供するためにいくつかのコンピューティングオプションが用意されている。これらは大きく次の 2 つに分類できる。

- バッチ推論（Batch inference）
    
- リアルタイム推論（Online inference）
    

今回の要件は **リアルタイム推論**であるため、API エンドポイントとしてモデルを公開する必要がある。

一般的なアーキテクチャは次のようになる。

```text
Client Application
        │
        ▼
Azure ML Managed Online Endpoint
        │
        ▼
GPU Inference Container
        │
        ▼
Machine Learning Model
```

クライアントは REST API を通じて推論リクエストを送信し、Azure Machine Learning が GPU コンテナで推論処理を実行する。

---

# 3 Managed Online Endpoint

Managed Online Endpoint は Azure Machine Learning が提供する **フルマネージドのリアルタイム推論サービス**である。ユーザーはインフラの管理を行う必要がなく、モデルのデプロイ、スケーリング、監視は Azure によって管理される。

主な特徴は次の通りである。

- フルマネージドの推論環境
    
- 自動スケーリング
    
- GPU サポート
    
- Docker コンテナ対応
    
- 高可用性
    

これにより、データサイエンティストや ML エンジニアはインフラ管理ではなくモデル開発に集中できる。

---

# 4 GPU 推論

機械学習モデルの推論処理は CPU でも実行できるが、深層学習モデルなどでは GPU を利用することで大幅なパフォーマンス向上が期待できる。

GPU を利用するメリットは次の通りである。

- 大規模行列演算の高速化
    
- 推論レイテンシの削減
    
- 高スループット処理
    

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
Azure Machine Learning の Managed Endpoint は GPU 対応 VM を使用した推論ノードを自動的にプロビジョニングできる。

例

```text
GPU VM
   └ NVIDIA GPU
        └ CUDA
            └ ML model inference
```

これにより、リアルタイム不正検出のような低レイテンシ要件に対応できる。

---

# 5 自動スケーリング

リアルタイム推論サービスでは、トラフィック量が時間によって大きく変動することがある。例えば金融サービスでは、昼間の取引量が夜間よりも大幅に多い。

Managed Endpoint はリクエスト数に応じて自動的にスケールする。

スケーリング例

```text
Requests per second
      │
      ▼
1 instance → 5 instances → 10 instances
```

Azure が自動的にインスタンス数を増減させるため、手動のインフラ管理は不要である。

---

# 6 Docker コンテナのサポート

今回のモデルは Docker コンテナとしてパッケージ化されている。Azure Machine Learning はコンテナベースのデプロイをネイティブにサポートしている。

デプロイ構成は次のようになる。

```text
Docker Image
   │
   ▼
Azure Container Registry
   │
   ▼
Azure ML Managed Endpoint
   │
   ▼
GPU inference container
```

これにより、ローカル環境とクラウド環境の実行環境を統一できる。

---

# 7 他のコンピューティングターゲットとの比較

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Kubernetes Service (AKS)]]
## Azure Kubernetes Service（GPU ノードプール）

AKS を使用すると GPU ノードを持つ Kubernetes クラスターでモデルを実行できる。

メリット

- 高い柔軟性
    
- Kubernetes ベースのスケーリング
    

しかし次の問題がある。

- Kubernetes クラスター管理が必要
    
- GPU ドライバー管理
    
- スケーリング設定
    

つまり **運用負荷が大きい**。

---

## Azure Container Instances（GPU）

Azure Container Instances は簡単にコンテナを実行できるサービスである。

メリット

- セットアップが簡単
    

しかし

- 自動スケーリングが限定的
    
- 本番推論向けではない
    

リアルタイム推論には適していない。

---

## Azure ML Compute Cluster

Compute Cluster は主に **トレーニングやバッチ推論**のためのコンピューティングリソースである。

用途

- モデル学習
    
- 大規模データ処理
    
- バッチ推論
    

リアルタイム API 推論には最適ではない。

---

# 8 推奨アーキテクチャ

最適なアーキテクチャは次のようになる。

```text
Client Application
       │
       ▼
Azure API / Application
       │
       ▼
Azure ML Managed Online Endpoint
       │
       ▼
GPU inference nodes
       │
       ▼
Fraud Detection Model
```

この構成により

- GPU アクセラレーション
    
- 自動スケーリング
    
- コンテナデプロイ
    
- インフラ管理の最小化
    

を実現できる。

---

# 9 まとめ

今回の要件

- リアルタイム不正検出モデル
    
- GPU アクセラレーション
    
- Docker コンテナ
    
- 自動スケーリング
    
- 運用オーバーヘッド最小化
    

これらを満たす Azure Machine Learning のコンピューティングターゲットは

**Azure Machine Learning Managed Online Endpoint（GPU 推論付き）**

である。

このサービスはフルマネージドのリアルタイム推論環境を提供し、GPU を利用した高性能推論と自動スケーリングを実現しながら、インフラ管理の負担を大幅に削減できる。