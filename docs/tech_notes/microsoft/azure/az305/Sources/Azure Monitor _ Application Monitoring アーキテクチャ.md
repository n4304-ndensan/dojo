[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Monitor]]
# Azure Monitor / Application Monitoring アーキテクチャ

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Application Insights]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Log Analytics]]
（Log Analytics・Application Insights・Service Map）

## 1. 概要

クラウドアプリケーションでは、アプリケーションだけでなく **インフラ・依存サービス・ユーザー行動** などを総合的に監視する必要があります。

Azureではこれを実現するために **Azure Monitor** を中心とした監視基盤が提供されています。

主な目的

- アプリケーションのパフォーマンス監視
    
- インフラリソースの監視
    
- ログ分析
    
- エラー分析
    
- ユーザー行動分析
    
- サービス依存関係の可視化
    

Azureではこれらを複数のサービスで構成します。

主要サービス

- Azure Monitor
    
- Azure Log Analytics
    
- Azure Application Insights
    
- Azure Service Map
    
- Azure Activity Log
    

---

# 2. Azure Monitor

Azure Monitorは **Azureの監視プラットフォームの中心となるサービス**です。

機能

- メトリック収集
    
- ログ収集
    
- アラート
    
- ダッシュボード
    
- 可視化
    

構造

```
Azure Monitor
      │
      ├ Metrics
      ├ Logs
      ├ Alerts
      └ Visualization
```

Azure Monitorは以下のデータを収集します。

- Azureリソースメトリック
    
- OSログ
    
- アプリケーションログ
    
- ネットワークログ
    

---

# 3. Azure Log Analytics

Log Analyticsは **ログ分析サービス**です。

Azure Monitorのログ分析エンジンとして機能します。

特徴

- Azureリソースログ収集
    
- アプリケーションログ収集
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
- VMログ収集
    
- カスタムログ分析
    
- KQLクエリ
    

Log Analyticsでは **Log Analytics Workspace** にログを保存します。

構造

```
Log Analytics Workspace
        │
        ├ Azure Resource Logs
        ├ Application Logs
        ├ VM Logs
        └ Custom Logs
```

---

## Log Analyticsの用途

主な用途

- インフラ監視
    
- ログ分析
    
- 相関分析
    
- トラブルシューティング
    
- セキュリティ分析
    

例

```
Web App CPU使用率
+
Application Response Time
+
Exceptionログ
```

これらを **1つのクエリで分析**できます。

---

# 4. Kusto Query Language（KQL）

Log Analyticsでは **KQL（Kusto Query Language）** を使用します。

例

```
requests
| summarize count() by resultCode
```

KQLを使うことで

- リクエストログ
    
- 例外ログ
    
- パフォーマンスメトリック
    

を横断的に分析できます。

---

# 5. Azure Application Insights

Application Insightsは **アプリケーションパフォーマンス監視（APM）サービス**です。

主な機能

- リクエスト追跡
    
- 例外監視
    
- 依存関係トラッキング
    
- パフォーマンス測定
    
- ユーザー行動分析
    

構造

```
Client Request
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

---

## Application Insightsで取得できる情報

例

- HTTPリクエスト
    
- SQLクエリ
    
- 外部API呼び出し
    
- 例外
    
- レスポンス時間
    

これにより

**コードレベルのパフォーマンス分析**

が可能になります。

---

# 6. 分散トレーシング

Application Insightsでは **分散トレーシング** を提供します。

例

```
Client
  │
  ▼
Web API
  │
  ▼
Service
  │
  ▼
Database
```

各リクエストの処理を **トレースID** で追跡できます。

---

# 7. ユーザー行動分析

Application Insightsでは

- ユーザーアクセス
    
- ページビュー
    
- UI操作
    

を分析できます。

例

- どのページが多く閲覧されるか
    
- どの機能がよく使われるか
    
- UIのクリック率
    

これにより

**ユーザー体験分析**

が可能になります。

---

# 8. Azure Service Map

Service Mapは **アプリケーション依存関係可視化ツール**です。

アプリケーションのコンポーネント間の関係を自動検出します。

例

```
Web App
   │
   ├ SQL Database
   ├ Storage
   └ Redis Cache
```

メリット

- マイクロサービス構造理解
    
- トラブルシューティング
    
- 依存関係分析
    

---

# 9. Azure Activity Log

Activity Logは **Azureリソース操作ログ**です。

記録される内容

- リソース作成
    
- リソース削除
    
- 設定変更
    
- 権限変更
    

例

```
VM Created
Storage Deleted
Network Updated
```

主な用途

- 管理操作監査
    
- セキュリティ監査
    

---

# 10. Azure監視サービスの役割比較

|サービス|主用途|
|---|---|
|Azure Monitor|監視基盤|
|Log Analytics|ログ分析|
|Application Insights|アプリケーション監視|
|Service Map|依存関係可視化|
|Activity Log|管理操作ログ|

---

# 11. アプリケーション監視アーキテクチャ

典型的な構成

```
User
 │
 ▼
Azure Web App
 │
 ├ Application Insights
 │
 ▼
Azure Monitor
 │
 ▼
Log Analytics Workspace
 │
 ├ Logs
 ├ Metrics
 └ Alerts
```

これにより

- アプリ監視
    
- インフラ監視
    
- ログ分析
    

を統合できます。

---

# 12. マイクロサービス監視アーキテクチャ

```
Client
  │
  ▼
API Gateway
  │
  ▼
Microservices
  │
  ├ Application Insights
  │
  ▼
Azure Monitor
  │
  ▼
Log Analytics
  │
  ▼
Service Map
```

これにより

- 分散トレーシング
    
- ログ分析
    
- サービス依存関係可視化
    

が可能になります。

---

# 13. Azure Monitor活用例

例1

**パフォーマンス問題分析**

```
CPU上昇
↓
レスポンス遅延
↓
例外増加
```

例2

**ユーザー行動分析**

```
ページビュー
↓
UIクリック
↓
コンバージョン率
```

例3

**サービス依存関係分析**

```
Web App
↓
Database
↓
External API
```

---

# 14. 監視サービスの使い分け

|目的|サービス|
|---|---|
|アプリケーション監視|Application Insights|
|ログ分析|Log Analytics|
|依存関係可視化|Service Map|
|操作履歴監査|Activity Log|
|統合監視|Azure Monitor|

---

# 15. まとめ

Azureの監視基盤は **Azure Monitor** を中心に構成されています。

役割

```
Azure Monitor
   │
   ├ Application Insights（アプリ監視）
   ├ Log Analytics（ログ分析）
   ├ Service Map（依存関係可視化）
   └ Activity Log（操作ログ）
```

この組み合わせにより

- アプリケーション監視
    
- インフラ監視
    
- ログ分析
    
- ユーザー行動分析
    
- 依存関係可視化
    

を統合的に実現できます。

---
