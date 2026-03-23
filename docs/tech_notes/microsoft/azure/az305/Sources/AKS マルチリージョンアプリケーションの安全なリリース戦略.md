[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Kubernetes Service (AKS)]]
# AKS マルチリージョンアプリケーションの安全なリリース戦略

（Deployment Rings + Approval Gates）

---

# 1 背景とシナリオ

現代のクラウドアプリケーションでは、単純に新しいバージョンを一度に本番へデプロイする方法はリスクが高い。特に **マイクロサービスアーキテクチャ** と **Kubernetes（AKS）** を利用する場合、システムは多くのサービスとリージョンで構成されるため、誤ったデプロイが広範囲に影響を与える可能性がある。

今回のシナリオでは、企業は **Azure Kubernetes Service (AKS)** を使用したマルチリージョンのマイクロサービスアプリケーションを運用している。CI/CD パイプラインには Azure DevOps が使用されており、新しいアプリケーションバージョンを安全にリリースする仕組みが必要になっている。

このシステムには次のような要件がある。

- 各リージョンの **ごく一部のユーザーで新バージョンをテストする**
    
- システムの **ヘルスチェックを監視する**
    
- 問題があれば **自動的にロールバックする**
    
- 次のユーザーグループへ進む前に **手動承認が必要**
    

このような要件を満たす最適なリリース戦略は

**Deployment Rings（承認ゲート付き展開リング）**

である。

---

# 2 Deployment Rings（展開リング）

Deployment Rings は、アプリケーションを段階的にユーザーへ公開する戦略である。ユーザーを複数の「リング」に分割し、リングごとに段階的にリリースする。

例

```text
Ring 0   Internal users
Ring 1   Canary users (1%)
Ring 2   Pilot users (10%)
Ring 3   Regional rollout (50%)
Ring 4   Global rollout (100%)
```

この戦略の目的は次の通りである。

- リスクの低減
    
- 問題の早期検出
    
- 段階的な拡張
    

---

# 3 マルチリージョン環境での展開

マルチリージョンアプリケーションでは、リージョンごとに段階的な展開を行う。

例

```text
Region A
  └ Canary users

Region B
  └ Canary users

Region C
  └ Canary users
```

展開フロー

```text
Canary → Pilot → Regional → Global
```

この方式により、すべてのユーザーに影響を与える前に問題を検出できる。

---

# 4 Azure DevOps における展開リング

Azure DevOps Release Pipeline では、複数のステージを使用してリングを実装できる。

例

```text
Stage 1  Canary deployment
Stage 2  Pilot deployment
Stage 3  Regional deployment
Stage 4  Global deployment
```

各ステージの特徴

|ステージ|対象ユーザー|
|---|---|
|Canary|1%|
|Pilot|10%|
|Regional|50%|
|Global|100%|

---

# 5 Approval Gates

Azure DevOps では、各ステージに **Approval Gates（承認ゲート）** を設定できる。

これは次のリングへ進む前に **人間の承認を必要とする仕組み**である。

フロー

```text
Deployment
   │
   ▼
Health Check
   │
   ▼
Manual Approval
   │
   ▼
Next Ring
```

この仕組みにより

- 誤ったデプロイを防止
    
- 運用チームの確認
    
- 安全なリリース管理
    

が可能になる。

---

# 6 自動ロールバック

Deployment Rings では、ヘルスチェックの結果に基づいて自動ロールバックが実行できる。

例

監視項目

- エラー率
    
- レスポンス時間
    
- CPU使用率
    
- Pod failure
    

異常検出

```text
Error rate > threshold
```

自動処理

```text
Rollback to previous version
```

この仕組みにより、問題が検出された場合は自動的に以前のバージョンに戻る。

---

# 7 AKS との統合

AKS 環境では、Deployment Rings は Kubernetes のデプロイメント戦略と組み合わせて使用される。

典型的な構成

```text
Azure DevOps Pipeline
        │
        ▼
AKS Deployment
        │
        ▼
Canary pods
        │
        ▼
Health monitoring
        │
        ▼
Approval Gate
        │
        ▼
Next ring
```

監視には以下のサービスが使用される。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Monitor]]
- Azure Monitor
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Application Insights]]
- Application Insights
    
- Prometheus
    
- Grafana
    

---

# 8 他の展開戦略との比較

## Blue-Green Deployment

Blue-Green は2つの環境を使用する。

```text
Blue  → current version
Green → new version
```

デプロイ後

```text
Traffic switch
```

メリット

- 即時ロールバック
    

デメリット

- ユーザー段階展開不可
    
- インフラコスト増加
    

---

## Canary Deployment

Canary は少数ユーザーでテストする方法である。

```text
1% → 10% → 50% → 100%
```

しかし

- 手動承認機能が標準ではない
    
- リング管理が弱い
    

---

## Rolling Update

Rolling Update は Kubernetes の標準デプロイ方式である。

```text
Pod 1 update
Pod 2 update
Pod 3 update
```

しかし

- ユーザーセグメント制御なし
    
- マルチリージョン制御なし
    

---

# 9 推奨アーキテクチャ

Azure DevOps + AKS のリリース構成

```text
Developers
     │
     ▼
Azure DevOps CI/CD
     │
     ▼
Deployment Rings
     │
     ├ Ring 0  Canary
     ├ Ring 1  Pilot
     ├ Ring 2  Regional
     └ Ring 3  Global
     │
     ▼
AKS Clusters (Multi-region)
```

この構成により

- 安全な段階リリース
    
- 自動ロールバック
    
- 手動承認
    

を実現できる。

---

# 10 まとめ

今回の要件

- 少数ユーザーで新バージョンをテスト
    
- ヘルスチェック
    
- 自動ロールバック
    
- 手動承認
    
- マルチリージョン AKS
    

これらを満たす展開戦略は

**Deployment Rings with Approval Gates**

である。

この戦略は

- 段階的リリース
    
- 安全性
    
- ガバナンス
    

を両立できるため、Azure DevOps を使用した AKS マイクロサービスアプリケーションのリリース戦略として最適である。