---
分類: Security
tags:
  - cloud/azure
  - cloud/azure/databricks
  - cloud/azure/data-lake-storage
  - cloud/azure/adls-gen2
  - cloud/azure/identity
  - cloud/azure/azure-ad
  - cloud/security/access-control
  - cloud/security/identity-based-access
  - cloud/data-engineering
  - exam/azure/databricks
---

# Azure Databricks Credential Passthrough による安全なデータアクセス

## 1. 背景（シナリオ）

Azure Databricks を使用して、大規模なデータ処理基盤を構築するプロジェクトが進められています。処理対象となるデータは Azure Data Lake Storage Gen2（ADLS Gen2）に保存されており、データエンジニアが Databricks クラスターを利用して分析・処理を行います。

しかし、データレイクには機密性の高いデータも含まれているため、すべてのユーザーがすべてのデータにアクセスできる状態では問題があります。特定のディレクトリやデータセットは、許可されたデータエンジニアのみがアクセスできるようにする必要があります。

つまり、Databricks を使用する場合でも、ADLS Gen2 側のアクセス制御ポリシーをそのまま適用し、ユーザー単位でのアクセス制御を維持する必要があります。

## 2. 要件整理

このシナリオから読み取れる主な要件は次の通りです。

まず、Azure Databricks から Azure Data Lake Storage Gen2 のデータへ安全にアクセスする必要があります。これはデータ処理のための基本的な要件です。

次に、ユーザーごとにアクセス権限を制御する必要があります。つまり、特定のディレクトリには許可されたユーザーのみがアクセスできるようにする必要があります。

さらに、Azure Databricks クラスター上で処理を行う際にも、ユーザーの権限がそのまま適用される仕組みが必要です。

このシナリオの要件は次の通りです。

- ADLS Gen2 の安全なアクセス  
- ユーザー単位のアクセス制御  
- ディレクトリレベルの権限管理  
- Azure AD ベースの認証  

## 3. 技術の基本概念

Azure Databricks では、ADLS Gen2 へアクセスする方法として **Credential Passthrough（資格情報パススルー）** という機能が提供されています。

Credential Passthrough を使用すると、Databricks クラスターはサービスアカウントではなく **実際にクエリを実行しているユーザーの Azure AD 資格情報** を使用してストレージへアクセスします。

つまり、Databricks からデータへアクセスする際に、そのユーザーが ADLS Gen2 で持っている権限がそのまま適用されます。

この仕組みにより、ストレージ側のアクセス制御ポリシーを Databricks でもそのまま利用できます。

## 4. アーキテクチャまたは設計のポイント

Credential Passthrough を使用したアーキテクチャでは、Azure Active Directory が認証の中心となります。

ユーザーが Databricks ノートブックやジョブを実行すると、そのユーザーの Azure AD トークンが使用されて ADLS Gen2 へアクセスします。

この構成では次のような仕組みが実現されます。

- ユーザーが Azure AD で認証される  
- Databricks がユーザーの認証情報を使用する  
- ADLS Gen2 の ACL（アクセス制御リスト）が適用される  
- 許可されたユーザーのみ特定ディレクトリへアクセスできる  

この方法により、ストレージレベルのセキュリティポリシーを維持できます。

## 5. 設計判断（なぜこの構成になるか）

Credential Passthrough を使用すると、Databricks は個別ユーザーの Azure AD 資格情報を使用してストレージへアクセスします。

これにより、次のようなセキュリティメリットがあります。

- ユーザー単位のアクセス制御  
- ディレクトリレベルの権限適用  
- Azure AD と統合された認証  
- アクセス監査の容易化  

もしサービスプリンシパルなどの共有認証情報を使用すると、すべてのユーザーが同じ権限でデータへアクセスする可能性があります。Credential Passthrough はこの問題を回避できます。

そのため、Databricks と ADLS Gen2 を安全に統合する際に最も推奨される方法です。

## 6. 他の選択肢が誤りな理由

Managed Identity は Azure サービスの認証に使用できますが、ユーザー単位のアクセス制御を直接適用するものではありません。通常はサービス単位のアクセスになります。

高並列クラスターは複数ユーザーの同時処理を最適化するためのクラスタータイプであり、アクセス制御とは関係ありません。

Photon ランタイムは Databricks の処理性能を向上させる機能であり、セキュリティ機能ではありません。

## 7. 最終回答

A. 資格情報パススルー（Credential Passthrough）

## 8. まとめ

Azure Databricks で ADLS Gen2 のデータを安全に利用するためには、ユーザー単位のアクセス制御を維持することが重要です。

Credential Passthrough を使用すると、Databricks は実際のユーザーの Azure AD 資格情報を使用してストレージへアクセスします。そのため、ADLS Gen2 の ACL やアクセス権限をそのまま適用できます。

この仕組みにより、ディレクトリ単位のセキュリティを維持しながら、大規模なデータ処理を安全に実行できます。