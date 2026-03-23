[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Recovery Services Vault]]
# Azure Recovery Services Vault 技術ドキュメント

## 1 概要

**Recovery Services Vault（リカバリーサービスボールト）** は、Azureにおけるバックアップおよび災害復旧を管理するための中核サービスである。

主な用途は次の2つである。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Backup]]
- Azure Backup（バックアップ管理）
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Site Recovery]]
- Azure Site Recovery（ディザスタリカバリ）
    

Recovery Services Vaultは、バックアップデータの保存場所であり、同時にバックアップや復旧操作の管理ポイントとして機能する。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Files]]
Azure VM、SQL Server、Azure Files、オンプレミスサーバーなど様々なリソースを保護することができる。

---

# 2 背景

クラウド環境においても、以下のリスクは常に存在する。

- アプリケーション障害
    
- 操作ミスによるデータ削除
    
- ランサムウェア
    
- リージョン障害
    
- インフラ障害
    

従来のオンプレミスでは、以下のような構成でバックアップを管理していた。

```
Server
 ↓
Backup Software
 ↓
Tape / Disk
 ↓
Offsite Storage
```

この方式には次の問題があった。

- バックアップインフラの管理コスト
    
- オフサイト保管の手間
    
- 災害時の復旧時間
    
- スケーラビリティ不足
    

Azureではこれらを解決するために

**Recovery Services Vault + Azure Backup**

というクラウドネイティブなバックアップアーキテクチャが提供されている。

---

# 3 サービスの仕組み

Recovery Services Vaultは、バックアップと復旧操作を統合管理するリソースである。

構造は次のようになっている。

```
Azure VM
   │
   ▼
Backup Extension
   │
   ▼
Recovery Services Vault
   │
   ▼
Azure Storage (Managed Backup Storage)
```

バックアップの流れ

1. VMにBackup Extensionがインストールされる
    
2. バックアップポリシーに従ってスナップショットが作成される
    
3. データがRecovery Services Vaultに保存される
    
4. 復旧時はVaultからデータを取得する
    

Vaultは

- バックアップ管理
    
- 保持期間管理
    
- セキュリティ制御
    
- 復旧操作
    

を一元管理する。

---

# 4 主要機能

## Azure VM Backup

Azure仮想マシンのバックアップを自動で実行する。

特徴

- アプリケーション整合バックアップ
    
- スナップショットベース
    
- フルマネージド
    

---

## バックアップポリシー

バックアップのスケジュールと保持期間を管理する。

例

|設定|例|
|---|---|
|バックアップ頻度|毎日|
|保持期間|30日|
|長期保持|1年|

---

## セキュリティ機能

Recovery Services Vaultには以下のセキュリティ機能がある。

### Soft Delete

誤削除されたバックアップを14日間保持する。

### Multi-User Authorization

重要操作には追加承認を要求する。

### Immutable Backup

バックアップの改ざん防止。

---

## 可用性

Recovery Services Vaultは高可用なストレージを使用する。

選択可能な冗長構成

|冗長構成|説明|
|---|---|
|LRS|同一データセンター内複製|
|ZRS|複数ゾーン複製|
|GRS|リージョン間複製|

---

# 5 関連Azureサービス

Recovery Services Vaultは複数のAzureサービスと連携する。

|サービス|役割|
|---|---|
|Azure Backup|バックアップ実行|
|Azure Site Recovery|災害復旧|
|Azure VM|バックアップ対象|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Storage Account]]
|Azure Storage|バックアップ保存|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Monitor]]
|Azure Monitor|バックアップ監視|

---

# 6 アーキテクチャ

典型的なバックアップ構成

```
Azure Virtual Machines
       │
       ▼
Azure Backup Service
       │
       ▼
Recovery Services Vault
       │
       ▼
Geo-Redundant Storage
```

複数VMを1つのVaultで管理することも可能。

```
VM1 ┐
VM2 ├─ Backup → Recovery Services Vault
VM3 ┤
VM4 ┘
```

---

# 7 サービス比較

Azureにはバックアップ関連のサービスが複数存在する。

|サービス|用途|
|---|---|
|Recovery Services Vault|Backup / Disaster Recovery|
|Backup Vault|新しいバックアップアーキテクチャ|
|Azure Backup|バックアップサービス|
|Azure Site Recovery|DR|

Recovery Services Vaultは

**Azure BackupとSite Recoveryの管理ポイント**

として使われる。

---

# 8 ユースケース

## 仮想マシンバックアップ

```
Azure VM
 ↓
Recovery Services Vault
 ↓
Geo-Redundant Storage
```

目的

- VM復旧
    
- OSディスク復旧
    
- フルVMリストア
    

---

## 災害復旧

```
Primary Region
   │
   ▼
Azure Site Recovery
   │
   ▼
Secondary Region
```

Recovery Services Vaultがレプリケーション設定を管理する。

---

## ハイブリッドバックアップ

```
On-Prem Server
      │
      ▼
Backup Agent
      │
      ▼
Recovery Services Vault
```

オンプレミス環境もAzureにバックアップできる。

---

# 9 設計指針

アーキテクトは次の点を判断する必要がある。

## 1 リージョン

Recovery Services Vaultは

**バックアップ対象と同じリージョン**

に作成する必要がある。

---

## 2 Vault設計

Vault設計には2つの考え方がある。

### 集中型

```
Single Vault
   │
   ├ VM1
   ├ VM2
   ├ VM3
```

メリット

- 管理が簡単
    

デメリット

- 障害影響範囲が広い
    

---

### 分散型

```
Vault1 → VM1
Vault2 → VM2
Vault3 → VM3
```

メリット

- 分離設計
    

デメリット

- 管理コスト増
    

---

## 3 冗長性

バックアップデータの耐障害性を決定する。

一般的な選択

- **GRS（Geo-Redundant Storage）**
    

---

## 4 セキュリティ

推奨設定

- Soft Delete 有効化
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Multi-Factor Authentication (MFA)]]
- MFA Delete
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Role-Based Access Control (RBAC)]]
- RBAC制御
    

---

# 10 まとめ

Recovery Services VaultはAzureのバックアップと災害復旧の中心的サービスである。

主なポイント

- Azure BackupとSite Recoveryの管理基盤
    
- バックアップデータ保存場所
    
- VM / SQL / Files / On-Premを保護可能
    
- バックアップポリシーと復旧操作を管理
    

設計では特に次の点が重要になる。

- リージョン配置
    
- 冗長ストレージ
    
- セキュリティ設定
    
- Vault分割戦略
    

Azureのビジネス継続性（BCP）設計では、Recovery Services Vaultは基盤となるコンポーネントである。