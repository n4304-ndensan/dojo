[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Machine Learning]]
# Azure Machine Learning モデルデプロイアーキテクチャ

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Managed Online Endpoint]]
（Managed Endpoints / GPU推論 / REST API）

## 1. 概要

企業が機械学習モデルを **REST API として公開**する場合、以下の要件がよく求められます。

主な要件

- REST APIとしてモデル推論を提供
    
- 需要に応じた自動スケーリング
    
- GPUアクセラレーション
    
- カスタムPython依存関係
    
- インフラ管理不要
    

Azureではこれを実現するために **Azure Machine Learning Managed Endpoints** を使用します。

Managed Endpointsは、機械学習モデルを **フルマネージドな推論サービスとしてデプロイする仕組み**です。

---

# 2. 機械学習モデルのデプロイとは

機械学習モデルは通常、以下の流れで利用されます。

1. モデル学習
    
2. モデル保存
    
3. 推論APIとして公開
    
4. アプリケーションから利用
    

典型構成

```
Client Application
        │
        │ REST API
        ▼
Model Inference Endpoint
        │
        ▼
Machine Learning Model
```

この **推論API** を提供するのが Azure Machine Learning Managed Endpoints です。

---

# 3. Azure Machine Learning

Azure Machine Learning は **機械学習ライフサイクル管理プラットフォーム**です。

主な機能

- モデル開発
    
- モデル学習
    
- モデル管理
    
- モデルデプロイ
    
- 推論API提供
    

構成

```
Data
 ↓
Model Training
 ↓
Model Registry
 ↓
Model Deployment
 ↓
Inference Endpoint
```

---

# 4. 推論 (Inference)

推論とは、学習済みモデルを使用して予測を行う処理です。

例

入力

```
画像
```

推論

```
AIモデル
```

出力

```
猫 / 犬
```

推論APIとして公開することで、アプリケーションから利用できます。

---

# 5. Azure Machine Learning Managed Endpoints

Managed Endpointsは **Azure MLのマネージド推論サービス**です。

特徴

- REST API提供
    
- 自動スケーリング
    
- GPUサポート
    
- 完全マネージド
    
- モデルバージョン管理
    

構成

```
Client
 ↓
Managed Endpoint
 ↓
Deployment
 ↓
Model Container
```

---

# 6. REST APIとしてのモデル公開

Managed EndpointはHTTP REST APIを提供します。

例

```
POST https://endpoint.azureml.net/score
```

入力

```json
{
  "data": [1.2, 3.4, 5.6]
}
```

出力

```json
{
  "prediction": "class_A"
}
```

これにより

- Webアプリ
    
- モバイルアプリ
    
- マイクロサービス
    

からモデルを利用できます。

---

# 7. 自動スケーリング

Managed Endpointはトラフィックに応じて **自動スケーリング**します。

```
Client Requests
        │
        ▼
Endpoint
   │
   ├ Pod
   ├ Pod
   ├ Pod
```

メリット

- 高可用性
    
- 負荷対応
    
- コスト最適化
    

---

# 8. カスタムPython依存関係

機械学習モデルは多くの場合、特定のPythonライブラリに依存します。

例

- TensorFlow
    
- PyTorch
    
- scikit-learn
    
- NumPy
    

Azure MLでは **Environment定義** を使います。

例

```
environment.yml

dependencies:
  - python=3.10
  - pip
  - pip:
      - torch
      - transformers
      - numpy
```

これにより

- 再現可能環境
    
- 依存関係管理
    

が可能になります。

---

# 9. GPUアクセラレーション

ディープラーニングモデルではGPUが必要になる場合があります。

例

- 画像認識
    
- NLP
    
- 音声認識
    

GPU推論

```
CPU inference → 遅い
GPU inference → 高速
```

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
Managed Endpointでは **GPU VMを指定可能**です。

例

```
Standard_NC6
Standard_ND40
```

---

# 10. Managed Endpoint アーキテクチャ

```
Client Application
        │
        │ HTTPS
        ▼
Azure ML Managed Endpoint
        │
        │
Deployment
        │
        ▼
Container
        │
        ▼
ML Model
```

特徴

- REST API
    
- GPUサポート
    
- autoscale
    

---

# 11. 他の選択肢との比較

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Kubernetes Service (AKS)]]
## Azure Kubernetes Service (AKS)

特徴

- Kubernetes管理
    
- 高い柔軟性
    

問題

- クラスタ管理必要
    
- ノード管理必要
    
- ネットワーク管理必要
    

要件

```
インフラ管理不要
```

に適合しません。

---

## Azure Container Instances (ACI)

特徴

- サーバーレスコンテナ
    

問題

- GPU制限
    
- ML専用機能なし
    
- 長時間推論向きでない
    

---

## Azure ML Compute Instance

用途

- Notebook
    
- モデル開発
    

問題

- 推論API向きではない
    
- autoscaleなし
    

---

# 12. Managed Endpointsのメリット

Managed Endpointは以下を提供します。

|機能|対応|
|---|---|
|REST API|○|
|Autoscale|○|
|GPU推論|○|
|依存関係管理|○|
|インフラ管理不要|○|

---

# 13. 典型MLアーキテクチャ

```
Data Sources
     │
     ▼
Azure Data Lake
     │
     ▼
Model Training
(Azure ML)
     │
     ▼
Model Registry
     │
     ▼
Managed Endpoint
     │
     ▼
Application
```

---

# 14. 推論パターン

## リアルタイム推論

```
User
 ↓
API
 ↓
ML Endpoint
 ↓
Prediction
```

例

- レコメンド
    
- fraud detection
    

---

## バッチ推論

```
Data
 ↓
Batch Job
 ↓
Model
 ↓
Predictions
```

例

- 売上予測
    
- リスク分析
    

---

# 15. Azure MLサービス比較

|サービス|用途|
|---|---|
|Compute Instance|開発|
|Compute Cluster|学習|
|Managed Endpoint|推論|
|Batch Endpoint|バッチ推論|

---

# 16. 試験での判断ポイント

試験問題では以下が重要です。

|キーワード|サービス|
|---|---|
|REST API推論|Managed Endpoint|
|インフラ管理不要|Managed Endpoint|
|GPU推論|Managed Endpoint|
|自動スケール|Managed Endpoint|
|MLモデル公開|Managed Endpoint|

---

# 17. まとめ

Azure Machine LearningでモデルをAPIとして公開する場合

要件

- REST API
    
- GPU
    
- autoscale
    
- Python依存関係
    
- インフラ管理不要
    

これらを満たす最適なサービスは

**Azure Machine Learning Managed Endpoints**

です。

Managed Endpointsは

- フルマネージド
    
- スケーラブル
    
- GPU対応
    

の **Azure標準のML推論基盤**です。

---

必要なら次に  
**Azure Machine Learning 完全アーキテクチャ（試験範囲）**

- ML Pipeline
    
- Feature Store
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Databricks]]
- Databricks連携
    
- Online Endpoint vs Batch Endpoint
    

を **体系図付きの完全ドキュメント**として整理できます。  
これは **AI系Azure問題の全体像**になります。