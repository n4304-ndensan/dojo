---
分類: Cloud
tags:
  - cloud/azure
  - cloud/azure/aks
  - cloud/azure/api-management
  - cloud/azure/api-management/premium
  - cloud/azure/active-directory
  - cloud/azure/application-gateway
  - cloud/azure/front-door
  - cloud/security
  - cloud/security/mtls
  - cloud/security/api-security
  - cloud/security/rate-limiting
  - cloud/networking/vnet
  - cloud/architecture/microservices
  - cloud/architecture/api-gateway
  - exam/azure/architecture
---

# AKS マイクロサービスのセキュアアクセス管理（Azure API Management Premium + Azure AD）

## 1. 背景（シナリオ）

企業は Azure Kubernetes Service（AKS）上でマイクロサービスアーキテクチャを構築しています。このアーキテクチャでは、複数のマイクロサービスが API として公開され、クライアントアプリケーションや外部サービスからアクセスされます。

しかし、マイクロサービスをインターネットに公開する場合、セキュリティ対策を慎重に設計する必要があります。特に、許可されたクライアントのみがアクセスできるようにするアクセス制御、通信内容の暗号化、そして不正利用を防ぐためのリクエスト制御が重要になります。

さらに、この環境ではユーザー認証やアクセス管理を Azure Active Directory（Azure AD）と統合する必要があります。つまり、API アクセス制御、ネットワーク制御、認証認可を一体化したセキュアな API 管理基盤が必要になります。

## 2. 要件整理

この問題では、マイクロサービスアーキテクチャにおけるセキュリティ要件が明確に示されています。これらの要件を整理すると、次のようになります。

まず、アクセス制御として、指定された IP アドレスのみがマイクロサービスにアクセスできるようにする必要があります。つまり、IP ホワイトリストによるネットワークレベルのアクセス制御が求められています。

次に、クライアントとマイクロサービス間の通信は、相互 TLS（mTLS）によって暗号化と認証が行われなければなりません。これは、クライアントとサーバー双方が証明書を使って認証するセキュア通信方式です。

さらに、クライアント ID に基づくレート制限が必要です。これは、特定のクライアントが API を過剰に利用することを防ぎ、サービスの公平な利用を保証するための仕組みです。

最後に、ユーザー認証とアクセス管理を Azure Active Directory と統合する必要があります。

これらの要件をまとめると、以下の機能が必要になります。

- IP ベースのアクセス制御  
- 相互 TLS（mTLS）による安全な通信  
- クライアント ID に基づくレート制限  
- Azure Active Directory による認証と認可  
- マイクロサービス API の集中管理  

## 3. 技術の基本概念

マイクロサービスアーキテクチャでは、多数の API を安全かつ効率的に管理するために API Gateway が利用されます。API Gateway はクライアントとバックエンドサービスの間に配置され、認証、アクセス制御、トラフィック制御などの機能を提供します。

Azure API Management（APIM）は、Azure が提供する API Gateway サービスです。APIM を使用すると、API の公開、認証、監視、トラフィック制御を一元的に管理できます。

また、API 管理ではレート制限や IP フィルタリングなどのポリシーを設定できるため、セキュリティとトラフィック管理を同時に実現できます。

さらに、API Management は Azure Active Directory と統合できるため、OAuth2 や OpenID Connect を使用した認証を実装することが可能です。これにより、ユーザー ID やアクセス権限を Azure AD で一元管理できます。

## 4. アーキテクチャまたは設計のポイント

このシナリオでは、AKS 上のマイクロサービスの前段に API Gateway を配置するアーキテクチャが適しています。Azure API Management を使用することで、API アクセスのセキュリティと管理を統合できます。

このアーキテクチャで重要な設計ポイントは次の通りです。

まず、Azure API Management Premium を API Gateway として配置します。クライアントは直接 AKS にアクセスするのではなく、APIM を経由して API を呼び出します。

次に、IP フィルタリングポリシーを設定し、指定された IP アドレスのみが API にアクセスできるようにします。

さらに、mTLS を使用してクライアント証明書を検証し、安全な通信とクライアント認証を実現します。

また、レート制限ポリシーを設定することで、クライアント ID に基づいて API 呼び出し回数を制御できます。

最後に、Azure Active Directory と統合することで、ユーザー認証とアクセス制御を中央管理できます。

## 5. 設計判断（なぜこの構成になるか）

この問題の正解は **B. 仮想ネットワーク接続と Azure Active Directory 連携を備えた Azure API Management Premium** です。

Azure API Management は API Gateway として機能し、マイクロサービスのアクセス管理を一元化できます。特に Premium 層では、仮想ネットワーク統合が可能であり、ネットワークレベルのセキュリティを強化できます。

また、APIM ではポリシー機能を使用して IP フィルタリングやレート制限を簡単に実装できます。これにより、指定された IP アドレスのみアクセス可能にしつつ、クライアントごとの利用制御を行うことができます。

さらに、Azure API Management Premium は mTLS をサポートしているため、クライアント証明書による相互認証を実現できます。これにより、通信の機密性とデータ整合性を確保できます。

加えて、Azure Active Directory と統合することで、ユーザー認証やアクセス権限の管理を Azure AD に委任できます。

このように、API 管理、セキュリティ、認証、トラフィック制御を統合的に提供できるサービスは Azure API Management Premium です。

## 6. 他の選択肢が誤りな理由

他の選択肢も Azure のネットワークサービスですが、API 管理やセキュリティ機能が十分ではありません。

Azure Application Gateway は L7 ロードバランサーであり、Web アプリケーションのトラフィック制御や WAF 機能を提供します。しかし、API レート制限や API 管理機能は限定的であり、マイクロサービス API の管理には適していません。

Azure Front Door はグローバルロードバランサーであり、複数リージョンのトラフィック分散やアプリケーションアクセラレーションを提供します。しかし、API 管理やクライアント ID に基づくレート制限などの機能は APIM ほど充実していません。

また、Azure API Management Standard 層は API 管理機能を提供しますが、Premium 層で利用できる仮想ネットワーク統合などの高度なセキュリティ機能が含まれていません。そのため、厳格なネットワーク制御が必要なこのシナリオには適していません。

## 7. 最終回答

B. 仮想ネットワーク接続と Azure Active Directory 連携を備えた Azure API Management Premium 層

## 8. まとめ

この問題は、Azure におけるマイクロサービスセキュリティアーキテクチャを理解しているかを確認する問題です。マイクロサービス環境では、API Gateway を中心としたセキュリティ設計が重要になります。

Azure API Management は、API の公開、認証、レート制限、IP フィルタリング、分析などの機能を提供する API 管理サービスです。特に Premium 層では仮想ネットワーク統合が可能であり、企業レベルのセキュリティ要件にも対応できます。

また、Azure Active Directory と統合することで、ユーザー認証やアクセス権限の管理を統合的に行うことができます。

したがって、AKS 上のマイクロサービスに対して厳格なアクセス制御、mTLS、レート制限、Azure AD 認証を実現する最適なソリューションは Azure API Management Premium です。