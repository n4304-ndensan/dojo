# sys.server_resource_stats 詳細解説

> 位置づけ: この文書は Azure SQL Managed Instance で使う `sys.server_resource_stats` の個別解説です。RCSI 研修や性能評価で触れることは多い一方、列の意味や更新粒度を誤読しやすいため、単体で読み返せるように切り出しています。
>
> 読み分け: Azure SQL Managed Instance での RCSI 評価全体は [[Azure SQL Managed Instance における RCSI 評価の前提整理]]、評価スクリプトの設計原則と参照実装は [[Azure SQL Managed Instance における RCSI 評価スクリプト設計ガイド]]、監視オブジェクトの一覧は [[SQL Server DMV一覧]]、フォルダ全体の案内は [[ManagedInstance ドキュメントガイド]] と [[SQL Server ドキュメントガイド]] を参照してください。

## 1. 文脈と目的

Azure SQL Managed Instance の性能を見始めると、かなり高い頻度で `sys.server_resource_stats` に出会います。CPU 使用率の確認、I/O の大まかな傾向確認、ストレージ利用の確認など、用途は広いのですが、現場では「これは OS の CPU メーターと同じなのか」「15 秒ごとに取れるならリアルタイム監視に使えるのか」「I/O は待機時間なのか転送量なのか」が曖昧なまま使われがちです。

この文書の目的は、`sys.server_resource_stats` を Azure SQL Managed Instance のサービス側資源メーターとして正しく読むことです。何が取れて、何が取れず、どういう粒度で更新され、どの判断に向いているのかを整理します。短時間試験の瞬間監視よりも、一定区間の比較や傾向把握に向く理由もここで明確にします。[server-resource-stats][mi-monitoring]

## 2. 背景と課題

オンプレミスの SQL Server では、OS カウンターやホスト監視、ストレージ監視と併せて Database Engine の状態を見に行くことが一般的です。これに対して Azure SQL Managed Instance はマネージドサービスであるため、利用者が直接ホスト OS のカウンターへ自由にアクセスする前提ではありません。その代わり、サービスティア上限に対してどれだけ CPU、I/O、ストレージを使っているかを、サービス側で集計したビューとして見せます。[server-resource-stats][mi-resource-limits]

ここで誤解が起きやすいのは、`sys.server_resource_stats` が「SQL Server が今この瞬間に何をしているか」を示す細粒度の実行ビューではないことです。現在進行中の待機や、特定クエリの消費量をその場で追いたいなら `sys.dm_exec_requests`、待機統計、Query Store のような別の手段が必要です。`sys.server_resource_stats` は、Azure SQL Managed Instance というサービス全体の資源使用を、15 秒報告単位で保持する履歴寄りのビューだと理解した方がぶれにくくなります。[mi-monitoring][dmv-list]

## 3. 全体像

`sys.server_resource_stats` は Azure SQL Managed Instance 専用のシステムビューです。CPU、I/O、ストレージ、SKU などの情報を返し、履歴は約 14 日保持されます。重要な性質は次の四点です。

1. 一行は 15 秒の報告区間を表します。
2. データは 5 分から 10 分間隔で収集・集計・更新されます。
3. `avg_cpu_percent` は OS の生 CPU ではなく、Managed Instance のサービスティア上限に対する割合です。
4. I/O とストレージの値は、インスタンス全体の総量として返る列が中心です。

このため、短いベンチマークを 10 秒おきに回して「直前の一回だけ CPU が跳ねたか」を見る用途には向きません。一方で、あるテスト区間全体の平均 CPU、一定期間の読み書き総量、ストレージ増加傾向を見る用途にはかなり有効です。[server-resource-stats][mi-monitoring]

## 4. 中核概念と用語

このビューを誤読しないために、先に重要語を揃えておきます。

### 4.1 報告区間

`start_time` と `end_time` が示す 15 秒間の区間です。各行はこの区間に対応しています。したがって、行数が細かいからといって、ビュー自体がリアルタイム更新されるわけではありません。[server-resource-stats]

### 4.2 更新遅延

データは 15 秒単位で報告されますが、収集・集計・ビュー更新は 5 分から 10 分単位です。ここが最も誤解されやすい点です。行の粒度と、ビューが見えるまでの遅延は別です。[server-resource-stats][mi-monitoring]

### 4.3 サービスティア上限に対する割合

`avg_cpu_percent` は、Managed Instance のサービスティアで許可された CPU 上限に対して、その区間でどれだけ使ったかを示します。Windows のタスクマネージャーに出るホスト CPU 利用率そのものではありません。[server-resource-stats]

### 4.4 インスタンス全体の総量

`io_request`、`io_bytes_read`、`io_bytes_written`、`storage_space_used_mb` は、個別クエリや個別データベースではなく、Managed Instance 全体の使用量として読むのが基本です。特にストレージはユーザーデータベースだけでなく、システムデータベースも含みます。[server-resource-stats]

## 5. 仕組み

このビューは Azure SQL Managed Instance 側で収集したリソース利用情報を表形式で返します。公式説明では、CPU 使用率、I/O、ストレージ、SKU、ハードウェア世代、仮想コア数などを返し、Managed Instance のバージョンに応じてビュー定義が変わる可能性があると明記されています。したがって、列を前提に固定的な運用コードを書くなら、バージョン差分への注意が必要です。[server-resource-stats]

最初に見る機会が多い列は、次のように整理できます。

| 列 | 読み方 | 実務での意味 |
| --- | --- | --- |
| `start_time`, `end_time` | UTC の 15 秒報告区間 | どの区間の値かを示します。ローカル時刻と混同しない方が安全です。 |
| `avg_cpu_percent` | サービスティア上限比の平均 CPU | 継続的な CPU 圧迫を見ます。瞬間ピークではなく区間平均です。 |
| `io_request` | 区間内の物理 I/O 回数 | I/O の忙しさの大きさをざっくり見ます。待機時間ではありません。 |
| `io_bytes_read`, `io_bytes_written` | 区間内の物理読み書きバイト数 | I/O 量の増減を見るのに向きます。 |
| `reserved_storage_mb` | 契約済みストレージ容量 | 使える上限側の文脈です。 |
| `storage_space_used_mb` | 実使用ストレージ量 | ユーザー DB とシステム DB を含む全体使用量です。 |
| `sku`, `hardware_generation`, `virtual_core_count` | サービスティアと基盤情報 | 数値を読む前提条件を与える列です。 |

特に `avg_cpu_percent` の定義は重要です。公式には、インスタンス内の全データベースについて全リソースプールの CPU 時間を合算し、そのサービスティアで使える CPU 時間で割ったものだと説明されています。つまり、SQL 文単位の CPU ではなく、Managed Instance 全体のコンピュート利用率です。[server-resource-stats]

## 6. アーキテクチャと設計上の含意

このビューをどの層のメーターとみなすかで、解釈はかなり変わります。`sys.dm_exec_requests` や待機 DMV は Database Engine の「現在の内部状態」を見るものですが、`sys.server_resource_stats` は Azure SQL Managed Instance というサービス全体の「区間集計された資源消費」を見るものです。両者は補完関係にあります。

設計上の含意として重要なのは、`sys.server_resource_stats` の値だけで原因を断定しないことです。たとえば `avg_cpu_percent` が高くても、個別クエリが悪いのか、クエリ数が多いのか、ある時刻だけ集中したのかはこのビュー単独では分かりません。同じく `io_bytes_written` が多くても、ログ主因なのかデータファイル主因なのか、どのデータベースの影響が大きいのかまでは切れません。原因の深掘りは Query Store、待機、実行中要求、ファイル I/O 統計と組み合わせて行うべきです。[mi-monitoring][dmv-list]

逆に、容量設計や比較試験では、このビューの強みが出ます。あるテスト A とテスト B で、どちらがインスタンス全体の CPU や I/O を少なく使うかを見るには、個別クエリよりこのビューの方が適しています。Azure SQL Managed Instance のリソース予算に対して、総量としてどれだけ圧迫したかを比較できるからです。[mi-monitoring][mi-performance-guidance]

## 7. 実装上の考慮点

クエリの書き方そのものは単純ですが、読むときの前提を外さないことが重要です。現場では三部名で `master.sys.server_resource_stats` と書いて対象を明示することが多く、これは「`master` データベースの `sys` スキーマにある `server_resource_stats` を読む」という意図を明確にする書き方です。三部名やスキーマの読み方そのものは [[SQL Server の User、Schema、tempdb.sys の名前解決]] で詳しく扱います。

直近の傾向を見るだけなら、次のようなクエリが基本になります。

```sql
SELECT TOP (100)
    start_time,
    end_time,
    avg_cpu_percent,
    io_request,
    io_bytes_read,
    io_bytes_written,
    storage_space_used_mb
FROM master.sys.server_resource_stats
ORDER BY end_time DESC;
```

ただし、この結果を秒単位のリアルタイムメーターだと思わない方が安全です。直近 2 分の挙動を知りたいときに値がまだ追いついていないことは普通に起こります。更新遅延を明示的に確認したいなら、`SYSUTCDATETIME()` と最新の `end_time` を比較すると、どれくらい遅れて見えているかを把握しやすくなります。

```sql
SELECT TOP (1)
    end_time,
    DATEDIFF(MINUTE, end_time, SYSUTCDATETIME()) AS minutes_behind
FROM master.sys.server_resource_stats
ORDER BY end_time DESC;
```

## 8. 運用とセキュリティ上の考慮点

このビューの参照には `VIEW SERVER STATE` 権限が必要です。Azure SQL Managed Instance で性能観測をチーム運用に載せるなら、誰にこの権限を持たせるかを先に設計しておく必要があります。[server-resource-stats][mi-monitoring]

また、時刻は UTC で返るため、アプリログや監視ダッシュボードがローカル時刻基準の場合は変換ルールを揃えた方が安全です。障害報告や性能比較では、この時刻の取り違えだけで原因切り分けがかなり崩れます。

さらに、公式にはビュー定義が Managed Instance のバージョンによって変わる可能性があると明記されています。`SELECT *` 依存の収集コードは将来の列追加に弱いため、必要列を明示して読む方が持続可能です。[server-resource-stats]

## 9. 使うべき場面と、使うべきでない場面

このビューが向いているのは、インスタンス全体の資源消費傾向、試験区間どうしの比較、継続的な CPU 圧迫や I/O 量の大きな変動を見る場面です。特に、どのテストがより多くのリソースを消費したかを比較する用途では、かなり扱いやすいビューです。[mi-performance-guidance]

一方で、向いていないのは、今まさに起きているブロッキングや、単一クエリのボトルネック調査、秒単位のピーク検知です。そうした用途では、`sys.dm_exec_requests`、待機 DMV、Query Store の方が適切です。このビューは「今この瞬間の犯人探し」ではなく、「一定区間の資源消費の傾向把握」に寄った道具だと考える方が正確です。[mi-monitoring]

## 10. よくある誤解や失敗パターン

### 10.1 `avg_cpu_percent` を OS CPU 使用率だと思う

誤りです。`avg_cpu_percent` は Managed Instance のサービスティア上限に対する割合です。ホスト OS の CPU 使用率そのものではありません。[server-resource-stats]

### 10.2 15 秒粒度だからリアルタイム監視に向くと思う

半分だけ正しく、半分は誤りです。報告区間は 15 秒ですが、ビューの更新には 5 分から 10 分の遅延があります。したがって、リアルタイム監視の代替にはなりません。[server-resource-stats][mi-monitoring]

### 10.3 `io_request` や `io_bytes_written` を待機時間だと思う

誤りです。これらは I/O の回数やバイト数であり、待機時間ではありません。I/O が遅いかどうかを知りたいなら、待機やファイル I/O 統計と組み合わせる必要があります。

### 10.4 `storage_space_used_mb` はユーザーデータだけだと思う

誤りです。公式には、Managed Instance 内の全データベースファイル、すなわちユーザーデータベースとシステムデータベースを含むと説明されています。[server-resource-stats]

### 10.5 `SELECT *` で固定収集しても大丈夫だと思う

危険です。公式には、Managed Instance のバージョンによってビュー定義が変わる可能性が明記されています。必要列を明示して読む方が安全です。[server-resource-stats]

## 11. 結論

`sys.server_resource_stats` は、Azure SQL Managed Instance をインスタンス全体の資源使用という視点で見るための重要なビューです。ただし、その本質はリアルタイム実行ビューではなく、15 秒区間の報告を 5 分から 10 分遅れで保持する、履歴寄りのサービスメーターです。

この前提を外さなければ、CPU、I/O、ストレージの傾向把握や試験比較には非常に有効です。逆に、この前提を外すと「今の CPU が高いはずなのに値が動かない」「I/O が多いから待機が多いに違いない」といった誤読につながります。`sys.server_resource_stats` は単独で万能ではありませんが、正しい責務で使えば Azure SQL Managed Instance の状態把握をかなり安定させるビューです。

## 参考

- [sys.server_resource_stats (Azure SQL Managed Instance)][server-resource-stats]
- [Monitoring Azure SQL Managed Instance performance using dynamic management views][mi-monitoring]
- [Tune applications and databases for performance in Azure SQL Managed Instance][mi-performance-guidance]
- [Overview of Azure SQL Managed Instance resource limits][mi-resource-limits]
- [SQL Server DMV一覧][dmv-list]

[server-resource-stats]: https://learn.microsoft.com/sql/relational-databases/system-catalog-views/sys-server-resource-stats-azure-sql-database?view=azuresqldb-current
[mi-monitoring]: https://learn.microsoft.com/azure/azure-sql/managed-instance/monitoring-with-dmvs?view=azuresql
[mi-performance-guidance]: https://learn.microsoft.com/azure/azure-sql/managed-instance/performance-guidance?view=azuresql#tune-your-database
[mi-resource-limits]: https://learn.microsoft.com/azure/azure-sql/managed-instance/resource-limits?view=azuresql
[dmv-list]: ../SQL%20Server%20DMV一覧.md
