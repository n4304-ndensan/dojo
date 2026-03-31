---
分類: Cloud
tags:
  - cloud/azure
  - cloud/azure/cosmos-db
  - cloud/azure/synapse-analytics
  - cloud/azure/synapse-link
  - cloud/architecture/analytical-processing
  - cloud/architecture/operational-vs-analytical
  - database/nosql
  - exam/azure/fundamentals
---

# Azure Synapse Link for Azure Cosmos DB による分析アーキテクチャ

## 1. 背景（シナリオ）

クラウドアプリケーションでは、運用データベース（OLTP）と分析処理（OLAP）を分離することが重要です。運用データベースはアプリケーションのトランザクション処理を担当し、低遅延で高速な応答が求められます。一方で、分析処理では大量のデータを集計・分析するため、複雑なクエリや大規模なデータ処理が発生します。

この問題では、Azure Cosmos DB を運用データベースとして使用しながら、Azure Synapse Analytics を利用して毎日データ分析を行う必要があります。しかし、分析処理によって Cosmos DB のトランザクション性能が低下してはいけません。

そのため、運用ワークロードと分析ワークロードを分離しつつ、データを効率的に分析できるアーキテクチャが求められます。

## 2. 要件整理

問題文のシナリオから、次の重要な要件を整理できます。

まず、Cosmos DB はアプリケーションの運用データベースとして使用されています。したがって、トランザクション処理のパフォーマンスは維持する必要があります。

このシナリオでは次の要件が存在します。

・Azure Cosmos DB に運用データが保存されている  
・Azure Synapse Analytics を使用して分析する  
・分析は毎日実行される  
・トランザクションワークロードへの影響を最小化する  
・ETL処理や複雑なデータ移動はできるだけ避ける  

つまり、運用データベースを直接クエリするのではなく、分析専用の仕組みを利用する必要があります。

## 3. 技術の基本概念

Azure Cosmos DB では、運用データと分析処理を分離するための機能として **Azure Synapse Link for Azure Cosmos DB** が提供されています。

Synapse Link は、Cosmos DB のデータを **分析ストア（Analytical Store）** に自動的に同期する機能です。この分析ストアは、分析ワークロード向けに最適化されています。

Synapse Link の主な特徴には次のようなものがあります。

・Cosmos DB の運用データを分析用ストアへ同期する  
・ETL処理なしで分析が可能  
・Azure Synapse Analytics とネイティブ統合  
・トランザクションワークロードに影響を与えない  

この仕組みにより、アプリケーションのパフォーマンスを維持しながら分析処理を実行できます。

## 4. アーキテクチャまたは設計のポイント

Synapse Link を使用すると、Cosmos DB のデータは2つのストレージに保存されます。

1つ目は **トランザクションストア（Operational Store）** です。これはアプリケーションが使用する通常のデータベースです。

2つ目は **分析ストア（Analytical Store）** です。これは分析ワークロード用に最適化されたストレージです。

この構成には次のメリットがあります。

・運用処理と分析処理を分離できる  
・ETLパイプラインが不要  
・ほぼリアルタイムでデータ分析が可能  
・Synapse SQLから直接クエリできる  

この仕組みにより、Cosmos DB のトランザクションパフォーマンスを維持しながらデータ分析が可能になります。

## 5. 設計判断（なぜこの構成になるか）

この問題の最も重要な要件は次の点です。

「Cosmos DB のトランザクションワークロードのパフォーマンスに影響を与えないこと」

通常のETL処理やデータ抽出を使用すると、Cosmos DB のリソース（RU）を消費し、トランザクション処理に影響を与える可能性があります。

しかし Azure Synapse Link を使用すると、データは自動的に分析ストアへ同期されるため、運用データベースを直接クエリする必要がありません。

その結果、次のメリットが得られます。

・トランザクション処理への影響を最小化  
・リアルタイムに近い分析  
・ETL不要  
・運用管理の簡素化  

そのため、このシナリオには Synapse Link が最適です。

## 6. 他の選択肢が誤りな理由

まず Cosmos DB Change Feed です。Change Feed はデータ変更を追跡する仕組みですが、分析のためには追加の処理パイプラインが必要になります。そのため運用負荷や複雑性が増加します。

次に Azure Data Factory です。Data Factory を使用すると Cosmos DB から Synapse にデータをコピーできますが、ETL処理が必要になり、定期的なデータ移動によってCosmos DBのパフォーマンスに影響を与える可能性があります。

最後に PolyBase です。PolyBase は外部データソースからデータをロードする機能ですが、リアルタイム分析には適しておらず、データ移動の管理も必要になります。

## 7. 最終回答

A. Azure Synapse Link for Azure Cosmos DB

## 8. まとめ

Azure Cosmos DB の運用データを Azure Synapse Analytics で分析する場合、トランザクション処理への影響を最小限に抑えることが重要です。

Azure Synapse Link は、Cosmos DB のデータを分析ストアへ自動的に同期する機能であり、運用ワークロードと分析ワークロードを分離できます。これにより、ETL処理なしでリアルタイムに近い分析が可能になります。

そのため、Cosmos DB のパフォーマンスに影響を与えずにデータ分析を行う場合には **Azure Synapse Link for Azure Cosmos DB** が最適なソリューションです。