# Topic-08 API、統合、イベント駆動

## 学習ゴール

API の入口保護、ワークフロー統合、イベント駆動、自動化という順に読み、疎結合連携の設計パターンを整理する。

## この Topic の全体像

API Management、Logic Apps、Service Bus、Event Hubs、Event Grid を中心に API 連携と非同期統合を扱う。

対象ドキュメント数: 7 件

## 第1章 学習マップ

### 1.1 学習順序

1. API 保護と認証: まず APIM と Entra ID を使った API 保護を理解する。
2. 業務ワークフローとシステム統合: Logic Apps を使った業務システム連携を読む。
3. イベント駆動と自動化: Event Grid、Event Hubs、Functions の連携パターンを押さえる。
4. 特殊ユースケース: 一般的な API/イベント以外の特殊領域も把握する。

### 1.2 セクション対応表

- API 保護と認証: 1 件 / [[Azure AD と API Management を用いた内部 API セキュリティ設計.md]]
- 業務ワークフローとシステム統合: 2 件 / [[Azure Logic Apps からオンプレミス SQL Server へ接続するアーキテクチャ.md]] / [[Azure Logic Apps とオンプレミス SAP システムのスケジュール統合.md]]
- イベント駆動と自動化: 3 件 / [[Azure AD のユーザー作成およびロール割り当てイベントをキャプチャし、Azure Cosmos DB に保存する設計.md]] / [[Azure イベントメッセージングアーキテクチャ整理.md]] / [[Azure イベント駆動オートメーション.md]]
- 特殊ユースケース: 1 件 / [[Azure における金融機関向けプライベートブロックチェーン設計.md]]

## 第2章 基礎概念と構成要素

### 2.1 API 保護と認証

まず APIM と Entra ID を使った API 保護を理解する。

主な出典: [[Azure AD と API Management を用いた内部 API セキュリティ設計.md]]

主要論点: Azure AD のクレームを使用する / 不正なリクエストをブロックする / 最小限の設定 / スケーラブルな管理 / Azure AD による権限管理 / API スコープ / クライアントアプリの権限 / JWT トークンとは / JWT 検証の役割 / API Management を使う理由

### 2.2 業務ワークフローとシステム統合

Logic Apps を使った業務システム連携を読む。

主な出典: [[Azure Logic Apps からオンプレミス SQL Server へ接続するアーキテクチャ.md]] / [[Azure Logic Apps とオンプレミス SAP システムのスケジュール統合.md]]

主要論点: 問題の本質 / On-premises Data Gateway / アーキテクチャ / 通信の仕組み / セキュリティ / TLS 暗号化 / 認証 / ポート制限 / Azure AD Application Proxy / Application Gateway

### 2.3 イベント駆動と自動化

Event Grid、Event Hubs、Functions の連携パターンを押さえる。

主な出典: [[Azure AD のユーザー作成およびロール割り当てイベントをキャプチャし、Azure Cosmos DB に保存する設計.md]] / [[Azure イベントメッセージングアーキテクチャ整理.md]] / [[Azure イベント駆動オートメーション.md]]

主要論点: 関連 Azure サービスの説明 / 技術的な仕組み / Azureイベントメッセージングの分類 / ストリーム処理系 / メッセージブローカー系 / イベント通知系 / IoT Devices / Azure IoT Hub / Azure Event Hubs / Azure Stream Analytics

### 2.4 特殊ユースケース

一般的な API/イベント以外の特殊領域も把握する。

主な出典: [[Azure における金融機関向けプライベートブロックチェーン設計.md]]

主要論点: プライベートブロックチェーン / Azure Blockchain Service / Azure Active Directory 統合 / 機密取引のサポート / 高可用性 / マネージドサービスの利点 / AKS 上の Ethereum / Corda on Azure VM / Hyperledger Fabric on AKS

## 第3章 設計判断の軸

### 3.1 API 保護と認証

- これらの API は Azure AD の **クレーム（claims）** を利用してアクセス制御を行う。 ([[Azure AD と API Management を用いた内部 API セキュリティ設計.md]])
- API へのアクセス制御は **JWT トークンのクレーム情報**を利用する。 ([[Azure AD と API Management を用いた内部 API セキュリティ設計.md]])
- Azure AD で発行された **正しいトークンのみ**を API に到達させる必要がある。 ([[Azure AD と API Management を用いた内部 API セキュリティ設計.md]])
- Web アプリケーションには API アクセス権を付与する。 ([[Azure AD と API Management を用いた内部 API セキュリティ設計.md]])
- API はトークンの次の項目を検証する必要がある。 ([[Azure AD と API Management を用いた内部 API セキュリティ設計.md]])
- JWT 検証は API 側または APIM 側で行う必要がある。 ([[Azure AD と API Management を用いた内部 API セキュリティ設計.md]])

### 3.2 業務ワークフローとシステム統合

- 企業では業務プロセスの自動化のために **Azure Logic Apps** を利用している。Logic Apps は SaaS 連携やデータ処理のワークフローをクラウド上で自動化するサービスであり、さまざまなシステムと接続することができる。 ([[Azure Logic Apps からオンプレミス SQL Server へ接続するアーキテクチャ.md]])
- 今回のシナリオでは、Logic Apps から **オンプレミス環境に存在する SQL Server データベースへデータを書き込む必要**がある。しかし、この SQL Server はセキュリティポリシーの制約により **インターネットから直接アクセスできないネットワーク環境**に配置されている。 ([[Azure Logic Apps からオンプレミス SQL Server へ接続するアーキテクチャ.md]])
- この状況で次の要件を満たす必要がある。 ([[Azure Logic Apps からオンプレミス SQL Server へ接続するアーキテクチャ.md]])
- 通常、クラウドサービスからオンプレミスのデータベースへアクセスする場合は、次のような方法が考えられる。 ([[Azure Logic Apps からオンプレミス SQL Server へ接続するアーキテクチャ.md]])
- 今回の問題では **Logic Apps からデータベースへの接続**であるため、Microsoft が推奨する方法は **On-premises Data Gateway（オンプレミスデータゲートウェイ）**となる。 ([[Azure Logic Apps からオンプレミス SQL Server へ接続するアーキテクチャ.md]])
- オンプレミスデータゲートウェイは、Azure サービスとオンプレミスのデータソースを安全に接続するためのコンポーネントである。 ([[Azure Logic Apps からオンプレミス SQL Server へ接続するアーキテクチャ.md]])

### 3.3 イベント駆動と自動化

- この二つのサービスを組み合わせることで、Azure AD の監査ログを取り込み、処理し、最終的に Cosmos DB に保存するという一連のイベントパイプラインを構築できる。 ([[Azure AD のユーザー作成およびロール割り当てイベントをキャプチャし、Azure Cosmos DB に保存する設計.md]])
- 選択肢 D は Notification Hubs と Stream Analytics の組み合わせであるが、Notification Hubs はモバイルアプリへのプッシュ通知を送るためのサービスであり、ログ収集やイベントストリーミングとは関係がない。そのため、この問題の要件とは全く一致しない。 ([[Azure AD のユーザー作成およびロール割り当てイベントをキャプチャし、Azure Cosmos DB に保存する設計.md]])
- Azure AD の監査ログを外部システムに保存する場合、一般的な設計パターンは「ログストリーミングサービス + サーバーレス処理 + データストア」という構成になる。Azure ではこのパターンを実現するために Event Hubs と Functions を組み合わせることが多く、ログ取り込みパイプラインとして広く利用されている。 ([[Azure AD のユーザー作成およびロール割り当てイベントをキャプチャし、Azure Cosmos DB に保存する設計.md]])
- IoT Devicesとは、インターネットに接続された物理デバイスを指します。 ([[Azure イベントメッセージングアーキテクチャ整理.md]])
- そのため、高スループットなイベント処理基盤が必要になります。 ([[Azure イベントメッセージングアーキテクチャ整理.md]])
- 必要なら次に **このドキュメントをさらにレベルアップした「Azureデータパイプライン完全版（Databricks / Synapse / Data Factory含む）」** を作れます。 ([[Azure イベントメッセージングアーキテクチャ整理.md]])

### 3.4 特殊ユースケース

- ブロックチェーンは分散型台帳を利用することで、取引の改ざん防止、監査可能性、トレーサビリティを提供する。 ([[Azure における金融機関向けプライベートブロックチェーン設計.md]])
- 今回のシナリオでは、金融機関が **支店間決済用のプライベートブロックチェーンネットワーク** を Azure 上に構築しようとしている。システムには次の要件がある。 ([[Azure における金融機関向けプライベートブロックチェーン設計.md]])
- これらの要件を満たす Azure のサービスとして推奨されるのが ([[Azure における金融機関向けプライベートブロックチェーン設計.md]])
- 企業のブロックチェーンシステムでは、参加者の認証とアクセス制御が重要である。 ([[Azure における金融機関向けプライベートブロックチェーン設計.md]])
- 金融システムではシステム停止が許されないため、ブロックチェーンネットワークには高可用性が必要である。 ([[Azure における金融機関向けプライベートブロックチェーン設計.md]])
- ノード障害が発生してもネットワークは継続して稼働する。 ([[Azure における金融機関向けプライベートブロックチェーン設計.md]])

## 第4章 ユースケースで理解する

### 4.1 API 保護と認証のユースケース

- Azure AD と API Management を用いた内部 API セキュリティ設計 （Azure AD Authorization + APIM JWT Validation）: 企業では複数の内部サービスを Azure 上で運用しており、内部 API を **Azure API Management（APIM）** を介して公開している。 出典: [[Azure AD と API Management を用いた内部 API セキュリティ設計.md]]

### 4.2 業務ワークフローとシステム統合のユースケース

- Azure Logic Apps からオンプレミス SQL Server へ接続するアーキテクチャ （インターネット接続のない環境でのデータ統合）: 企業では業務プロセスの自動化のために **Azure Logic Apps** を利用している。Logic Apps は SaaS 連携やデータ処理のワークフローをクラウド上で自動化するサービスであり、さまざまなシステムと接続することができる。 出典: [[Azure Logic Apps からオンプレミス SQL Server へ接続するアーキテクチャ.md]]
- Azure Logic Apps とオンプレミス SAP システムのスケジュール統合: ある企業では、Azure 上で **Logic Apps** を使用して業務プロセスの自動化を行っています。この Logic App は **オンプレミス環境に存在する SAP システム**と統合する必要があります。 出典: [[Azure Logic Apps とオンプレミス SAP システムのスケジュール統合.md]]

### 4.3 イベント駆動と自動化のユースケース

- Azure AD のユーザー作成およびロール割り当てイベントをキャプチャし、Azure Cosmos DB に保存する設計: 企業がクラウド環境でユーザー管理を行う場合、多くの組織は **Azure Active Directory（現在の Microsoft Entra ID）** を利用して認証・認可を管理する。Azure ADでは、ユーザーの作成、削除、グループ変更、ロールの割り当てなど、管理操作が行われるたびに **監査ログ（Audit Logs）**... 出典: [[Azure AD のユーザー作成およびロール割り当てイベントをキャプチャし、Azure Cosmos DB に保存する設計.md]]
- Azure イベント駆動アーキテクチャ完全ドキュメント: クラウドシステムでは、システムのスケーラビリティや柔軟性を高めるために **イベント駆動アーキテクチャ (Event-Driven Architecture)** が広く採用されています。 出典: [[Azure イベントメッセージングアーキテクチャ整理.md]]
- Azure イベント駆動オートメーション （Event Grid + Logic Apps によるインフラ監視）: Azure環境では、リソースの状態や設定変更に応じて自動処理を実行する **イベント駆動アーキテクチャ**が重要になる。 出典: [[Azure イベント駆動オートメーション.md]]

### 4.4 特殊ユースケースのユースケース

- Azure における金融機関向けプライベートブロックチェーン設計 （Azure Blockchain Service）: 金融機関では、支店間の決済や資金移動を安全かつ透明性の高い方法で処理するために **ブロックチェーン（Distributed Ledger Technology: DLT）** を採用するケースが増えている。 出典: [[Azure における金融機関向けプライベートブロックチェーン設計.md]]

## 第5章 学習チェックポイント

- まず API 保護と認証 → 業務ワークフローとシステム統合 → イベント駆動と自動化 → 特殊ユースケース の順で読むと、基礎から応用まで流れを追いやすい。
- 第2章でサービスや構成要素の位置づけを理解し、第3章で制約条件を確認してから第4章のユースケースを読む。
- 同じサービスでも、要件によって選定理由が変わるため、出典リンク先の問題設定を必ず確認する。
- 類似ケースが複数ある場合は、第4章のユースケースを比較して判断軸を揃える。

## 関連用語

- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Microsoft Entra ID]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#App Registration]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure API Management]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Functions]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Cosmos DB]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Logic Apps]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Event Hubs]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Event Grid]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Monitor]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Log Analytics]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Stream Analytics]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure App Service]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure ExpressRoute]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Power BI]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Service Bus]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Databricks]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure IoT Hub]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Role-Based Access Control (RBAC)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Kubernetes Service (AKS)]]

## 出典ドキュメント

- [[Azure AD と API Management を用いた内部 API セキュリティ設計.md]]
- [[Azure AD のユーザー作成およびロール割り当てイベントをキャプチャし、Azure Cosmos DB に保存する設計.md]]
- [[Azure Logic Apps からオンプレミス SQL Server へ接続するアーキテクチャ.md]]
- [[Azure Logic Apps とオンプレミス SAP システムのスケジュール統合.md]]
- [[Azure イベントメッセージングアーキテクチャ整理.md]]
- [[Azure イベント駆動オートメーション.md]]
- [[Azure における金融機関向けプライベートブロックチェーン設計.md]]
