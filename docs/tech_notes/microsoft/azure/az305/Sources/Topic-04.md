# Topic-04 AKS とコンテナプラットフォーム

## 学習ゴール

AKS の基盤、入口、ストレージ、セキュリティとリリースを順にたどり、Kubernetes 基盤の全体像を掴む。

## この Topic の全体像

AKS、Ingress、コンテナ レジストリ、Kubernetes ストレージ、リリース戦略を再編成する。

対象ドキュメント数: 9 件

## 第1章 学習マップ

### 1.1 学習順序

1. AKS 基盤とスケーリング: クラスター基盤、ノード、スケーリング、リージョン耐性を入口として理解する。
2. 入口制御とマイクロサービス: Ingress、Dapr、Istio を通してマイクロサービス通信を整理する。
3. ストレージとステートフル設計: Azure Files を中心にステートフル構成の要件を確認する。
4. セキュリティ、認証、リリース: ACR、認証、ガバナンス、継続的デプロイをまとめて見る。

### 1.2 セクション対応表

- AKS 基盤とスケーリング: 3 件 / [[AKS を使用したリージョン障害耐性のある Kubernetes アーキテクチャ.md]] / [[Azure Kubernetes Service (AKS) スケーリング設計.md]] / [[コンテナ化アプリケーションのためのマネージド Kubernetes（Azure Kubernetes Service）.md]]
- 入口制御とマイクロサービス: 1 件 / [[AKS マイクロサービスアーキテクチャ設計ガイド.md]]
- ストレージとステートフル設計: 2 件 / [[AKS ステートフルアプリケーションの永続ストレージ（Azure Files RWX）.md]] / [[AKS ステートフルアプリケーションの共有ストレージ設計.md]]
- セキュリティ、認証、リリース: 3 件 / [[AKS マルチリージョンアプリケーションの安全なリリース戦略.md]] / [[Azure Container Registry SKU 比較ドキュメント.md]] / [[Azure DevOps Pipeline を利用した AKS への継続的デプロイ（ACR イメージ更新トリガー）.md]]

## 第2章 基礎概念と構成要素

### 2.1 AKS 基盤とスケーリング

クラスター基盤、ノード、スケーリング、リージョン耐性を入口として理解する。

主な出典: [[AKS を使用したリージョン障害耐性のある Kubernetes アーキテクチャ.md]] / [[Azure Kubernetes Service (AKS) スケーリング設計.md]] / [[コンテナ化アプリケーションのためのマネージド Kubernetes（Azure Kubernetes Service）.md]]

主要論点: Azure Kubernetes Service（AKS） / Azure Traffic Manager / アーキテクチャまたは設計のポイント / A 可用性ゾーンに分散した単一クラスター / B 単一ゾーンの AKS クラスター / D 複数リージョンにまたがる単一クラスター / Kubernetesのスケーリングレイヤ / 主なスケーリング機能 / Horizontal Pod Autoscaler (HPA) / 例

### 2.2 入口制御とマイクロサービス

Ingress、Dapr、Istio を通してマイクロサービス通信を整理する。

主な出典: [[AKS マイクロサービスアーキテクチャ設計ガイド.md]]

主要論点: ― Dapr と Istio を利用した分散アプリケーション機能の実装 ― / AKS におけるマイクロサービス構成 / Dapr（Distributed Application Runtime） / Dapr による Pub/Sub メッセージング / Istio（Service Mesh） / Istio によるトラフィックルーティング / Flux / Dapr / 完成アーキテクチャ / マイクロサービス機能の整理

### 2.3 ストレージとステートフル設計

Azure Files を中心にステートフル構成の要件を確認する。

主な出典: [[AKS ステートフルアプリケーションの永続ストレージ（Azure Files RWX）.md]] / [[AKS ステートフルアプリケーションの共有ストレージ設計.md]]

主要論点: Kubernetes 永続ストレージの基本 / アクセスモードの概念 / Azure Files の特徴 / なぜ Azure Files（RWX）が適切なのか / Azure Disk（ReadWriteOnce） / ポッドの静的 IP / コンテナローカルエフェメラルストレージ / Kubernetes における永続ストレージ / Kubernetes のアクセスモード / Azure Files

### 2.4 セキュリティ、認証、リリース

ACR、認証、ガバナンス、継続的デプロイをまとめて見る。

主な出典: [[AKS マルチリージョンアプリケーションの安全なリリース戦略.md]] / [[Azure Container Registry SKU 比較ドキュメント.md]] / [[Azure DevOps Pipeline を利用した AKS への継続的デプロイ（ACR イメージ更新トリガー）.md]]

主要論点: Deployment Rings（展開リング） / マルチリージョン環境での展開 / Azure DevOps における展開リング / Approval Gates / 自動ロールバック / AKS との統合 / 他の展開戦略との比較 / Blue-Green Deployment / Canary Deployment / Rolling Update

## 第3章 設計判断の軸

### 3.1 AKS 基盤とスケーリング

- しかし、この企業が構築しようとしているシステムはビジネスクリティカルなサービスであり、非常に高い可用性が求められています。そのため、単なるノードレベルの冗長化だけではなく、**Azure データセンター全体の障害にも耐えられる構成**が必要です。 ([[AKS を使用したリージョン障害耐性のある Kubernetes アーキテクチャ.md]])
- また、運用面での要件もあります。企業は Kubernetes の制御プレーンを自分で管理することを避けたいと考えており、管理作業を可能な限り少なくしたいと考えています。つまり、フルマネージドの AKS のメリットを最大限に活用する必要があります。 ([[AKS を使用したリージョン障害耐性のある Kubernetes アーキテクチャ.md]])
- このような条件を満たす Kubernetes アーキテクチャを設計する必要があります。 ([[AKS を使用したリージョン障害耐性のある Kubernetes アーキテクチャ.md]])
- この問題では、AKS クラスター設計に関する複数の要件があります。これらを整理することで最適な構成が見えてきます。 ([[AKS を使用したリージョン障害耐性のある Kubernetes アーキテクチャ.md]])
- まず、可用性に関する要件があります。企業は単一のデータセンター障害だけでなく、Azure のリージョン全体の障害にも耐えられるシステムを構築する必要があります。 ([[AKS を使用したリージョン障害耐性のある Kubernetes アーキテクチャ.md]])
- 次に、Kubernetes 管理に関する要件があります。企業は Kubernetes の制御プレーン管理をできるだけ Azure に任せたいと考えています。 ([[AKS を使用したリージョン障害耐性のある Kubernetes アーキテクチャ.md]])

### 3.2 入口制御とマイクロサービス

- 今回の問題では、AKS 上のマイクロサービスアプリケーションに対して次の2つの機能を実装する必要がある。 ([[AKS マイクロサービスアーキテクチャ設計ガイド.md]])
- これらの要件を満たすために適切な技術は次の通りである。 ([[AKS マイクロサービスアーキテクチャ設計ガイド.md]])
- Kubernetes やコンテナ環境で動作し、マイクロサービスに必要な共通機能を提供する。 ([[AKS マイクロサービスアーキテクチャ設計ガイド.md]])
- のようなイベント駆動型処理が必要になる。 ([[AKS マイクロサービスアーキテクチャ設計ガイド.md]])
- Dapr は内部で次のようなメッセージング基盤と接続できる。 ([[AKS マイクロサービスアーキテクチャ設計ガイド.md]])
- そのため **Pub/Sub メッセージング機能を実装する場合は Dapr が推奨される**。 ([[AKS マイクロサービスアーキテクチャ設計ガイド.md]])

### 3.3 ストレージとステートフル設計

- Azure Kubernetes Service（AKS）では、多くのアプリケーションがコンテナとしてデプロイされます。Kubernetes のコンテナは通常ステートレスに設計されますが、実際のアプリケーションではデータを保持する必要がある **ステートフルアプリケーション**も存在します。 ([[AKS ステートフルアプリケーションの永続ストレージ（Azure Files RWX）.md]])
- このようなアプリケーションでは **ポッドが再起動してもデータが保持される永続ストレージ**が必要になります。 ([[AKS ステートフルアプリケーションの永続ストレージ（Azure Files RWX）.md]])
- 今回のシナリオでは、AKS 上のアプリケーションが次の条件を満たすストレージを必要としています。 ([[AKS ステートフルアプリケーションの永続ストレージ（Azure Files RWX）.md]])
- まず、ストレージは Kubernetes によって **動的にプロビジョニング**される必要があります。つまり、手動でストレージを作成するのではなく、アプリケーションの要求に応じて自動的に作成される仕組みが必要です。 ([[AKS ステートフルアプリケーションの永続ストレージ（Azure Files RWX）.md]])
- さらに、Kubernetes ではポッドがノード障害やスケーリングによって **別のノードに再スケジュール**されることがあります。そのため、ストレージはノードに依存しない形でアクセス可能でなければなりません。 ([[AKS ステートフルアプリケーションの永続ストレージ（Azure Files RWX）.md]])
- このような条件を満たす AKS 用ストレージソリューションを選択する必要があります。 ([[AKS ステートフルアプリケーションの永続ストレージ（Azure Files RWX）.md]])

### 3.4 セキュリティ、認証、リリース

- 今回のシナリオでは、企業は **Azure Kubernetes Service (AKS)** を使用したマルチリージョンのマイクロサービスアプリケーションを運用している。CI/CD パイプラインには Azure DevOps が使用されており、新しいアプリケーションバージョンを安全にリリースする仕組みが必要になっている。 ([[AKS マルチリージョンアプリケーションの安全なリリース戦略.md]])
- このシステムには次のような要件がある。 ([[AKS マルチリージョンアプリケーションの安全なリリース戦略.md]])
- このような要件を満たす最適なリリース戦略は ([[AKS マルチリージョンアプリケーションの安全なリリース戦略.md]])
- これは次のリングへ進む前に **人間の承認を必要とする仕組み**である。 ([[AKS マルチリージョンアプリケーションの安全なリリース戦略.md]])

## 第4章 ユースケースで理解する

### 4.1 AKS 基盤とスケーリングのユースケース

- AKS を使用したリージョン障害耐性のある Kubernetes アーキテクチャ: ある企業では、コンテナ化されたアプリケーションを Azure 上で運用するために **Azure Kubernetes Service（AKS）** を導入しようとしています。AKS は Kubernetes のマネージドサービスであり、インフラストラクチャ管理を Azure が担当するため、企業はアプリケーションの開発と運用に集中できま... 出典: [[AKS を使用したリージョン障害耐性のある Kubernetes アーキテクチャ.md]]
- Azure Kubernetes Service (AKS) スケーリング設計 （Windows + Linux コンテナ / バーストワークロード）: ある組織では **Azure Kubernetes Service (AKS)** を利用してコンテナ基盤を構築する予定である。 このクラスターは次の特徴を持つ。 出典: [[Azure Kubernetes Service (AKS) スケーリング設計.md]]
- コンテナ化アプリケーションのためのマネージド Kubernetes（Azure Kubernetes Service）: ある企業では、既存のアプリケーションをコンテナ化し、クラウド上で運用する計画を立てています。コンテナ技術を採用することで、アプリケーションのデプロイを効率化し、スケーラビリティやポータビリティを向上させることができます。 出典: [[コンテナ化アプリケーションのためのマネージド Kubernetes（Azure Kubernetes Service）.md]]

### 4.2 入口制御とマイクロサービスのユースケース

- AKS マイクロサービスアーキテクチャ設計ガイド: クラウドネイティブアプリケーションでは、従来のモノリシックアーキテクチャではなく **マイクロサービスアーキテクチャ** が主流となっている。マイクロサービスではアプリケーションを小さなサービス単位に分割し、それぞれを独立したコンテナとして実行する。これにより、スケーラビリティや開発効率が向上する。 出典: [[AKS マイクロサービスアーキテクチャ設計ガイド.md]]

### 4.3 ストレージとステートフル設計のユースケース

- AKS ステートフルアプリケーションの永続ストレージ（Azure Files RWX）: Azure Kubernetes Service（AKS）では、多くのアプリケーションがコンテナとしてデプロイされます。Kubernetes のコンテナは通常ステートレスに設計されますが、実際のアプリケーションではデータを保持する必要がある **ステートフルアプリケーション**も存在します。 出典: [[AKS ステートフルアプリケーションの永続ストレージ（Azure Files RWX）.md]]
- AKS ステートフルアプリケーションの共有ストレージ設計 （Azure Files + ReadWriteMany）: Kubernetes 上でアプリケーションを実行する場合、多くのワークロードは **ステートレス（stateless）** として設計される。しかし実際の業務システムでは、以下のようにデータを保持する **ステートフル（stateful）アプリケーション** が必要になる。 出典: [[AKS ステートフルアプリケーションの共有ストレージ設計.md]]

### 4.4 セキュリティ、認証、リリースのユースケース

- AKS マルチリージョンアプリケーションの安全なリリース戦略 （Deployment Rings + Approval Gates）: 現代のクラウドアプリケーションでは、単純に新しいバージョンを一度に本番へデプロイする方法はリスクが高い。特に **マイクロサービスアーキテクチャ** と **Kubernetes（AKS）** を利用する場合、システムは多くのサービスとリージョンで構成されるため、誤ったデプロイが広範囲に影響を与える可能性がある。 出典: [[AKS マルチリージョンアプリケーションの安全なリリース戦略.md]]
- Azure Container Registry SKU 比較ドキュメント （Basic / Standard / Premium）: **Azure Container Registry (ACR)** は、DockerコンテナイメージおよびOCIアーティファクトを保存するための **プライベートコンテナレジストリサービス**である。 出典: [[Azure Container Registry SKU 比較ドキュメント.md]]
- Azure DevOps Pipeline を利用した AKS への継続的デプロイ（ACR イメージ更新トリガー）: Azure Container Registry (ACR) に新しいコンテナイメージがプッシュされた際に、 Azure Kubernetes Service (AKS) へ自動的にアプリケーションをデプロイする仕組みは **CI/CD (Continuous Integration / Continuous Deployment)**... 出典: [[Azure DevOps Pipeline を利用した AKS への継続的デプロイ（ACR イメージ更新トリガー）.md]]

## 第5章 学習チェックポイント

- まず AKS 基盤とスケーリング → 入口制御とマイクロサービス → ストレージとステートフル設計 → セキュリティ、認証、リリース の順で読むと、基礎から応用まで流れを追いやすい。
- 第2章でサービスや構成要素の位置づけを理解し、第3章で制約条件を確認してから第4章のユースケースを読む。
- 同じサービスでも、要件によって選定理由が変わるため、出典リンク先の問題設定を必ず確認する。
- 類似ケースが複数ある場合は、第4章のユースケースを比較して判断軸を揃える。

## 関連用語

- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Kubernetes Service (AKS)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Files]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Storage Account]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Blob Storage]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure NetApp Files]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Ingress Controller]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Dapr]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Istio]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Cache for Redis]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Service Bus]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Monitor]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Application Insights]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure DNS]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Role-Based Access Control (RBAC)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Managed Identity]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Functions]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Container Registry]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Private Link]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Key Vault]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Resource Manager (ARM)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Network (VNet)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Load Balancer]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure App Service]]

## 出典ドキュメント

- [[AKS ステートフルアプリケーションの永続ストレージ（Azure Files RWX）.md]]
- [[AKS ステートフルアプリケーションの共有ストレージ設計.md]]
- [[AKS マイクロサービスアーキテクチャ設計ガイド.md]]
- [[AKS マルチリージョンアプリケーションの安全なリリース戦略.md]]
- [[AKS を使用したリージョン障害耐性のある Kubernetes アーキテクチャ.md]]
- [[Azure Container Registry SKU 比較ドキュメント.md]]
- [[Azure DevOps Pipeline を利用した AKS への継続的デプロイ（ACR イメージ更新トリガー）.md]]
- [[Azure Kubernetes Service (AKS) スケーリング設計.md]]
- [[コンテナ化アプリケーションのためのマネージド Kubernetes（Azure Kubernetes Service）.md]]
