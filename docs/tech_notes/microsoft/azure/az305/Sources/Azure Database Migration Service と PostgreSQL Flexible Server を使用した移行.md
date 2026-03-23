---
分類: Cloud
tags:
  - cloud/azure
  - cloud/azure/database-migration-service
  - cloud/azure/postgresql
  - cloud/azure/postgresql-flexible-server
  - cloud/architecture/database-migration
  - cloud/architecture/high-availability
  - cloud/architecture/scalability
  - database/postgresql
  - exam/azure/fundamentals
---

# Azure Database Migration Service と PostgreSQL Flexible Server を使用した移行

## 1. 背景（シナリオ）

企業がオンプレミスのデータベースをクラウドへ移行する場合、アプリケーションの可用性を維持しながら移行を行うことが非常に重要です。特にミッションクリティカルなデータベースでは、移行中のダウンタイムを最小限に抑える必要があります。

このシナリオでは、オンプレミスで稼働している PostgreSQL データベースを Azure へ移行する必要があります。また、移行プロセスではリアルタイムのデータレプリケーションを利用し、システム停止時間を最小化することが求められています。

さらに、移行後の環境では可変ワークロードに対応できるスケーラビリティ、強力なセキュリティ機能、そして業界標準へのコンプライアンスが必要です。

## 2. 要件整理

問題文のシナリオから、データベース移行アーキテクチャの要件を整理すると次の通りです。

このシナリオでは単純なデータコピーではなく、運用中のデータベースをクラウドへ移行する高度な移行プロセスが必要になります。

重要な要件は以下の通りです。

・オンプレミス PostgreSQL を Azure に移行する  
・ダウンタイムを最小限にする  
・リアルタイムデータレプリケーションが必要  
・移行後のワークロード変動に対応する  
・高可用性と信頼性が必要  
・高度なセキュリティとコンプライアンスが必要  

このような要件を満たすためには、移行専用サービスとマネージドデータベースサービスを組み合わせる必要があります。

## 3. 技術の基本概念

Azureでは、オンプレミスデータベースをクラウドへ移行するための専用サービスとして **Azure Database Migration Service（DMS）** が提供されています。

Azure Database Migration Serviceは、オンプレミスデータベースからAzureへの移行を支援するサービスであり、次のような機能を提供します。

・オンライン移行（ダウンタイム最小化）  
・リアルタイムデータレプリケーション  
・移行の自動化  
・複数データベースエンジンのサポート  

このサービスにより、移行期間中もオンプレミスデータベースを稼働させながらAzureへデータを同期できます。

移行先のデータベースとしては **Azure Database for PostgreSQL – Flexible Server** が適しています。このサービスは、Azureのフルマネージド PostgreSQL サービスです。

## 4. アーキテクチャまたは設計のポイント

このシナリオでは、次の2つのAzureサービスを組み合わせることで最適なアーキテクチャを構築できます。

まず Azure Database Migration Service を使用してオンプレミス PostgreSQL のデータを Azure に移行します。このサービスはオンライン移行をサポートしているため、移行中もオンプレミスのデータベースを使用できます。

次に、移行先として Azure Database for PostgreSQL – Flexible Server を使用します。このサービスはフルマネージドデータベースであり、高可用性やスケーラビリティを提供します。

Flexible Server の特徴には次のようなものがあります。

・自動バックアップ  
・高可用性構成  
・自動パッチ適用  
・スケーラブルなコンピューティング  
・仮想ネットワーク統合  
・データ暗号化  

この構成により、移行後の運用管理も簡素化されます。

## 5. 設計判断（なぜこの構成になるか）

この問題では、次の要件を満たす必要があります。

・ダウンタイムを最小化する移行  
・リアルタイムデータレプリケーション  
・スケーラブルなクラウドデータベース  
・セキュリティとコンプライアンス  

Azure Database Migration Serviceはオンライン移行をサポートしており、移行中に継続的なデータレプリケーションを実行できます。そのため、ダウンタイムを最小限に抑えた移行が可能です。

さらに、Azure Database for PostgreSQL – Flexible Serverはフルマネージドサービスであり、高可用性、セキュリティ、スケーラビリティを提供します。

そのため、この2つのサービスを組み合わせることで、すべての要件を満たすことができます。

## 6. 他の選択肢が誤りな理由

Azure Site Recoveryは、主に仮想マシンやアプリケーションのディザスタリカバリーを目的としたサービスです。データベース移行専用サービスではありません。

Azure Data Factoryはデータ統合やETL処理のためのサービスです。データコピーは可能ですが、リアルタイムレプリケーションを利用したデータベース移行には最適ではありません。

そのため、このシナリオでは Database Migration Service の方が適切です。

## 7. 最終回答

A. Azure Database Migration Service  
D. Azure Database for PostgreSQL – Flexible Server

## 8. まとめ

オンプレミス PostgreSQL データベースをAzureへ移行する場合、ダウンタイムを最小化しながら安全に移行することが重要です。Azure Database Migration Service を使用することで、オンライン移行とリアルタイムデータレプリケーションが可能になります。

また、移行先として Azure Database for PostgreSQL – Flexible Server を使用することで、スケーラブルで高可用性なマネージドデータベース環境を構築できます。

この2つのサービスを組み合わせることで、移行の効率性、パフォーマンス、セキュリティ、そして運用の簡素化を同時に実現できます。