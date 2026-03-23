---
分類: Security
tags:
  - cloud/azure
  - cloud/azure/key-vault
  - cloud/azure/managed-identity
  - cloud/azure/rbac
  - security/secrets-management
  - security/identity
  - security/least-privilege
  - exam/azure
---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Managed Identity]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Key Vault]]
# Azure Key Vault と Managed Identity を使用したシークレットアクセス制御

## 1. 背景（シナリオ）

ある Azure アプリケーションでは、アプリケーションの設定情報や接続文字列などの機密情報を安全に管理するために **Azure Key Vault** を使用しています。このアプリケーションは Azure 上で実行されており、Key Vault からシークレットを取得して利用する必要があります。

アプリケーションは認証のために **Managed Identity（マネージド ID）** を使用しています。Managed Identity を使用すると、アプリケーションはパスワードや接続文字列などの資格情報をコードに埋め込む必要がなくなります。Azure が自動的に認証を管理するため、セキュリティの観点から非常に安全な仕組みです。

しかし、このシナリオでは重要なセキュリティ要件があります。アプリケーションは特定の Key Vault に保存されたシークレットのみを読み取ることができる必要があります。また、その Managed Identity が Azure 内の他のリソースや操作に対する権限を持たないようにする必要があります。

つまり、この設計では **最小特権の原則（Least Privilege）** を守りながら、Managed Identity に対して必要最小限の Key Vault アクセス権を付与する必要があります。

---

## 2. 要件整理

この問題では、アクセス管理の設計に関する重要な要件があります。まず、それぞれの条件を整理して理解することが重要です。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Microsoft Entra ID]]
最初に、認証方法に関する要件があります。アプリケーションは Azure の Managed Identity を使用して Key Vault にアクセスします。これは Azure AD によって管理される ID であり、アプリケーション専用のサービスプリンシパルとして機能します。

次に、アクセス対象の範囲に関する要件があります。アプリケーションは Azure 内のすべての Key Vault にアクセスできる必要はなく、特定の Key Vault のみを対象とする必要があります。

- **特定の Key Vault のみアクセス可能**

さらに、アクセス操作の範囲も制限する必要があります。アプリケーションはシークレットを読み取るだけでよく、他の操作は必要ありません。

- シークレットの **読み取りのみ**

最後に、セキュリティ上の要件があります。Managed Identity は Azure の他のリソースに対して不要な権限を持つべきではありません。

- **Azure 内の他の権限を持たない**

このような要件を満たすためには、Key Vault のアクセス制御機能を適切に使用する必要があります。

---

## 3. 技術の基本概念

### Managed Identity

Managed Identity は Azure リソースに自動的に割り当てられる ID であり、Azure AD によって管理されます。この ID を使用することで、アプリケーションは Azure サービスに安全に認証できます。

Managed Identity を使用する最大の利点は、アプリケーションが資格情報を管理する必要がないことです。Azure がトークンの発行と管理を自動的に行うため、秘密情報をコードや構成ファイルに保存する必要がありません。

Managed Identity は主に次のような Azure サービスと組み合わせて使用されます。

- Azure Key Vault
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Storage Account]]
- Azure Storage
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
- Azure SQL Database
- Azure Resource Manager

この仕組みにより、アプリケーションは安全な方法で Azure リソースにアクセスできます。

---

### Azure Key Vault Access Policy

Azure Key Vault には、リソースへのアクセスを制御するための **Access Policy** という仕組みがあります。Access Policy は Key Vault 内のキー、シークレット、証明書などのリソースに対するアクセス権を細かく制御することができます。

Access Policy を使用すると、特定の Azure AD プリンシパル（ユーザー、サービスプリンシパル、Managed Identity など）に対して必要な操作のみを許可することができます。

Key Vault Access Policy の特徴を理解するためには、どのような権限を設定できるのかを整理することが重要です。

- シークレットの取得（Get）
- シークレット一覧の取得（List）
- シークレットの作成
- シークレットの削除

このように、Access Policy を使用することで非常に細かいアクセス制御を実現できます。

---

## 4. アーキテクチャまたは設計のポイント

今回のシナリオでは、Managed Identity を Key Vault の Access Policy に登録し、その ID に対してシークレットの読み取り権限のみを付与します。

まず、Key Vault に Access Policy を設定し、アプリケーションの Managed Identity をプリンシパルとして追加します。その際、許可する操作として **Secret の Get と List** のみを設定します。

この設定により、アプリケーションは Key Vault 内のシークレットを読み取ることができますが、それ以外の操作（作成、削除、変更など）は実行できません。

さらに、この権限は特定の Key Vault にのみ適用されるため、Managed Identity は他の Azure リソースに対して権限を持つことはありません。

このような構成により、最小特権の原則を守りながら安全にシークレットを取得できるアーキテクチャを構築できます。

---

## 5. 設計判断（なぜこの構成になるか）

この問題の重要なポイントは、Managed Identity に対して **最小限の Key Vault 権限のみを付与すること**です。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Role-Based Access Control (RBAC)]]
Azure RBAC を使用して Key Vault へのアクセスを管理することも可能ですが、Key Vault Access Policy は Key Vault 内のデータプレーン操作を細かく制御するために設計されています。

Access Policy を使用すると、特定の Key Vault に対してのみ権限を付与することができ、さらにシークレットの取得など特定の操作だけを許可することができます。

このような細粒度のアクセス制御は、セキュリティ要件が厳しい環境では非常に重要です。

---

## 6. 他の選択肢が誤りな理由

### Azure RBAC

Azure RBAC は Azure リソース全体のアクセス制御を管理する仕組みです。Key Vault でも RBAC を使用できますが、この問題では Key Vault 内のデータプレーン操作を細かく制御する必要があります。そのため Access Policy を使用する方が適切です。

### 条件付きアクセス

条件付きアクセスは Azure AD のサインインポリシーを制御する機能です。例えば、多要素認証やデバイス条件などを設定するための機能であり、Key Vault 内のアクセス権限を制御するものではありません。

### Azure AD アプリケーション権限

Azure AD のアプリケーション権限は Microsoft Graph などの API にアクセスする際に使用されます。この問題では Managed Identity が Key Vault にアクセスする必要があるため、この方法は適していません。

---

## 7. 最終回答

B. Key Vault のアクセスポリシーを使用する