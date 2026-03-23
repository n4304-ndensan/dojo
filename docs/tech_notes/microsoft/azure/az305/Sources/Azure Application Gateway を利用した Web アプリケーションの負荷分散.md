---
分類: Networking
tags:
  - cloud/azure
  - cloud/azure/application-gateway
  - cloud/azure/networking
  - cloud/azure/load-balancing
  - cloud/azure/waf
  - cloud/architecture/high-availability
  - cloud/architecture/web-application
  - cloud/architecture/security
  - exam/azure/az900
---

# Azure Application Gateway を利用した Web アプリケーションの負荷分散

## 1. 背景（シナリオ）

ある企業では、Azure App Service 上で動作する Web アプリケーションを複数の Azure リージョンに展開し、高可用性を実現しようとしています。ユーザーからのリクエストを複数のバックエンドインスタンスへ適切に分散し、アプリケーションのパフォーマンスと信頼性を維持する必要があります。

さらに、このアプリケーションはインターネットに公開されているため、SQLインジェクションやクロスサイトスクリプティング（XSS）などのアプリケーション層攻撃から保護する必要があります。

そのため、ロードバランシング機能だけでなく、アプリケーションレベルのルーティングやセキュリティ機能を提供できるサービスを設計に組み込む必要があります。

## 2. 要件整理

このシナリオの要件を整理すると、次のようになります。

この設計では単なるロードバランサーではなく、アプリケーション層（Layer 7）で動作するサービスが必要になります。

- 複数リージョンに Web アプリケーションを展開  
- URL ベースのルーティングが必要  
- Cookie を使用したセッションアフィニティ  
- Web Application Firewall（WAF）による保護  
- HTTP / HTTPS トラフィックの負荷分散  

これらの要件を満たすには、Layer 7 ロードバランサーと WAF 機能を備えたサービスが必要になります。

## 3. 技術の基本概念

Azure Application Gateway は、アプリケーション層（Layer 7）で動作するロードバランサーです。HTTP/HTTPS トラフィックを処理し、Web アプリケーション向けの高度なルーティング機能を提供します。

Application Gateway には次のような機能があります。

まず、URL ベースのルーティングです。リクエストの URL パスに基づいて異なるバックエンドプールへトラフィックを送ることができます。

次に、セッションアフィニティ（Cookie-based session affinity）です。ユーザーのリクエストを同じバックエンドインスタンスにルーティングすることで、セッション状態を維持できます。

さらに、Web Application Firewall（WAF）機能が統合されています。これにより、SQL インジェクションやクロスサイトスクリプティングなどの一般的な Web 攻撃を防ぐことができます。

## 4. アーキテクチャまたは設計のポイント

このシナリオでは、Azure Application Gateway をフロントエンドのロードバランサーとして配置します。

Application Gateway はインターネットからの HTTP/HTTPS リクエストを受け取り、URL ベースのルールに従って適切なバックエンドへトラフィックをルーティングします。

例えば、`/api` パスへのリクエストは API サービスへ、`/images` パスへのリクエストは静的コンテンツサービスへルーティングすることが可能です。

さらに、Cookie を利用したセッションアフィニティを設定することで、ユーザーセッションを維持できます。

また、WAF を有効化することで、Web アプリケーションへの攻撃を検知・ブロックすることができます。

## 5. 設計判断（なぜこの構成になるか）

この問題では次の3つの機能が特に重要です。

- URL ベースのルーティング  
- Cookie によるセッションアフィニティ  
- Web Application Firewall  

Azure Application Gateway はこれらの機能をすべて提供する Layer 7 ロードバランサーです。

そのため、Web アプリケーションのトラフィック管理とセキュリティを同時に実現できる最適なサービスとなります。

## 6. 他の選択肢が誤りな理由

A. Azure Front Door  

Front Door はグローバルロードバランシングサービスですが、Cookie ベースのセッションアフィニティ機能が Application Gateway ほど直接的に提供されません。

C. Azure Load Balancer  

Azure Load Balancer は Layer 4 ロードバランサーであり、URL ベースのルーティングや WAF をサポートしていません。

D. Azure Traffic Manager  

Traffic Manager は DNS ベースのトラフィックルーティングサービスです。アプリケーションレベルのルーティングや WAF 機能は提供されません。

## 7. 最終回答

B. Azure Application Gateway

## 8. まとめ

Web アプリケーションのロードバランシングでは、単なるトラフィック分散だけでなく、アプリケーションレベルの機能が重要になる場合があります。

Azure Application Gateway は次の機能を提供します。

- URL ベースのルーティング  
- Cookie ベースのセッションアフィニティ  
- Web Application Firewall（WAF）  

これにより、安全で高可用性の Web アプリケーションアーキテクチャを構築できます。