
# Azure ワークロード用 ID（実務・試験コンパクト整理）

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Key Vault]]
Azureでアプリケーションが Azure SQL、Storage、Key Vault などへアクセスする場合、主に次の4種類のIDを使います。

- クライアントシークレット付き Service Principal  
- 証明書付き Service Principal  
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Managed Identity]]
- System-assigned Managed Identity（SAMI）  
- User-assigned Managed Identity（UAMI）

基本原則はシンプルです。

- **Azure上で動くアプリ → Managed Identity を優先**
- **Azure外のアプリ → Service Principal**

以下では、それぞれの特徴・メリット・デメリット・使うケースを簡潔に整理します。

---

# クライアントシークレット付き Service Principal

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Microsoft Entra ID]]
これは Azure Entra ID（旧Azure AD）に登録したアプリケーションIDを **パスワード（client secret）で認証する方式**です。  
Azureで長く使われてきた一般的なアプリ認証方法です。

特徴として、アプリケーションは **client ID と client secret を使ってトークンを取得**し、そのトークンで Azure リソースへアクセスします。  
つまり、**認証情報をアプリケーション側に保存する必要があります。**

メリットは、**Azure外のシステムからでも使える汎用性**です。  
例えば GitHub Actions、Jenkins、オンプレミスアプリなどが Azure にアクセスする場合によく使われます。

デメリットは、**secret管理の運用負荷**です。  
secretには期限があり、定期的なローテーションが必要になります。また、secretが漏洩すると不正アクセスにつながるため、保管と配布の管理も必要になります。

そのため実務では、**Managed Identity が使える場合は基本的にこちらは選ばれません。**

使うケースは次のような場面です。

- Azure外のアプリケーション
- CI/CD（GitHub Actions、Azure DevOps など）
- 外部SaaSや自社ツールからAzureにアクセスする場合

---

# 証明書付き Service Principal

これは Service Principal を **パスワードではなく証明書で認証する方式**です。  
client secret の代わりに **X.509証明書**を使います。

仕組みとしては、アプリケーションが証明書を使って Azure Entra ID に対して認証し、アクセストークンを取得します。

メリットは、**client secret よりセキュアな認証ができること**です。  
証明書は漏洩リスクが低く、比較的長期間利用できるため、セキュリティポリシーが厳しい環境ではこちらが使われることがあります。

一方デメリットは、**証明書の運用管理が必要になること**です。  
証明書の配布、保管、更新、失効などの管理が必要になり、環境によっては secret より運用が複雑になります。

そのため実務では、**企業PKIなど証明書運用の仕組みがある組織で使われることが多いです。**

使うケースは次のような場面です。

- 高セキュリティ要件の環境
- 企業PKIが整備されている環境
- 長期認証が必要なサービス

ただし Azure 上のワークロードでは、通常は **Managed Identity の方が運用が簡単**です。

---

# System-assigned Managed Identity（SAMI）

System-assigned Managed Identity は **Azureリソースに自動的に作成されるID**です。

例えば次のリソースに直接付与できます。

- Virtual Machine
- App Service
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Functions]]
- Azure Functions
- Container Apps

このIDは Azure が自動管理するため、**パスワードや証明書を管理する必要がありません。**

アプリケーションは Azure の内部エンドポイントからトークンを取得し、そのトークンを使って Azure リソースへアクセスします。

メリットは、**認証情報管理が完全に不要になること**です。  
secretも証明書も存在しないため、漏洩リスクやローテーション作業がありません。

また、このIDは **リソースとライフサイクルが一体**です。  
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
つまり VM を削除すると、そのIDも自動的に削除されます。

デメリットは、**リソースごとに別IDになること**です。  
複数のVMやアプリで同じIDを共有することはできません。

そのため、SAMIは **単一リソースが他のAzureサービスにアクセスするケース**に向いています。

典型例は次のような構成です。

- App Service → Key Vault
- Azure Function → Storage
- VM → Azure SQL
- Container App → Storage

試験では次の条件が出たら **SAMI が正解になりやすいです。**

- 単一のAzureリソース
- 認証情報を保存しない
- 運用負荷を減らす

---

# User-assigned Managed Identity（UAMI）

User-assigned Managed Identity は **Azureに独立したIDを作成し、それを複数のリソースに割り当てる方式**です。

SAMIとの違いは、IDがリソースに紐づかず、**独立したAzureリソースとして存在すること**です。

そのため、同じIDを次のように複数のリソースで共有できます。

- 複数VM
- 複数App Service
- 複数コンテナ

メリットは、**IDを共有できること**です。  
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
例えば50台のVMが同じAzure SQL Databaseにアクセスする場合、UAMIを1つ作成し、それをすべてのVMに割り当てることで、SQL側の権限管理も1つのIDだけで済みます。

また、VMを削除してもIDは残るため、**スケールアウト環境でも管理が簡単**になります。

デメリットは、SAMIより **管理対象のリソースが1つ増えること**ですが、実務上はほとんど問題になりません。

UAMIは次のようなケースに向いています。

- スケールアウトVM
- マイクロサービス
- 複数アプリが同じリソースにアクセス
- 共有IDが必要な環境

試験では次の条件が出たら **UAMI が正解になりやすいです。**

- 複数VM
- 共有ID
- スケールアウト
- 運用管理を簡単にする

---

# 試験での判断ルール

まず「Azure上で動くアプリか」を見ます。

Azure上なら **Managed Identity を優先**します。  
Azure外なら **Service Principal** を使います。

次に「IDを共有するか」を確認します。

共有するなら **User-assigned Managed Identity**  
共有しないなら **System-assigned Managed Identity**

---

# 試験用の覚え方

Azureリソース  
→ Managed Identity

単一リソース  
→ System-assigned Managed Identity

複数リソースで共有  
→ User-assigned Managed Identity

Azure外アプリ  
→ Service Principal
