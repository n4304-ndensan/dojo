---
分類: Cloud
tags:
  - cloud/azure
  - cloud/azure/app-service
  - cloud/azure/sql-database
  - cloud/azure/sql-elastic-pool
  - cloud/architecture/multi-tier
  - cloud/architecture/scalability
  - cloud/architecture/cost-optimization
  - cloud/architecture/paas
  - exam/azure
---

# Azure での多層アプリケーション設計（Web層とデータベース層の最適化）

## 1. 背景（シナリオ）

ある企業が、既存の **多層アプリケーション（Multi-tier Application）** を Azure に移行しようとしています。

このアプリケーションは主に次の2つの層で構成されています。

- **Web層**：ユーザーからのリクエストを処理するフロントエンド
- **データベース層**：アプリケーションデータを保存するバックエンド

このシステムには以下の特徴があります。

- Web 層のトラフィックは **予測不可能（spiky workload）**
- データベース層は **安定したパフォーマンス** が必要
- **コスト最適化**が重要
- データベースの **パフォーマンスを独立して最適化**できる必要がある

そのため、Azure 上で **スケーラブルかつコスト効率の良い構成**を選択する必要があります。

---

## 2. 要件整理

問題文から読み取れる設計要件を整理すると、適切な Azure サービスが見えてきます。

まず、Web 層の要件です。

Web 層は **負荷が予測できない**ため、以下の機能が重要になります。

- オートスケール
- 高可用性
- 運用負荷の低減

次に、データベース層の要件です。

データベース層には以下の要件があります。

- 一貫したパフォーマンス
- コスト最適化
- パフォーマンス指標に基づいた調整

これらの要件をまとめると次のようになります。

- Web 層：スケーラブルな PaaS
- DB 層：コスト効率よく性能を管理
- DB パフォーマンスを独立して調整可能

---

## 3. 技術の基本概念

この問題に関連する Azure の重要なサービスを整理します。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure App Service]]
### Azure App Service

Azure App Service は **Web アプリケーション向けの PaaS サービス**です。

特徴は次の通りです。

- 自動スケーリング
- OS管理不要
- CI/CD対応
- 高可用性

Web 層のような **変動トラフィックを持つアプリケーション**に非常に適しています。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
### Azure SQL Database

Azure SQL Database は **フルマネージドのリレーショナルデータベースサービス**です。

特徴は次の通りです。

- 自動バックアップ
- 自動パッチ
- 高可用性
- パフォーマンス階層の選択

しかし、単一データベースの場合は **固定リソース割り当て**になります。

---

### Azure SQL Database Elastic Pool

Elastic Pool は **複数の SQL Database がリソースを共有する仕組み**です。

通常、データベースごとに CPU / IO を確保するとコストが高くなります。

Elastic Pool では以下の仕組みを使います。

- 複数DBでリソース共有
- 使用量に応じた分配
- コスト最適化

これは **負荷がばらつく複数のDB環境**に最適です。

---

## 4. アーキテクチャまたは設計のポイント

今回のシナリオでは、次のような構成が理想です。

**Web 層**

- Azure App Service
- オートスケール対応
- PaaSによる運用簡略化

**データベース層**

- Azure SQL Database Elastic Pool
- 複数DBのリソース共有
- コスト効率の向上

この設計のメリットは次の通りです。

まず、Web 層では App Service によって **トラフィックの急増に自動対応**できます。

次に、DB 層では Elastic Pool によって **リソースの共有とコスト最適化**が可能になります。

また、Elastic Pool 内では **データベースごとに性能調整が可能**です。

---

## 5. 設計判断（なぜこの構成になるか）

この構成が最適な理由は、**PaaS + コスト最適化 + スケーラビリティ**を同時に満たすためです。

まず Web 層についてです。

Web トラフィックは予測が難しいため、**自動スケーリング可能な PaaS**が理想です。

App Service は以下を提供します。

- Auto Scale
- 高可用性
- 管理不要

次に DB 層です。

Elastic Pool は次のメリットがあります。

- 複数 DB のリソース共有
- DB ごとのパフォーマンス最適化
- コスト削減

そのため、**変動ワークロード + コスト最適化**に適しています。

---

## 6. 他の選択肢が誤りな理由

### A. App Service + 単一 Azure SQL Database

この構成は基本的には良い構成ですが、問題があります。

単一 DB の場合：

- リソース共有ができない
- DB 増加時にコストが高くなる
- 柔軟な最適化が難しい

Elastic Pool の方が **将来的な拡張とコスト効率に優れています**。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
### B. VM Scale Sets + SQL Elastic Pool

VM Scale Sets は **IaaS ベースのスケーリング**です。

問題点は次の通りです。

- OS管理
- パッチ管理
- 運用コスト増加

Web 層では **PaaS の App Service の方が適切**です。

---

### D. VM Scale Sets + SQL Hyperscale

Hyperscale は **非常に大規模な単一データベース**向けです。

特徴：

- 数TB〜PBデータ
- 高速読み取り
- 大規模分析

しかし今回の要件は

- コスト最小化
- DB最適化

であり、**Hyperscale は過剰スペック**です。

---

## 7. 最終回答

**C. Web 層に Azure App Service プラン、データベース層に Azure SQL Database Elastic Pool**

---

## 8. まとめ

Azure で多層アプリケーションを設計する場合、各層の特性に応じたサービス選択が重要です。

今回のポイントは次の通りです。

まず Web 層は **予測不能な負荷**を処理する必要があります。  
そのため **Azure App Service のオートスケール**が最適です。

次に DB 層は **コスト最適化と性能管理**が重要です。

Elastic Pool を使用することで

- 複数 DB のリソース共有
- コスト効率の向上
- DB 単位の性能調整

が可能になります。

このように

**App Service + SQL Elastic Pool**

の組み合わせは、**スケーラブルでコスト効率の高い Azure アーキテクチャ**の典型例です。