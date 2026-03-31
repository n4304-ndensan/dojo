---
分類: Cloud
tags:
  - cloud/azure
  - cloud/azure/aks
  - cloud/azure/event-grid
  - cloud/azure/functions
  - cloud/architecture/event-driven
  - cloud/architecture/autoscaling
  - cloud/kubernetes
  - cloud/kubernetes/keda
  - cloud/kubernetes/hpa
  - exam/azure/az900
---

# AKS で Event Grid イベントを処理するためのスケーリング構成

## 1. 背景（シナリオ）

あるシステムでは、Azure Event Grid からのイベントを処理する Azure Functions アプリケーションが存在します。このアプリは現在、Azure Functions の消費プランで実行されており、イベントの発生量に応じて自動的にスケーリングされます。

しかし、今後このアプリケーションを Azure Kubernetes Service (AKS) クラスターへ移行する計画があります。そのため、AKS 環境でも Event Grid からのイベントを効率よく処理できるように、適切なスケーリング構成を設定する必要があります。

特に重要なのは、イベント量に応じて自動的にポッド数を増減させることです。イベント駆動型アーキテクチャでは、イベント数が急増する可能性があるため、Kubernetes 環境でも柔軟なスケーリング機能が必要になります。

## 2. 要件整理

このシナリオの要件を整理すると、次のようなポイントが重要になります。

このアーキテクチャでは Azure Functions の消費プランのような自動スケーリング挙動を Kubernetes 上で再現する必要があります。

- Azure Functions アプリを AKS に移行する  
- Azure Event Grid のイベントを処理する  
- イベント量に応じて自動的にスケーリングする  
- Kubernetes 上で効率的にリソースを管理する  

このような要件から、イベント駆動型のスケーリング機能と Kubernetes の標準スケーリング機能の両方を組み合わせる必要があります。

## 3. 技術の基本概念

AKS でイベント駆動型ワークロードを処理する場合、主に次の2つの仕組みが重要になります。

まず Kubernetes Event-driven Autoscaling（KEDA）です。KEDA はイベント駆動型アプリケーションのための Kubernetes オートスケーリングコンポーネントです。キューの長さやイベント数などの外部イベントに基づいてポッド数をスケーリングできます。Azure Event Grid や Azure Service Bus などのイベントソースと統合できる点が特徴です。

次に Horizontal Pod Autoscaler（HPA）です。HPA は Kubernetes の標準機能であり、CPU 使用率やカスタムメトリクスに基づいてポッド数を自動調整します。アプリケーションの負荷が高くなるとポッド数を増やし、負荷が下がるとスケールダウンします。

## 4. アーキテクチャまたは設計のポイント

このシナリオでは、KEDA と HPA を組み合わせて使用する構成が適しています。

まず KEDA を有効化することで、Azure Event Grid のイベント数に基づいて Kubernetes ワークロードをスケーリングできます。イベントが増えるとポッド数が自動的に増加し、イベントが減るとポッド数が減少します。

さらに HPA を設定することで、CPU やメモリなどのリソース使用率に基づいた追加のスケーリングが可能になります。これにより、アプリケーションが高負荷状態になった場合でも安定して処理を継続できます。

このように、イベントベースのスケーリングとリソースベースのスケーリングを組み合わせることで、柔軟で効率的なオートスケーリングを実現できます。

## 5. 設計判断（なぜこの構成になるか）

この問題では「Event Grid のイベントを処理する」「Azure Functions 消費プランのような自動スケーリングを実現する」という要件が重要です。

KEDA はイベント数をトリガーにして Kubernetes ワークロードをスケーリングできるため、イベント駆動型アーキテクチャに最適です。

一方で HPA は CPU やメモリ使用率などのリソースメトリクスに基づいてスケーリングを実行します。

この2つを組み合わせることで、イベント量とシステム負荷の両方に対応した柔軟なスケーリングが可能になります。

## 6. 他の選択肢が誤りな理由

A. Azure Application Gateway  

Application Gateway はロードバランサーおよび Web アプリケーションファイアウォール機能を提供しますが、イベント処理やスケーリング機能とは直接関係ありません。

D. Azure Virtual WAN  

Virtual WAN はネットワーク接続を管理するサービスであり、AKS のイベント処理やオートスケーリングには関係しません。

## 7. 最終回答

B. Kubernetes Event-driven Autoscaling（KEDA）を有効にする  
C. Horizontal Pod Autoscaler（HPA）を設定する

## 8. まとめ

Azure Functions を AKS に移行する場合、イベント駆動型のスケーリングを Kubernetes 環境で実現する必要があります。

そのためには次の2つの構成が重要になります。

- KEDA（イベントベースのスケーリング）  
- HPA（リソースベースのスケーリング）

この2つを組み合わせることで、Azure Event Grid のイベント量やシステム負荷に応じて効率的にポッドをスケーリングできるようになります。