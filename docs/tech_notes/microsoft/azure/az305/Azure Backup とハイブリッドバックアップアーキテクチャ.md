# Azure Backup とハイブリッドバックアップアーキテクチャ

（オンプレミス Windows Server バックアップ）

## 1 概要

企業がオンプレミスの **Windows Server ファイルサーバー**を運用している場合、ハードウェア障害や災害に備えてバックアップを取得する必要がある。

Azureではこれを実現するために **Azure Backup** を利用することができる。

Azure Backupは次の特徴を持つ。

- オンプレミスサーバーのクラウドバックアップ
    
- Azure VMのバックアップ
    
- SQL / Files / SAP HANAなどの保護
    
- 長期保持
    

オンプレミス環境では次の構成がよく使用される。

```
On-Premises Server
      │
      ▼
Azure Backup Agent
      │
      ▼
Recovery Services Vault
      │
      ▼
Azure Storage
```

この構成により

**オンプレミス → Azureクラウド**

へのバックアップが可能になる。

---

# 2 背景

オンプレミスのファイルサーバーでは次の問題がよく発生する。

- ハードディスク故障
    
- RAID障害
    
- ランサムウェア
    
- 誤削除
    
- 災害（火災・地震）
    

従来のバックアップ方式

```
File Server
   │
   ▼
Local Backup Disk
   │
   ▼
Tape Storage
```

この方式には次の課題がある。

- オフサイト保管が困難
    
- 管理コスト
    
- 復旧時間
    
- テープ管理
    

クラウドバックアップでは

```
Server
   │
   ▼
Cloud Backup
```

となるため

- オフサイト保存
    
- 自動管理
    
- 高耐久性
    

が実現できる。

---

# 3 サービスの仕組み

オンプレミスバックアップでは **Azure Backup + Recovery Services Vault** が利用される。

構造

```
Windows Server
      │
      ▼
Azure Backup Agent
      │
      ▼
Recovery Services Vault
      │
      ▼
Azure Storage
```

バックアップ処理

1. サーバーに **Azure Backup Agent** をインストール
    
2. Recovery Services Vaultへ登録
    
3. バックアップポリシー設定
    
4. Azureへバックアップ転送
    

Windows Admin Centerを使用すると

- バックアップ設定
    
- 管理
    
- モニタリング
    

をGUIから実行できる。

---

# 4 Windows Admin Center との統合

**Windows Admin Center (WAC)** は Windows Server の管理ツールである。

役割

- サーバー管理
    
- ストレージ管理
    
- ネットワーク管理
    
- Azure連携
    

Azureと統合すると次の管理が可能になる。

```
Windows Admin Center
        │
        ▼
Azure Integration
        │
        ▼
Azure Backup
```

メリット

- GUIでバックアップ構成
    
- Azure連携簡単
    
- 管理の集中化
    

---

# 5 主要機能

## Azure Backup

Azure Backupはフルマネージドバックアップサービスである。

特徴

- インフラ管理不要
    
- 暗号化
    
- 長期保存
    

バックアップ対象

|リソース|バックアップ|
|---|---|
|Azure VM|○|
|SQL Server|○|
|Azure Files|○|
|On-Prem Server|○|

---

## Recovery Services Vault

Recovery Services Vaultはバックアップデータの管理リソースである。

役割

- バックアップ保存
    
- 復旧管理
    
- ポリシー管理
    

---

## バックアップポリシー

バックアップのスケジュールを設定する。

例

|設定|例|
|---|---|
|頻度|毎日|
|保持|30日|
|長期保持|1年|

---

## セキュリティ

Azure Backupには次のセキュリティ機能がある。

### 暗号化

データは転送時・保存時に暗号化される。

### Soft Delete

誤削除時に復旧可能。

### RBAC

アクセス制御。

---

# 6 関連Azureサービス

この分野は **Business Continuity / Disaster Recovery** に分類される。

主なサービス

|サービス|役割|
|---|---|
|Azure Backup|バックアップ|
|Recovery Services Vault|バックアップ管理|
|Azure Site Recovery|災害復旧|
|Azure Storage|バックアップ保存|
|Azure Monitor|バックアップ監視|

---

# 7 サービス比較

バックアップ関連サービス

|サービス|用途|
|---|---|
|Azure Backup|バックアップサービス|
|Recovery Services Vault|バックアップ管理|
|Site Recovery|DR|
|Backup Vault|新しいバックアップモデル|

---

# 8 アーキテクチャ

## オンプレミスバックアップ

```
On-Prem File Server
        │
        ▼
Azure Backup Agent
        │
        ▼
Recovery Services Vault
        │
        ▼
Azure Storage
```

---

## Windows Admin Center 管理構成

```
Windows Admin Center
        │
        ▼
Azure Integration
        │
        ▼
Azure Backup
        │
        ▼
Recovery Services Vault
```

---

# 9 ユースケース

## ファイルサーバー保護

```
File Server
   │
   ▼
Azure Backup
   │
   ▼
Recovery Services Vault
```

ハードウェア障害時

```
Azure Backup
   │
   ▼
Restore
   │
   ▼
New Server
```

---

## ランサムウェア対策

クラウドバックアップにより

- 改ざん防止
    
- 復旧可能
    

---

## 長期保存

Azure Backupは長期保存にも対応。

例

|保存|期間|
|---|---|
|日次|30日|
|月次|12ヶ月|
|年次|10年|

---

# 10 設計指針

アーキテクトは次の判断を行う。

---

## 1 Backup方式

オンプレミスでは

- Azure Backup Agent
    
- MARS Agent
    

を利用する。

---

## 2 Recovery Services Vault設計

Vaultは

- リージョン
    
- RBAC
    
- 冗長ストレージ
    

を考慮する。

---

## 3 セキュリティ

推奨設定

- Soft Delete 有効
    
- RBAC
    
- Backup Encryption
    

---

## 4 ネットワーク

バックアップ通信は

HTTPS

を使用する。

---

# 11 まとめ

オンプレミスファイルサーバーのデータ保護には

**Azure Backup + Recovery Services Vault**

が使用される。

Windows Admin CenterをAzureに登録することで

- Azure Backup構成
    
- バックアップ管理
    
- モニタリング
    

を統合管理できる。

この構成により

- ファイルサーバー障害
    
- データ消失
    
- 災害
    

からデータを保護することができる。