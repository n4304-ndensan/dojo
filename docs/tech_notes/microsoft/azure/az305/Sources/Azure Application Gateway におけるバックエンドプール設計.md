---
分類: Networking
tags:
  - cloud/azure
  - cloud/azure/application-gateway
  - cloud/azure/load-balancer
  - cloud/networking/load-balancing
  - cloud/networking/layer7
  - cloud/networking/url-routing
  - cloud/architecture/web-architecture
  - exam/azure
---

# Azure Application Gateway におけるバックエンドプール設計

## 1. 背景（シナリオ）

ある組織では、Azure上に複数の仮想マシン（VM）を配置し、その上でWebアプリケーションを実行しています。このアプリケーションはユーザーからのHTTPリクエストを処理し、URLパスに応じて異なる処理を実行します。

例えば次のようなURLがあります。

- `/api`
- `/home`

このようなURLパスに応じて、Application Gatewayを使用してトラフィックを適切な仮想マシンへルーティングする必要があります。

Azure Application Gatewayはレイヤー7（アプリケーション層）のロードバランサーであり、URLパスなどのHTTPリクエスト内容を基にルーティングを行うことができます。

そのため、Application Gatewayのバックエンドプールを適切に構成することが重要になります。

## 2. 要件整理

この問題で重要な要件は次の通りです。

- Azure Application Gatewayを使用する
- 仮想マシンへトラフィックをロードバランスする
- URLパスベースのルーティング（/api や /home）
- 仮想マシンは仮想ネットワーク内に存在する
- セキュアな内部通信が必要

Application Gatewayはバックエンドプールとして複数のターゲットを登録することができます。

## 3. 技術の基本概念

Azure Application Gatewayには、トラフィックを送信する先として **Backend Pool（バックエンドプール）** を定義します。

バックエンドプールには以下のようなリソースを登録できます。

- 仮想マシン
- 仮想マシンスケールセット
- IPアドレス
- Azure Load Balancer

Application GatewayはHTTPリクエストを受け取り、設定されたルールに基づいてバックエンドプールへトラフィックを転送します。

さらに、Application Gatewayは **URLパスベースのルーティング** をサポートしており、特定のURLパスを特定のバックエンドに送ることができます。

## 4. アーキテクチャまたは設計のポイント

このシナリオでは、Application Gatewayがインターネットからトラフィックを受信し、仮想ネットワーク内部の仮想マシンへルーティングします。

その際、仮想マシンの前段に **Internal Load Balancer（ILB）** を配置する構成が推奨されます。

Internal Load Balancerを使用することで、次のメリットがあります。

まず、仮想マシンを直接インターネットに公開する必要がなくなります。これによりセキュリティが向上します。

次に、Application GatewayはILBをバックエンドプールとして利用することで、複数の仮想マシンにトラフィックを分散できます。

さらに、ILBは仮想ネットワーク内のトラフィックのみを処理するため、安全な内部通信が可能になります。

## 5. 設計判断（なぜこの構成になるか）

この問題では、Application Gatewayのバックエンドプール構成を問われています。

Internal Load Balancerを利用すると、Application GatewayはILBの内部IPを通じて仮想マシン群にトラフィックを送信できます。

この構成は以下の理由で適しています。

- VMをインターネットに公開する必要がない
- 内部ネットワークで安全にロードバランスできる
- Application GatewayとVM間の通信を簡素化できる

そのため、ILBをバックエンドとして構成するのが最適です。

## 6. 他の選択肢が誤りな理由

### A. パブリックIPアドレス

パブリックIPは通常、Application Gatewayのフロントエンド構成で使用されます。

バックエンドプールとして直接使用するものではありません。

### C. プライベートIPアドレス

仮想マシンのプライベートIPを直接バックエンドに設定することも可能ですが、この問題ではロードバランシング構成として適切ではありません。

ILBを利用することでよりスケーラブルな構成になります。

### D. Azure Front Door

Azure Front DoorはグローバルなHTTPロードバランサーです。

Application Gatewayのバックエンド構成として使用するサービスではありません。

## 7. 最終回答

Azure Application Gatewayで仮想マシンへトラフィックをルーティングするための適切なバックエンド構成は次です。

**Internal Load Balancer (ILB)**

したがって **正解は B** です。

## 8. まとめ

Azure Application Gatewayはレイヤー7ロードバランサーであり、URLパスベースのルーティングをサポートします。

バックエンド構成では、仮想マシンを直接公開するのではなく、Internal Load Balancerを使用することで安全でスケーラブルな構成を実現できます。

ILBを使用することで次のメリットがあります。

- VMのインターネット公開を防ぐ
- 内部ネットワークで安全に通信できる
- スケーラブルなロードバランシング

そのため、Application Gatewayと仮想マシン環境では **Application Gateway → Internal Load Balancer → VM** という構成がよく使用されます。