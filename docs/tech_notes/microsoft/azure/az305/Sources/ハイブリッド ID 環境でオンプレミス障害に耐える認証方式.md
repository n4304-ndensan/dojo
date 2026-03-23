---
分類: Security
tags:
  - cloud/azure
  - cloud/azure/entra-id
  - cloud/azure/ad-connect
  - cloud/azure/password-hash-sync
  - cloud/architecture/hybrid-identity
  - security/identity
  - security/authentication
  - exam/azure
---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Hybrid Identity]]
# ハイブリッド ID 環境でオンプレミス障害に耐える認証方式

## 1. 背景（シナリオ）

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Microsoft Entra ID]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure AD Connect]]
ある企業では、オンプレミスの Active Directory と Azure AD（現在の Microsoft Entra ID）を統合した **ハイブリッド ID 環境**を構築しています。この環境では Azure AD Connect を使用して、オンプレミスのユーザーアカウントをクラウドに同期しています。

このようなハイブリッド ID モデルでは、多くの場合ユーザーはオンプレミスの Active Directory によって認証されます。しかし企業のセキュリティチームは、災害対策の観点から重要な要件を提示しました。それは **オンプレミスの Active Directory が利用できなくなった場合でもユーザーが Azure リソースにサインインできるようにすること**です。

例えば、オンプレミスのデータセンター障害やネットワーク障害が発生した場合でも、ユーザーは Microsoft 365 や Azure ポータルなどのクラウドサービスにアクセスできる必要があります。

このような状況に対応するためには、オンプレミスインフラストラクチャに依存しない認証方式を選択する必要があります。

---

## 2. 要件整理

この問題では、認証方式の選択に関する重要な要件があります。まず、システム設計に影響する条件を整理する必要があります。

最初の要件は ID 管理モデルです。企業は既に Azure AD Connect を使用してオンプレミス Active Directory と Azure AD を同期しています。

- **ハイブリッド ID 環境**

次に、認証の可用性に関する要件があります。企業はオンプレミスの Active Directory が停止した場合でも、ユーザーが Azure リソースへアクセスできる必要があります。

- **オンプレミス AD 障害時でも認証可能**

さらに、クラウドリソースへのアクセスを継続する必要があります。

- Azure ポータル  
- Azure リソース  
- Microsoft 365  

このような条件を満たすためには、クラウド側で認証を完結できる方式を選択する必要があります。

---

## 3. 技術の基本概念

### Azure AD Connect

Azure AD Connect は、オンプレミスの Active Directory と Azure AD を同期するためのツールです。このツールを使用することで、ユーザーアカウント、グループ、パスワードなどの情報をクラウドに同期できます。

Azure AD Connect は複数の認証方式をサポートしており、それぞれ異なる特徴を持っています。

Azure AD Connect がサポートする主な認証方式を理解するためには、それぞれの仕組みを整理することが重要です。

主な認証方式には次のようなものがあります。

- Password Hash Synchronization（PHS）
- Pass-through Authentication（PTA）
- Active Directory Federation Services（AD FS）

これらの方式は、認証がどこで行われるかによって大きく異なります。

---

### Password Hash Synchronization（PHS）

Password Hash Synchronization は、オンプレミス Active Directory のユーザーパスワードのハッシュを Azure AD に同期する仕組みです。この方式では、Azure AD がクラウド内で直接ユーザー認証を行います。

重要なポイントは、パスワードの **平文が同期されるわけではない**という点です。Azure AD には追加のハッシュ処理が行われたパスワードハッシュのみが保存されます。

この方式の特徴を理解するためには、クラウド認証の仕組みを整理する必要があります。

Password Hash Synchronization の主な特徴には次のようなものがあります。

- Azure AD が直接認証を実行する  
- オンプレミス AD への接続が不要  
- オンプレミス障害時でも認証可能  

この仕組みにより、クラウド側で独立した認証が可能になります。

---

## 4. アーキテクチャまたは設計のポイント

今回のシナリオでは、ユーザー認証を Azure AD 側で実行できるようにすることが重要です。Password Hash Synchronization を使用すると、オンプレミスのユーザーパスワードハッシュが Azure AD に同期されます。

ユーザーが Azure ポータルや Microsoft 365 にサインインする際、Azure AD はクラウドに保存されたパスワードハッシュを使用して認証を行います。そのため、オンプレミスのドメインコントローラーに問い合わせる必要がありません。

もしオンプレミスの Active Directory が停止しても、Azure AD はクラウド側に保存されたパスワードハッシュを使用して認証を続けることができます。

この構成により、オンプレミスインフラストラクチャの障害がクラウドサービスの認証に影響することを防ぐことができます。

---

## 5. 設計判断（なぜこの構成になるか）

この問題の重要なポイントは **認証がどこで実行されるか**です。

Pass-through Authentication や AD FS のような方式では、ユーザー認証はオンプレミスの Active Directory に対して行われます。そのため、オンプレミス環境が停止した場合には認証ができなくなります。

一方で Password Hash Synchronization を使用すると、Azure AD がクラウド側で認証を行います。そのため、オンプレミスのインフラストラクチャが利用できない場合でもユーザーは Azure にサインインすることができます。

このように、クラウド認証を実現する Password Hash Synchronization が最も適した認証方式になります。

---

## 6. 他の選択肢が誤りな理由

### Pass-through Authentication

PTA は Azure AD からオンプレミスのエージェントを通じて Active Directory に認証を問い合わせます。そのためオンプレミス AD が停止している場合は認証ができません。

### Active Directory Federation Services

AD FS はフェデレーション認証を提供するサービスですが、オンプレミスの AD FS サーバーと Active Directory に依存します。オンプレミス環境が停止すると認証も停止します。

### Seamless Single Sign-On

Seamless SSO はユーザー体験を向上させる機能であり、認証方式そのものではありません。またオンプレミス Active Directory に依存します。

---

## 7. 最終回答

A. パスワードハッシュ同期（Password Hash Synchronization）