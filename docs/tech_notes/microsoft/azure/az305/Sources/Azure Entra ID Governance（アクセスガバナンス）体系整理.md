[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Microsoft Entra ID]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Identity Governance]]
# Azure Entra ID Governance（アクセスガバナンス）体系整理

Azure Entra ID（旧 Azure Active Directory）では、ユーザーの認証だけでなく、  
**誰がどのリソースにアクセスできるかを管理する仕組み**が重要になります。  
この領域は **Identity Governance（アイデンティティガバナンス）**と呼ばれます。

Identity Governance は、ユーザーアクセスの **付与・管理・監査・削除**までのライフサイクルを管理する機能群です。  
企業では、プロジェクトチーム、外部パートナー、管理者権限など様々なアクセスが存在するため、  
これらを適切に管理することがセキュリティとコンプライアンスの観点で重要になります。

Azure Entra ID では主に次の機能を組み合わせてアクセスガバナンスを実現します。

- Entitlement Management  
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Access Reviews]]
- Access Reviews  
- Privileged Identity Management（PIM）  
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Conditional Access]]
- Conditional Access  
- Identity Protection  

これらはそれぞれ役割が異なります。

---

# Entitlement Management

Entitlement Management は、Azure Entra ID の **アクセスライフサイクル管理機能**です。  
ユーザーがアプリケーションやグループなどのリソースへアクセスする際の **申請・承認・期限管理・レビュー**を一元的に管理できます。

この機能では **Access Package（アクセスパッケージ）**という仕組みを使用します。  
アクセスパッケージは、複数のリソースをまとめて管理する単位です。

例えば、プロジェクトチーム向けのアクセスパッケージを作成すると、次のようなリソースをまとめて付与できます。

- Azure AD グループ
- Webアプリケーション
- SharePoint サイト
- Teams チーム

ユーザーはアクセスパッケージを申請し、管理者やプロジェクトマネージャーが承認することでアクセス権が付与されます。

Entitlement Management ではアクセス期限を設定することもできます。  
例えば、90日間のアクセスを許可し、期限が切れると自動的にアクセスを削除することが可能です。

この仕組みにより、プロジェクトベースのチームや外部パートナーなど、  
**一時的なアクセスが必要なユーザーの管理を自動化**できます。

---

# Access Reviews

Access Reviews は、ユーザーのアクセス権を **定期的に確認する機能**です。  
管理者やリソース所有者は、ユーザーが現在持っているアクセス権をレビューし、  
継続するか削除するかを判断できます。

例えば、プロジェクトチームのメンバーが変更された場合でも、  
90日ごとにアクセスレビューを実施することで不要なユーザーを削除できます。

レビューの結果、承認されなかったユーザーのアクセス権は自動的に削除されます。  
この機能は、特に次のようなケースで重要になります。

- 外部ユーザーの管理  
- プロジェクトチームのアクセス管理  
- 長期間使用されていないアカウントの整理  

Access Reviews は Entitlement Management と組み合わせて利用されることが多く、  
アクセスのライフサイクル管理の一部として機能します。

---

# Privileged Identity Management（PIM）

Privileged Identity Management（PIM）は、**管理者権限を安全に管理する機能**です。

通常、管理者権限を常時付与するとセキュリティリスクが高くなります。  
PIM では、ユーザーは必要なときだけ一時的に管理者権限を有効化できます。

この仕組みは **Just-in-Time（JIT）アクセス**と呼ばれます。

例えば次のような管理者ロールに利用されます。

- Global Administrator
- Azure Subscription Owner
- Security Administrator

ユーザーが管理者権限を有効化する際には、  
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Multi-Factor Authentication (MFA)]]
承認フローや多要素認証（MFA）を要求することも可能です。

この機能により、管理者権限の乱用や不正アクセスを防ぐことができます。

---

# Conditional Access

Conditional Access は、**ログイン条件に基づいてアクセスを制御するポリシー機能**です。

ユーザーのログイン状況を評価し、  
アクセスを許可するか、追加認証を要求するか、またはアクセスを拒否するかを判断します。

条件として利用できる情報には次のものがあります。

- ユーザーまたはグループ
- アプリケーション
- ユーザーの場所（IPアドレス）
- デバイス状態
- サインインリスク

例えば次のようなポリシーを設定できます。

- 社外ネットワークからのアクセスはMFA必須
- 管理者は必ずMFAを要求
- 非準拠デバイスからのアクセスを拒否

Conditional Access は **認証時のセキュリティポリシー**として機能します。

---

# Identity Protection

Identity Protection は、ユーザーアカウントの **セキュリティリスクを検出・評価する機能**です。

Azure は機械学習や行動分析を利用して、  
異常なサインインやアカウント侵害の可能性を検出します。

例えば次のようなリスクが検出されます。

- 不審な場所からのログイン
- 不可能な移動（Impossible travel）
- マルウェア感染端末からのログイン
- パスワード漏洩

これらのリスクに対して、次の対策を自動的に適用できます。

- パスワード変更の強制
- MFA要求
- アカウントのブロック

Identity Protection は、**アカウント侵害を防ぐためのセキュリティ機能**です。

---

# 機能の役割まとめ

Azure Entra ID Governance の機能は、それぞれ役割が異なります。

|機能|役割|
|---|---|
Entitlement Management | アクセスの申請・承認・期限管理 |
Access Reviews | アクセス権の定期レビュー |
Privileged Identity Management | 管理者権限の管理 |
Conditional Access | ログイン条件によるアクセス制御 |
Identity Protection | 不正アクセスリスクの検出 |

---

# 試験での判断のポイント

Azure Entra ID の試験問題では、  
**アクセス管理・認証・セキュリティ**の違いを理解していることが重要です。

アクセス申請、承認、期限管理などが求められる場合は  
**Entitlement Management** が使用されます。

既存のアクセス権を定期的に確認する場合は  
**Access Reviews** が使用されます。

管理者権限の安全な管理が必要な場合は  
**Privileged Identity Management** が使用されます。

ログイン時の条件によるアクセス制御は  
**Conditional Access** が担当します。

アカウントのリスク検出や侵害対策は  
**Identity Protection** が担当します。

これらを理解しておくと、Azure Entra ID のアクセス管理に関する問題を体系的に判断できるようになります。