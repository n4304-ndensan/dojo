[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Container Registry]]
# Azure Container Registry SKU 比較ドキュメント

（Basic / Standard / Premium）

## 1 概要

**Azure Container Registry (ACR)** は、DockerコンテナイメージおよびOCIアーティファクトを保存するための **プライベートコンテナレジストリサービス**である。

主な用途

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Kubernetes Service (AKS)]]
- Kubernetes (AKS) へのコンテナ配布
    
- CI/CDパイプラインのイメージ管理
    
- マルチリージョンコンテナ配布
    
- Helmチャート / OCIアーティファクト管理
    

典型構成

```
Developer
   │
   ▼
CI/CD Pipeline
   │
   ▼
Azure Container Registry
   │
   ▼
AKS Cluster
```

ACRには次の3つのSKUが存在する。

|SKU|用途|
|---|---|
|Basic|開発 / 小規模|
|Standard|本番|
|Premium|エンタープライズ / マルチリージョン|

---

# 2 背景

コンテナベースアーキテクチャでは **コンテナイメージ管理**が重要になる。

従来

```
Docker Hub
```

を利用するケースが多かったが、企業では次の問題がある。

- セキュリティ
    
- プライベートイメージ管理
    
- レイテンシ
    
- リージョン分散
    

そのためクラウドネイティブ環境では

```
Private Container Registry
```

が必要になる。

Azureではこれを

**Azure Container Registry**

が提供する。

---

# 3 Azure Container Registry の仕組み

ACRは **Docker Registry v2互換サービス**である。

基本構造

```
Developer
   │
   ▼
Docker Push
   │
   ▼
Azure Container Registry
   │
   ▼
Docker Pull
   │
   ▼
AKS / Container Apps
```

処理フロー

1. 開発者がコンテナイメージをビルド
    
2. `docker push` でACRへ保存
    
3. AKSが `docker pull` で取得
    

---

# 4 SKUの違い

## Basic SKU

Basicは **最小機能のレジストリ**である。

用途

- 開発環境
    
- テスト環境
    

特徴

|項目|内容|
|---|---|
|ストレージ|小|
|パフォーマンス|低|
|Webhook|あり|
|Geo Replication|なし|

制限

- スケール制限
    
- レプリケーションなし
    

---

## Standard SKU

Standardは **本番利用の一般的なSKU**である。

用途

- 小〜中規模プロダクション
    
- AKS運用
    

特徴

|項目|内容|
|---|---|
|ストレージ|中|
|パフォーマンス|中|
|Webhook|あり|
|Geo Replication|なし|

改善点

- Basicより高スループット
    
- より多くのリクエスト処理
    

ただし

**マルチリージョンレプリケーションは不可**

---

## Premium SKU

Premiumは **エンタープライズ向けSKU**である。

特徴

|機能|内容|
|---|---|
|Geo Replication|あり|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Private Link]]
|Private Link|あり|
|Content Trust|あり|
|高スループット|あり|

最大の特徴

**Geo Replication**

---

# 5 Geo Replication

Geo Replicationは

**複数リージョンにイメージを自動コピー**

する機能である。

構成

```
ACR (Primary)
   │
   ├ Replica (US)
   ├ Replica (Europe)
   └ Replica (Asia)
```

イメージPush

```
docker push
```

すると

```
Primary
  │
  ├ Replica1
  ├ Replica2
  └ Replica3
```

へ自動同期される。

メリット

- 低レイテンシ
    
- 高可用性
    
- マルチリージョンAKS対応
    

---

# 6 関連Azureサービス

ACRは主に **コンテナエコシステム**と統合する。

|サービス|用途|
|---|---|
|Azure Kubernetes Service|コンテナ実行|
|Azure Container Apps|サーバレスコンテナ|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Functions]]
|Azure Functions|コンテナ実行|
|Azure DevOps|CI/CD|
|GitHub Actions|CI/CD|

---

# 7 アーキテクチャ

## 単一リージョン

```
Developer
   │
   ▼
ACR
   │
   ▼
AKS
```

---

## マルチリージョン（Premium）

```
ACR Primary
   │
   ├ Replica US
   ├ Replica EU
   └ Replica Asia

AKS (US)
AKS (EU)
AKS (Asia)
```

AKSは最も近いレプリカからPullする。

---

# 8 ユースケース

## AKSデプロイ

```
CI/CD
  │
  ▼
ACR
  │
  ▼
AKS
```

---

## マルチリージョンアプリ

```
Users
  │
  ▼
Front Door
  │
  ├ AKS US
  └ AKS EU
```

コンテナはACR Geo Replicationから取得。

---

## DevOpsパイプライン

```
GitHub
  │
  ▼
Build
  │
  ▼
ACR
  │
  ▼
Deploy
```

---

# 9 設計指針

アーキテクトは次を判断する。

## 1 環境規模

|環境|SKU|
|---|---|
|開発|Basic|
|本番|Standard|
|マルチリージョン|Premium|

---

## 2 AKS統合

ACRは

```
AKS Managed Identity
```

と統合する。

---

## 3 セキュリティ

Premiumでは

- Private Link
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Role-Based Access Control (RBAC)]]
- RBAC
    
- Content Trust
    

が利用可能。

---

## 4 レイテンシ

マルチリージョンAKSでは

**Premium + Geo Replication**

が推奨される。

---

# 10 まとめ

Azure Container Registryは

**Azureのコンテナイメージ管理サービス**

である。

SKUの違い

|SKU|主用途|
|---|---|
|Basic|開発|
|Standard|本番|
|Premium|マルチリージョン|

重要ポイント

- Basic / Standard → レプリケーションなし
    
- Premium → **Geo Replicationあり**
    

今回の問題では

- **複数リージョンAKS**
    
- **自動レプリケーション**
    

が必要なため

**Azure Container Registry Premium**

が正解となる。