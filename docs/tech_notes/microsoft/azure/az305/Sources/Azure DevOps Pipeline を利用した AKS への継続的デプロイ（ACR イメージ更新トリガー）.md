[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Kubernetes Service (AKS)]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Container Registry]]
## Azure DevOps Pipeline を利用した AKS への継続的デプロイ（ACR イメージ更新トリガー）

### 1 概要

Azure Container Registry (ACR) に新しいコンテナイメージがプッシュされた際に、  
Azure Kubernetes Service (AKS) へ自動的にアプリケーションをデプロイする仕組みは  
**CI/CD (Continuous Integration / Continuous Deployment)** パイプラインによって実現される。

Azure 環境ではこの仕組みを実装するために **Azure DevOps Pipelines** を利用することが一般的である。

この構成により

・コンテナイメージの更新検知  
・自動ビルド  
・自動デプロイ

が実現され、コンテナアプリケーションの **継続的デリバリー (Continuous Delivery)** が可能になる。

---

# 2 背景

コンテナアプリケーション開発では、次のような流れが一般的である。

```
コード変更
↓
コンテナイメージビルド
↓
レジストリにPush
↓
Kubernetesへデプロイ
```

しかし手動でこれを行うと

・デプロイミス  
・環境差異  
・作業負荷

が発生する。

そのため **CI/CD パイプライン** によって自動化する。

Azure の標準的な DevOps アーキテクチャでは

```
Source Code
↓
Build Pipeline
↓
Container Registry
↓
Release Pipeline
↓
AKS
```

という構成が採用される。

---

# 3 サービスの仕組み

Azure DevOps Pipeline は

・ビルド  
・テスト  
・デプロイ

を自動化するパイプラインサービスである。

AKS デプロイの流れは次のようになる。

```
Developer
   │
   ▼
Git Repository
   │
   ▼
Azure DevOps Pipeline
   │
   ▼
Docker Build
   │
   ▼
Azure Container Registry
   │
   ▼
AKS Deployment
```

イメージ更新をトリガーにデプロイを実行できる。

```
ACR
│
│  new image push
▼
Pipeline Trigger
│
▼
kubectl apply
│
▼
AKS
```

---

# 4 主要機能

### CI（Continuous Integration）

コード変更時に自動的に

・ビルド  
・テスト

を実行する。

```
Git Push
   │
   ▼
Pipeline
   │
   ├ Build
   ├ Test
   └ Image Build
```

---

### CD（Continuous Deployment）

ビルドされたコンテナイメージを

AKS クラスターに自動デプロイする。

```
Container Image
      │
      ▼
Kubernetes Manifest
      │
      ▼
kubectl apply
```

---

### ACR トリガー

ACR の新しいイメージが作成されると

Pipeline を自動起動できる。

```
Docker Push
      │
      ▼
Azure Container Registry
      │
      ▼
Pipeline Trigger
```

---

### Kubernetes Deployment

Pipeline から AKS へデプロイする際には

・kubectl  
・Helm  
・Kubernetes Manifest

などを使用する。

例

```
kubectl apply -f deployment.yaml
```

---

# 5 関連Azureサービス

この CI/CD アーキテクチャでは以下の Azure サービスが連携する。

|サービス|役割|
|---|---|
|Azure DevOps|CI/CD|
|Azure Container Registry|コンテナイメージ|
|Azure Kubernetes Service|実行環境|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Monitor]]
|Azure Monitor|ログ|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Key Vault]]
|Azure Key Vault|シークレット管理|

典型的な構成

```
Developer
  │
  ▼
Git Repository
  │
  ▼
Azure DevOps
  │
  ▼
ACR
  │
  ▼
AKS
```

---

# 6 アーキテクチャ

典型的な AKS デプロイパイプライン

```
Developers
     │
     ▼
Git Repository
     │
     ▼
Azure DevOps Pipeline
     │
     ├ Build Docker Image
     │
     ├ Push Image
     │
     ▼
Azure Container Registry
     │
     ▼
Deploy
     │
     ▼
Azure Kubernetes Service
```

クラスター内部

```
AKS Cluster
│
├ Node Pool
│
├ Deployment
│
│   ├ Pod
│   ├ Pod
│   └ Pod
│
└ Service
```

---

# 7 他サービスとの違い

### Azure Automation Runbook

Runbook は

・スクリプト自動化  
・定期処理

のためのサービス。

用途

```
VM Start
VM Stop
Cleanup
```

CI/CD パイプライン用途には不適。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Resource Manager (ARM)]]
### ARM Template

ARM テンプレートは

**Infrastructure as Code**

のための仕組み。

```
VM
VNet
AKS
```

などの **インフラ作成** に使う。

アプリケーションデプロイ用途ではない。

---

### CRON Job

CRON は

```
毎日
毎時間
```

などの **スケジュール実行**。

コンテナイメージ更新トリガーには適さない。

---

# 8 ユースケース

Azure DevOps + AKS は

次のようなシステムで使用される。

### マイクロサービス

```
Service A
Service B
Service C
```

それぞれ CI/CD を実装。

---

### Webアプリ

```
Frontend
Backend
API
```

コンテナとして自動デプロイ。

---

### SaaS

頻繁なリリースが必要なサービス。

---

# 9 設計指針

Azure DevOps Pipeline を使うべきケース

・コンテナCI/CD  
・AKS デプロイ  
・自動リリース  
・Gitベース開発

別の選択肢

|サービス|用途|
|---|---|
|GitHub Actions|GitHubベースCI/CD|
|ArgoCD|GitOps|
|Flux|GitOps|

AKSでは **GitOps** もよく利用される。

---

# 10 まとめ

Azure Container Registry のイメージ更新に基づき  
AKS にコンテナを継続的にデプロイするには

**CI/CD パイプラインが必要である。**

Azure 環境では

**Azure DevOps Pipeline**

が標準的な実装となる。

構成

```
Git
↓
Azure DevOps Pipeline
↓
ACR
↓
AKS
```

この設計により

・自動ビルド  
・自動デプロイ  
・継続的リリース

が実現できる。