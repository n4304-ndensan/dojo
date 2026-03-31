# Topic-01-Identity

## 学習ゴール

[[Azure用語集.md#Microsoft Entra ID]] を中心に、人・アプリ・ワークロードの認証方式を整理し、シークレットを持たない設計へつなげる。

## このTopicの全体像

この Topic では、Microsoft Entra ID、Managed Identity、Azure Key Vault、外部 ID、ハイブリッド ID を 1 本の学習導線で扱う。  
試験対策では「誰が認証するか」「どこで権限を絞るか」「秘密情報を残すか」の 3 点で判断する。

---

# 第1章 学習マップ

## 1.1 学習順序

1. まず `Microsoft Entra ID` でユーザー認証、SSO、アプリ登録、外部ユーザー管理を押さえる。
2. 次に `Managed Identity` でワークロード認証へ進み、アプリに資格情報を持たせない設計を理解する。
3. そのうえで `Azure Key Vault` にシークレットと証明書を集約し、RBAC とローテーションを結び付ける。
4. 最後にハイブリッド ID と Identity Governance を見て、オンプレミス AD や外部コラボレーションまで拡張する。

## 1.2 Azureリソース一覧

- Microsoft Entra ID
- Managed Identity / Workload Identity
- Azure Key Vault
- Microsoft Entra External ID / B2B / B2C
- Microsoft Entra ID Connect / Hybrid Identity
- Access Reviews / Conditional Access / PIM

---

# 第2章 Azureリソース解説

## Resource: Microsoft Entra ID

### 概要

[[Azure用語集.md#Microsoft Entra ID]] は Azure 全体の認証基盤であり、ユーザー、グループ、アプリ登録、外部ユーザー、トークン発行の中心になる。

### できること

- SSO
- OAuth 2.0 / OpenID Connect / SAML による認証
- 外部ユーザーの B2B 招待
- アプリ登録と API スコープ定義
- Conditional Access と MFA の適用

### 技術仕様

- ユーザー認証とアプリ認証を同一基盤で扱う。
- Delegated Permissions と Application Permissions を分けて考える。
- 外部コラボレーションは B2B、消費者向け認証は B2C 系で整理する。
- Access Reviews、Entitlement Management、PIM で権限のライフサイクルを閉じる。

### SDK / API

- Microsoft Graph API
- Microsoft Graph SDK
- Azure CLI / PowerShell によるアプリ登録・ロール割り当て

### 他サービスとの比較

- Entra ID vs App Service Easy Auth: Easy Auth は入口、認証基盤は Entra ID。
- Entra ID vs カスタム認証: SaaS や社内アプリでは標準トークン基盤を優先する。

### どのようなときに使うか

- 社内アプリの SSO を統一したいとき
- API のトークン発行元を一元化したいとき
- 外部パートナーをゲストとして招待したいとき
- 条件付きアクセスや MFA を入口で適用したいとき

### 関連シナリオ

- [[Scenarios/Scenario-EntraID.md#社内webアプリのsso]]
- [[Scenarios/Scenario-EntraID.md#外部ユーザーのb2b招待とアクセスレビュー]]
- [[Scenarios/Scenario-EntraID.md#委任アクセスでgraph-apiを呼び出す]]

### 主な出典

- [[Sources/Topic-01.md]]
- [[Sources/Azure Entra ID におけるシングルサインオン方式.md]]
- [[Sources/Azure Entra ID 外部ユーザーアクセス.md]]
- [[Sources/Azure AD を利用した ASP.NET Core アプリのユーザー委任アクセス設計.md]]

## Resource: Managed Identity

### 概要

[[Azure用語集.md#Managed Identity]] は Azure リソースに割り当てる ID であり、パスワードや接続文字列を埋め込まずに Azure リソースへ認証するための基本手段。

### できること

- App Service / Functions / VM から Key Vault や SQL Database に認証
- AKS で Workload Identity を使ったポッド単位認証
- ユーザー割り当て ID による複数リソースでの ID 共用

### 技術仕様

- システム割り当て ID はリソースとライフサイクルが一致する。
- ユーザー割り当て ID は複数リソースで再利用できる。
- AKS は Workload Identity と Federated Credential を使うと Kubernetes Service Account と Entra ID をつなげられる。
- 権限付与は RBAC 側で最小権限に絞る。

### SDK / API

- Azure Identity SDK
- `DefaultAzureCredential`
- Azure CLI の `az login --identity`

### 他サービスとの比較

- Managed Identity vs Service Principal: シークレット管理を減らすなら Managed Identity。
- Managed Identity vs 接続文字列埋め込み: 接続文字列に秘密を残さない設計を優先する。

### どのようなときに使うか

- Key Vault、Storage、SQL にパスワードレス接続したいとき
- AKS ワークロードにポッド単位権限を与えたいとき
- シークレットローテーション運用を減らしたいとき

### 関連シナリオ

- [[Scenarios/Scenario-ManagedIdentity.md#app-service-から-key-vault-へ接続する]]
- [[Scenarios/Scenario-ManagedIdentity.md#functions-から-storage-と-sql-へ接続する]]
- [[Scenarios/Scenario-ManagedIdentity.md#aks-workload-identity-で-azure-sql-へ接続する]]

### 主な出典

- [[Sources/Topic-01.md]]
- [[Sources/AKS における Azure リソース認証アーキテクチャ.md]]
- [[Sources/AKS から Azure SQL Database に安全に接続する認証方式.md]]
- [[Sources/Azure ワークロード用 ID.md]]

## Resource: Azure Key Vault

### 概要

[[Azure用語集.md#Azure Key Vault]] はシークレット、証明書、暗号鍵を一元管理するセキュリティ リソース。

### できること

- シークレット保存
- 証明書管理
- キー管理
- バージョン管理
- ローテーション
- RBAC またはアクセスポリシーによるアクセス制御

### 技術仕様

- 保存データは暗号化される。
- Entra ID と統合してユーザーやアプリを認証する。
- Managed Identity と組み合わせるとシークレットレス運用がしやすい。
- Private Endpoint を使えばネットワーク面も閉域化できる。

### SDK / API

- Azure SDK for Key Vault
- REST API
- Azure CLI `az keyvault`

### 他サービスとの比較

- Key Vault vs App Configuration: 機密情報は Key Vault、一般設定は App Configuration。
- Key Vault vs Storage/SAS 保管: 専用の秘密情報管理機能が必要なら Key Vault。

### どのようなときに使うか

- API キー、接続文字列、証明書を中央集約したいとき
- アプリ構成から秘密情報を分離したいとき
- ローテーションと監査証跡を持たせたいとき

### 関連シナリオ

- [[Scenarios/Scenario-KeyVault.md#共通構成シークレットを集約する]]
- [[Scenarios/Scenario-KeyVault.md#app-service-から-key-vault-の秘密を参照する]]
- [[Scenarios/Scenario-KeyVault.md#vm-から-key-vault-の証明書を参照する]]

### 主な出典

- [[Sources/Topic-01.md]]
- [[Sources/Azure Key Vault による機密構成データのセキュア管理.md]]
- [[Sources/Azure Key Vault と管理された ID によるセキュアなシークレット管理.md]]
- [[Sources/Azure VM 上の ASP.NET Core アプリから Key Vault を安全に利用する設計.md]]

## Resource: Hybrid Identity and Identity Governance

### 概要

オンプレミス AD と Azure をつなぐ同期、外部ユーザー管理、アクセスレビュー、PIM をまとめて扱う運用レイヤ。

### できること

- AD Connect による同期
- Password Hash Sync / Pass-through Authentication / Seamless SSO
- 外部ユーザーのレビュー
- JIT 管理者権限

### 技術仕様

- 同期範囲は Domain / OU / Group ベースで調整する。
- 外部ユーザーは B2B 招待後に Access Reviews で定期棚卸しする。
- 特権アクセスは PIM で昇格時間を制御する。

### SDK / API

- Microsoft Graph
- Entra 管理ポータル
- Azure CLI / PowerShell

### 他サービスとの比較

- PHS vs PTA: クラウド依存度とオンプレミス可用性要件で選ぶ。
- B2B vs B2C: パートナー招待か、一般顧客向け ID 基盤かで分ける。

### どのようなときに使うか

- 既存 AD を維持しつつクラウド認証を始めるとき
- 外部開発者やパートナーの権限を期限付きで管理したいとき
- 監査対応で権限棚卸しを定期化したいとき

### 関連シナリオ

- [[Scenarios/Scenario-EntraID.md#ハイブリッド-id-の認証方式を選定する]]
- [[Scenarios/Scenario-EntraID.md#外部ユーザーのb2b招待とアクセスレビュー]]

### 主な出典

- [[Sources/Topic-01.md]]
- [[Sources/Azure AD Connect によるハイブリッドID同期設計.md]]
- [[Sources/Microsoft Entra ID ハイブリッド認証方式ドキュメント.md]]
- [[Sources/外部ユーザーアクセスの定期監査（Azure AD Access Reviews）.md]]

---

# 第3章 設計判断ガイド

## 3.1 認証方式を選ぶとき

- ユーザー本人のサインインなら Entra ID。
- アプリがユーザーの代理で API を呼ぶなら Delegated Permissions。
- アプリ自身の権限で API を呼ぶなら Application Permissions。
- Azure リソースが別の Azure リソースへ接続するなら Managed Identity。

## 3.2 権限付与を選ぶとき

- データ プレーン権限は対象リソースの RBAC を使う。
- 外部ユーザーは B2B と Access Reviews をセットで考える。
- 管理者権限は PIM で常時付与を避ける。

## 3.3 シークレット管理を選ぶとき

- 秘密値があるなら Key Vault に寄せる。
- 取得側は `DefaultAzureCredential` と Managed Identity を優先する。
- ネットワーク要件が強い場合は Private Endpoint を追加する。

---

# 第4章 関連シナリオ

- [[Scenarios/Scenario-EntraID.md]]
- [[Scenarios/Scenario-ManagedIdentity.md]]
- [[Scenarios/Scenario-KeyVault.md]]

