---
分類: Cloud
tags:
  - cloud/azure
  - cloud/azure/identity
  - cloud/azure/azure-ad
  - cloud/azure/ad-connect
  - security/identity
  - security/sso
  - security/authentication
  - cloud/architecture/hybrid-identity
  - exam/azure
---

# Azure AD Connect Pass-through Authentication によるハイブリッドSSO

## 1. 背景（シナリオ）

ある組織では、複数のオンプレミス Active Directory フォレストを含む複雑な IT 環境を運用しています。  
この組織はクラウドサービスの導入を進めており、Azure 上のアプリケーションとオンプレミスアプリケーションの両方を利用しています。

ユーザー体験を向上させるために、組織は **シームレスなシングルサインオン（SSO）** を実現する必要があります。

この環境では、ユーザーが次のような体験を得られることが求められています。

- オンプレミスの資格情報でログイン
- クラウドアプリケーションへ自動アクセス
- 追加ログインプロンプトなし
- ユーザー操作の最小化

つまり、オンプレミス AD と Azure AD を統合した **ハイブリッド ID アーキテクチャ** を構築する必要があります。

---

## 2. 要件整理

このシナリオの重要な要件を整理すると、次のポイントが挙げられます。

まず、認証に関する要件です。

- オンプレミス Active Directory を利用
- Azure AD と統合
- シングルサインオン

次に、ユーザー体験の要件です。

- 追加のサインインプロンプトなし
- 既存資格情報の利用
- シームレス認証

最後に、環境に関する要件です。

- 複数の AD フォレスト
- ハイブリッドクラウド環境

このことから、**オンプレミス認証を利用した Azure AD 認証方式**が必要になります。

---

## 3. 技術の基本概念

Azure では、オンプレミス Active Directory とクラウドの Azure AD を統合するために **Azure AD Connect** が提供されています。

Azure AD Connect は、オンプレミス ID とクラウド ID を同期するためのツールです。

Azure AD Connect では、次の認証方式を選択できます。

まず、最も一般的な方式です。

- **Pass-through Authentication (PTA)**  
  認証をオンプレミス AD で実行

次に、クラウド認証方式です。

- **Password Hash Synchronization (PHS)**  
  パスワードハッシュを Azure AD に同期

最後に、フェデレーション方式です。

- **AD FS Federation**

これらの中で、**Pass-through Authentication (PTA)** はオンプレミス認証をリアルタイムで使用する方式です。

---

## 4. アーキテクチャまたは設計のポイント

Azure AD Connect と Pass-through Authentication を使用すると、次のような認証フローになります。

まず、ユーザーは Azure AD にログインします。

次に、Azure AD は認証リクエストを **オンプレミス AD に転送**します。

このとき、Azure AD Connect の PTA エージェントが認証処理を実行します。

この構成の特徴を整理すると次の通りです。

- パスワードはクラウドに保存されない
- 認証はオンプレミス AD で実行
- Azure AD とリアルタイム認証
- シームレス SSO

この仕組みにより、ユーザーは企業ネットワークにログインした状態でクラウドアプリケーションへアクセスできます。

---

## 5. 設計判断（なぜこの構成になるか）

この問題では **ユーザー操作を最小限にするシームレス SSO** が重要です。

Azure AD Connect with Pass-through Authentication を使用すると、次のメリットがあります。

まず、ユーザー体験です。

- 追加ログイン不要
- シングルサインオン
- 既存資格情報の利用

次に、セキュリティです。

- パスワードをクラウドに保存しない
- 認証はオンプレミス AD

さらに、運用面です。

- AD FS のような複雑なインフラが不要
- シンプルな構成

そのため、ハイブリッド ID 環境において非常に一般的な構成となります。

---

## 6. 他の選択肢が誤りな理由

まず **Azure AD Federation（AD FS）** です。

AD FS は SSO を提供できますが、次のようなデメリットがあります。

- 追加インフラが必要
- 運用管理が複雑
- フェデレーションサーバー管理

この問題では **ユーザー操作を最小限にし、シンプルな構成**が求められるため最適ではありません。

次に **Azure AD Domain Services** です。

Azure AD DS はマネージドドメインサービスですが、オンプレミス AD フォレストの SSO 統合用途には適していません。

最後に **Azure AD B2B** です。

B2B は外部ユーザー（パートナーなど）とのコラボレーション用サービスです。

内部ユーザーの認証には使用されません。

---

## 7. 最終回答

**A. Azure AD Connect with Pass-through Authentication**

---

## 8. まとめ

Azure のハイブリッド ID 設計では、認証方式の違いを理解することが重要です。

主な方式を整理すると次の通りです。

まず、クラウド認証です。

- Password Hash Sync  
  → パスワードハッシュを Azure AD に同期

次に、オンプレミス認証です。

- **Pass-through Authentication**  
  → 認証はオンプレミス AD

最後に、フェデレーションです。

- AD FS  
  → フェデレーションベース認証

このため、**オンプレミス AD を利用しながらシームレス SSO を実現する場合は Azure AD Connect with Pass-through Authentication を使用するのが最適なソリューション**になります。