---
分類: Cloud
tags:
  - cloud/azure
  - cloud/azure/sql
  - cloud/azure/sql/managed-instance
  - cloud/azure/disaster-recovery
  - cloud/azure/high-availability
  - cloud/azure/failover-group
  - cloud/azure/virtual-machines
  - cloud/architecture/high-availability
  - cloud/architecture/disaster-recovery
  - cloud/architecture/resilience
  - database/sql-server
  - exam/azure/az900
---

# Azure SQL Managed Instance と Auto-Failover Group による災害復旧設計

## 1. 背景（シナリオ）

ある組織では、複数の Azure 仮想マシン上で動作する多層アプリケーションを運用しています。このアプリケーションの中心には SQL Server データベースがあり、ビジネス運用において極めて重要な役割を果たしています。

このシステムには厳しい可用性要件があり、地域レベルの障害（リージョン障害）が発生した場合でも、アプリケーションの停止時間を最小限に抑えつつ、データの整合性を維持する必要があります。

さらに、単に高可用性を実現するだけではなく、運用コストを最適化した災害復旧（Disaster Recovery）戦略を導入することも求められています。そのため、信頼性・自動化・コスト効率のバランスを取ったソリューションが必要となります。

## 2. 要件整理

このシナリオの要件を整理すると、次のようなポイントが重要になります。

- SQL Server を利用するアプリケーション  
- 地域障害に耐えられる災害復旧  
- 高可用性（High Availability）  
- フェイルオーバー時のデータ整合性  
- ダウンタイムの最小化  
- コスト効率の高いソリューション  

これらの条件から、リージョン間レプリケーションと自動フェイルオーバーを組み合わせたデータベースサービスが必要であることが分かります。

## 3. 技術の基本概念

Azure では、SQL Server ワークロードをクラウドで実行するための複数のデータベースサービスが提供されています。その中でも Azure SQL Managed Instance は、オンプレミス SQL Server との互換性を高く維持しながら、フルマネージドのデータベース環境を提供するサービスです。

Azure SQL Managed Instance は、次のような特徴を持っています。

- SQL Server と高い互換性  
- 自動バックアップ  
- 自動パッチ適用  
- 組み込みの高可用性  
- フルマネージドサービスによる運用負荷の削減  

さらに、Auto-Failover Group という機能を使用すると、異なる Azure リージョン間でデータベースをレプリケーションし、障害発生時に自動的にフェイルオーバーすることができます。

## 4. アーキテクチャまたは設計のポイント

このシナリオのようなミッションクリティカルなアプリケーションでは、複数レベルの冗長性を設計することが重要です。

まず、プライマリリージョンに Azure SQL Managed Instance を配置し、アプリケーションのメインデータベースとして使用します。

次に、別の Azure リージョンにセカンダリの Managed Instance を配置します。このセカンダリ環境はプライマリデータベースとレプリケーションされます。

Auto-Failover Group を構成すると、次のような機能を実現できます。

- リージョン間のデータベースレプリケーション  
- 自動フェイルオーバー  
- 読み取り専用セカンダリ  
- 共通接続エンドポイント  

この構成により、プライマリリージョンに障害が発生した場合でも、セカンダリリージョンへ自動的に切り替えることができます。

## 5. 設計判断（なぜこの構成になるか）

この問題では「高可用性」「データ整合性」「ダウンタイム最小化」「コスト最適化」の4つが重要です。

Azure SQL Managed Instance はフルマネージドサービスであり、SQL Server の互換性を維持しながら高可用性機能を提供します。また、Auto-Failover Group を利用することで、リージョン間レプリケーションと自動フェイルオーバーを実現できます。

この仕組みにより、次のメリットがあります。

- リージョン障害に対応できる  
- 自動フェイルオーバーによるダウンタイム最小化  
- データ整合性の維持  
- 管理作業の削減によるコスト最適化  

そのため、このシナリオの要件を最もバランス良く満たすソリューションとなります。

## 6. 他の選択肢が誤りな理由

この問題では他の Azure サービスも選択肢として提示されていますが、要件を十分に満たしていません。

A. Azure SQL Database Geo-Replication  

Geo-Replication はリージョン間レプリケーションを提供しますが、自動フェイルオーバーや管理機能の観点で Auto-Failover Group よりも運用負荷が高くなる可能性があります。

B. Azure Backup（Geo-Redundant Storage）  

バックアップはデータ保護には有効ですが、高可用性や即時フェイルオーバーを提供するものではありません。災害復旧には利用できますが、ダウンタイムを最小化する仕組みではありません。

D. Azure Traffic Manager  

Traffic Manager はグローバルなトラフィックルーティングサービスであり、アプリケーションレベルの負荷分散には役立ちますが、データベースの高可用性やデータ整合性を提供するものではありません。

## 7. 最終回答

C. Azure SQL Managed Instance と Auto-Failover Group

## 8. まとめ

ミッションクリティカルなアプリケーションのデータベースでは、高可用性と災害復旧の両方を考慮した設計が必要です。

Azure SQL Managed Instance は SQL Server 互換性を維持しながらフルマネージドの高可用性環境を提供します。さらに Auto-Failover Group を組み合わせることで、リージョン障害にも対応できる自動フェイルオーバーを実現できます。

そのため、Azure SQL Managed Instance と Auto-Failover Group の組み合わせは、データ整合性を維持しつつダウンタイムを最小化し、コスト効率の高い災害復旧ソリューションを提供します。