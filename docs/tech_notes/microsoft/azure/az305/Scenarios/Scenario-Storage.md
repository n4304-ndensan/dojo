# Scenario: Storage

## シナリオ一覧

- Blob Storage で WORM 保持を実装する
- Storage Account を Private Endpoint で閉域化する
- Azure Files RWX で AKS の共有ストレージを構成する
- File Sync でオンプレミス共有をクラウド拡張する
- VM を Recovery Services Vault で保護する
- Site Recovery で災害復旧を設計する
- NAS から ADLS Gen2 へ AzCopy で移行する
- 大容量ファイルを Blob Storage へ移送する

## blob-storage-で-worm-保持を実装する

シナリオ  
監査ログや金融記録を一定期間変更不可で保持する。

構成  
Blob Container  
↓  
Immutable Policy  
↓  
Archive / Long-term Retention

ポイント  
- 法規制対応の基本パターン
- ライフサイクル管理と合わせてコストを調整できる

関連リソース  
Azure Blob Storage / Immutable Storage

出典  
- [[Sources/Azure Blob Storage における WORM（Write Once Read Many）データ保持.md]]
- [[Sources/Azure Blob Storage における機密データの長期保持設計.md]]

## storage-account-を-private-endpoint-で閉域化する

シナリオ  
Storage Account を社内ネットワークや特定 VNet からのみ利用する。

構成  
VNet  
↓  
Private Endpoint  
↓  
Storage Account

ポイント  
- Firewall と合わせると公開面を縮めやすい

関連リソース  
Azure Private Endpoint / Azure Storage Account

出典  
- [[Sources/Azure Storage アカウントのネットワーク制御（Private Endpoint と Storage Firewall）.md]]

## azure-files-rwx-で-aks-の共有ストレージを構成する

シナリオ  
AKS ワークロードに共有ファイル ボリュームを渡す。

構成  
AKS Pods  
↓  
PVC  
↓  
Azure Files

ポイント  
- RWX が必要なときの標準構成

関連リソース  
Azure Files / AKS

出典  
- [[Sources/AKS ステートフルアプリケーションの永続ストレージ（Azure Files RWX）.md]]

## file-sync-でオンプレミス共有をクラウド拡張する

シナリオ  
既存のオンプレミス ファイルサーバーを残したまま Azure 側へ同期する。

構成  
On-prem File Server  
↔  
Azure File Sync  
↔  
Azure Files

ポイント  
- 既存 SMB 運用を変えずにクラウド拡張しやすい

関連リソース  
Azure File Sync / Azure Files

出典  
- [[Sources/Azure File Sync を利用したハイブリッドファイル共有.md]]
- [[Sources/Azure Files を利用した SMB ベースアプリケーションの移行設計.md]]

## vm-を-recovery-services-vault-で保護する

シナリオ  
Azure VM のバックアップ ポリシーと復元点を一元管理する。

構成  
Azure VM  
↓  
Azure Backup  
↓  
Recovery Services Vault

ポイント  
- Soft Delete と Immutable Backup を合わせると削除耐性が上がる

関連リソース  
Azure Backup / Recovery Services Vault

出典  
- [[Sources/Azure Recovery Services Vault 技術ドキュメント.md]]
- [[Sources/Azure Backup による VM バックアップポリシー設計.md]]

## site-recovery-で災害復旧を設計する

シナリオ  
リージョン障害やデータセンター障害に備えて VM をフェイルオーバーさせる。

構成  
Primary VM  
↓  
Azure Site Recovery  
↓  
Secondary Region

ポイント  
- Backup と DR は役割が違う
- RTO / RPO から必要なレプリケーション方式を決める

関連リソース  
Azure Site Recovery / Azure Virtual Machines

出典  
- [[Sources/Azure Site Recovery による災害復旧（DR）設計.md]]
- [[Sources/RTO・RPO 要件に基づく Azure 災害復旧（DR）ソリューションの選択.md]]

## nas-から-adls-gen2-へ-azcopy-で移行する

シナリオ  
NAS 上の大量ファイルを分析基盤向けに ADLS Gen2 へ移行する。

構成  
NAS  
↓  
AzCopy  
↓  
ADLS Gen2

ポイント  
- 単純高速転送は AzCopy が有利
- 反復運用や変換が必要なら Data Factory も比較する

関連リソース  
Azure Data Lake Storage Gen2 / AzCopy

出典  
- [[Sources/オンプレミス NAS から Azure Data Lake Storage Gen2 へのデータ移行設計.md]]

## 大容量ファイルを-blob-storage-へ移送する

シナリオ  
オンプレミスの TB 級データを Blob Storage へ移す。

構成  
On-prem Data  
↓  
AzCopy / Data Box  
↓  
Azure Blob Storage

ポイント  
- 回線条件でオンライン転送か物理転送かを分ける

関連リソース  
Azure Blob Storage / AzCopy / Data Box

出典  
- [[Sources/Azure 大容量データ転送設計.md]]
- [[Sources/Azure 大容量データ移行サービスまとめ.md]]
