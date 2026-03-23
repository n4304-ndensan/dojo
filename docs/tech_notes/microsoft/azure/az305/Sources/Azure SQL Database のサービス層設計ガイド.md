[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
## Azure SQL Database のサービス層設計ガイド

### ― サーバーレスコンピューティングモデル（General Purpose Serverless）―

---

# 1 概要

Azure SQL Database は、Microsoft Azure が提供する **フルマネージドのリレーショナルデータベースサービス（PaaS）**である。  
Microsoft SQL Server エンジンをベースにしており、ユーザーはデータベースの管理やバックアップ、パッチ適用などの運用作業をほぼ意識することなく利用できる。

Azure SQL Database には複数のサービス層（Service Tier）が存在し、それぞれが異なるワークロード特性に対応している。

今回の要件は以下である。

- コンピューティングリソースが **ワークロード需要に応じて自動スケール**
    
- **1秒単位の課金**
    
- コスト効率を重視
    

これらの要件を満たすサービス層は

**General Purpose（サーバーレス）**

である。

このサーバーレスモデルは Azure SQL Database における **自動スケーリング型のコンピューティングモデル**であり、従量課金と自動スケールを実現する。

---

# 2 背景

従来のデータベース環境では、データベースサーバーは常に稼働しており、ワークロードが少ない時間帯でも同じリソースを消費する。

典型的な問題

- 夜間は負荷が低い
    
- バッチ処理時のみ高負荷
    
- 常時最大サイズのサーバーを確保する必要
    

つまり

**ピーク負荷に合わせてリソースを確保する必要がある**

結果

- コストが高い
    
- リソース効率が悪い
    

この問題を解決するために Azure SQL Database では

**Serverless compute**

が提供されている。

Serverlessモデルでは

- CPU自動スケール
    
- 自動一時停止
    
- 従量課金
    

が可能になる。

---

# 3 Azure SQL Database のサービス層

Azure SQL Database には複数のサービス層がある。

|サービス層|主用途|
|---|---|
|Basic|小規模アプリ|
|Standard|中規模アプリ|
|General Purpose|一般的な業務DB|
|Business Critical|高可用トランザクション|
|Hyperscale|超大規模DB|

今回の要件は

**自動スケール + 秒課金**

であるため

**General Purpose Serverless**

が適している。

---

# 4 Serverless コンピューティングモデル

Serverless モデルでは、Azure が自動的にコンピューティングリソースを調整する。

通常の構成

```text
Application
     │
     ▼
Azure SQL Database
     │
     ▼
Compute Resources
```

Serverless構成では次のようになる。

```text
Application
     │
     ▼
Azure SQL Database
     │
     ▼
Auto Scaling Compute
     │
     ├ Low workload → small compute
     └ High workload → larger compute
```

つまり

**CPUが自動的に増減する。**

---

# 5 自動スケーリング

Serverless モデルでは vCore が自動調整される。

例

```text
Min vCore = 0.5
Max vCore = 4
```

ワークロード

```text
Low load
   ↓
0.5 vCore

High load
   ↓
4 vCore
```

この仕組みにより

- リソース最適化
    
- コスト削減
    

が可能になる。

---

# 6 自動一時停止（Auto Pause）

Serverlessモデルの特徴として

**Auto Pause**

がある。

一定時間アクセスがない場合

```text
Database
   │
Idle
   │
   ▼
Auto Pause
```

再度アクセスすると

```text
Request
   │
   ▼
Auto Resume
```

これにより

**アイドル時間の課金を削減できる。**

---

# 7 秒単位課金

Serverlessでは

**秒単位課金**

が可能。

通常のSQL Database

```text
固定料金
月額課金
```

Serverless

```text
Compute usage
     │
     ▼
Per second billing
```

つまり

**使用した分だけ支払うモデル。**

---

# 8 他サービス層との違い

## Basic / Standard

用途

- 小規模DB
    
- 固定リソース
    

問題

- 自動スケールなし
    
- 秒課金なし
    

---

## Business Critical

用途

- 高トランザクション
    
- 低レイテンシ
    

特徴

- ローカルSSD
    
- Always On replica
    

問題

- コスト高
    
- Serverlessなし
    

---

## Hyperscale

用途

- 数TB〜数百TB
    

特徴

- 分散ストレージ
    
- 超大規模
    

問題

- 秒課金なし
    

---

## General Purpose Serverless

唯一

- Auto scale
    
- 秒課金
    
- Auto pause
    

を提供する。

---

# 9 アーキテクチャ

Serverless SQL Database を使用した構成

```text
Application
     │
     ▼
Azure App Service
     │
     ▼
Azure SQL Database
(Serverless)
     │
     ▼
Auto Scaling Compute
     │
     ▼
Azure Storage
```

---

# 10 ユースケース

Serverless SQL Database は次のようなケースに最適。

### 開発環境

```text
Dev App
   │
   ▼
Serverless SQL
```

利用時間が短いためコスト削減。

---

### 不規則ワークロード

```text
Business App
   │
Traffic spikes
   │
   ▼
Serverless SQL
```

負荷に応じてスケール。

---

### SaaSアプリ

```text
Tenant Apps
    │
    ▼
Serverless SQL
```

アイドル時間が多い。

---

# 11 設計指針

Azure SQL Database の選択基準

|条件|推奨|
|---|---|
|自動スケール|Serverless|
|秒課金|Serverless|
|大規模DB|Hyperscale|
|高トランザクション|Business Critical|
|一般用途|General Purpose|

---

# 12 まとめ

今回の要件

- コンピューティング自動スケール
    
- 秒課金
    
- コスト効率
    

これらを満たす Azure SQL Database のサービス層は

**General Purpose（Serverless）**

である。

アーキテクチャ

```text
Application
   │
   ▼
Azure SQL Database
(Serverless)
   │
   ▼
Auto Scaling Compute
   │
   ▼
Per Second Billing
```

Serverless SQL Database は

- 自動スケーリング
    
- 自動停止
    
- 従量課金
    

を提供するため、**不規則なワークロードのアプリケーションに最適なデータベースプラットフォーム**となる。