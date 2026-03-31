---
分類: Security
tags:
  - cloud/azure
  - cloud/azure/functions
  - cloud/azure/managed-identity
  - cloud/azure/rbac
  - security/iam
  - security/identity-management
  - security/secret-management
  - cloud/architecture/serverless
  - cloud/architecture/least-privilege
  - exam/azure
---

# Azure Functions の安全な認証設計（Managed Identity + RBAC）

## 1. 背景（シナリオ）

Azure 上でホストされるエンタープライズアプリケーションを設計しており、そのシステムでは **Azure Functions** を利用して複数の Azure リソースへアクセスする必要があります。例えば、Azure Storage、Azure Key Vault、Azure SQL Database などのサービスに対して、安全に接続する必要があります。

このアプリケーションは機密データを処理するため、セキュリティ要件やコンプライアンス要件が非常に厳格です。そのため、認証情報の漏洩リスクを最小限に抑える設計が必要になります。また、運用管理の観点から、資格情報のローテーションや秘密情報の管理といった管理負荷をできるだけ減らすことも重要です。

このような条件の中で、Azure Functions が Azure リソースへ安全にアクセスするための最適な認証方式を選択する必要があります。

## 2. 要件整理

この問題では、単に認証できればよいのではなく、**セキュリティ・管理性・運用負荷のバランス**が重要になります。

シナリオから読み取れる要件を整理すると次の通りです。

まず、Azure Functions が複数の Azure リソースへ安全にアクセスできる必要があります。さらに、機密データを扱うため、資格情報の管理や漏洩リスクを最小限に抑える必要があります。

また、運用管理の観点から、秘密情報の管理や証明書のローテーションなどの管理作業をできるだけ減らす必要があります。

これらをまとめると、次の条件になります。

・Azure Functions から Azure リソースへの安全な認証  
・資格情報（パスワードやキー）をコードに保存しない  
・最小権限の原則によるアクセス制御  
・管理オーバーヘッドの最小化  

このような要件を満たす仕組みが **Azure Managed Identity** です。

## 3. 技術の基本概念

Azure Managed Identity は、Azure リソースに対して **自動的に管理されるサービスプリンシパル（ID）** を提供する機能です。

この仕組みを利用すると、Azure Functions は Azure AD によって管理される ID を持つことができます。その ID を使用して、Azure リソースへ安全にアクセスすることができます。

Managed Identity の最大の特徴は、**認証情報を管理する必要がないこと**です。Azure が自動的に資格情報を管理し、定期的に更新します。

そのため、アプリケーションコードの中に次のような情報を保存する必要がありません。

・接続文字列  
・APIキー  
・クライアントシークレット  
・証明書

## 4. アーキテクチャまたは設計のポイント

Azure Functions から他の Azure リソースにアクセスする場合、Managed Identity と RBAC を組み合わせる設計が推奨されます。

まず、Azure Functions に **Managed Identity** を有効化します。これにより、Function App に Azure AD の ID が割り当てられます。

次に、その ID に対して **RBAC（Role-Based Access Control）** を使用して必要な権限を付与します。

例えば次のような権限設定が可能です。

・Storage Blob Data Reader  
・Key Vault Secrets User  
・SQL DB Contributor  

このように RBAC を利用することで、Azure Functions が必要なリソースのみにアクセスできるように制御できます。

## 5. 設計判断（なぜこの構成になるか）

Managed Identity を利用する最大のメリットは、**秘密情報の管理が不要になること**です。

通常の認証では、クライアントシークレットや証明書などの資格情報を管理する必要があります。しかし Managed Identity を利用すれば、Azure が自動的に認証情報を管理します。

さらに RBAC を利用することで、最小権限の原則に基づいたアクセス制御を実現できます。これにより、アプリケーションが必要以上の権限を持つことを防ぐことができます。

また、Azure のネイティブ機能として統合されているため、追加の認証サーバーやキー管理システムを構築する必要もありません。

## 6. 他の選択肢が誤りな理由

### A Azure AD + 条件付きアクセス

条件付きアクセスは主に **ユーザー認証のセキュリティ制御**に使用される機能です。例えば、MFA や場所ベースのアクセス制御などを実装する場合に使用されます。

しかし、この問題では **サービス間認証（Service-to-Service Authentication）** が必要であるため、最適な選択肢ではありません。

### C OAuth 2.0 カスタム認証サーバー

OAuth 2.0 を使用したカスタム認証サーバーの構築は可能ですが、開発・運用の負担が非常に大きくなります。

また、Azure には Managed Identity というネイティブな仕組みが存在するため、カスタム実装を行う必要はありません。

### D API Management + サブスクリプションキー

API Management のサブスクリプションキーやクライアント証明書を使用する方法では、キーや証明書の管理が必要になります。

この方式ではキーのローテーションや秘密管理の負担が増えるため、管理オーバーヘッドが大きくなります。

## 7. 最終回答

B. **ロールベースアクセス制御（RBAC）を備えた Azure Managed Identity**

## 8. まとめ

Azure Functions が Azure リソースにアクセスする場合、最も安全で管理しやすい方法は **Managed Identity + RBAC** の組み合わせです。

この方式の重要なポイントを整理すると次の通りです。

まず、Managed Identity によって Azure が認証情報を自動管理します。次に、RBAC を使用して最小権限のアクセス制御を実装します。

これにより、秘密情報をコードに保存する必要がなくなり、セキュリティと運用管理の両方を改善できます。

試験では **「Azure Functions」「シークレット管理不要」「サービス間認証」「最小管理オーバーヘッド」** というキーワードが出てきた場合、Managed Identity が正解になるケースが非常に多いです。