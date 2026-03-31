[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Monitor]]
# Azure Monitor による Windows VM ログ集中監視設計

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Log Analytics]]
（Log Analytics Workspace + Log Analytics Agent）

---

# 1 背景

ある企業では、Azure 上で **200台の Windows 仮想マシン（VM）** を運用している。  
運用チームは、これらの VM の **システムログ（Windows Event Log）** を監視し、  
異常が発生した場合に **リアルタイムでアラートを受信できる監視システム**を構築する必要がある。

このシステムには次の要件がある。

### 要件

- 200台の Windows VM のログを収集
    
- **警告（Warning）およびエラー（Error）イベントを監視**
    
- **単一のダッシュボードに統合**
    
- **リアルタイムアラート**
    
- Azure ネイティブ監視サービスを使用
    

これらの要件を満たす最適な構成は

**Log Analytics Workspace  
+  
Log Analytics Agent**

である。

---

# 2 Azure Monitor の概要

Azure Monitor は Azure の統合監視プラットフォームであり、次の機能を提供する。

|機能|説明|
|---|---|
|ログ収集|VM / アプリ / サービスのログ|
|メトリック監視|CPU / Memory など|
|アラート|異常検知|
|ダッシュボード|可視化|
|クエリ分析|Kusto Query Language|

Azure Monitor のログ分析基盤として使用されるのが

**Log Analytics Workspace**

である。

---

# 3 Log Analytics Workspace

Log Analytics Workspace は、Azure Monitor の **集中ログストレージおよび分析エンジン**である。

複数の Azure リソースから収集されたログは、すべてこのワークスペースに保存される。

構成イメージ

```text
Azure Resources
      │
      ▼
Log Analytics Workspace
      │
      ▼
Query / Dashboard / Alert
```

このワークスペースにより

- ログの保存
    
- クエリ分析
    
- ダッシュボード
    
- アラート
    

が可能になる。

---

# 4 Log Analytics Agent

Log Analytics Agent（MMA: Microsoft Monitoring Agent）は、  
仮想マシンからログを収集し、Log Analytics Workspace に送信するエージェントである。

各 VM にエージェントをインストールすると、次のデータを収集できる。

|データ|例|
|---|---|
|Windows Event Logs|System / Security / Application|
|パフォーマンスデータ|CPU / Memory|
|カスタムログ|アプリログ|

今回のシナリオでは

**Windows System Event Log**

を収集する。

---

# 5 ログ収集フロー

ログ収集の流れは次の通り。

```text
Windows VM (200台)
       │
       ▼
Log Analytics Agent
       │
       ▼
Log Analytics Workspace
       │
       ▼
Azure Monitor
       │
       ├─ Dashboard
       └─ Alert
```

この構成により、すべての VM のログを **1つの集中ログストア**に集約できる。

---

# 6 警告・エラーイベントの収集

Windows Event Log では、イベントには **Level** が設定されている。

|Level|意味|
|---|---|
|Information|通常ログ|
|Warning|警告|
|Error|エラー|
|Critical|重大障害|

Azure Monitor では Kusto Query Language（KQL）を使用してログを検索できる。

例

```kusto
Event
| where EventLevelName in ("Warning","Error")
```

これにより

- Warning
    
- Error
    

イベントのみ抽出できる。

---

# 7 ダッシュボード

Log Analytics のデータは **Azure Dashboard** に表示できる。

構成

```text
Log Analytics Query
        │
        ▼
Azure Monitor Workbook
        │
        ▼
Dashboard
```

ダッシュボードでは

- エラー件数
    
- VM別ログ
    
- 時系列イベント
    

などを可視化できる。

---

# 8 リアルタイムアラート

Azure Monitor では **ログベースアラート**を作成できる。

例

```kusto
Event
| where EventLevelName == "Error"
```

条件

```text
Error events > 0
```

これにより

```text
Error発生
↓
アラートトリガー
↓
メール / Teams / Webhook
```

が実行される。

---

# 9 アーキテクチャ

全体構成は次の通り。

```text
200 Windows VMs
        │
        ▼
Log Analytics Agent
        │
        ▼
Log Analytics Workspace
        │
        ▼
Azure Monitor
        │
        ├─ Log Query (KQL)
        ├─ Dashboard
        └─ Alert Rules
```

この構成により、すべての VM のログを統合監視できる。

---

# 10 他の選択肢が適切でない理由

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Event Hubs]]
### Azure Event Hubs

Event Hubs はストリーミングデータの取り込みサービスであり、  
ログ分析・ダッシュボード・アラート機能は提供しない。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Storage Account]]
### Azure Storage

Storage はログ保存には使用できるが、

- クエリ
    
- ダッシュボード
    
- アラート
    

がネイティブで提供されない。

---

### Dependency Agent

Dependency Agent は **Service Map** 用のエージェントであり、  
Windows Event Log の集中監視には使用されない。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Monitor Agent]]
# 11 Azure Monitor Agent（補足）

現在の Azure では

**Azure Monitor Agent（AMA）**

が新しいエージェントとして推奨されている。

最新構成

```text
Log Analytics Workspace
+
Azure Monitor Agent
```

ただし試験問題では従来の

```text
Log Analytics Agent
```

が答えになることが多い。

---

# 12 まとめ

今回の要件

- 200台の Windows VM
    
- System Event Log 収集
    
- Warning / Error イベント監視
    
- 単一ダッシュボード
    
- リアルタイムアラート
    

最適な構成

**Log Analytics Workspace  
+  
Log Analytics Agent**

この構成により、Azure Monitor を使用して大規模 VM 環境のログを集中監視し、リアルタイムアラートとダッシュボードを実現できる。