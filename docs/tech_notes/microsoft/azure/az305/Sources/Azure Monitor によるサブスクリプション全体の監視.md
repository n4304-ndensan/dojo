---
分類: Cloud
tags:
  - cloud/azure
  - cloud/azure/monitor
  - cloud/azure/observability
  - cloud/azure/log-analytics
  - cloud/azure/application-insights
  - cloud/architecture/monitoring
  - cloud/architecture/observability
  - devops/monitoring
  - exam/azure
---

# Azure Monitor によるサブスクリプション全体の監視

## 1. 背景（シナリオ）

ある組織では Azure サブスクリプション内に複数のリソースを運用しています。これには次のようなサービスが含まれます。

- Azure Virtual Machines  
- Azure Database  
- Azure Storage Accounts  

これらのリソースはビジネスアプリケーションの基盤であり、安定した運用が求められます。

そのため、運用チームは次のことを実現する必要があります。

- リソースの健全性の監視  
- パフォーマンスの可視化  
- 問題発生時の通知  

このような要件を満たすためには、サブスクリプション内のすべてのリソースから監視データを収集し、分析および可視化できるサービスが必要になります。

---

## 2. 要件整理

この問題の要件を整理すると次の通りです。

まず、監視対象です。

- 仮想マシン
- データベース
- ストレージアカウント
- その他 Azure リソース

次に、監視機能です。

- パフォーマンスメトリクスの収集
- ログデータの収集
- リソースの状態監視

さらに、運用機能です。

- アラート通知
- 可視化ダッシュボード
- データ分析

これらすべてを統合的に提供するサービスが **Azure Monitor** です。

---

## 3. 技術の基本概念

**Azure Monitor** は Azure 環境の監視を行うための統合プラットフォームです。

Azure Monitor は次の2種類のデータを収集します。

まず、メトリクスです。

- CPU使用率
- メモリ使用率
- ディスクIO
- ネットワークトラフィック

これらは数値データとして収集されます。

次に、ログです。

- リソースログ
- アクティビティログ
- 診断ログ

ログはイベントベースのデータであり、システムの動作を詳細に分析できます。

---

## 4. アーキテクチャまたは設計のポイント

Azure Monitor は複数の監視コンポーネントを統合しています。

主要な構成要素を理解すると全体像が分かりやすくなります。

まず、メトリクス収集です。

- Azure Monitor Metrics  
  → リソースパフォーマンス監視

次に、ログ分析です。

- Log Analytics  
  → ログクエリ分析（KQL）

さらに、アプリケーション監視です。

- Application Insights  
  → アプリケーションパフォーマンス監視

そして通知機能です。

- Azure Monitor Alerts  
  → メール / SMS / Webhook 通知

これらの機能により、Azure 環境全体を包括的に監視できます。

---

## 5. 設計判断（なぜこの構成になるか）

この問題では **サブスクリプション全体の監視** が必要です。

Azure Monitor は次の機能を提供します。

まず、統合監視です。

- すべての Azure リソースを監視
- サブスクリプションレベルの監視

次に、データ分析です。

- メトリクス分析
- ログクエリ（KQL）

さらに、可視化です。

- Azure Dashboards
- Workbooks

最後に、アラートです。

- 閾値ベース通知
- 自動アクション

このため、Azure環境の運用監視の中心的サービスとなります。

---

## 6. 他の選択肢が誤りな理由

まず **Azure Security Center（Microsoft Defender for Cloud）** です。

これはセキュリティ管理サービスであり、パフォーマンス監視の主目的ではありません。

次に **Azure Log Analytics** です。

Log Analytics はログ分析機能ですが、Azure Monitor のコンポーネントの一部です。

単体ではメトリクス収集やアラート機能を完全には提供しません。

最後に **Azure Application Insights** です。

Application Insights はアプリケーション監視専用であり、VM やストレージなどのリソース監視には適していません。

---

## 7. 最終回答

**A. Azure Monitor**

---

## 8. まとめ

Azure の監視サービスは用途ごとに役割があります。

主なサービスを整理すると理解しやすくなります。

まず、統合監視です。

- **Azure Monitor**  
  → Azure リソース全体の監視

次に、ログ分析です。

- **Log Analytics**  
  → ログクエリ分析

さらに、アプリ監視です。

- **Application Insights**  
  → アプリケーションパフォーマンス管理

そして、セキュリティ監視です。

- **Microsoft Defender for Cloud**

そのため、**サブスクリプション内のすべてのAzureリソースの監視データを収集・分析・可視化する場合は Azure Monitor を使用するのが最適な選択**となります。