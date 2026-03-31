---
分類: Cloud
tags:
  - cloud/azure
  - cloud/azure/data-factory
  - cloud/azure/blob-storage
  - cloud/azure/sql-database
  - cloud/data-engineering/etl
  - cloud/data-engineering/data-integration
  - cloud/architecture/data-pipeline
  - cloud/automation/scheduling
  - exam/azure/architecture
---

# Azure Data Factory を利用したBlob StorageからSQL Databaseへの自動データ転送

## 1. 背景（シナリオ）

ある企業では、WebアプリケーションのアクセスログをAzure Blob Storageに保存しています。これらのログデータは、分析やレポート作成のためにAzure SQL Databaseへ定期的に取り込む必要があります。

このデータ転送は毎月実行されるため、手動操作ではなく自動化された仕組みを構築することが重要です。また、運用管理の負担を最小限に抑えながら、信頼性の高いデータ転送を実現する必要があります。

このような要件では、データの抽出・転送・ロードを自動化できるクラウドベースのデータ統合サービスを使用することが最適です。

---

## 2. 要件整理

この問題では、ログデータをBlob StorageからAzure SQL Databaseへ転送する仕組みを設計する必要があります。

シナリオから読み取れる要件を整理すると次のようになります。

まず、データ転送は定期的に実行される必要があります。このケースでは月次処理のため、スケジュール実行機能が必要です。

次に、データソースはAzure Blob Storageであり、データの保存先はAzure SQL Databaseです。つまり、異なるAzureサービス間でデータを移動する仕組みが必要になります。

さらに、管理作業を最小限に抑える必要があります。つまり、インフラ管理やスクリプト管理を最小化できるマネージドサービスが望ましいです。

これらの要件を整理すると次の通りです。

- Blob StorageからSQL Databaseへのデータ転送  
- 月次スケジュール実行  
- 自動化されたデータパイプライン  
- 運用管理の最小化  

---

## 3. 技術の基本概念

Azure Data Factory（ADF）は、クラウドベースのデータ統合サービスです。異なるデータソース間でデータの移動や変換を行うデータパイプラインを構築できます。

Data Factoryの中心概念は「パイプライン」です。パイプラインは、データの移動や処理を自動化するワークフローです。

ADFでは次のような機能を提供しています。

- データコピー（Copy Activity）  
- データ変換  
- スケジュールトリガー  
- パイプライン監視  

特にCopy Activityは、Blob StorageからAzure SQL Databaseなどの異なるデータソース間でデータを簡単に移動できる機能です。

また、スケジュールトリガーを利用することで、毎日、毎週、毎月などの定期的な実行が可能になります。

---

## 4. アーキテクチャまたは設計のポイント

このシナリオでは、Azure Data Factoryを中心としたデータパイプラインを構築します。

まず、Blob Storageをデータソースとして設定します。次に、Azure SQL Databaseをデータの保存先として設定します。

その後、ADFのCopy Activityを使用してデータ転送処理を定義します。

データ処理の流れは次のようになります。

1. Azure Data Factoryのパイプラインを作成  
2. Blob Storageをソースとして設定  
3. Azure SQL Databaseを宛先として設定  
4. Copy Activityでデータ転送処理を構築  
5. 月次スケジュールトリガーを設定  

この構成により、ログデータは毎月自動的にSQL Databaseへロードされます。

また、ADFには監視機能もあるため、パイプラインの実行状態や失敗を確認することができます。

---

## 5. 設計判断（なぜこの構成になるか）

この問題のポイントは「定期的なデータ転送」と「管理負担の最小化」です。

Azure Data Factoryはデータ統合サービスとして設計されており、異なるデータストレージ間のデータ移動を簡単に実装できます。

ADFを利用することで次のメリットがあります。

- スケジュール実行による自動化  
- 大量データの転送に対応  
- マネージドサービスによる管理負担削減  
- Azureサービス間のネイティブ統合  

また、GUIベースでパイプラインを構築できるため、カスタムコードを書く必要がほとんどありません。

そのため、Blob StorageからAzure SQL Databaseへ定期的にデータをロードするシナリオでは、Azure Data Factoryが最適な選択となります。

---

## 6. 他の選択肢が誤りな理由

AzCopyはBlob Storageへのデータコピーを行うコマンドラインツールです。一時的なデータ転送には便利ですが、スケジュール管理やデータパイプラインの自動化機能は提供していません。

データ移行アシスタント（Data Migration Assistant）は、主にSQL ServerからAzure SQL Databaseへの移行を支援するツールです。定期的なデータ転送には適していません。

Azure Logic Appsはワークフロー自動化ツールですが、大量データの転送処理には適しておらず、カスタムコネクタの作成などにより運用管理が複雑になります。

---

## 7. 最終回答

B.  
Azure Data Factory

---

## 8. まとめ

Azure Data Factoryは、クラウドベースのデータ統合サービスであり、データパイプラインを構築して異なるデータソース間のデータ移動を自動化できます。

このシナリオでは次の要件を満たす必要があります。

- Blob StorageからSQL Databaseへのデータ転送  
- 月次スケジュール実行  
- 管理負担の最小化  

Azure Data Factoryを利用することで、スケジュールされたデータパイプラインを構築し、ログデータを自動的にAzure SQL Databaseへロードできます。

そのため、このシナリオの最適なソリューションは **Azure Data Factory** です。