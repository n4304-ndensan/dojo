# Azure 高可用性とディザスタリカバリー設計ドキュメント

## 1. 概要

クラウドシステムでは、障害や災害が発生してもサービスを継続するために  
**高可用性（High Availability: HA）** と **ディザスタリカバリー（Disaster Recovery: DR）** の設計が必要です。

Azureではこれを実現するために複数のサービスが提供されています。

主な目的

- サービス停止の最小化
    
- データ損失の最小化
    
- 自動復旧
    
- 災害対策
    

重要な指標として以下が使用されます。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#RTO]]
**RTO (Recovery Time Objective)**  
障害発生からサービス復旧までの許容時間

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#RPO]]
**RPO (Recovery Point Objective)**  
許容できるデータ損失時間

例

|項目|意味|
|---|---|
|RTO 30分|30分以内にサービス復旧|
|RPO 5分|最大5分のデータ損失|

---

# 2. Azureの可用性アーキテクチャ

Azureはインフラレベルで可用性を提供します。

## 2.1 Azure Region

Azureは世界中に **リージョン（地域）** を持っています。

例

- East US
    
- West Europe
    
- Japan East
    
- Japan West
    

リージョンは地理的に独立したデータセンター群です。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Availability Zone]]
## 2.2 Availability Zone

Availability Zone は  
**同一リージョン内の独立したデータセンター群**です。

特徴

- 電源独立
    
- ネットワーク独立
    
- 冷却独立
    

構造

```
Region
 ├ Zone 1
 ├ Zone 2
 └ Zone 3
```

用途

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
- VM冗長化
    
- DB冗長化
    
- 高可用性
    

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Availability Set]]
## 2.3 Availability Set

Availability Set は VM の高可用性を提供します。

構造

```
Availability Set
 ├ Fault Domain
 └ Update Domain
```

Fault Domain  
→ ハードウェア障害対策

Update Domain  
→ メンテナンス時の停止分散

用途

- VM冗長化
    
- 同一リージョン内 HA
    

---

# 3. Azure ディザスタリカバリーサービス

Azureには主に以下のDRサービスがあります。

|サービス|用途|
|---|---|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Backup]]
|Azure Backup|バックアップ復旧|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Site Recovery]]
|Azure Site Recovery|VMレプリケーション|
|Geo Replication|データベースDR|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Storage Account]]
|Azure Storage GRS|ストレージDR|

---

# 4. Azure Backup

Azure Backup は  
**バックアップベースの復旧サービス**です。

特徴

- VMバックアップ
    
- SQLバックアップ
    
- 長期保存
    
- ポイントインタイム復元
    

構造

```
VM
 ↓
Backup Vault
 ↓
Recovery
```

用途

- データ保護
    
- 長期保存
    
- 誤削除対策
    

注意点

RPO  
→ バックアップ間隔依存

RTO  
→ 復元時間必要

そのため

**DR用途には向かない**

---

# 5. Azure Site Recovery

Azure Site Recovery (ASR) は  
**仮想マシンのレプリケーションDRサービス**です。

特徴

- VMレプリケーション
    
- 自動フェイルオーバー
    
- 多層アプリ復旧
    
- DRテスト可能
    

構造

```
Primary Region
 VM

 ↓ Replication

Secondary Region
 VM Replica

 ↓

Failover
```

ASRが適するケース

- VMワークロード
    
- SQL Server on VM
    
- 多層アプリ
    
- DR対策
    

メリット

- RPO 数分
    
- RTO 数十分
    
- 自動復旧
    

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
# 6. Azure SQL Database Geo Replication

Azure SQL Database は  
**PaaSデータベースDR** を提供します。

Geo Replication は  
別リージョンへデータを複製します。

構造

```
Primary DB
 ↓ Replication
Secondary DB
```

特徴

- 非同期レプリケーション
    
- 手動フェイルオーバー
    
- 読み取り専用セカンダリ
    

用途

- SaaS
    
- グローバルサービス
    

---

# 7. Azure Storage レプリケーション

Azure Storage は複数のレプリケーション方式があります。

|方式|内容|
|---|---|
|LRS|ローカル冗長|
|ZRS|ゾーン冗長|
|GRS|地域冗長|
|RA-GRS|読み取り可能GRS|

構造

```
Primary Region
   ↓
Secondary Region
```

用途

- Blob
    
- Data Lake
    
- ファイル保存
    

---

# 8. Azure Traffic Manager

Traffic Manager は  
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure DNS]]
**DNSベースの負荷分散サービス**です。

用途

- マルチリージョン
    
- フェイルオーバー
    
- 地理ルーティング
    

構造

```
User
 ↓
Traffic Manager
 ↓
Region A
Region B
```

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Front Door]]
# 9. Azure Front Door

Azure Front Door は  
**グローバルレイヤー7ロードバランサー**です。

特徴

- HTTP/HTTPS
    
- グローバル負荷分散
    
- CDN
    
- WAF
    

構造

```
User
 ↓
Azure Front Door
 ↓
App Service
VM
AKS
```

---

# 10. DR設計パターン

## 10.1 Backup型

```
Production
 ↓
Backup
 ↓
Restore
```

特徴

- 安価
    
- 復旧遅い
    

用途

- データ保護
    

---

## 10.2 Active-Passive

```
Primary
 ↓ Replication
Secondary
```

特徴

- DR対策
    
- 手動切替
    

---

## 10.3 Active-Active

```
Region A
 ↕
Region B
```

特徴

- 高可用性
    
- グローバルサービス
    

---

# 11. Azure HA/DRサービス比較

|サービス|用途|
|---|---|
|Availability Zone|リージョン内HA|
|Availability Set|VM HA|
|Azure Backup|バックアップ|
|Azure Site Recovery|VM DR|
|SQL Geo Replication|DB DR|
|Traffic Manager|DNS負荷分散|
|Front Door|グローバルL7LB|

---

# 12. 試験対策判断フロー

Azure試験では以下の判断が重要です。

VM障害対策  
→ Availability Set / Zone

VM DR  
→ Azure Site Recovery

DB DR (PaaS)  
→ Geo Replication

データ保護  
→ Azure Backup

グローバル負荷分散  
→ Traffic Manager / Front Door

---

# まとめ

Azureの可用性設計は  
**インフラ冗長 + データ冗長 + DR設計** の組み合わせで実現します。

基本原則

1. Availability Zoneでリージョン内冗長
    
2. Site RecoveryでリージョンDR
    
3. Backupでデータ保護
    
4. Traffic Manager / Front Doorでグローバル負荷分散
    

これらを組み合わせることで、  
**高可用性かつ災害耐性の高いクラウドシステムを構築できます。**

---

必要なら次に  
**Azure試験で最も重要なアーキテクチャ判断表（20パターン）**  
も作れます。  
これを覚えると **Azure問題の7〜8割が瞬時に解けるようになります。**