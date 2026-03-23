# Topic-04-Compute

## 学習ゴール

[[Azure用語集.md#Azure App Service]]、[[Azure用語集.md#Azure Functions]]、[[Azure用語集.md#Azure Kubernetes Service (AKS)]]、[[Azure用語集.md#Azure Virtual Machines]] を横並びで比較し、実行基盤選定の判断を作る。

## このTopicの全体像

この Topic では、Web/PaaS、サーバーレス、コンテナ、IaaS、バッチ処理を 1 本に束ねる。  
判断軸は「どこまで Azure に任せるか」「スケール単位は何か」「特殊要件をどこで吸収するか」。

---

# 第1章 学習マップ

## 1.1 学習順序

1. App Service で標準的な Web 実行基盤を押さえる。
2. Functions でイベント駆動・サーバーレスへ進む。
3. AKS でコンテナ オーケストレーションを理解する。
4. VM / VMSS / Batch / Dedicated Host で特殊ワークロードへ広げる。

## 1.2 Azureリソース一覧

- Azure App Service
- Azure Functions
- Azure Kubernetes Service (AKS)
- Azure Virtual Machines / VM Scale Sets
- Azure Batch / Dedicated Host
- Azure Container Registry

---

# 第2章 Azureリソース解説

## Resource: Azure App Service

### 概要

[[Azure用語集.md#Azure App Service]] は Web アプリを最短で運用するための標準 PaaS。

### できること

- Web API / Web アプリのホスティング
- Easy Auth
- Deployment Slots
- マネージド証明書
- Key Vault 参照

### 技術仕様

- IIS / Linux ベースのマネージド実行環境を提供する。
- スロットでゼロダウンタイム展開ができる。
- VNet Integration や Private Endpoint でネットワーク要件へ寄せられる。
- 監視は Application Insights と相性がよい。

### SDK / API

- Azure SDK
- Kudu / SCM API
- Azure CLI `az webapp`

### 他サービスとの比較

- App Service vs VM: インフラ管理を減らすなら App Service。
- App Service vs AKS: コンテナ基盤の細粒度制御が不要なら App Service。
- App Service vs Functions: 常駐 Web と HTTP アプリは App Service が自然。

### どのようなときに使うか

- 標準的な Web アプリを素早く公開したいとき
- 証明書、認証、デプロイを PaaS に寄せたいとき

### 関連シナリオ

- [[Scenarios/Scenario-AppService.md#entra-id-で-web-アプリ認証を統一する]]
- [[Scenarios/Scenario-AppService.md#deployment-slots-でゼロダウンタイム展開を行う]]
- [[Scenarios/Scenario-AppService.md#app-service-から-key-vault-の秘密を参照する]]

### 主な出典

- [[Sources/Topic-05.md]]
- [[Sources/Azure App Service HTTPS 構成設計.md]]
- [[Sources/Azure App Service のゼロダウンタイムデプロイ（Deployment Slots）.md]]
- [[Sources/Azure App Service 高可用性設計.md]]

## Resource: Azure Functions

### 概要

[[Azure用語集.md#Azure Functions]] はイベント到着時だけ実行したい処理をコード単位で動かすサーバーレス実行基盤。

### できること

- HTTP API
- Queue / Service Bus / Event Hubs トリガー
- Timer / Blob トリガー
- Durable Functions によるオーケストレーション

### 技術仕様

- Consumption はゼロスケールとコスト効率に強い。
- Premium はコールドスタート回避と VNet、長時間実行に強い。
- Managed Identity と Application Insights を標準構成にしやすい。

### SDK / API

- Azure Functions runtime
- Azure SDK
- Azure CLI `az functionapp`

### 他サービスとの比較

- Functions vs App Service: イベント駆動・短時間処理なら Functions。
- Functions vs Logic Apps: コード主体なら Functions、コネクタ主体なら Logic Apps。

### どのようなときに使うか

- 非同期メッセージ処理
- 軽量な API
- イベント駆動 ETL の前段処理

### 関連シナリオ

- [[Scenarios/Scenario-Functions.md#storage-queue-トリガーで自動スケールする]]
- [[Scenarios/Scenario-Functions.md#service-bus-メッセージをコールドスタートなしで処理する]]
- [[Scenarios/Scenario-Functions.md#event-hubs-イベントをリアルタイム処理する]]

### 主な出典

- [[Sources/Topic-05.md]]
- [[Sources/Azure Functions のホスティングプランとスケーリング（Consumption Plan）.md]]
- [[Sources/Azure FunctionsでService Busメッセージをコールドスタートなしで処理する設計.md]]
- [[Sources/Azure Durable Functions によるワークフローオーケストレーション.md]]

## Resource: Azure Kubernetes Service (AKS)

### 概要

[[Azure用語集.md#Azure Kubernetes Service (AKS)]] はコンテナ オーケストレーションを Azure マネージドで提供する基盤。

### できること

- ノードプール運用
- HPA / Cluster Autoscaler
- Ingress
- Workload Identity
- ステートフル ワークロード

### 技術仕様

- Linux / Windows ノードプールを分けて運用できる。
- Azure Files の RWX ボリュームで共有ストレージを持てる。
- Application Gateway Ingress Controller や Service Mesh と組み合わせやすい。
- ACR 連携と CI/CD が前提構成になる。

### SDK / API

- Kubernetes API
- Azure CLI `az aks`
- Helm / kubectl

### 他サービスとの比較

- AKS vs App Service: コンテナ制御とプラットフォーム制御のどちらを優先するか。
- AKS vs Functions: 長寿命コンテナや複雑なマイクロサービスなら AKS。

### どのようなときに使うか

- 複数マイクロサービスを一括運用したいとき
- Pod 単位のスケーリングやサービスメッシュが必要なとき
- コンテナ実行環境を標準化したいとき

### 関連シナリオ

- [[Scenarios/Scenario-AKS.md#リージョン障害に耐える-aks-基盤を作る]]
- [[Scenarios/Scenario-AKS.md#azure-files-rwx-でステートフルワークロードを支える]]
- [[Scenarios/Scenario-AKS.md#acr-更新をトリガーに-aks-へ継続的デプロイする]]

### 主な出典

- [[Sources/Topic-04.md]]
- [[Sources/Azure Kubernetes Service (AKS) スケーリング設計.md]]
- [[Sources/AKS ステートフルアプリケーションの永続ストレージ（Azure Files RWX）.md]]
- [[Sources/AKS マルチリージョンアプリケーションの安全なリリース戦略.md]]

## Resource: Azure Virtual Machines and VM Scale Sets

### 概要

[[Azure用語集.md#Azure Virtual Machines]] は IaaS 制御を必要とするワークロードの受け皿であり、VMSS はそのスケールアウト版。

### できること

- レガシー アプリの移行
- SQL Server on VM
- カスタム OS / ミドルウェア構成
- VMSS による水平スケール

### 技術仕様

- ディスク、キャッシュ、可用性ゾーン、バックアップ、Site Recovery を個別に設計する。
- VMSS はアプリ サーバー層の自動スケールに向く。
- Dedicated Host は物理分離要件に使う。

### SDK / API

- ARM / Compute API
- Azure CLI `az vm` / `az vmss`

### 他サービスとの比較

- VM vs App Service: OS 制御が必要なら VM。
- VMSS vs AKS: アプリ単位スケールか、コンテナ単位スケールか。

### どのようなときに使うか

- COM 依存や Windows サービス依存など、PaaS 化が難しいとき
- 既存 SQL Server を IaaS で維持したいとき
- 物理分離や特殊ドライバが必要なとき

### 関連シナリオ

- [[Scenarios/Scenario-AppService.md#高可用な-web-実行基盤として-app-service-と-vm-を比較する]]
- [[Scenarios/Scenario-AzureSQL.md#sql-server-互換性を優先して-managed-instance-か-vm-を選ぶ]]

### 主な出典

- [[Sources/Topic-05.md]]
- [[Sources/Azure 仮想マシンの高可用性とパフォーマンスアーキテクチャ.md]]
- [[Sources/Azure Virtual Machine Scale Sets による自動スケーリング.md]]
- [[Sources/Azure Dedicated Host による物理的分離インフラ設計.md]]

## Resource: Azure Batch and Container Registry

### 概要

Azure Batch は大規模バッチ/HPC 実行、ACR はコンテナ イメージ配布の標準レジストリ。

### できること

- GPU バッチ処理
- 優先度別プール設計
- プライベート コンテナ イメージ管理
- AKS / CI/CD 連携

### 技術仕様

- Batch は Dedicated / Low Priority VM を使い分ける。
- ACR は Basic / Standard / Premium で機能差がある。
- Premium はネットワーク分離や geo-replication を取りやすい。

### SDK / API

- Azure Batch SDK
- Azure CLI `az acr`

### 他サービスとの比較

- Batch vs AKS: ジョブ実行中心なら Batch、常駐サービスなら AKS。
- ACR vs 外部レジストリ: Azure 内統合と権限管理を優先するなら ACR。

### どのようなときに使うか

- GPU/HPC ジョブを時間単位で処理したいとき
- コンテナ イメージ供給元を統一したいとき

### 関連シナリオ

- [[Scenarios/Scenario-AKS.md#acr-更新をトリガーに-aks-へ継続的デプロイする]]

### 主な出典

- [[Sources/Topic-04.md]]
- [[Sources/Azure Batch による GPU バッチワークロードのコスト最適化.md]]
- [[Sources/Azure Batch におけるコスト最適化と HPC ワークロード設計.md]]
- [[Sources/Azure Container Registry SKU 比較ドキュメント.md]]

---

# 第3章 設計判断ガイド

## 3.1 実行基盤を選ぶとき

- Web アプリ中心なら App Service。
- イベント駆動で短い処理なら Functions。
- コンテナ群を統合運用するなら AKS。
- OS 制御やレガシー互換が必要なら VM。

## 3.2 スケール方法を選ぶとき

- HTTP/アプリ全体は App Service plan。
- トリガー単位なら Functions。
- Pod 単位なら AKS。
- VM インスタンス単位なら VMSS。

## 3.3 運用境界を決めるとき

- インフラを減らしたいほど PaaS / serverless を優先する。
- 互換性や特殊要件が増えるほど VM / AKS 側へ寄る。

---

# 第4章 関連シナリオ

- [[Scenarios/Scenario-AppService.md]]
- [[Scenarios/Scenario-Functions.md]]
- [[Scenarios/Scenario-AKS.md]]

