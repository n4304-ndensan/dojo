# SQL Server ドキュメントガイド

このフォルダには、SQL Server の基礎理解、RCSI の評価、監視オブジェクトの参照、個別調査メモが混在しています。内容自体は関連していますが、入口が分かりにくいまま文書を増やすと、役割の重複や説明の再掲が起きやすくなります。

このガイドは、各文書の責務と読む順番を明確にし、あとから文書を追加するときにも「どこへ何を書くべきか」がぶれにくい状態を作るための索引です。

## 1. このフォルダの主な文書

最初に全体像をつかむため、文書群を役割ごとに整理します。

- 主教材: [[SQL Server 入門から始める RCSI 性能評価研修]]
- 全体像の深掘り: [[SQL Server の全体像 詳細解説]]
- 分離レベルの比較判断: [[SQL Server Snapshot Isolation と RCSI と Read Committed の違い]]
- 分離レベルの総合解説: [[SQL Server の Isolation Level（分離レベル）詳細解説]]
- 監視オブジェクトの索引: [[SQL Server DMV一覧]]
- Azure SQL Managed Instance 固有文書の索引: [[ManagedInstance/ManagedInstance ドキュメントガイド]]
- `sys.server_resource_stats` の個別解説: [[ManagedInstance/sys.server_resource_stats 詳細解説]]
- 名前解決とスキーマの整理: [[SQL Server の User、Schema、tempdb.sys の名前解決]]
- Azure SQL Managed Instance 向けの前提整理: [[ManagedInstance/Azure SQL Managed Instance における RCSI 評価の前提整理]]
- Azure SQL Managed Instance 向けの評価設計: [[ManagedInstance/Azure SQL Managed Instance における RCSI 評価スクリプト設計ガイド]]
- XE を使った SQL 実行内容の調査メモ: [[XE_SQLイベント_比較|XEでSQL実行内容を確認するための整理]]
- 個別 DMV / DMF の列定義メモ: `DMV/` 配下

## 2. 読む順番の目安

SQL Server や RCSI をこれから学ぶ場合は、次の順番で読むと理解がつながりやすくなります。

1. [[SQL Server 入門から始める RCSI 性能評価研修]]
2. [[SQL Server の全体像 詳細解説]]
3. [[SQL Server の Isolation Level（分離レベル）詳細解説]]
4. [[SQL Server Snapshot Isolation と RCSI と Read Committed の違い]]
5. [[SQL Server DMV一覧]]

そのうえで、環境や課題に応じて個別文書を追加します。Azure SQL Managed Instance の文書は [[ManagedInstance/ManagedInstance ドキュメントガイド]] を入口にすると迷いにくく、前提整理は [[ManagedInstance/Azure SQL Managed Instance における RCSI 評価の前提整理]]、評価設計は [[ManagedInstance/Azure SQL Managed Instance における RCSI 評価スクリプト設計ガイド]] を参照します。XE で SQL 実行内容を追うなら [[XE_SQLイベント_比較|XEでSQL実行内容を確認するための整理]] を参照します。

## 3. 用途別の入口

読みたい内容が決まっている場合は、次の入口から入ると迷いにくくなります。

- SQL Server と RCSI をゼロから理解したい: [[SQL Server 入門から始める RCSI 性能評価研修]]
- Database Engine の内部構造を一本の流れで理解したい: [[SQL Server の全体像 詳細解説]]
- Isolation Level を体系的に理解したい: [[SQL Server の Isolation Level（分離レベル）詳細解説]]
- RCSI と SNAPSHOT の選び分けを整理したい: [[SQL Server Snapshot Isolation と RCSI と Read Committed の違い]]
- DMV / DMF / Query Store の役割分担を確認したい: [[SQL Server DMV一覧]]
- `sys.server_resource_stats` を詳しく知りたい: [[ManagedInstance/sys.server_resource_stats 詳細解説]]
- `tempdb.sys` や `master.sys` の読み方を整理したい: [[SQL Server の User、Schema、tempdb.sys の名前解決]]
- Azure SQL Managed Instance の文書群をまとめて見たい: [[ManagedInstance/ManagedInstance ドキュメントガイド]]
- Azure SQL Managed Instance で評価の前提を整理したい: [[ManagedInstance/Azure SQL Managed Instance における RCSI 評価の前提整理]]
- Azure SQL Managed Instance で評価設計と参照スクリプトを確認したい: [[ManagedInstance/Azure SQL Managed Instance における RCSI 評価スクリプト設計ガイド]]
- XE のイベント粒度を整理したい: [[XE_SQLイベント_比較|XEでSQL実行内容を確認するための整理]]

## 4. 追加・更新時の方針

このフォルダを持続可能に保つため、文書追加や追記は次の方針でそろえます。

- 一つの文書に一つの責務を持たせます。入門、比較、実践、調査メモ、リファレンスを混ぜすぎません。
- 環境依存の内容は、タイトルと冒頭で対象環境を明示します。たとえば Azure SQL Managed Instance 固有の話は、汎用 SQL Server 解説へ混ぜ込まず `ManagedInstance/` 配下の専用文書へ寄せます。
- 評価系の文書は、その場の修正ログではなく、誤解しやすい前提、推奨観測項目、読み方を中心に構成します。
- 列定義や DMV 個票の詳細は `DMV/` 配下へ寄せ、主教材や補足資料には「何を判断するために使うか」を書きます。
- 既存文書で詳しく説明している概念は、重複して長く再掲せず、必要に応じてリンクでつなぎます。
- 主文体は、フォルダ全体で読み味が揃うように、基本的にですます調で統一します。
- 新しいトップレベル文書を追加したら、このガイドも更新します。

## 5. 迷ったときの判断基準

新しい内容を書こうとして置き場に迷ったときは、次の基準で切り分けると整理しやすくなります。

- 概念と評価の流れをまとめたいなら、主教材かその補足資料へ追記します。
- 分離レベル、監視方式、機能差の比較を深めたいなら、比較用の独立文書を使います。
- 特定環境のスクリプト読解や運用上の注意なら、環境名を含む実践文書として分けます。Azure SQL Managed Instance 固有なら `ManagedInstance/` 配下を優先します。
- 調査ログの解釈やツールの使い分けなら、調査メモとして分けます。
- 列の意味や DMV 単体の読み方なら、`DMV/` 配下へ追加します。

この基準で文書を分けておくと、後から見返したときにも「まずどこを見るべきか」がぶれにくくなります。
