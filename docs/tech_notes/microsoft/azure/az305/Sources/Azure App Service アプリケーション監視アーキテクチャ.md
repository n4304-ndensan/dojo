[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure App Service]]
# Azure App Service アプリケーション監視アーキテクチャ

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Monitor]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Application Insights]]
（Azure Monitor + Application Insights）

---

# 1 背景とシナリオ

クラウドでアプリケーションを運用する場合、単にアプリケーションをデプロイするだけでは不十分である。実際の運用では次のような問題を常に監視する必要がある。

- レスポンスが遅くなる
    
- エラーが発生する
    
- 外部サービスとの通信が失敗する
    
- 特定の機能だけパフォーマンスが悪化する
    

特に **Azure App Service** 上で Web アプリケーションを実行する場合、次の観点で監視が必要になる。

監視要件

- アプリケーションパフォーマンスの監視
    
- 障害や例外の検出
    
- 外部依存関係（DB・API）の可視化
    
- 異常の自動検知
    
- カスタムメトリクスの収集
    

これらの要件を満たす Azure サービスは

**Azure Monitor + Application Insights**

である。

---

# 2 Azure Monitor とは

Azure Monitor は **Azure 全体の監視プラットフォーム**であり、クラウド環境のメトリクス・ログ・トレースを統合的に管理する。

Azure Monitor は次の役割を持つ。

- Azure リソースの監視
    
- メトリクス収集
    
- ログ分析
    
- アラート
    
- ダッシュボード
    

Azure Monitor は複数のデータソースを統合する。

```text
Azure resources
Applications
VM
Containers
Network

        │
        ▼
Azure Monitor
        │
        ▼
Logs / Metrics / Alerts
```

しかし Azure Monitor だけでは **アプリケーション内部の詳細なテレメトリ**は取得できない。

そのため **Application Insights** が使用される。

---

# 3 Application Insights

Application Insights は **アプリケーション監視（APM）サービス**であり、アプリケーション内部のテレメトリを収集する。

監視できる情報

- リクエスト
    
- 応答時間
    
- 例外
    
- 依存関係呼び出し
    
- カスタムイベント
    

構造

```text
Application
     │
     ▼
Application Insights SDK
     │
     ▼
Telemetry Data
     │
     ▼
Azure Monitor
```

Application Insights は Azure Monitor に統合されている。

---

# 4 Application Insights が提供する主要機能

## 4.1 パフォーマンス監視

Application Insights はアプリケーションのパフォーマンスをリアルタイムで監視する。

収集されるメトリクス

- request rate
    
- response time
    
- failure rate
    

例

```text
Request: 120/sec
Average response: 250ms
Failure rate: 0.5%
```

これによりパフォーマンス問題を迅速に検出できる。

---

## 4.2 依存関係追跡

多くのクラウドアプリケーションは複数のサービスに依存している。

例

- SQL Database
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Cache for Redis]]
- Redis
    
- REST API
    
- Storage
    

Application Insights は依存関係呼び出しを自動的に追跡する。

```text
Web App
   │
   ├ SQL Database
   ├ Redis Cache
   └ External API
```

これにより

- ボトルネックの特定
    
- 外部サービスの遅延
    
- 失敗原因
    

を可視化できる。

---

## 4.3 異常検出（AI-based detection）

Application Insights は機械学習ベースの異常検出を提供する。

例

正常

```text
Response time: 200ms
```

異常

```text
Response time: 1500ms
```

AI は通常のパターンを学習し、異常な挙動を検出する。

---

## 4.4 分散トレーシング

マイクロサービスアーキテクチャでは、1つのリクエストが複数サービスを通過する。

例

```text
User Request
     │
     ▼
API Gateway
     │
     ▼
Web Service
     │
     ▼
Database
```

Application Insights は **分散トレーシング**により、リクエストの流れを追跡できる。

---

## 4.5 カスタムテレメトリ

開発者は独自のメトリクスを追加できる。

例

```csharp
telemetry.TrackEvent("OrderCreated");
```

収集できるもの

- business events
    
- custom metrics
    
- custom traces
    

これによりビジネスロジックの監視が可能になる。

---

# 5 Azure Monitor と Application Insights の統合

Azure Monitor は監視プラットフォームとして次の機能を提供する。

|機能|説明|
|---|---|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Log Analytics]]
|Log Analytics|ログクエリ|
|Metrics|リソースメトリクス|
|Alerts|アラート通知|
|Dashboards|可視化|

Application Insights はこれらに **アプリケーションデータを提供する役割**を持つ。

統合構造

```text
Application
      │
      ▼
Application Insights
      │
      ▼
Azure Monitor
      │
      ├ Alerts
      ├ Logs
      ├ Dashboards
      └ Analytics
```

---

# 6 他の選択肢との比較

## Azure Log Analytics

Log Analytics はログ分析ツールである。

用途

- ログ検索
    
- クエリ分析
    
- トラブルシューティング
    

しかし

- 自動異常検出が弱い
    
- 依存関係追跡なし
    
- アプリ監視機能が不足
    

そのため単体では不十分である。

---

## Azure Service Health

Azure Service Health は

**Azure プラットフォームの状態**

を通知するサービスである。

例

- Azure Datacenter 障害
    
- メンテナンス通知
    
- Azure サービス停止
    

しかし

- アプリケーション監視ではない
    
- パフォーマンス監視不可
    

---

## Azure Network Watcher

Network Watcher は **ネットワーク診断ツール**である。

用途

- NSG flow logs
    
- 接続トラブルシューティング
    
- VPN診断
    

しかし

- アプリケーション監視ではない
    
- テレメトリ収集不可
    

---

# 7 推奨アーキテクチャ

Azure App Service の監視構成

```text
Users
   │
   ▼
Azure App Service
   │
   ▼
Application Insights
   │
   ▼
Azure Monitor
   │
   ├ Alerts
   ├ Logs
   ├ Dashboards
   └ AI anomaly detection
```

この構成により

- パフォーマンス監視
    
- エラー追跡
    
- 依存関係分析
    
- 自動異常検知
    

が実現できる。

---

# 8 実装方法

基本設定

1. App Service に Application Insights を有効化
    
2. SDK をアプリケーションに追加
    
3. Azure Monitor アラート設定
    

例

監視アラート

- response time > 2s
    
- failure rate > 5%
    
- dependency latency > threshold
    

---

# 9 まとめ

今回の要件

- App Service アプリケーション監視
    
- 異常検出
    
- 依存関係分析
    
- カスタムテレメトリ
    

これらを満たす Azure サービスは

**Azure Monitor + Application Insights**

である。

理由

1. エンドツーエンド監視
    
2. AIベース異常検出
    
3. 依存関係トラッキング
    
4. カスタムテレメトリ
    

そのため Azure App Service のアプリケーション監視には **Application Insights を Azure Monitor と組み合わせて使用することが最適な設計**となる。