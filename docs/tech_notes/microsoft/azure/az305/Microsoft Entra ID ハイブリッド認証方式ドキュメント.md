# Microsoft Entra ID ハイブリッド認証アーキテクチャ

（PHS / PTA / Seamless SSO / AD FS）

---

# 1 概要

企業がクラウドサービスを導入する際、多くの場合すでに **オンプレミスの Active Directory (AD)** を運用している。
このような環境では、既存のユーザー管理を維持しながらクラウドサービスを利用するために **Microsoft Entra ID（旧 Azure Active Directory）との統合**が必要になる。

Microsoft Entra ID は Azure や Microsoft 365、その他 SaaS アプリケーションの認証基盤として機能するクラウドディレクトリサービスである。しかし、多くの企業ではユーザー情報がすでにオンプレミスの Active Directory に存在しているため、両者を統合して一貫した認証基盤を構築する必要がある。

この統合を実現するための主要なツールが **Microsoft Entra Connect（旧 Azure AD Connect）** である。
Entra Connect を利用することで、オンプレミスの AD とクラウドの Entra ID の間でユーザー情報を同期し、企業ユーザーが同じ資格情報を使用してクラウドサービスにアクセスできるようになる。

基本的な統合構造は次のようになる。

```
On-Premises Active Directory
        │
        │  ID同期
        ▼
Microsoft Entra Connect
        │
        ▼
Microsoft Entra ID
        │
        ▼
Microsoft 365 / Azure / SaaS
```

この統合の主な目的は次の3つである。

* ユーザー ID の同期
* 認証基盤の統合
* シングルサインオン（SSO）の実現

特に企業環境では、ユーザーが複数回パスワードを入力することなくシステムへアクセスできるようにすることが重要であり、そのための仕組みとして **Seamless Single Sign-On (Seamless SSO)** が提供されている。

---

# 2 背景

企業の IT 環境は、従来はオンプレミス中心で構築されていた。
Active Directory はその中核であり、ユーザー認証やアクセス制御を担う重要なディレクトリサービスとして広く利用されてきた。

しかし近年では、次のような理由からクラウドサービスの利用が急速に拡大している。

* Microsoft 365 の導入
* Azure の利用
* SaaS アプリケーションの普及
* リモートワークの増加

これにより企業は **オンプレミスとクラウドが混在するハイブリッド環境** を運用することになった。

このような環境では次の問題が発生する。

1. ユーザーアカウントが複数存在する
2. パスワード管理が複雑になる
3. ログイン操作が煩雑になる

例えば次のような状況が起こり得る。

```
PCログイン → AD認証
Microsoft 365 → 別認証
社内アプリ → 別認証
```

ユーザーは同じ日に何度もパスワードを入力することになる。

この問題を解決するために導入されるのが **シングルサインオン（SSO）** である。

特に企業ネットワーク内のドメイン参加 PC を使用している場合、ユーザーは PC にログインした時点で認証が完了しているため、クラウドサービスにアクセスする際に再度パスワードを入力する必要はない。
この体験を実現する技術が **Seamless Single Sign-On** である。

---

# 3 ハイブリッド認証モデル

Entra ID とオンプレミス AD を統合する際のアーキテクチャは、次の2つの要素で構成される。

1. **ID同期（Identity Synchronization）**
2. **認証方式（Authentication Method）**

概念的には次のような構造になる。

```
Active Directory
      │
      ▼
Identity Synchronization
      │
      ▼
Microsoft Entra ID
      │
      ▼
Authentication Method
```

ID同期によってユーザーアカウントがクラウドにコピーされ、その後どこで認証を行うかによって認証方式が決定される。

Microsoft Entra ID では、主に次の3つの認証方式が提供されている。

| 認証方式                                         | 概要                |
| -------------------------------------------- | ----------------- |
| Password Hash Synchronization (PHS)          | パスワードハッシュをクラウドへ同期 |
| Pass-through Authentication (PTA)            | 認証をオンプレミスADで実行    |
| Active Directory Federation Services (AD FS) | フェデレーション認証        |

---

# 4 Seamless Single Sign-On (SSO)

Seamless SSO は、企業ネットワーク内のドメイン参加 PC からクラウドサービスにアクセスする際に **ユーザーがパスワードを再入力しなくてもログインできる仕組み** である。

ユーザーの体験は非常にシンプルである。

```
PCログイン
↓
Microsoft 365 アクセス
↓
自動ログイン
```

この仕組みは **Kerberos 認証** を利用して実現される。

```
User
 │
 ▼
Domain Joined PC
 │
 ▼
Kerberos Ticket
 │
 ▼
Microsoft Entra ID
```

Seamless SSO は **PHS または PTA と組み合わせて使用する**。

---

# 5 Password Hash Synchronization (PHS)

PHS は最もシンプルな認証方式であり、Microsoft が最も推奨している方式である。

この方式では、オンプレミス Active Directory のパスワードハッシュが Entra ID に同期される。
ただし、同期されるのはパスワードそのものではなく **ハッシュ化された値の再ハッシュ**である。

```
Active Directory
     │
     ▼
Password Hash
     │
     ▼
Hash Sync
     │
     ▼
Microsoft Entra ID
```

認証はクラウド側で行われる。

```
User Login
     │
     ▼
Microsoft Entra ID
```

PHS の特徴は次の通りである。

| 項目       | 内容                 |
| -------- | ------------------ |
| 認証場所     | Microsoft Entra ID |
| オンプレミス依存 | なし                 |
| 可用性      | 高い                 |
| 構成       | シンプル               |

メリット

* クラウドのみで認証可能
* 高可用性
* 運用が簡単

デメリット

* パスワードハッシュがクラウドに保存される

---

# 6 Pass-through Authentication (PTA)

PTA は **クラウドにパスワードを保存したくない企業向け** の方式である。

この方式では、ユーザーのログイン要求がクラウドからオンプレミス AD に転送され、AD がパスワードを検証する。

認証フローは次のようになる。

```
User Login
     │
     ▼
Microsoft Entra ID
     │
     ▼
PTA Agent
     │
     ▼
Active Directory
```

PTA Agent はオンプレミスにインストールされるサービスであり、Entra ID と AD の間で認証要求を仲介する。

特徴

| 項目         | 内容       |
| ---------- | -------- |
| 認証場所       | オンプレミスAD |
| パスワード保存    | クラウド保存なし |
| セキュリティポリシー | ADポリシー適用 |

メリット

* パスワードがクラウドに保存されない
* ADポリシーを維持できる

デメリット

* オンプレミス依存
* PTA Agentが必要

---

# 7 AD FS

AD FS はフェデレーション認証方式であり、企業が完全に自社の認証基盤を管理したい場合に使用される。

```
User
 │
 ▼
Microsoft Entra ID
 │
 ▼
AD FS
 │
 ▼
Active Directory
```

AD FS は非常に柔軟な認証ポリシーを実装できるが、次のようなデメリットがある。

* インフラ構築が必要
* 運用コストが高い
* 構成が複雑

そのため現在では **PHS または PTA が推奨されるケースが多い**。

---

# 8 認証方式比較

| 項目        | PHS      | PTA    | AD FS   |
| --------- | -------- | ------ | ------- |
| 認証場所      | Entra ID | オンプレAD | AD FS   |
| クラウドパスワード | 保存あり     | 保存なし   | 保存なし    |
| インフラ      | 最小       | 中      | 多       |
| 可用性       | 高        | AD依存   | AD FS依存 |

---

# 9 試験シナリオ

問題

> 企業ネットワーク内の企業マシンを使用する際に、ユーザーが一度もパスワードを入力する必要がないようにする

この要求は

**Seamless SSO**

を意味している。

Seamless SSO は次の認証方式と組み合わせて利用できる。

* Password Hash Synchronization (PHS)
* Pass-through Authentication (PTA)

そのため正解は

| 選択肢 | 理由                |
| --- | ----------------- |
| PHS | Seamless SSOと併用可能 |
| PTA | Seamless SSOと併用可能 |

---

# 10 不正解の選択肢

### AD FS

AD FS はフェデレーション認証方式であり、
この問題が求めている **同期方式** とは異なる。

### OpenID Connect

OpenID Connect は

**認証プロトコル**

であり

* AD同期
* Entra ID統合

とは関係がない。

---

# 11 設計指針

Microsoft の推奨構成

| シナリオ           | 推奨方式  |
| -------------- | ----- |
| シンプル構成         | PHS   |
| クラウドにパスワード保存不可 | PTA   |
| 高度な認証ポリシー      | AD FS |

---

# 12 まとめ

オンプレミス Active Directory と Microsoft Entra ID の統合では、次の3つの認証方式が存在する。

| 認証方式  | 特徴         |
| ----- | ---------- |
| PHS   | クラウド認証     |
| PTA   | オンプレミス認証   |
| AD FS | フェデレーション認証 |

企業ネットワーク内のドメイン参加 PC から **パスワード入力なしでアクセスする要件** は

**Seamless Single Sign-On**

によって実現される。

この機能は

* **PHS**
* **PTA**

と組み合わせて利用できるため、試験問題の正解は

**PHS と PTA**

となる。
