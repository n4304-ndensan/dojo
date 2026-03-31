[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure App Service]]
# Azure App Service HTTPS 構成設計

（カスタムドメイン + SSL自動更新）

---

# 1 背景

ある組織では、新しい Web アプリケーションを **Azure App Service** 上に構築し、インターネットに公開する予定である。Azure App Service は PaaS 型の Web アプリケーション実行環境であり、インフラ管理を Azure に任せながらアプリケーションのデプロイと運用を行うことができる。そのため、企業の Web サービスや API サービスのホスティング基盤として広く利用されている。

今回のアプリケーションは一般ユーザーや外部サービスからアクセスされるため、通信の安全性を確保することが重要である。現在の Web セキュリティの標準として、インターネット上の Web アプリケーションは **HTTPS（TLS）通信のみを許可する構成**が推奨されている。HTTPS を使用することで、ユーザーとサーバー間の通信は暗号化され、盗聴や改ざんのリスクを防ぐことができる。

また、ユーザーにとって分かりやすい URL を提供するため、Azure のデフォルトドメインではなく、企業が所有する **カスタムドメイン**（例: `www.example.com`）を使用する必要がある。このカスタムドメインで HTTPS を利用するためには SSL/TLS 証明書が必要となるが、証明書には有効期限があるため、定期的な更新が必要になる。もし証明書の更新を忘れてしまうと、ブラウザがセキュリティ警告を表示したり、接続が拒否されるなどの問題が発生する。

そのため今回のシステムでは、以下のセキュリティ要件を満たす構成を採用する必要がある。

- Web アプリケーションがカスタムドメインでアクセスできること
    
- HTTP 通信ではなく HTTPS 通信のみを許可すること
    
- SSL/TLS 証明書の更新が自動的に行われること
    

これらの要件を満たす最適な Azure の機能が **App Service Managed Certificate（App Service 管理証明書）**である。

---

# 2 Azure App Service Managed Certificate

App Service Managed Certificate は、Azure App Service に組み込まれている **無料の SSL/TLS 証明書管理機能**である。この機能を利用すると、カスタムドメインに対して Azure が自動的に証明書を発行し、App Service にバインドして HTTPS 通信を有効にすることができる。

通常、SSL 証明書は認証局（CA）から購入し、サーバーにインストールし、有効期限が近づくたびに更新作業を行う必要がある。しかし Managed Certificate を利用すると、証明書の発行、インストール、更新をすべて Azure が自動的に行うため、運用負荷を大幅に削減することができる。

この機能は特に以下のようなシナリオで有効である。

- App Service 上の Web アプリケーションを HTTPS 化したい場合
    
- SSL 証明書の管理作業を自動化したい場合
    
- 追加コストをかけずに HTTPS を実装したい場合
    

Managed Certificate の主な特徴は次の通りである。

|機能|内容|
|---|---|
|無料|SSL/TLS 証明書を追加費用なしで利用可能|
|自動更新|証明書の期限が近づくと Azure が自動更新|
|App Service 統合|App Service に直接バインド可能|
|HTTPS対応|TLS 通信を簡単に有効化|

---

# 3 全体アーキテクチャ

Managed Certificate を利用した Web アプリケーションの構成は次のようになる。

```text
User
 │
 ▼
Custom Domain (www.example.com)
 │
 ▼
HTTPS (TLS)
 │
 ▼
Azure App Service
```

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure DNS]]
ユーザーはブラウザからカスタムドメインにアクセスする。DNS はそのドメインを Azure App Service に解決し、通信は HTTPS（TLS）によって暗号化される。SSL 証明書は Azure が管理しているため、運用担当者が証明書を手動で管理する必要はない。

---

# 4 カスタムドメイン設定

Azure App Service のデフォルトドメインは通常 `*.azurewebsites.net` であるが、企業の Web サイトでは独自ドメインを使用するのが一般的である。そのため、まず Azure App Service にカスタムドメインを追加する必要がある。

例えば次のようなドメインを利用する。

```
www.example.com
```

DNS では通常、CNAME レコードを設定して App Service に紐付ける。

|レコード|値|
|---|---|
|CNAME|example.azurewebsites.net|

この設定により、ユーザーが `www.example.com` にアクセスすると Azure App Service にルーティングされる。

---

# 5 HTTPS の有効化

カスタムドメインを設定した後、Managed Certificate を App Service にバインドすることで HTTPS を有効化できる。さらに、セキュリティを強化するために **HTTPS Only** 設定を有効にすることが推奨される。

HTTPS Only を有効にすると、HTTP でアクセスされた場合でも自動的に HTTPS にリダイレクトされる。

通信の流れは次のようになる。

```text
HTTP Request
      │
      ▼
Redirect
      │
      ▼
HTTPS Request
```

App Service の設定画面では以下のように設定する。

```
HTTPS Only = Enabled
```

この設定により、すべての通信が TLS によって暗号化される。

---

# 6 SSL証明書の自動更新

SSL/TLS 証明書には通常 1 年程度の有効期限がある。証明書が期限切れになるとブラウザが警告を表示し、ユーザーが Web サイトにアクセスできなくなる可能性がある。そのため通常は証明書の更新作業を定期的に行う必要がある。

Managed Certificate を利用すると、証明書のライフサイクル管理は Azure によって自動化される。

```text
Certificate Lifecycle

Issue
 │
 ▼
Install
 │
 ▼
Automatic Renewal
```

Azure は証明書の期限が近づくと自動的に新しい証明書を発行し、App Service に再バインドする。この処理はバックグラウンドで実行されるため、アプリケーションの運用担当者が更新作業を行う必要はない。

---

# 7 証明書管理フロー

Managed Certificate を利用した場合の証明書管理の流れは次のようになる。

```text
Custom Domain
      │
      ▼
Azure App Service
      │
      ▼
Managed Certificate
      │
      ▼
Automatic Renewal
```

この仕組みにより、証明書の有効期限管理や手動更新作業を完全に排除できる。

---

# 8 他の選択肢が適切でない理由

Application Gateway はレイヤー7ロードバランサとして動作し、Web Application Firewall（WAF）などの機能を提供する。しかし、単純に App Service の HTTPS を有効にするだけの目的で導入するには構成が複雑であり、追加のインフラ管理が必要になる。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Key Vault]]
Azure Key Vault を使用して証明書を管理する方法もあるが、この方法では証明書の取得や更新を別途管理する必要があり、Managed Certificate と比較すると運用負荷が増える。

証明書を手動でアップロードする方法も存在するが、この場合は証明書の期限管理や更新作業を定期的に実施する必要があるため、運用ミスのリスクが高くなる。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Front Door]]
Azure Front Door はグローバルロードバランシングや CDN 機能を提供するサービスであり、大規模な Web アプリケーションでは有効だが、単純に HTTPS を有効化するだけの用途では過剰な構成となる。

---

# 9 推奨アーキテクチャ

最もシンプルで安全な構成は次の通りである。

```text
User
 │
 ▼
DNS
 │
 ▼
Custom Domain
 │
 ▼
Azure App Service
 │
 ▼
Managed SSL Certificate
```

この構成では、ユーザーはカスタムドメインを通じて App Service にアクセスし、通信は Managed Certificate によって TLS で暗号化される。

---

# 10 メリット

この構成を採用することで、いくつかの重要なメリットが得られる。

まず、証明書の更新が完全に自動化されるため、証明書期限切れによるサービス停止リスクを防ぐことができる。また、Azure が証明書管理を行うため、運用チームが証明書を管理する必要がなくなる。

さらに、追加のインフラを導入する必要がないため、構成がシンプルでコストも抑えられる。Managed Certificate は無料で提供されているため、HTTPS を導入するための追加費用は発生しない。

セキュリティ面でも、HTTPS Only 設定を有効にすることで、すべての通信が TLS によって暗号化され、ユーザーとサーバー間の通信を安全に保護することができる。

---

# 11 まとめ

今回のシナリオでは、Azure App Service 上の Web アプリケーションに対して次の要件が求められている。

- カスタムドメインでのアクセス
    
- HTTPS 通信の強制
    
- SSL/TLS 証明書の自動更新
    

これらの要件を満たす最適な Azure 機能は **App Service Managed Certificate** である。

Managed Certificate を利用することで、HTTPS 通信を簡単に有効化できるだけでなく、証明書の発行や更新を Azure に任せることができるため、セキュリティと運用効率の両方を向上させることができる。