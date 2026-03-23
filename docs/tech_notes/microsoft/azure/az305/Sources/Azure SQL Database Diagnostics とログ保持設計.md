[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
# Azure SQL Database Diagnostics とログ保持設計

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Log Analytics]]
（Storage Account + Log Analytics Workspace）

---

# 1 背景

Azure SQL Database を運用する場合、データベースの状態やパフォーマンス、セキュリティイベントを監視するために **Diagnostics（診断ログ）**を有効化することが一般的である。

Diagnostics を有効にすると、SQL Database からさまざまなログやメトリクスを外部の監視サービスに送信できる。

代表的な送信先は次の通りである。

|送信先|用途|
|---|---|
|Storage Account|長期ログ保存|
|Log Analytics Workspace|クエリ分析・監視|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Event Hubs]]
|Event Hub|リアルタイム処理|

今回のシナリオでは、Azure SQL Database の診断ログを次の2つのサービスに送信している。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Storage Account]]
- **Azure Storage Account**
    
- **Azure Log Analytics Workspace**
    

さらにログデータとして **SQLInsights** を送信している。

---

# 2 SQLInsights とは

SQLInsights は Azure SQL Database の **詳細なパフォーマンス診断情報**を提供するログカテゴリである。

主な情報

- クエリパフォーマンス
    
- データベース負荷
    
- リソース使用量
    
- 実行統計
    

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Monitor]]
これらのデータは Azure Monitor を通じて収集され、以下の分析に利用される。

- パフォーマンス監視
    
- トラブルシューティング
    
- 容量計画
    

---

# 3 Diagnostics の送信先

Azure SQL Database の診断設定では、ログの送信先として次のサービスを設定できる。

### Storage Account

主な用途

- 長期ログ保存
    
- 監査ログ保管
    
- コンプライアンス対応
    

特徴

- 安価
    
- 長期保存可能
    
- クエリ分析には向かない
    

---

### Log Analytics Workspace

主な用途

- ログ分析
    
- クエリ実行
    
- Azure Monitor
    
- Sentinel 連携
    

特徴

- Kusto Query Language (KQL) で検索
    
- ダッシュボード作成
    
- アラート作成
    

---

# 4 Storage Account のログ保持

Azure Storage に送信された診断ログは **Retention Policy（保持ポリシー）**を設定できる。

今回のシナリオでは

```text
Retention = 90 days
```

と設定されている。

つまり

```text
Storage Account
↓
90日間保存
↓
自動削除
```

となる。

これは **Storage 側で設定した保存期間**に従う。

---

# 5 Log Analytics のログ保持

Log Analytics Workspace は独自のログ保持ポリシーを持つ。

デフォルト保持期間

```text
30 days
```

ただし Azure Monitor では最大保持期間を拡張できる。

最大保持期間

```text
730 days (約2年)
```

この期間までログを保持できる。

---

# 6 Storage と Log Analytics の違い

ログ保存方法には明確な違いがある。

|項目|Storage Account|Log Analytics|
|---|---|---|
|保存用途|長期保存|ログ分析|
|コスト|低コスト|比較的高い|
|検索機能|なし|KQL クエリ|
|最大保持|設定依存|730日|

---

# 7 データフロー

今回のログ収集フローは次のようになる。

```text
Azure SQL Database
        │
        ▼
Diagnostics Settings
        │
        ├── Storage Account
        │        │
        │        ▼
        │     90 days retention
        │
        ▼
Log Analytics Workspace
        │
        ▼
最大730日保存
```

---

# 8 他の選択肢が不適切な理由

### B

Storage: 30 days

問題

Storage の保持期間は **設定値に依存**する。  
今回の設定は **90 日**である。

---

### C

Log Analytics: 無期限

Log Analytics には **無期限保存は存在しない**。

最大保存期間

```text
730 days
```

である。

---

### D

Storage: 730 days

Storage は自動的に 730 日保存するわけではない。  
保存期間は **ユーザー設定値**に依存する。

---

# 9 推奨構成

Azure Monitor を利用したログ収集の標準構成は次の通り。

```text
Azure SQL Database
        │
        ▼
Diagnostics
        │
        ├── Storage Account (長期保存)
        │
        ▼
Log Analytics (分析)
```

この構成により

- 長期ログ保存
    
- ログ分析
    
- アラート作成
    

が可能になる。

---

# 10 最終回答

正解

**A**

```text
Storage retention : 90 days
Log Analytics max retention : 730 days
```

---

# 11 まとめ

今回のポイント

|サービス|最大保持|
|---|---|
|Storage Account|設定値 (今回は90日)|
|Log Analytics Workspace|最大730日|

そのためログ保存期間は

```text
Storage : 90 days
Log Analytics : 730 days
```

となる。

この構成は **Azure Monitor を利用した SQL Database の標準的なログ管理アーキテクチャ**である。