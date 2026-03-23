# Topic-09 データエンジニアリングと分析

## 学習ゴール

データ取り込み、変換、分析、共有までを一続きで見て、分析基盤の流れを理解できる状態を目指す。

## この Topic の全体像

Data Factory、Synapse、Databricks、Data Explorer などの分析基盤を集約する。

対象ドキュメント数: 9 件

## 第1章 学習マップ

### 1.1 学習順序

1. データレイクと ETL: まず Data Factory、Data Lake、ETL の基本的な流れを整理する。
2. Synapse と Databricks 分析: 次に分析実行エンジンとしての Synapse と Databricks を比較する。
3. データ共有とパイプライン: 組織間共有や増分パイプラインを扱う。

### 1.2 セクション対応表

- データレイクと ETL: 3 件 / [[Azure Data Factory 大容量データコピー設計.md]] / [[Azure データウェアハウス ETL アーキテクチャ設計.md]] / [[Azure メダリオンアーキテクチャのデータレイク設計.md]]
- Synapse と Databricks 分析: 5 件 / [[Azure Databricks 分析基盤におけるデータレイクファイル形式設計.md]] / [[Azure Synapse Analytics におけるデータレイク分析アーキテクチャ設計.md]] / [[Azure データエンジニアリングアーキテクチャ.md]] / [[SAP HANA → Azure Synapse Analytics 増分データパイプライン設計.md]] / [[大規模ログ分析に適した Azure サービス（Azure Data Explorer）.md]]
- データ共有とパイプライン: 1 件 / [[Azure Synapse Pipelines と Azure Data Share を組み合わせたデータ統合・共有アーキテクチャ.md]]

## 第2章 基礎概念と構成要素

### 2.1 データレイクと ETL

まず Data Factory、Data Lake、ETL の基本的な流れを整理する。

主な出典: [[Azure Data Factory 大容量データコピー設計.md]] / [[Azure データウェアハウス ETL アーキテクチャ設計.md]] / [[Azure メダリオンアーキテクチャのデータレイク設計.md]]

主要論点: ADFの基本構成 / Integration Runtime (IR) / Self-hosted Integration Runtime / SHIRの役割 / データコピーの問題 / 逐次コピー / Parallel Copy / 並列処理 / Parallel Copy アーキテクチャ / Azure Synapse Analytics

### 2.2 Synapse と Databricks 分析

次に分析実行エンジンとしての Synapse と Databricks を比較する。

主な出典: [[Azure Databricks 分析基盤におけるデータレイクファイル形式設計.md]] / [[Azure Synapse Analytics におけるデータレイク分析アーキテクチャ設計.md]] / [[Azure データエンジニアリングアーキテクチャ.md]] / [[SAP HANA → Azure Synapse Analytics 増分データパイプライン設計.md]] / [[大規模ログ分析に適した Azure サービス（Azure Data Explorer）.md]]

主要論点: データレイクにおけるファイル形式の重要性 / Parquetとは / 効率的なカラムスキャン / Predicate Pushdown / スキーマの取り締まり / パーティション分割 / Databricksとの統合 / 他のフォーマットとの比較 / CSV / JSON

### 2.3 データ共有とパイプライン

組織間共有や増分パイプラインを扱う。

主な出典: [[Azure Synapse Pipelines と Azure Data Share を組み合わせたデータ統合・共有アーキテクチャ.md]]

主要論点: サービスの仕組み / Synapse Pipelines（データ統合） / Azure Data Share（データ共有） / 主要機能 / Synapse Pipelines / Copy Activity / Mapping Data Flow / スケジュール実行 / Azure Data Share / データスナップショット共有

## 第3章 設計判断の軸

### 3.1 データレイクと ETL

- クラウド上の **Azure Synapse Analytics** にデータをコピーする必要がある。 ([[Azure Data Factory 大容量データコピー設計.md]])
- オンプレミス環境にあるデータソースにアクセスするためには ([[Azure Data Factory 大容量データコピー設計.md]])
- このデータウェアハウスでは、以下のような複数のシステムからデータを収集する必要がある。 ([[Azure データウェアハウス ETL アーキテクチャ設計.md]])
- これらの異なるデータソースを統合し、分析基盤へロードするためには **ETL（Extract / Transform / Load）パイプライン**が必要になる。 ([[Azure データウェアハウス ETL アーキテクチャ設計.md]])
- このデータ統合ソリューションには、以下の機能要件がある。 ([[Azure データウェアハウス ETL アーキテクチャ設計.md]])
- これらの要件を満たす Azure サービスとして最も適しているのは ([[Azure データウェアハウス ETL アーキテクチャ設計.md]])

### 3.2 Synapse と Databricks 分析

- 今回のシナリオでは、次のようなデータ処理要件がある。 ([[Azure Databricks 分析基盤におけるデータレイクファイル形式設計.md]])
- これらの要件を満たすデータレイクのファイル形式として最適なのは ([[Azure Databricks 分析基盤におけるデータレイクファイル形式設計.md]])
- Parquet はこれらの要件を満たす設計になっている。 ([[Azure Databricks 分析基盤におけるデータレイクファイル形式設計.md]])
- 500GB 全体を読み込む必要がない。 ([[Azure Databricks 分析基盤におけるデータレイクファイル形式設計.md]])
- この要件は **データウェアハウス処理とデータレイク処理の両方を含むハイブリッド分析アーキテクチャ**である。 ([[Azure Synapse Analytics におけるデータレイク分析アーキテクチャ設計.md]])
- 今回の要件を満たす構成は次のようになる。 ([[Azure Synapse Analytics におけるデータレイク分析アーキテクチャ設計.md]])

### 3.3 データ共有とパイプライン

- このアーキテクチャでは、2つの主要な機能が必要になる。 ([[Azure Synapse Pipelines と Azure Data Share を組み合わせたデータ統合・共有アーキテクチャ.md]])
- この2つの要件を満たすために次の Azure サービスを組み合わせる。 ([[Azure Synapse Pipelines と Azure Data Share を組み合わせたデータ統合・共有アーキテクチャ.md]])
- この環境から研究データを ApexCore 側の Data Lake に取り込み、分析する必要がある。 ([[Azure Synapse Pipelines と Azure Data Share を組み合わせたデータ統合・共有アーキテクチャ.md]])

## 第4章 ユースケースで理解する

### 4.1 データレイクと ETLのユースケース

- Azure Data Factory 大容量データコピー設計 （オンプレミス SQL Server → Azure Synapse Analytics）: ある企業では、オンプレミス環境に存在する **SQL Server データベース**から クラウド上の **Azure Synapse Analytics** にデータをコピーする必要がある。 出典: [[Azure Data Factory 大容量データコピー設計.md]]
- Azure データウェアハウス ETL アーキテクチャ設計 （Azure SQL + Oracle 統合）: ある企業では、複数のデータソースからデータを統合する **クラウド型データウェアハウス**を設計している。 このデータウェアハウスでは、以下のような複数のシステムからデータを収集する必要がある。 出典: [[Azure データウェアハウス ETL アーキテクチャ設計.md]]
- Azure メダリオンアーキテクチャのデータレイク設計 （Azure Databricks + Delta Lake）: 近年、多くの企業はデータドリブン経営を実現するために **データレイクアーキテクチャ**を採用している。特に金融・医療・政府などの規制産業では、データの整合性や監査性が非常に重要であり、単なるファイルベースのデータレイクでは要件を満たせない場合がある。 出典: [[Azure メダリオンアーキテクチャのデータレイク設計.md]]

### 4.2 Synapse と Databricks 分析のユースケース

- Azure Databricks 分析基盤におけるデータレイクファイル形式設計 （Partitioned Parquet）: データエンジニアリングチームは、Azure Databricks を中心とした分析プラットフォームを構築している。複数のデータソース（業務システム、ログ、IoTデータ、外部APIなど）から毎日大量のデータが取り込まれ、データサイエンティストやアナリストがそれを利用して分析や機械学習を行う。 出典: [[Azure Databricks 分析基盤におけるデータレイクファイル形式設計.md]]
- Azure Synapse Analytics におけるデータレイク分析アーキテクチャ設計 （Dedicated SQL Pool + Serverless Spark Pool）: 企業は大量のデータを蓄積・分析するために **Azure Synapse Analytics** を利用してデータ分析基盤を構築している。Azure Synapse は、データウェアハウス機能とビッグデータ分析機能を統合したサービスであり、SQL 分析と Spark 分析の両方を同一プラットフォームで実行できる。 出典: [[Azure Synapse Analytics におけるデータレイク分析アーキテクチャ設計.md]]
- Azure データエンジニアリングアーキテクチャ （Data Lake + Synapse + Databricks + Data Factory）: 企業がオンプレミスの **Microsoft SQL Server** に保存しているトランザクションデータをクラウドへ移行し、分析基盤を構築する場合、Azureでは **モダンデータプラットフォームアーキテクチャ** を採用する。 出典: [[Azure データエンジニアリングアーキテクチャ.md]]
- SAP HANA → Azure Synapse Analytics 増分データパイプライン設計 （Azure Data Factory + CDC / Watermark パターン）: 企業のデータ基盤では、オンプレミスの業務システム（ERP、SAP、CRMなど）からクラウドのデータウェアハウスへデータを定期的に取り込む必要がある。特に分析基盤として **Azure Synapse Analytics** を利用する場合、運用システムへの負荷を最小限に抑えながらデータを取り込む設計が重要となる。 出典: [[SAP HANA → Azure Synapse Analytics 増分データパイプライン設計.md]]
- 大規模ログ分析に適した Azure サービス（Azure Data Explorer）: 企業のクラウド環境では、複数の Azure サービスから大量のログが生成されます。例えば、仮想マシン、App Service、Storage Account などのリソースは、それぞれ運用ログや診断ログ、パフォーマンスデータを継続的に生成します。 出典: [[大規模ログ分析に適した Azure サービス（Azure Data Explorer）.md]]

### 4.3 データ共有とパイプラインのユースケース

- Azure Synapse Pipelines と Azure Data Share を組み合わせたデータ統合・共有アーキテクチャ: 本シナリオでは、ApexCore Consulting が Azure 上のデータ分析基盤を構築し、パートナー企業 IrondClad Inc. とデータ連携を行う。 このアーキテクチャでは、2つの主要な機能が必要になる。 出典: [[Azure Synapse Pipelines と Azure Data Share を組み合わせたデータ統合・共有アーキテクチャ.md]]

## 第5章 学習チェックポイント

- まず データレイクと ETL → Synapse と Databricks 分析 → データ共有とパイプライン の順で読むと、基礎から応用まで流れを追いやすい。
- 第2章でサービスや構成要素の位置づけを理解し、第3章で制約条件を確認してから第4章のユースケースを読む。
- 同じサービスでも、要件によって選定理由が変わるため、出典リンク先の問題設定を必ず確認する。
- 類似ケースが複数ある場合は、第4章のユースケースを比較して判断軸を揃える。

## 関連用語

- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Factory]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Synapse Analytics]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Lake Storage Gen2]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Databricks]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Storage Account]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Share]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Power BI]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Box]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Logic Apps]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Medallion Architecture]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Monitor]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Log Analytics]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Explorer]]

## 出典ドキュメント

- [[Azure Data Factory 大容量データコピー設計.md]]
- [[Azure Databricks 分析基盤におけるデータレイクファイル形式設計.md]]
- [[Azure Synapse Analytics におけるデータレイク分析アーキテクチャ設計.md]]
- [[Azure Synapse Pipelines と Azure Data Share を組み合わせたデータ統合・共有アーキテクチャ.md]]
- [[Azure データウェアハウス ETL アーキテクチャ設計.md]]
- [[Azure データエンジニアリングアーキテクチャ.md]]
- [[Azure メダリオンアーキテクチャのデータレイク設計.md]]
- [[SAP HANA → Azure Synapse Analytics 増分データパイプライン設計.md]]
- [[大規模ログ分析に適した Azure サービス（Azure Data Explorer）.md]]
