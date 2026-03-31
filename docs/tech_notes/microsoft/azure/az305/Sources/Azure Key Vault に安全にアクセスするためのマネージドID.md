---
分類: Security
tags:
  - cloud/azure
  - cloud/azure/key-vault
  - cloud/azure/managed-identity
  - cloud/azure/app-service
  - security/identity
  - security/iam
  - cloud/security/secrets-management
  - cloud/security/least-privilege
  - exam/azure/security
---

# Azure Key Vault に安全にアクセスするためのマネージドID

## 1. 背景（シナリオ）

ある組織では、App3という名前のAzure Webアプリをデプロイする予定です。このアプリケーションは、Azure Key Vault に保存された機密情報（APIキー、接続文字列、パスワードなど）を実行時に取得して利用します。

しかし、セキュリティ上の理由から、アプリケーションコード内に認証情報をハードコードすることは禁止されています。そのため、アプリケーションは安全な方法でKey Vaultに認証し、必要なシークレットを取得する仕組みを導入する必要があります。

このようなシナリオでは、アプリケーションがAzureサービスとして実行される際に、Azureの認証基盤を利用して安全にKey Vaultへアクセスできる仕組みを採用することが重要です。

---

## 2. 要件整理

この問題では、WebアプリがAzure Key Vaultへ安全にアクセスするための認証方法を選択する必要があります。

シナリオから読み取れる重要な要件は次の通りです。

まず、アプリケーションコードに認証情報を埋め込まないことです。これはセキュリティ上のベストプラクティスであり、資格情報の漏洩リスクを防ぐために重要です。

次に、Azure Key Vaultのシークレットに対して安全にアクセスできる仕組みが必要です。つまり、Azure Active Directoryを利用した認証が必要になります。

さらに、資格情報の管理やローテーションを手動で行う必要がないことも重要です。Azureが自動的に認証情報を管理する仕組みが望まれます。

この要件を整理すると次のようになります。

- 認証情報をコードにハードコードしない  
- Azure Key Vaultに安全にアクセスする  
- 資格情報の管理を自動化する  
- Azure AD認証を利用する  

---

## 3. 技術の基本概念

Azureには「Managed Identity（マネージドID）」という仕組みがあります。これは、Azureリソースが他のAzureサービスにアクセスする際に使用するためのIDを自動的に提供する機能です。

Managed Identityを利用すると、アプリケーションはAzure ADを通じて認証されるため、パスワードやAPIキーなどの認証情報を管理する必要がなくなります。

Managed Identityには次の2種類があります。

まず「System-assigned Managed Identity」です。これはAzureリソース（Webアプリなど）に対して自動的に割り当てられるIDであり、そのリソースのライフサイクルに紐づきます。

次に「User-assigned Managed Identity」です。これは独立したIDとして作成され、複数のAzureリソースから共有して利用することができます。

この問題のシナリオでは、単一のWebアプリがKey Vaultにアクセスするため、System-assigned Managed Identityが最もシンプルな構成となります。

---

## 4. アーキテクチャまたは設計のポイント

WebアプリがKey Vaultにアクセスする際には、次のようなアーキテクチャを採用します。

まず、WebアプリにSystem-assigned Managed Identityを有効化します。これにより、Azure Active Directory内にそのアプリ専用のIDが作成されます。

次に、Azure Key Vault側でアクセス権限を設定します。具体的には、Managed Identityに対してシークレットの取得や一覧取得などの必要な権限を付与します。

この構成では、WebアプリはAzure ADを通じてKey Vaultへ認証されます。

この仕組みの流れは次の通りです。

1. WebアプリにManaged Identityを有効化  
2. Azure ADにIDが作成される  
3. Key Vaultにアクセス権を設定  
4. アプリはManaged IdentityでKey Vaultへ認証  

この方法により、認証情報をコードに保存する必要がなくなります。

---

## 5. 設計判断（なぜこの構成になるか）

この問題では「ハードコードされた認証情報を使用しない」という条件が非常に重要です。

Managed Identityは、この要件を満たすためのAzureのベストプラクティスです。

Managed Identityを使用するメリットは次の通りです。

- 認証情報をコードに保存する必要がない  
- Azureが認証情報を自動管理する  
- 資格情報のローテーションが不要  
- Azure ADによる安全な認証  
- 最小権限の原則を適用可能  

このため、WebアプリからKey Vaultへアクセスする場合にはManaged Identityを利用するのが最も安全で推奨される方法になります。

---

## 6. 他の選択肢が誤りな理由

まずサービスプリンシパルについて説明します。サービスプリンシパルはAzure ADでアプリケーション認証に使用できますが、クライアントシークレットや証明書を管理する必要があります。これは認証情報を安全に管理する必要があるため、ハードコードされた認証情報を避けたいという要件に完全には適していません。

次にSASトークンについてです。Shared Access Signature（SAS）は主にAzure Storageサービスで使用されるアクセス制御メカニズムであり、Azure Key Vaultの認証には使用されません。

最後にAzure Policyです。Azure Policyはリソースのガバナンスやコンプライアンスを管理するためのサービスであり、アプリケーション認証の仕組みではありません。

---

## 7. 最終回答

A.  
システム割り当てマネージドID（System-assigned Managed Identity）

---

## 8. まとめ

Azure Key Vaultに安全にアクセスするためには、認証情報をアプリケーションコードに保存しないことが重要です。

Azure Managed Identityを利用することで、次のメリットを得ることができます。

- 認証情報の管理不要  
- Azure ADによる安全な認証  
- 自動資格ローテーション  
- 最小権限アクセス制御  

特にAzure WebアプリからKey Vaultにアクセスする場合は、**System-assigned Managed Identity**を使用するのがAzureのベストプラクティスです。