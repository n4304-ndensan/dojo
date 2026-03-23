# Topic-05-Storage

## 学習ゴール

[[Azure用語集.md#Azure Storage Account]] を中心に、保存方式、共有方式、保持、バックアップ、DR を切り分けて説明できるようにする。

## このTopicの全体像

この Topic では、Storage Account、Blob、Files、ADLS Gen2、Backup、Recovery Services Vault、Site Recovery、データ転送サービスを扱う。  
判断軸は「何を保存するか」「どう共有するか」「どれだけ保持するか」「どう復旧するか」。

---

# 第1章 学習マップ

## 1.1 学習順序

1. Storage Account と Blob/Files の違いを押さえる。
2. ADLS Gen2 と分析系ストレージの位置付けを見る。
3. Backup、Recovery Services Vault、Site Recovery で保護を整理する。
4. AzCopy / Data Box / File Sync で移送とハイブリッド共有を補完する。

## 1.2 Azureリソース一覧

- Azure Storage Account
- Azure Blob Storage
- Azure Files / Azure File Sync
- Azure Data Lake Storage Gen2
- Recovery Services Vault / Azure Backup / Azure Site Recovery
- Azure NetApp Files / AzCopy / Data Box

---

# 第2章 Azureリソース解説

## Resource: Azure Storage Account and Blob Storage

### 概要

[[Azure用語集.md#Azure Storage Account]] は Azure ストレージの論理境界であり、[[Azure用語集.md#Azure Blob Storage]] はその中のオブジェクト保存機能。

### できること

- Hot / Cool / Archive 階層
- バージョニング
- Soft Delete
- WORM
- ライフサイクル管理
- RBAC / SAS

### 技術仕様

- GPv2 が基準アカウントになる。
- Blob は非構造化ファイル、バックアップ、ログ、メディア保管に向く。
- WORM や Immutable Policy でコンプライアンス保持を実装できる。
- Private Endpoint と Firewall でネットワーク制御を追加できる。

### SDK / API

- Azure Storage SDK
- Blob REST API
- AzCopy / Azure CLI

### 他サービスとの比較

- Blob vs Files: オブジェクト保管か、SMB/NFS 共有か。
- Blob vs ADLS Gen2: 分析基盤の名前空間が必要なら ADLS Gen2。

### どのようなときに使うか

- 画像、動画、ログ、バックアップを大量保存したいとき
- 階層型コスト最適化をしたいとき
- WORM 保持やアーカイブ保管が必要なとき

### 関連シナリオ

- [[Scenarios/Scenario-Storage.md#blob-storage-で-worm-保持を実装する]]
- [[Scenarios/Scenario-Storage.md#storage-account-を-private-endpoint-で閉域化する]]
- [[Scenarios/Scenario-Storage.md#大容量ファイルを-blob-storage-へ移送する]]

### 主な出典

- [[Sources/Topic-06.md]]
- [[Sources/Azure Blob Storage における WORM（Write Once Read Many）データ保持.md]]
- [[Sources/Azure Blob Storage のアクセス階層設計（Hot  Cool  Archive）.md]]
- [[Sources/Azure Storage 設計ドキュメント.md]]

## Resource: Azure Files and Hybrid File Sharing

### 概要

[[Azure用語集.md#Azure Files]] は SMB/NFS ファイル共有をマネージドで提供し、File Sync と組み合わせるとオンプレミス共有にもつながる。

### できること

- SMB / NFS 共有
- AKS の RWX 永続ボリューム
- File Sync によるキャッシュ付きハイブリッド共有
- ZRS / 冗長性設計

### 技術仕様

- AKS では共有ボリュームの標準選択肢になる。
- Azure File Sync はオンプレミスの既存ファイルサーバーを温存しつつクラウド同期できる。
- App Service や VM からも共有マウントできる。

### SDK / API

- Azure Files SDK
- SMB / NFS
- Azure CLI `az storage share`

### 他サービスとの比較

- Azure Files vs NetApp Files: 超高性能 POSIX/NFS が要るなら NetApp Files。
- Azure Files vs Blob: ファイル共有のプロトコル互換が必要なら Azure Files。

### どのようなときに使うか

- 既存 SMB アプリを移行したいとき
- AKS や App Service に共有ファイル領域が必要なとき
- オンプレミスとクラウドで同じファイルを見たいとき

### 関連シナリオ

- [[Scenarios/Scenario-Storage.md#azure-files-rwx-で-aks-の共有ストレージを構成する]]
- [[Scenarios/Scenario-Storage.md#file-sync-でオンプレミス共有をクラウド拡張する]]

### 主な出典

- [[Sources/Topic-04.md]]
- [[Sources/Topic-06.md]]
- [[Sources/AKS ステートフルアプリケーションの共有ストレージ設計.md]]
- [[Sources/Azure File Sync を利用したハイブリッドファイル共有.md]]

## Resource: Azure Data Lake Storage Gen2

### 概要

[[Azure用語集.md#Azure Data Lake Storage Gen2]] は分析ワークロード向けの階層名前空間付きストレージ。

### できること

- HDFS 互換データレイク
- Databricks / Synapse / Data Factory の共通ストレージ
- 大容量ファイルの高効率転送

### 技術仕様

- フォルダ単位の ACL と分析系エンジン連携が強み。
- Parquet / Delta Lake の保存先として使いやすい。
- NAS からの移送は AzCopy や Data Factory を組み合わせる。

### SDK / API

- ADLS Gen2 REST API
- Azure Storage SDK
- AzCopy

### 他サービスとの比較

- ADLS Gen2 vs Blob: 分析向け名前空間と ACL が要るなら ADLS Gen2。
- ADLS Gen2 vs Files: 分析基盤のバックエンドであって SMB 共有ではない。

### どのようなときに使うか

- データレイク基盤を作るとき
- オンプレミス HDFS ワークロードを Azure に寄せるとき

### 関連シナリオ

- [[Scenarios/Scenario-Storage.md#nas-から-adls-gen2-へ-azcopy-で移行する]]
- [[Scenarios/Scenario-DataPlatform.md#data-factory-でオンプレミスデータをデータレイクへ取り込む]]

### 主な出典

- [[Sources/Topic-06.md]]
- [[Sources/Azure Data Lake Storage Gen2 による HDFS ワークロードのクラウド移行.md]]
- [[Sources/オンプレミス NAS から Azure Data Lake Storage Gen2 へのデータ移行設計.md]]
- [[Sources/マルチテナントアプリケーション向け Azure Data Lake Storage Gen2 設計.md]]

## Resource: Backup and Disaster Recovery

### 概要

[[Azure用語集.md#Recovery Services Vault]] を中心に、Azure Backup と Site Recovery でバックアップと DR を分けて設計する。

### できること

- VM バックアップ
- オンプレミス バックアップ
- ポリシー管理
- Site Recovery によるレプリケーションとフェイルオーバー

### 技術仕様

- バックアップは保護点と保持期間を設計する。
- DR は RTO / RPO とフェイルオーバー手順で判断する。
- Immutable Backup や Soft Delete で削除耐性を強められる。

### SDK / API

- Recovery Services API
- Azure CLI / PowerShell

### 他サービスとの比較

- Backup vs Site Recovery: 前者は復元点、後者はサービス継続と切替。
- GRS/GZRS vs Backup: 冗長化だけでは運用復旧点は代替できない。

### どのようなときに使うか

- VM やファイルサーバーの復旧点を持ちたいとき
- リージョン障害への継続運用計画が必要なとき

### 関連シナリオ

- [[Scenarios/Scenario-Storage.md#vm-を-recovery-services-vault-で保護する]]
- [[Scenarios/Scenario-Storage.md#site-recovery-で災害復旧を設計する]]

### 主な出典

- [[Sources/Topic-06.md]]
- [[Sources/Azure Recovery Services Vault 技術ドキュメント.md]]
- [[Sources/Azure Backup と Recovery Services Vault によるバックアップおよびディザスタリカバリー戦略.md]]
- [[Sources/Azure Site Recovery による災害復旧（DR）設計.md]]

## Resource: Data Transfer and Specialized File Services

### 概要

AzCopy、Data Box、NetApp Files は「どう運ぶか」「どこまで性能が要るか」を解く補助リソース。

### できること

- オンプレミスからの大容量転送
- 物理デバイス移送
- 高性能 NFS 共有

### 技術仕様

- オンライン転送は AzCopy。
- 回線が足りないときは Data Box。
- Linux 共有性能が厳しいときは NetApp Files。

### SDK / API

- AzCopy
- Data Box 管理 API

### 他サービスとの比較

- AzCopy vs Data Factory: 単純転送なら AzCopy、パイプライン制御なら Data Factory。
- Azure Files vs NetApp Files: 高性能ファイル共有要件で分ける。

### どのようなときに使うか

- TB 級データ移行
- 高 IOPS / 低レイテンシの共有ストレージ

### 関連シナリオ

- [[Scenarios/Scenario-Storage.md#大容量ファイルを-blob-storage-へ移送する]]

### 主な出典

- [[Sources/Topic-06.md]]
- [[Sources/Azure 大容量データ移行サービスまとめ.md]]
- [[Sources/Azure 大容量データ転送設計.md]]
- [[Sources/Linux VM 向けの高性能共有ファイルシステム（Azure NetApp Files）.md]]

---

# 第3章 設計判断ガイド

## 3.1 保存先を選ぶとき

- オブジェクトなら Blob。
- SMB/NFS 共有なら Files。
- 分析基盤の共通ストレージなら ADLS Gen2。

## 3.2 保持方式を選ぶとき

- コスト優先なら階層化。
- 規制保持なら WORM / Immutable。
- 事故復旧なら Backup。
- サイト障害継続なら Site Recovery。

## 3.3 移送方法を選ぶとき

- 回線で十分なら AzCopy / Data Factory。
- 回線が足りないなら Data Box。

---

# 第4章 関連シナリオ

- [[Scenarios/Scenario-Storage.md]]

