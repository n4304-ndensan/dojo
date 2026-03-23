# Scenario: Azure Policy

## シナリオ一覧

- 管理グループでリージョン制限を適用する
- SQL Database の TDE を自動修復付きで強制する
- AKS ワークロードへセキュリティ基準を強制する
- タグと標準構成を全サブスクリプションへ展開する
- 特定リソースグループだけへデプロイを許可する

## 管理グループでリージョン制限を適用する

シナリオ  
複数サブスクリプションに対して許可リージョンを統一し、違反作成を防止する。

構成  
Management Group  
↓  
Azure Policy  
↓  
Subscriptions

ポイント  
- 上位スコープで一括適用できる
- `Deny` で逸脱を止められる
- 組織再編時も配下継承で維持しやすい

関連リソース  
Management Group / Azure Policy

出典  
- [[Sources/Azure Policy を管理グループで適用する理由（リージョン制限ポリシー）.md]]
- [[Sources/Azure Policy によるリージョン制御とコンプライアンス.md]]

## sql-database-の-tde-を自動修復付きで強制する

シナリオ  
新規・既存の SQL Database に TDE を必ず有効化し、未設定時は自動修復する。

構成  
Azure SQL Database  
↓  
Azure Policy (`DeployIfNotExists`)  
↓  
Remediation Task

ポイント  
- 監査だけでなく修復まで自動化できる
- セキュリティ基準の運用差分を減らせる
- RBAC と Managed Identity を修復側で設計する

関連リソース  
Azure Policy / Azure SQL Database

出典  
- [[Sources/Azure Policy による Azure SQL Database TDE 強制設計.md]]
- [[Sources/Azure SQL Database の透過的データ暗号化（TDE）を自動修復付きで強制するガバナンス設計.md]]

## aks-ワークロードへセキュリティ基準を強制する

シナリオ  
AKS 上のワークロードへ許可レジストリや読み取り専用ファイルシステムなどの制約をかける。

構成  
AKS Cluster  
↓  
Azure Policy for AKS  
↓  
Admission / Audit

ポイント  
- クラスター全体のガードレールになる
- 人手レビューより一貫性が高い
- Kubernetes ネイティブ設定だけでは足りない範囲を補完できる

関連リソース  
Azure Policy / AKS

出典  
- [[Sources/AKS コンテナセキュリティガバナンス設計.md]]

## タグと標準構成を全サブスクリプションへ展開する

シナリオ  
部門ごとの Azure サブスクリプションに共通タグと標準構成を配布する。

構成  
Management Group  
↓  
Policy / Template / Assignment  
↓  
Department Subscriptions

ポイント  
- コスト配賦と棚卸しがしやすい
- 手動構築のばらつきを減らせる
- Policy と IaC を分けると保守しやすい

関連リソース  
Management Group / Azure Policy / ARM

出典  
- [[Sources/Azure Blueprints による標準化デプロイ設計.md]]
- [[Sources/Azure のコスト管理とコスト配分（Cost Management + タグ）.md]]

## 特定リソースグループだけへデプロイを許可する

シナリオ  
開発者へ必要最小限のデプロイ権限だけを委任する。

構成  
Developer Group  
↓  
Custom RBAC Role  
↓  
Target Resource Group

ポイント  
- Policy ではなく RBAC で操作範囲を絞る
- 組み込みロールで広すぎる場合はカスタムロールを使う

関連リソース  
Azure RBAC / Resource Group

出典  
- [[Sources/Azure ARM テンプレートデプロイ制御設計.md]]
- [[Sources/Azure RBAC とスコープ階層によるリソース作成権限の判断.md]]
