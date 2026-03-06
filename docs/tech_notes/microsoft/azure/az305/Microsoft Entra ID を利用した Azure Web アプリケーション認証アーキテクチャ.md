# Microsoft Entra ID を利用した Azure Web アプリケーション認証アーキテクチャ

（Application Registration / SSO / Device-based Access）

---

# 1 概要

企業が Azure 上に Web アプリケーションを公開する場合、ユーザー認証をどのように実装するかは非常に重要な設計ポイントである。特に企業アプリケーションでは、単純なログイン機能だけではなく、次のようなセキュリティ要件が求められることが多い。

* 企業ユーザーのみアクセス可能
* シングルサインオン（SSO）
* 企業管理デバイスからのみアクセス許可
* クラウドベースの認証

このような要件を満たすために Azure では **Microsoft Entra ID（旧 Azure Active Directory）** を利用した認証基盤を構築することが一般的である。

今回のシナリオでは次の条件が与えられている。

* Azure Web アプリケーションをインターネット公開
* 認証は **Microsoft Entra ID**
* ユーザーは **Entra ID Join された Windows 10 PC**
* ユーザーは **認証画面を表示されない**
* **会社所有の PC のみアクセス可能**

このような設計では

**Entra ID Application Registration**

を利用して Web アプリケーションを Entra ID に登録する必要がある。

基本構成

```
User
 │
 ▼
Windows 10 (Entra ID Joined)
 │
 ▼
Microsoft Entra ID
 │
 ▼
Azure Web Application
```

この構成により

* シングルサインオン
* デバイスベースアクセス制御
* クラウド認証

が実現される。

---

# 2 背景

従来の Web アプリケーションでは、アプリケーションごとに独自のユーザー管理と認証システムを構築する必要があった。

典型的な構成

```
User
 │
 ▼
Web Application
 │
 ▼
Application Database
 │
 ▼
User Authentication
```

この構成では次の問題が発生する。

* ユーザー管理の重複
* パスワード管理の負担
* セキュリティリスク
* シングルサインオンが困難

企業環境ではすでに

* Active Directory
* Microsoft Entra ID

などのディレクトリサービスが存在しているため、アプリケーションはこれらの認証基盤を利用する方が合理的である。

そのため Azure では

**Identity as a Service**

という設計思想が採用されている。

つまり

```
Application
   │
   ▼
Microsoft Entra ID
   │
   ▼
User Authentication
```

という構造になる。

---

# 3 Microsoft Entra ID Application Registration

## 概要

Application Registration は、アプリケーションを Microsoft Entra ID に登録し、Entra ID を認証プロバイダーとして利用するための仕組みである。

登録されたアプリケーションは

* OAuth2
* OpenID Connect

などのプロトコルを使用して Entra ID から認証トークンを取得できる。

基本構造

```
Application
      │
      ▼
Application Registration
      │
      ▼
Microsoft Entra ID
```

Application Registration によって

* アプリケーション ID
* 認証エンドポイント
* リダイレクト URI
* トークン発行設定

などが管理される。

---

# 4 Azure Web App 認証フロー

Azure Web アプリケーションが Entra ID を使用する場合、認証フローは次のようになる。

```
User
 │
 ▼
Azure Web App
 │
 ▼
Redirect to Entra ID
 │
 ▼
User Authentication
 │
 ▼
Token Issued
 │
 ▼
Application Access
```

ただし、今回のシナリオではユーザーが **Entra ID Join された PC** を使用しているため、Entra ID はすでにユーザーの認証状態を認識している。

そのため

```
User Access
 │
 ▼
Token Issued Automatically
 │
 ▼
Application Access
```

という **シームレス認証（SSO）** が実現される。

---

# 5 Device-based Access Control

今回の問題では

> 会社所有のコンピューターからのみアクセス

という要件も存在する。

この要件は **デバイスベースの認証制御** によって実現される。

Microsoft Entra ID では

**Device Identity**

という概念が存在する。

デバイスは次のいずれかの状態になる。

| 状態                  | 説明         |
| ------------------- | ---------- |
| Azure AD Registered | 個人デバイス     |
| Azure AD Joined     | 企業デバイス     |
| Hybrid Joined       | AD + Entra |

今回のシナリオでは

**Entra ID Joined Windows 10 PC**

が使用されている。

そのため

```
Device Identity
     │
     ▼
Microsoft Entra ID
     │
     ▼
Application Access
```

という制御が可能になる。

この仕組みは通常 **Conditional Access** と組み合わせて利用される。

---

# 6 関連 Azure サービス

Entra ID を利用した Web アプリケーション認証では、次のサービスが関係する。

| サービス                     | 役割       |
| ------------------------ | -------- |
| Microsoft Entra ID       | 認証基盤     |
| Application Registration | アプリ登録    |
| Azure App Service        | Webアプリ実行 |
| Conditional Access       | デバイス制御   |
| Entra ID Join            | デバイス管理   |

---

# 7 不正解の選択肢

## Entra ID Managed Identity

Managed Identity は

**アプリケーションが Azure リソースへアクセスするための認証**

である。

例

```
App Service
   │
   ▼
Managed Identity
   │
   ▼
Key Vault
```

つまり

**ユーザー認証ではない**

ため今回の問題には適さない。

---

## Entra ID Application Proxy

Application Proxy は

**オンプレミスアプリケーションをインターネット公開する仕組み**

である。

典型構成

```
User
 │
 ▼
Entra ID
 │
 ▼
Application Proxy
 │
 ▼
On-Prem Application
```

今回のアプリケーションは

**Azure Web App**

であるため不要。

---

## Azure Policy

Azure Policy は

**Azureリソースのガバナンス**

を目的とするサービスである。

例

* VM サイズ制限
* リージョン制御
* タグ強制

ユーザー認証とは関係しない。

---

# 8 試験シナリオの解説

問題の要件

1. Azure Web アプリケーション
2. Entra ID 認証
3. ユーザーは Entra ID Join PC
4. 認証画面なし
5. 企業デバイスのみ

この条件から

**Entra ID Application Registration**

を使用して

* Entra ID 認証
* シングルサインオン

を実装する必要がある。

そのため正解は

**Entra ID Application Registration**

となる。

---

# 9 設計指針

企業アプリケーションの認証設計では次の点を考慮する。

### 1 認証基盤

| 方式       | 用途     |
| -------- | ------ |
| Entra ID | クラウド認証 |
| AD       | オンプレ認証 |

---

### 2 SSO

| 方法                       | 用途      |
| ------------------------ | ------- |
| Seamless SSO             | 企業PC    |
| Application Registration | Web App |

---

### 3 デバイス制御

| 機能                 | 用途     |
| ------------------ | ------ |
| Conditional Access | デバイス制御 |
| Entra Join         | 企業PC   |

---

# 10 まとめ

Azure Web アプリケーションの認証では

**Microsoft Entra ID**

を利用することが推奨される。

アプリケーションは

**Application Registration**

を行うことで Entra ID と統合できる。

この構成により

* シングルサインオン
* クラウド認証
* デバイスベースアクセス

が実現される。

そのためこの問題の正解は

**Entra ID Application Registration**

となる。
