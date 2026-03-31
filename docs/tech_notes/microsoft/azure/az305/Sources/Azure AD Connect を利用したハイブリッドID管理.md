---
分類: Security
tags:
  - cloud/azure
  - cloud/azure/entra-id
  - cloud/azure/ad-connect
  - cloud/identity/hybrid-identity
  - cloud/identity/sso
  - security/iam
  - security/identity-management
  - exam/azure/security
---

# Azure AD Connect を利用したハイブリッドID管理

## 1. 背景（シナリオ）

ある企業では、オンプレミス環境とクラウド環境の両方でユーザーを管理しています。オンプレミスにはActive Directoryが存在し、社内ユーザーは既にこのディレクトリサービスを使用して認証されています。

同時に、企業はAzureを導入してクラウドリソース（仮想マシン、アプリケーション、データベースなど）を利用しています。そのため、ユーザーはオンプレミス環境だけでなくAzureリソースにもアクセスする必要があります。

このような状況では、ユーザーが新しいクラウドアカウントを作成するのではなく、既存のオンプレミスActive Directoryの認証情報を使用してAzureにサインインできるようにすることが重要です。

この要件を満たすためには、オンプレミスActive DirectoryとAzure Active Directory（現在はMicrosoft Entra ID）を統合する仕組みが必要になります。

---

## 2. 要件整理

この問題では、オンプレミスのActive DirectoryとAzure Active Directoryを統合する方法を選択する必要があります。

シナリオから読み取れる主な要件は次の通りです。

まず、既存のオンプレミスActive Directoryのユーザーアカウントを利用できる必要があります。つまり、ユーザーが同じユーザー名とパスワードでクラウドサービスへログインできる必要があります。

次に、オンプレミスとクラウドのIDを統合したハイブリッドID環境を構築する必要があります。

さらに、ユーザーアカウントやグループ情報などのディレクトリデータをクラウドと同期する仕組みが必要になります。

これらの要件を整理すると次の通りです。

- オンプレミスActive Directoryとの統合  
- Azure Active Directoryとの同期  
- 同一認証情報でのサインイン  
- ハイブリッドID管理  

---

## 3. 技術の基本概念

Azure環境でオンプレミスActive DirectoryとAzure Active Directoryを統合する場合、「ハイブリッドID」という概念が重要になります。

ハイブリッドIDとは、オンプレミスディレクトリとクラウドディレクトリを統合して、ユーザーが同一のIDで両方の環境にアクセスできるようにする仕組みです。

この統合を実現するためのツールがAzure AD Connectです。

Azure AD Connectは、オンプレミスActive DirectoryとAzure Active Directoryの間でディレクトリ情報を同期するためのツールです。

Azure AD Connectでは次のような機能を提供しています。

- ユーザーアカウントの同期  
- グループ情報の同期  
- パスワードハッシュ同期  
- シングルサインオン（SSO）  

これにより、ユーザーはオンプレミスとクラウドの両方で同じ認証情報を使用できます。

---

## 4. アーキテクチャまたは設計のポイント

ハイブリッドID環境では、オンプレミスActive Directoryがユーザー情報の「ソース」として機能します。

Azure AD Connectをオンプレミス環境にインストールすることで、ディレクトリ情報がAzure Active Directoryへ同期されます。

この構成では、次のような処理が行われます。

1. オンプレミスActive Directoryにユーザーを作成  
2. Azure AD Connectがディレクトリ情報を同期  
3. Azure Active Directoryにユーザーが作成される  
4. ユーザーは同じ認証情報でAzureへログイン  

また、Azure AD Connectでは次のような認証方式も利用できます。

- パスワードハッシュ同期  
- パススルー認証  
- フェデレーション認証  

これにより、企業のセキュリティ要件に合わせた認証方式を選択できます。

---

## 5. 設計判断（なぜこの構成になるか）

この問題では「既存のオンプレミス認証情報を利用する」という点が重要です。

Azure AD Connectは、オンプレミスActive DirectoryとAzure Active Directoryを同期するための標準ツールです。

このツールを利用することで、次のメリットがあります。

- ユーザーアカウントの自動同期  
- 同一認証情報でのクラウドログイン  
- シングルサインオン  
- ハイブリッドID環境の構築  

また、Azure AD ConnectはMicrosoftが推奨するハイブリッドIDソリューションでもあります。

そのため、オンプレミスActive DirectoryをAzure Active Directoryと統合する場合にはAzure AD Connectを使用するのが最適です。

---

## 6. 他の選択肢が誤りな理由

Azure AD Domain Servicesは、マネージドドメインサービスを提供するサービスです。ドメイン参加やLDAP、グループポリシーなどを提供しますが、オンプレミスActive DirectoryをAzure ADと同期するためのツールではありません。

Azure AD B2Cは、外部ユーザー（顧客）向けの認証サービスです。企業内部ユーザーのID統合には使用されません。

パススルー認証は認証方式の一つですが、オンプレミスActive DirectoryとAzure ADの同期を実現するツールではありません。通常はAzure AD Connectと組み合わせて使用されます。

---

## 7. 最終回答

B.  
Azure AD Connect

---

## 8. まとめ

オンプレミスActive DirectoryとAzure Active Directoryを統合する場合、ハイブリッドID環境を構築する必要があります。

Azure AD Connectは次の機能を提供します。

- ディレクトリ同期  
- パスワードハッシュ同期  
- シングルサインオン  
- ハイブリッドID管理  

そのため、既存のオンプレミス認証情報を使用してAzureリソースへサインインできるようにするには **Azure AD Connect** を使用するのが最適です。