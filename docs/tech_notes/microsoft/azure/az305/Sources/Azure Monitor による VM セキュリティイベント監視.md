[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Monitor]]
# Azure Monitor による VM セキュリティイベント監視

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Log Analytics]]
## Windows Event Log と Linux Syslog の Log Analytics テーブル設計

---

# 1 背景（シナリオ）

クラウド環境で仮想マシン（VM）を運用する場合、**セキュリティイベントの監視**は非常に重要である。

企業では通常、以下のようなイベントを監視する。

|イベント種類|例|
|---|---|
|ログイン試行|成功 / 失敗ログイン|
|権限変更|管理者権限変更|
|システム変更|サービス変更|
|不正アクセス|侵入試行|

Azure ではこれらのイベント監視を **Azure Monitor + Log Analytics** を使って行う。

今回のシナリオでは

- Windows VM
    
- Linux VM
    

両方のセキュリティイベントを監視するために

```text
Azure Monitor Alert
```

を設定する必要がある。

その際に

```text
Log Analytics テーブル
```

を正しく選択する必要がある。

---

# 2 Azure Monitor のログ収集アーキテクチャ

Azure Monitor で VM のログを収集する場合の構成は次のようになる。

```
Virtual Machine
      │
      │ Azure Monitor Agent
      │
      ▼
Log Analytics Workspace
      │
      ▼
Log Analytics Tables
      │
      ▼
Azure Monitor Alerts
```

つまり

1. VM がログを生成
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Monitor Agent]]
2. Azure Monitor Agent が収集
    
3. Log Analytics に保存
    
4. KQL クエリで検索
    
5. アラート作成
    

という流れになる。

---

# 3 Windows と Linux のログの違い

Windows と Linux ではログの保存方法が異なる。

|OS|ログ形式|
|---|---|
|Windows|Event Log|
|Linux|Syslog|

---

# 4 Windows Event Log

Windows OS ではシステムログは

```
Event Viewer
```

に保存される。

主なログ

|ログ種類|内容|
|---|---|
|Application|アプリログ|
|System|OSログ|
|Security|セキュリティログ|

セキュリティ監査では

```text
Security Event Log
```

が重要になる。

例

|イベント|Event ID|
|---|---|
|ログイン成功|4624|
|ログイン失敗|4625|
|権限変更|4672|

---

# 5 Log Analytics の Windows テーブル

Azure Monitor が Windows セキュリティイベントを収集すると

```
SecurityEvent
```

テーブルに保存される。

テーブル

|テーブル|用途|
|---|---|
|SecurityEvent|Windows セキュリティログ|
|Event|一般イベントログ|

重要ポイント

```
SecurityEvent
= Windows Security Log
```

である。

---

# 6 Linux Syslog

Linux ではシステムログは

```
Syslog
```

として保存される。

通常の保存場所

```
/var/log/syslog
/var/log/messages
```

ログ例

|イベント|例|
|---|---|
|SSHログイン|sshd|
|サービス起動|systemd|
|認証ログ|auth|

---

# 7 Log Analytics の Linux テーブル

Linux VM の Syslog は

```
Syslog
```

テーブルに保存される。

例

|フィールド|説明|
|---|---|
|Computer|VM名|
|Facility|ログ種類|
|Severity|重要度|
|ProcessName|プロセス|

---

# 8 Log Analytics 主要テーブル

Azure Monitor では多くのテーブルが存在する。

|テーブル|用途|
|---|---|
|SecurityEvent|Windows セキュリティログ|
|Syslog|Linux Syslog|
|AzureActivity|Azure 管理操作|
|AzureDiagnostics|Azure リソース診断ログ|

---

# 9 AzureActivity テーブル

AzureActivity は

```
Azure 管理操作
```

を記録する。

例

|イベント|
|---|
|VM作成|
|VM削除|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Role-Based Access Control (RBAC)]]
|RBAC変更|

つまり

```
OSログではない
```

---

# 10 AzureDiagnostics テーブル

AzureDiagnostics は

```
Azure PaaS 診断ログ
```

を保存する。

例

|サービス|
|---|
|App Service|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Key Vault]]
|Key Vault|
|SQL Database|

VM OS ログではない。

---

# 11 監視対象ログ

今回の問題

```
Windows VM
Linux VM
```

の

```
セキュリティイベント
```

を監視する。

つまり

|OS|テーブル|
|---|---|
|Windows|SecurityEvent|
|Linux|Syslog|

---

# 12 例：KQL クエリ

Windows セキュリティログ

```kusto
SecurityEvent
| where EventID == 4625
```

ログイン失敗を検出できる。

---

Linux SSH ログ

```kusto
Syslog
| where ProcessName == "sshd"
```

SSHログインイベントを確認できる。

---

# 13 他の選択肢が誤りな理由

### A

Windows: Event

Event テーブルは

```
一般イベント
```

であり

```
SecurityEvent
```

が正しい。

---

### B

AzureActivity / AzureDiagnostics

これらは

```
Azure管理ログ
```

であり

```
OSセキュリティログではない
```

---

### D

Linux AzureActivity

Linux OS ログではない。

---

# 14 最終回答

正解

```
C

Windows : SecurityEvent
Linux : Syslog
```

---

# 15 最終アーキテクチャ

```
Windows VM
   │
   ▼
Security Event Log
   │
   ▼
SecurityEvent (Log Analytics)


Linux VM
   │
   ▼
Syslog
   │
   ▼
Syslog Table
```

---

# 16 まとめ

Azure Monitor で VM のセキュリティイベントを監視する場合

|OS|Log Analytics テーブル|
|---|---|
|Windows|SecurityEvent|
|Linux|Syslog|

つまり

```
Windows Security Log → SecurityEvent
Linux Syslog → Syslog
```

である。

この知識は **Azure Monitor / Sentinel / Defender for Cloud の試験問題でも頻出**である。