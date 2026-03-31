---
分類: Containers
tags:
  - cloud/azure
  - cloud/azure/aks
  - cloud/azure/container
  - cloud/architecture/container-orchestration
  - cloud/architecture/microservices
  - devops/docker
  - devops/kubernetes
  - exam/azure
---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Kubernetes Service (AKS)]]
# コンテナ化アプリケーションのためのマネージド Kubernetes（Azure Kubernetes Service）

## 1. 背景（シナリオ）

ある企業では、既存のアプリケーションをコンテナ化し、クラウド上で運用する計画を立てています。コンテナ技術を採用することで、アプリケーションのデプロイを効率化し、スケーラビリティやポータビリティを向上させることができます。

しかし、IT チームは Kubernetes の管理経験が限られており、Kubernetes クラスターを自分たちで構築・運用することには不安があります。Kubernetes の運用にはコントロールプレーンの管理、アップグレード、ノード管理などの専門知識が必要であり、経験が少ないチームにとっては大きな負担となる可能性があります。

また、チームは既に Docker を使用しており、Docker Compose のようなツールに慣れています。そのため、新しい環境でも既存の Docker ベースのワークフローをできるだけ活用できるソリューションが望まれています。

このような状況では、Kubernetes の運用負荷を最小限に抑えながら、コンテナオーケストレーションを実現できる Azure サービスを選択する必要があります。

---

## 2. 要件整理

この問題では、コンテナプラットフォームの選択に関して複数の条件があります。これらの条件を整理することで、適切なサービスを選択することができます。

まず、最も重要な要件は **Kubernetes を使用すること**です。企業はコンテナオーケストレーションのために Kubernetes を利用したいと考えています。

次に、運用に関する要件があります。IT チームは Kubernetes の管理経験が限られているため、インフラ管理の負担を減らす必要があります。

さらに、開発者のワークフローに関する要件があります。チームは Docker Compose のようなツールに慣れているため、既存の Docker ベースのワークフローを活用できることが望ましいです。

これらの条件をまとめると、次のような要件になります。

- Kubernetes ベースのコンテナオーケストレーション  
- Kubernetes 管理作業の最小化  
- Docker ベースのワークフローとの互換性  
- コンテナアプリケーションのスケーラブルな実行環境  

このような条件を満たす Azure サービスを選択する必要があります。

---

## 3. 技術の基本概念

### Kubernetes

Kubernetes はコンテナ化されたアプリケーションを管理するためのオーケストレーションプラットフォームです。コンテナのデプロイ、スケーリング、ロードバランシング、自己修復などの機能を提供します。

Kubernetes は非常に強力なプラットフォームですが、その運用には多くの管理作業が必要になります。例えば、クラスターの構築、コントロールプレーンの管理、アップグレード、ネットワーク設定などがあります。

そのため、多くの企業では Kubernetes を直接管理するのではなく、クラウドプロバイダーが提供する **マネージド Kubernetes サービス**を利用することが一般的です。

---

### Azure Kubernetes Service（AKS）

Azure Kubernetes Service（AKS）は、Azure が提供するマネージド Kubernetes サービスです。このサービスでは、Kubernetes のコントロールプレーンが Azure によって管理されるため、ユーザーはノードやワークロードの管理に集中することができます。

AKS を使用することで、Kubernetes クラスターの構築や管理に関する多くの作業を Azure に任せることができます。これにより、Kubernetes の経験が少ないチームでもコンテナオーケストレーションを利用することが可能になります。

AKS の主な特徴を理解するためには、その機能を整理することが重要です。

AKS は次のような機能を提供します。

- Kubernetes コントロールプレーンの完全管理  
- 自動スケーリング  
- Azure ネットワークとの統合  
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Monitor]]
- Azure Monitor による監視  

これにより、企業は Kubernetes の利点を活用しながら運用負荷を大幅に削減できます。

---

## 4. アーキテクチャまたは設計のポイント

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Container Registry]]
AKS を使用する場合、コンテナイメージは通常 Azure Container Registry や Docker Hub などのレジストリに保存されます。Kubernetes クラスターはこれらのイメージを取得してコンテナを実行します。

また、Docker Compose を使用しているチームでも Kubernetes への移行は比較的容易です。Docker Compose の設定ファイルを Kubernetes マニフェストに変換するツールが存在するためです。

例えば、Kompose というツールを使用すると、Docker Compose ファイルを Kubernetes の YAML マニフェストに変換できます。これにより、既存の Docker ベースのワークフローを維持しながら Kubernetes 環境へ移行できます。

このような仕組みにより、Docker に慣れた開発チームでも Kubernetes 環境を比較的スムーズに利用できるようになります。

---

## 5. 設計判断（なぜこの構成になるか）

この問題では **マネージド Kubernetes の提供**が重要な要件です。

AKS は Kubernetes コントロールプレーンを Azure が管理するため、ユーザーはクラスターのメンテナンスやアップグレードを気にする必要がありません。そのため Kubernetes の経験が限られているチームでも利用しやすいという利点があります。

さらに、AKS は標準の Kubernetes を使用しているため、Docker ベースのツールや Kubernetes エコシステムをそのまま活用できます。

これらの理由から、Azure Kubernetes Service はこのシナリオに最も適したサービスとなります。

---

## 6. 他の選択肢が誤りな理由

### Azure Container Instances

ACI はコンテナを簡単に実行できるサーバーレスサービスですが、Kubernetes のオーケストレーション機能は提供していません。そのため Kubernetes を使用するという要件を満たしません。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure App Service]]
### Azure App Service with Containers

App Service はコンテナ化された Web アプリケーションをホストすることができますが、Kubernetes ベースのオーケストレーション環境ではありません。

### Azure Red Hat OpenShift

Azure Red Hat OpenShift は Kubernetes ベースのプラットフォームですが、OpenShift 固有のエコシステムを使用します。標準 Kubernetes 環境を求める場合は AKS の方がシンプルです。

---

## 7. 最終回答

A. Azure Kubernetes Service（AKS）