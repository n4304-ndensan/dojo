# Azure SQL 移行サービス整理（試験・実務ミニガイド）

オンプレミス SQL Server を Azure に移行する場合、主に次の4つのサービスが候補になります。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
- Azure SQL Database（単一データベース / Hyperscale）
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Managed Instance]]
- Azure SQL Managed Instance
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
- SQL Server on Azure Virtual Machines
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Synapse Analytics]]
- Azure Synapse Analytics

これらは **互換性・管理負荷・用途**が異なるため、移行要件に応じて選択します。

---

## Azure SQL Database（Azure SQL データベース）

Azure SQL Database は、Azure が完全に管理する **PaaS 型のリレーショナルデータベースサービス**です。  
OS管理、パッチ適用、バックアップ、可用性構成などはすべて Azure が自動管理します。

このサービスはクラウドネイティブなアプリケーション向けに設計されており、  
スケーラビリティや運用の簡素化に優れています。

ただし、オンプレミス SQL Server のすべての機能をサポートしているわけではありません。  
特に次のような機能は制限があります。

- SQL Server Agent
- 一部の CLR
- クロスデータベースクエリ

そのため、既存 SQL Server をそのまま移行する場合は **再設計が必要になることがあります。**

主に次のようなケースで利用されます。

- クラウドネイティブアプリケーション
- 新規アプリケーション
- 運用管理を最小化したい場合

---

## Azure SQL Database Hyperscale（Azure SQL データベース ハイパースケール）

Hyperscale は Azure SQL Database の **大容量・高スケーラビリティ版**です。  
ストレージとコンピュートを分離したアーキテクチャを採用し、非常に大きなデータベースを扱うことができます。

最大100TBまでスケール可能で、大規模データベースの処理性能を向上させる設計になっています。

しかし、基本的な機能は Azure SQL Database と同じため、  
次のような制限は同様に存在します。

- SQL Server Agent 非対応
- CLR制限
- クロスデータベースクエリ制限

そのため、**サイズ問題は解決できますが、機能互換性の問題は解決できません。**

主に次のようなケースで利用されます。

- 非常に大きなデータベース
- 高いスケーラビリティが必要なアプリケーション

---

## Azure SQL Managed Instance（Azure SQL マネージド インスタンス）

Azure SQL Managed Instance は、オンプレミス SQL Server と **高い互換性を持つ PaaS 型データベースサービス**です。

SQL Server の多くの機能をサポートしており、既存環境をほぼそのまま移行できる設計になっています。

特に次のような機能に対応しています。

- SQL Server Agent
- CLR アセンブリ
- クロスデータベースクエリ
- Linked Server
- Service Broker

また、PaaSサービスのため次の運用作業は Azure が自動管理します。

- OS パッチ
- データベースバックアップ
- 高可用性
- 障害回復

そのため、**SQL Server の互換性を維持しつつ、管理負荷を減らしたい場合に最適なサービス**です。

主に次のようなケースで利用されます。

- オンプレミス SQL Server の移行
- SQL Server 機能を多く使用している環境
- 再設計を最小化したい場合

---

## SQL Server on Azure Virtual Machines（Azure 仮想マシン上の SQL Server）

SQL Server on Azure VM は、Azure 仮想マシン上に **通常の SQL Server をインストールして使用する IaaS 型サービス**です。

この方式ではオンプレミス SQL Server と完全に同じ環境を構築できるため、  
**100% の機能互換性を維持することができます。**

CLR、SQL Server Agent、クロスデータベースクエリなど、すべての SQL Server 機能が利用可能です。

しかし、次のような管理作業はユーザーが実施する必要があります。

- OS管理
- パッチ適用
- バックアップ
- 高可用性構成
- 監視

そのため、互換性は最も高いですが、**管理オーバーヘッドも最も大きい**方式になります。

主に次のようなケースで利用されます。

- 既存システムをそのまま移行する場合
- SQL Server の完全互換が必要な場合
- 特殊な構成やカスタマイズが必要な場合

---

## Azure Synapse Analytics（Azure Synapse 分析）

Azure Synapse Analytics は、大規模データ分析やデータウェアハウス用途のためのサービスです。

トランザクション処理（OLTP）ではなく、  
**分析処理（OLAP）やビッグデータ分析**に最適化されています。

そのため、通常の SQL Server アプリケーションの移行先としては適していません。

主な用途は次の通りです。

- データウェアハウス
- BI分析
- 大規模データ処理

---

# SQL サービス選択の基本ルール

オンプレ SQL Server 移行では、次の基準で選択することが多いです。

オンプレ SQL Server と完全互換が必要  
→ **SQL Server on Azure VM**

SQL Server 機能を多く使っているが管理を減らしたい  
→ **Azure SQL Managed Instance**

クラウドネイティブアプリ  
→ **Azure SQL Database**

大規模分析  
→ **Azure Synapse Analytics**

---

# 試験用の覚え方

SQL Server 機能が多い  

- CLR
- SQL Server Agent
- Cross-database query

このような要件が出た場合は

**Azure SQL Managed Instance**

が最も適したサービスになることが多いです。