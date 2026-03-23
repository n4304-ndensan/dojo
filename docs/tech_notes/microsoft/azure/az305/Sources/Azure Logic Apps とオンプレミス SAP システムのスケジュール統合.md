---
分類: Cloud
tags:
  - cloud/azure
  - cloud/azure/logic-apps
  - cloud/azure/connectors
  - cloud/azure/on-premises-data-gateway
  - cloud/azure/scheduling
  - cloud/azure/integration
  - cloud/azure/sap
  - cloud/architecture/integration
  - cloud/architecture/batch-processing
  - cloud/architecture/hybrid-cloud
  - exam/azure
---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Logic Apps]]
# Azure Logic Apps とオンプレミス SAP システムのスケジュール統合

## 1. 背景（シナリオ）

ある企業では、Azure 上で **Logic Apps** を使用して業務プロセスの自動化を行っています。この Logic App は **オンプレミス環境に存在する SAP システム**と統合する必要があります。

この統合処理には以下の特徴があります。

- Logic App は **事前定義されたスケジュール（例：毎日・毎時間）で実行**される
- SAP システムから **大量のデータを取得・処理**する必要がある
- SAP は **オンプレミス環境に存在**しており、Azure から直接アクセスできない

したがって、Logic Apps には以下の2つの機能が必要になります。

1. **スケジュールベースでワークフローを開始するトリガー**
2. **Azure からオンプレミス SAP へ安全に接続する仕組み**

これらを満たす最適な構成を選択する必要があります。

---

## 2. 要件整理

この問題の要件を整理すると、設計のポイントが明確になります。

まず、Logic App の実行方法に関する要件があります。  
Logic App は **外部イベントではなくスケジュールによって起動**する必要があります。つまり、時間ベースのトリガーが必要です。

次に、接続先のシステムに関する要件があります。  
対象となる SAP システムは **オンプレミス環境**にあるため、Azure から安全にアクセスするための **ハイブリッド接続**が必要です。

さらに、処理データ量の要件もあります。  
SAP から **大量のデータをバッチ処理として取得**する必要があるため、信頼性や再試行機能を備えた統合サービスが望ましいです。

整理すると、以下の要件になります。

- スケジュール実行
- Azure からオンプレミス接続
- SAP 専用操作（RFC / BAPI / IDoc）
- 大量データ処理に対応
- 再試行・信頼性・監視機能

---

## 3. 技術の基本概念

この問題を理解するために、関連する Azure サービスの基本概念を整理します。

### Azure Logic Apps

Azure Logic Apps は、クラウドベースの **ワークフロー自動化サービス**です。  
さまざまなサービスとの統合を **コネクタ**を通じて実現できます。

Logic Apps の構成は大きく次の2つです。

- **トリガー**  
  ワークフローの開始条件  
  例：HTTP、スケジュール、イベントなど

- **アクション（コネクタ）**  
  実際の処理（SAP操作、データ保存など）

---

### Recurrence（再帰）トリガー

Recurrence トリガーは **時間ベースのトリガー**です。

例えば次のようなスケジュールが設定できます。

- 毎日
- 毎時間
- 毎週
- 指定時間

これは **バッチ処理や定期データ同期**でよく使用されます。

---

### On-premises Data Gateway

オンプレミス Data Gateway は、Azure サービスから **オンプレミス環境への安全な接続を提供するコンポーネント**です。

主な特徴は以下です。

- Azure → オンプレミス接続を安全に確立
- HTTPS による暗号化通信
- ファイアウォールのインバウンド開放不要
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Power BI]]
- Logic Apps / Power Platform / Power BI などで利用可能

これにより、クラウドから社内ネットワークの SAP にアクセスできます。

---

### SAP Connector

Logic Apps には **SAP 専用コネクタ**が用意されています。

代表的な操作は次の通りです。

- RFC 呼び出し
- BAPI 呼び出し
- IDoc 処理
- SAP テーブル取得

これにより **SAP統合をカスタム開発なしで実装**できます。

---

## 4. アーキテクチャまたは設計のポイント

今回の要件を満たすための典型的なアーキテクチャは以下の構成になります。

1. Logic App が **Recurrence トリガー**で起動
2. Logic App が **SAP コネクタ**を使用
3. SAP コネクタが **オンプレミスデータゲートウェイ経由で SAP に接続**

この構成には以下のメリットがあります。

- スケジュールによる自動実行
- Azure とオンプレミスの安全な接続
- SAP 専用 API の利用
- 再試行・監視などのマネージド機能

結果として、**大規模データの定期バッチ処理**に適した設計になります。

---

## 5. 設計判断（なぜこの構成になるか）

この設計が最適な理由は、要件とサービス機能が完全に一致しているためです。

まず、スケジュール実行の要件に対しては **Recurrence トリガー**が最適です。  
HTTP やイベントベースのトリガーでは、定期実行を保証できません。

次に、オンプレミス接続の問題があります。  
Azure から直接 SAP にアクセスすることはできないため、**オンプレミスデータゲートウェイ**が必要になります。

さらに、SAP 操作には専用コネクタが用意されています。  
これにより RFC や BAPI の呼び出しが可能になります。

この3つを組み合わせることで、以下が実現できます。

- 定期実行
- 安全なハイブリッド接続
- SAP ネイティブ操作
- バッチデータ処理

---

## 6. 他の選択肢が誤りな理由

### A. HTTPトリガー + SAPコネクタ

HTTPトリガーは **イベント駆動型トリガー**です。

つまり以下の特徴があります。

- 外部から HTTP リクエストが来たときのみ実行
- スケジュール実行ではない

そのため **定期バッチ処理には不適切**です。

また、HTTP トリガーだけでは **オンプレミス SAP 接続の仕組み**も提供されません。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Functions]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Service Bus]]
### C. Service Bus トリガー + Azure Functions プロキシ

Service Bus トリガーは **メッセージ駆動型イベント処理**です。

つまり以下の特徴があります。

- キューまたはトピックのメッセージで起動
- スケジュール実行ではない

また、SAP 接続のために

- Service Bus
- Azure Functions

など追加コンポーネントが必要になり、**アーキテクチャが不必要に複雑になります**。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure API Management]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Event Grid]]
### D. Event Grid トリガー + API Management

Event Grid は **イベント通知サービス**です。

例えば次のようなイベントを処理します。

- リソース作成
- ファイルアップロード
- カスタムイベント

しかし、Event Grid は **スケジュール処理には向いていません**。

また、API Management は **API 管理サービス**であり、オンプレミス SAP 接続の仕組みではありません。

---

## 7. 最終回答

**B. 再帰（Recurrence）トリガー + SAPコネクタ（オンプレミスデータゲートウェイ経由）**

---

## 8. まとめ

Azure Logic Apps でオンプレミス SAP と統合する場合、重要なポイントは **スケジュール実行とハイブリッド接続**です。

この問題のポイントは次の通りです。

まず、定期実行が必要な場合は **Recurrence トリガー**を使用します。

次に、オンプレミスシステムと接続する場合は **On-premises Data Gateway** を使用します。

そして SAP 操作には **SAP Connector** を利用します。

つまり、以下の組み合わせがベストプラクティスになります。

- Recurrence Trigger（スケジュール）
- SAP Connector（SAP操作）
- On-premises Data Gateway（オンプレミス接続）

この構成により、**安全・スケーラブル・信頼性の高い SAP 統合ワークフロー**を実現できます。