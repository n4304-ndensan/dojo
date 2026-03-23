# Topic-12 監視と運用

## 学習ゴール

Azure Monitor の基盤からアプリ可観測性、VM/SQL ログ、Private Link までを順番に見て、監視設計を体系化する。

## この Topic の全体像

Azure Monitor、Application Insights、Log Analytics、診断設定を中心に運用監視を扱う。

対象ドキュメント数: 10 件

## 第1章 学習マップ

### 1.1 学習順序

1. Azure Monitor 基盤: まず Monitor、Log Analytics、Agent の役割分担を固める。
2. アプリケーション可観測性: 次に App Service や Application Insights による APM を理解する。
3. VM とインフラログ監視: VM、Windows イベント、セキュリティ監視を読む。
4. SQL と運用ログ設計: SQL Diagnostics やログ保持設計を整理する。
5. 閉域監視: 最後に AMPLS と Private Link による閉域監視を確認する。

### 1.2 セクション対応表

- Azure Monitor 基盤: 2 件 / [[Azure Monitor _ Application Monitoring アーキテクチャ.md]] / [[Azure Monitor Agent を利用したログ収集設.md]]
- アプリケーション可観測性: 2 件 / [[Azure App Service アプリケーション監視アーキテクチャ.md]] / [[Azure App Service のパフォーマンス監視と依存関係トラッキング.md]]
- VM とインフラログ監視: 2 件 / [[Azure Monitor による VM セキュリティイベント監視.md]] / [[Azure Monitor による Windows VM ログ集中監視設計.md]]
- SQL と運用ログ設計: 3 件 / [[Azure SQL Database Diagnostics とログ保持設計.md]] / [[Azure SQL Database の診断設定とログ送信先の制約.md]] / [[Azure SQL Database の診断設定とログ送信先の制約2.md]]
- 閉域監視: 1 件 / [[Azure Monitor Private Link Scope（AMPLS）設計ガイド.md]]

## 第2章 基礎概念と構成要素

### 2.1 Azure Monitor 基盤

まず Monitor、Log Analytics、Agent の役割分担を固める。

主な出典: [[Azure Monitor _ Application Monitoring アーキテクチャ.md]] / [[Azure Monitor Agent を利用したログ収集設.md]]

主要論点: Azure Monitor / Azure Log Analytics / Log Analyticsの用途 / Kusto Query Language（KQL） / Azure Application Insights / Application Insightsで取得できる情報 / 分散トレーシング / ユーザー行動分析 / Azure Service Map / Azure Activity Log

### 2.2 アプリケーション可観測性

次に App Service や Application Insights による APM を理解する。

主な出典: [[Azure App Service アプリケーション監視アーキテクチャ.md]] / [[Azure App Service のパフォーマンス監視と依存関係トラッキング.md]]

主要論点: Azure Monitor とは / Application Insights / Application Insights が提供する主要機能 / パフォーマンス監視 / 依存関係追跡 / 異常検出（AI-based detection） / 分散トレーシング / カスタムテレメトリ / Azure Monitor と Application Insights の統合 / Azure Log Analytics

### 2.3 VM とインフラログ監視

VM、Windows イベント、セキュリティ監視を読む。

主な出典: [[Azure Monitor による VM セキュリティイベント監視.md]] / [[Azure Monitor による Windows VM ログ集中監視設計.md]]

主要論点: Windows Event Log と Linux Syslog の Log Analytics テーブル設計 / Azure Monitor のログ収集アーキテクチャ / Windows と Linux のログの違い / Windows Event Log / Log Analytics の Windows テーブル / Linux Syslog / Log Analytics の Linux テーブル / Log Analytics 主要テーブル / AzureActivity テーブル / AzureDiagnostics テーブル

### 2.4 SQL と運用ログ設計

SQL Diagnostics やログ保持設計を整理する。

主な出典: [[Azure SQL Database Diagnostics とログ保持設計.md]] / [[Azure SQL Database の診断設定とログ送信先の制約.md]] / [[Azure SQL Database の診断設定とログ送信先の制約2.md]]

主要論点: SQLInsights とは / Diagnostics の送信先 / Storage Account / Log Analytics Workspace / Storage Account のログ保持 / Log Analytics のログ保持 / Storage と Log Analytics の違い / データフロー / B / C

### 2.5 閉域監視

最後に AMPLS と Private Link による閉域監視を確認する。

主な出典: [[Azure Monitor Private Link Scope（AMPLS）設計ガイド.md]]

主要論点: ― Log Analytics Workspace へのログ取り込みを Microsoft バックボーンに閉じる設計 ― / まず AMPLS とは何か / なぜ AMPLS は 1つでよいのか / ではなぜ Private Endpoint は 1つでは足りないのか / この問題の答えを整理するとどうなるか / AMPLS の最小数 / Private Endpoint の最小数 / ありがちな誤解 / 「VNet が3つあるなら Private Endpoint も3つ必要では？」 / 「Private Endpoint が2つ必要なら AMPLS も2つ必要では？」

## 第3章 設計判断の軸

### 3.1 Azure Monitor 基盤

- クラウドアプリケーションでは、アプリケーションだけでなく **インフラ・依存サービス・ユーザー行動** などを総合的に監視する必要があります。 ([[Azure Monitor _ Application Monitoring アーキテクチャ.md]])
- 今回のシナリオでは、Azure サブスクリプションに **WS1 という名前の Log Analytics Workspace** が存在し、パブリックエンドポイントからアクセス可能な構成となっている。このワークスペースに対して、複数リージョンに配置された Windows 仮想マシンからログを収集する必要がある。 ([[Azure Monitor Agent を利用したログ収集設.md]])
- しかし、単純にすべてのログを収集すると不要なログデータが大量に取り込まれ、Log Analytics のコストが増加してしまう。そのため今回の要件では ([[Azure Monitor Agent を利用したログ収集設.md]])
- する必要があるため、不要なログを収集する設計は避ける必要がある。 ([[Azure Monitor Agent を利用したログ収集設.md]])
- リージョンごとにログ収集要件が異なるため、DCR を分ける必要がある。 ([[Azure Monitor Agent を利用したログ収集設.md]])
- したがって必要な DCR の最小数は ([[Azure Monitor Agent を利用したログ収集設.md]])

### 3.2 アプリケーション可観測性

- クラウドでアプリケーションを運用する場合、単にアプリケーションをデプロイするだけでは不十分である。実際の運用では次のような問題を常に監視する必要がある。 ([[Azure App Service アプリケーション監視アーキテクチャ.md]])
- 特に **Azure App Service** 上で Web アプリケーションを実行する場合、次の観点で監視が必要になる。 ([[Azure App Service アプリケーション監視アーキテクチャ.md]])
- これらの要件を満たす Azure サービスは ([[Azure App Service アプリケーション監視アーキテクチャ.md]])
- 運用チームは、アプリケーションのパフォーマンスを継続的に監視し、問題が発生した際には迅速に原因を特定できるようにしたいと考えています。特に、アプリケーションのレスポンスが遅くなった場合に、どのコンポーネントがボトルネックになっているのかを分析できる仕組みが必要です。 ([[Azure App Service のパフォーマンス監視と依存関係トラッキング.md]])
- さらに、アプリケーションは既に本番環境で稼働しているため、監視機能を導入する際には **アプリケーションコードの変更を必要としない方法**が求められています。コード変更が必要な場合は、再ビルドや再デプロイが必要になるため、運用負荷が増加してしまいます。 ([[Azure App Service のパフォーマンス監視と依存関係トラッキング.md]])
- このような条件を満たしながら、アプリケーションのパフォーマンス分析と依存関係の追跡を行える監視ソリューションを選択する必要があります。 ([[Azure App Service のパフォーマンス監視と依存関係トラッキング.md]])

### 3.3 VM とインフラログ監視

- 異常が発生した場合に **リアルタイムでアラートを受信できる監視システム**を構築する必要がある。 ([[Azure Monitor による Windows VM ログ集中監視設計.md]])
- が新しいエージェントとして推奨されている。 ([[Azure Monitor による Windows VM ログ集中監視設計.md]])

### 3.4 SQL と運用ログ設計

- つまり、検討すべきポイントは二つある。一つは、同じログカテゴリを複数の Log Analytics ワークスペースに送信できるかどうかである。もう一つは、同じログを複数のストレージアカウントへ送信できるかどうかである。この二つの制約を理解していないと、正しい選択肢を判断することができない。 ([[Azure SQL Database の診断設定とログ送信先の制約.md]])
- Azure の診断設定では、1つのリソースに対して複数の診断設定を作成することができる。しかし、ログ送信先の種類によっては制約が存在する。 ([[Azure SQL Database の診断設定とログ送信先の制約.md]])
- 一方で、ストレージアカウントについてはこのような制約は存在しない。同じログカテゴリを複数のストレージアカウントへ送信することが可能であり、新しい診断設定を作成して別のストレージアカウントにログを保存することができる。これはストレージアカウントがログ分析ではなく単なる保存先として機能するためである。 ([[Azure SQL Database の診断設定とログ送信先の制約.md]])
- 一方でストレージアカウントは、ログデータをファイルとして保存するだけのシンプルなストレージである。そのため同じログを複数のストレージアカウントに保存することに技術的な問題はなく、バックアップやリージョン分散などの目的で複数のストレージに保存することが可能になっている。 ([[Azure SQL Database の診断設定とログ送信先の制約.md]])
- このようなログ送信の制約は Azure Monitor の設計を理解するうえで重要であり、Azure のアーキテクチャ試験でも頻繁に問われるポイントである。 ([[Azure SQL Database の診断設定とログ送信先の制約.md]])
- つまり、検討すべきポイントは二つある。一つは、同じログカテゴリを複数の Log Analytics ワークスペースに送信できるかどうかである。もう一つは、同じログを複数のストレージアカウントへ送信できるかどうかである。この二つの制約を理解していないと、正しい選択肢を判断することができない。 ([[Azure SQL Database の診断設定とログ送信先の制約2.md]])

### 3.5 閉域監視

- 2. Azure Monitor リソース（例: Log Analytics Workspace）を AMPLS に接続する ([[Azure Monitor Private Link Scope（AMPLS）設計ガイド.md]])
- 今回の要件では、全VMが最終的にアクセスしたい先は **Workspace1** である。Workspace1 を Private Link 化するために、Workspace1 を1つの AMPLS に関連付ければよい。 ([[Azure Monitor Private Link Scope（AMPLS）設計ガイド.md]])
- これに対して VNet 側から接続してくる。 ([[Azure Monitor Private Link Scope（AMPLS）設計ガイド.md]])
- 問題文の「管理上の努力を最小化」という要件にも一致する。もし VNet ごとに AMPLS を分けると、 ([[Azure Monitor Private Link Scope（AMPLS）設計ガイド.md]])
- したがって **Private Endpoint は最小で2つ必要**になる。 ([[Azure Monitor Private Link Scope（AMPLS）設計ガイド.md]])
- 次に、接続元ネットワークの到達性を見る。 ([[Azure Monitor Private Link Scope（AMPLS）設計ガイド.md]])

## 第4章 ユースケースで理解する

### 4.1 Azure Monitor 基盤のユースケース

- Azure Monitor / Application Monitoring アーキテクチャ （Log Analytics・Application Insights・Service Map）: クラウドアプリケーションでは、アプリケーションだけでなく **インフラ・依存サービス・ユーザー行動** などを総合的に監視する必要があります。 出典: [[Azure Monitor _ Application Monitoring アーキテクチャ.md]]
- Azure Monitor Agent を利用したログ収集設計: Azure 環境では、仮想マシンやアプリケーションから生成されるログを一元的に収集し、監視・分析を行うために **Azure Monitor** が利用される。Azure Monitor はクラウドおよびオンプレミスのリソースからテレメトリデータを収集し、ログ分析やアラート、可観測性（Observability）を提供する統合監視サービス... 出典: [[Azure Monitor Agent を利用したログ収集設.md]]

### 4.2 アプリケーション可観測性のユースケース

- Azure App Service アプリケーション監視アーキテクチャ （Azure Monitor + Application Insights）: クラウドでアプリケーションを運用する場合、単にアプリケーションをデプロイするだけでは不十分である。実際の運用では次のような問題を常に監視する必要がある。 出典: [[Azure App Service アプリケーション監視アーキテクチャ.md]]
- Azure App Service のパフォーマンス監視と依存関係トラッキング: ある企業では、Azure App Service 上で Web アプリケーションを運用しています。このアプリケーションは複数のコンポーネントで構成されており、外部 API やデータベースなどの依存関係を持っています。 出典: [[Azure App Service のパフォーマンス監視と依存関係トラッキング.md]]

### 4.3 VM とインフラログ監視のユースケース

- Azure Monitor による VM セキュリティイベント監視: クラウド環境で仮想マシン（VM）を運用する場合、**セキュリティイベントの監視**は非常に重要である。 出典: [[Azure Monitor による VM セキュリティイベント監視.md]]
- Azure Monitor による Windows VM ログ集中監視設計 （Log Analytics Workspace + Log Analytics Agent）: ある企業では、Azure 上で **200台の Windows 仮想マシン（VM）** を運用している。 運用チームは、これらの VM の **システムログ（Windows Event Log）** を監視し、 異常が発生した場合に **リアルタイムでアラートを受信できる監視システム**を構築する必要がある。 出典: [[Azure Monitor による Windows VM ログ集中監視設計.md]]

### 4.4 SQL と運用ログ設計のユースケース

- Azure SQL Database Diagnostics とログ保持設計 （Storage Account + Log Analytics Workspace）: Azure SQL Database を運用する場合、データベースの状態やパフォーマンス、セキュリティイベントを監視するために **Diagnostics（診断ログ）**を有効化することが一般的である。 出典: [[Azure SQL Database Diagnostics とログ保持設計.md]]
- Azure SQL Database の診断設定とログ送信先の制約: Azure 環境では、アプリケーションやデータベースの状態を監視し、問題発生時に迅速に原因を特定するためにログの収集が非常に重要である。Azure SQL Database も例外ではなく、パフォーマンスやクエリ実行状況、リソース使用状況などを確認するためのログを出力することができる。これらのログは、運用監視、セキュリティ監査、パフォーマ... 出典: [[Azure SQL Database の診断設定とログ送信先の制約.md]]
- Azure SQL Database の診断設定とログ送信先の制約: Azure 環境では、アプリケーションやデータベースの状態を監視し、問題発生時に迅速に原因を特定するためにログの収集が非常に重要である。Azure SQL Database も例外ではなく、パフォーマンスやクエリ実行状況、リソース使用状況などを確認するためのログを出力することができる。これらのログは、運用監視、セキュリティ監査、パフォーマ... 出典: [[Azure SQL Database の診断設定とログ送信先の制約2.md]]

### 4.5 閉域監視のユースケース

- Azure Monitor Private Link Scope（AMPLS）設計ガイド: このシナリオは、Azure Monitor / Log Analytics へのログ通信を **パブリック経由ではなく Microsoft のバックボーンネットワークだけで流したい**、という要件に対して、**Azure Monitor Private Link Scope（AMPLS）** と **Private Endpoint**... 出典: [[Azure Monitor Private Link Scope（AMPLS）設計ガイド.md]]

## 第5章 学習チェックポイント

- まず Azure Monitor 基盤 → アプリケーション可観測性 → VM とインフラログ監視 → SQL と運用ログ設計 → 閉域監視 の順で読むと、基礎から応用まで流れを追いやすい。
- 第2章でサービスや構成要素の位置づけを理解し、第3章で制約条件を確認してから第4章のユースケースを読む。
- 同じサービスでも、要件によって選定理由が変わるため、出典リンク先の問題設定を必ず確認する。
- 重複文書がある場合は `同一内容` 表記のある出典もあわせて確認する。

## 関連用語

- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure App Service]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Cache for Redis]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Monitor]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Application Insights]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Log Analytics]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Private Link]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Monitor Agent]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Network (VNet)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Private Endpoint]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure ExpressRoute]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure DNS]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Monitor Private Link Scope (AMPLS)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Role-Based Access Control (RBAC)]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Key Vault]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Storage Account]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Event Hubs]]
- [[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]

## 出典ドキュメント

- [[Azure App Service アプリケーション監視アーキテクチャ.md]]
- [[Azure App Service のパフォーマンス監視と依存関係トラッキング.md]]
- [[Azure Monitor _ Application Monitoring アーキテクチャ.md]]
- [[Azure Monitor Agent を利用したログ収集設.md]]
- [[Azure Monitor Private Link Scope（AMPLS）設計ガイド.md]]
- [[Azure Monitor による VM セキュリティイベント監視.md]]
- [[Azure Monitor による Windows VM ログ集中監視設計.md]]
- [[Azure SQL Database Diagnostics とログ保持設計.md]]
- [[Azure SQL Database の診断設定とログ送信先の制約.md]]
- [[Azure SQL Database の診断設定とログ送信先の制約2.md]]
