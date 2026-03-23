---
分類: Cloud
tags:
  - cloud/azure
  - cloud/azure/databricks
  - cloud/azure/data-lake-storage
  - cloud/azure/adls-gen2
  - security/identity
  - security/access-control
  - cloud/architecture/data-platform
  - cloud/architecture/big-data
  - exam/azure
---

# Azure Databricks と ADLS Gen2 の安全なアクセス（Azure AD Credential Passthrough）

## 1. 背景（シナリオ）

ある組織では、Azure Databricks を利用してデータ分析および機械学習のワークロードを実行するデータプラットフォームを構築しています。

この環境では、データは **Azure Data Lake Storage Gen2（ADLS Gen2）** に保存されており、データサイエンティストは Databricks を使用して次のような処理を実行します。

- 構造化データの分析
- 非構造化データの処理
- 並列機械学習実験
- 大規模データ処理

しかし、データレイクには機密データも含まれるため、**厳格なアクセス制御**が必要です。  
特に、ユーザーごとにアクセスできるディレクトリを制御する必要があります。

そのため、次の要件を満たす安全なアクセス方式を選択する必要があります。

- ユーザー単位のアクセス制御
- データディレクトリレベルの権限管理
- Databricks からの安全なデータアクセス
- 並列処理を妨げない認証方式

---

## 2. 要件整理

この問題では、Databricks と ADLS Gen2 の統合における **認証方式の選択**がポイントになります。

シナリオから読み取れる主な要件は次の通りです。

まず、セキュリティに関する要件があります。

- データディレクトリごとの厳格なアクセス制御
- ユーザー単位の権限管理

次に、分析環境に関する要件です。

- 複数のデータサイエンティストが利用
- 並列機械学習実験
- Databricks クラスターからのデータアクセス

このことから、**ユーザーの Azure AD アイデンティティを使用してデータアクセスを制御できる仕組み**が必要になります。

---

## 3. 技術の基本概念

Azure Databricks では、ADLS Gen2 へのアクセス方法としていくつかの認証方式が存在します。

まず、最も重要な方式が **Azure AD Credential Passthrough** です。

Azure AD Credential Passthrough とは、Databricks からストレージにアクセスする際に **実際のユーザーの Azure AD 資格情報をそのまま使用する仕組み**です。

この方式の特徴を整理すると次の通りです。

- Azure AD のユーザー ID をそのまま使用
- ADLS Gen2 の ACL と統合
- ディレクトリ単位のアクセス制御
- ユーザーごとのアクセス監査

つまり、Databricks クラスターが共通資格情報でアクセスするのではなく、**実行ユーザーのアイデンティティでアクセス**します。

---

## 4. アーキテクチャまたは設計のポイント

Databricks と ADLS Gen2 を安全に統合する場合、Credential Passthrough を使用すると以下のような構成になります。

まず、ユーザーは Azure AD アカウントで Databricks にログインします。

次に Databricks クラスターで処理を実行すると、そのユーザーの資格情報が ADLS Gen2 へパススルーされます。

この構成の特徴を整理すると次の通りです。

- Databricks → ADLS Gen2 アクセス時にユーザー ID を使用
- ADLS Gen2 ACL によるアクセス制御
- ディレクトリ単位の権限管理
- ユーザー単位の監査ログ

この仕組みにより、複数のデータサイエンティストが同時に Databricks を利用しても、それぞれの権限に基づいたデータアクセスが保証されます。

---

## 5. 設計判断（なぜこの構成になるか）

この問題では **ユーザー単位の厳格なアクセス管理**が重要です。

Credential Passthrough を使用すると、次のメリットがあります。

まず、セキュリティ面のメリットです。

- ユーザー単位の認証
- データディレクトリごとのアクセス制御
- 監査ログの取得

次に、運用面のメリットです。

- 資格情報管理が不要
- 追加シークレット不要
- Databricks と Azure AD の統合

これにより、**ユーザーごとに異なるアクセス権を持つデータレイク環境**を安全に運用できます。

---

## 6. 他の選択肢が誤りな理由

まず **Azure Managed Identity** について説明します。

マネージド ID は Azure サービス間の認証に使用されます。しかし、この方式ではアクセスは **サービス単位**になります。

つまり、ユーザーごとのアクセス制御ができません。

次に **Service Principal** です。

Service Principal はアプリケーション認証に使用されますが、Databricks の全ユーザーが同じ資格情報でデータアクセスする可能性があります。

そのため、厳格なユーザー単位アクセス制御には適していません。

最後に **Key Vault シークレット**です。

Key Vault はシークレット管理のサービスであり、アクセス制御の仕組みではありません。

データレイクのディレクトリレベル権限管理には適していません。

---

## 7. 最終回答

**A. Azure AD Credential Passthrough**

---

## 8. まとめ

Azure Databricks と ADLS Gen2 を統合する場合、アクセス方式の選択は非常に重要です。

主なアクセス方式を整理すると理解しやすくなります。

まず、ユーザー単位のアクセス制御です。

- **Azure AD Credential Passthrough**  
  Databricks 実行ユーザーの ID を使用

次に、サービス認証です。

- **Managed Identity**
- **Service Principal**

最後に、シークレット管理です。

- **Azure Key Vault**

このため、**データディレクトリ単位の厳格なアクセス制御が必要な Databricks 環境では Azure AD Credential Passthrough が最適な認証方式**となります。