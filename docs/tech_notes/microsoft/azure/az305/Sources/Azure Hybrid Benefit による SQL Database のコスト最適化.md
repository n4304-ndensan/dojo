---
分類: Cloud
tags:
  - cloud/azure
  - cloud/azure/sql-database
  - cloud/azure/hybrid-benefit
  - cloud/architecture/multi-tenant
  - cloud/architecture/cost-optimization
  - cloud/architecture/scalability
  - database/relational
  - exam/azure/fundamentals
---

# Azure Hybrid Benefit による SQL Database のコスト最適化

## 1. 背景（シナリオ）

Azureでマルチテナントアプリケーションを運用する場合、多数のデータベースを効率的に管理しながらコストを最適化する必要があります。特に、複数の顧客（テナント）が同じアプリケーションを利用する構成では、データベースごとのワークロードが大きく変動することがよくあります。

このような環境では、データベースリソースを柔軟にスケーリングできることが重要です。また、クラウド運用ではインフラ管理の負担をできるだけ減らすことも重要な要素になります。

さらに、このシナリオでは既存のオンプレミスSQL Serverライセンスをすでに所有しているため、それらをAzure環境でも活用してコストを削減することが求められています。

## 2. 要件整理

問題文の要件を整理すると、データベースアーキテクチャの設計において次のポイントが重要になります。

このシナリオでは単なるデータベースデプロイではなく、コスト最適化とスケーラビリティの両立が求められています。

具体的な要件は次の通りです。

・マルチテナントアプリケーションである  
・複数データベース間でワークロードが変動する  
・動的なリソース割り当てが必要  
・個々のデータベースをスケールできる  
・運用管理の負担を最小化する  
・既存のSQL Serverライセンスを活用する  

この中で最も重要なポイントは **既存のSQL ServerライセンスをAzureで活用すること**です。

## 3. 技術の基本概念

Azure SQL Databaseでは、データベースの料金モデルや購入方法を選択できます。代表的なモデルには次のようなものがあります。

・Pay-as-you-go（従量課金）  
・vCore購入モデル  
・Reserved Instances（予約容量）  
・Azure Hybrid Benefit  

これらの中で **Azure Hybrid Benefit** は、既存のオンプレミスSQL ServerライセンスをAzureで再利用できる仕組みです。

通常、AzureでSQL Databaseを使用する場合はSQL Serverライセンス料金が含まれています。しかしAzure Hybrid Benefitを使用すると、既存のライセンスを利用することでライセンス費用を削減できます。

## 4. アーキテクチャまたは設計のポイント

Azure Hybrid Benefitは、Software Assurance付きのSQL ServerライセンスをAzureで再利用できる仕組みです。この仕組みにより、企業はクラウド移行時のコストを大幅に削減できます。

Azure Hybrid Benefitの主なメリットには次のようなものがあります。

・既存のSQL ServerライセンスをAzureで利用できる  
・SQL Databaseのライセンス費用を削減できる  
・最大約40%のコスト削減が可能  
・vCoreモデルと組み合わせて使用できる  
・Azure SQL DatabaseやSQL Managed Instanceに適用可能  

この仕組みにより、既存のオンプレミス投資を無駄にすることなくクラウドへ移行できます。

## 5. 設計判断（なぜこの構成になるか）

この問題では複数の要件が提示されていますが、最も重要なポイントは次の要件です。

「既存のオンプレミスSQL Serverライセンスを活用すること」

この要件を満たすAzureの機能が **Azure Hybrid Benefit** です。

Azure Hybrid Benefitを利用すると、企業がすでに購入しているSQL ServerライセンスをAzureでも使用できるため、クラウドの利用コストを大幅に削減できます。

また、Azure SQL DatabaseのPaaSサービスを利用することで、パッチ管理やバックアップなどの運用作業もAzureが自動で実行するため、運用負荷も最小化できます。

## 6. 他の選択肢が誤りな理由

まず Pay-as-you-go について説明します。このモデルは使用量に応じて課金される方式で柔軟性は高いですが、既存のSQL Serverライセンスを活用する機能はありません。

次に vCore モデルです。vCoreモデルはCPUやメモリのリソースを明確に管理できる購入モデルですが、単体ではライセンスコスト削減の仕組みは提供していません。

最後に Reserved Instances です。予約インスタンスは長期間の利用を前提に割引を提供する仕組みですが、ワークロードが変動する環境では柔軟性が低くなる可能性があります。

## 7. 最終回答

D. Azure Hybrid Benefit

## 8. まとめ

Azure Hybrid Benefitは、既存のオンプレミスSQL ServerライセンスをAzure環境で再利用できる仕組みです。この機能を使用することで、Azure SQL Databaseのライセンスコストを大幅に削減できます。

マルチテナントアプリケーションのような環境では、コスト効率とスケーラビリティの両方が重要になります。Azure Hybrid Benefitを利用することで、既存のライセンス投資を活用しながら、Azureのスケーラブルなデータベースサービスを利用することができます。

そのため、このシナリオの要件を満たす最適な選択は **Azure Hybrid Benefit** です。