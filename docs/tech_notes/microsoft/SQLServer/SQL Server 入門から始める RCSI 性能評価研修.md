# SQL Server 入門から始める RCSI 性能評価研修

> 位置づけ: この文書は SQLServer フォルダの主教材です。SQL Server の基礎、RCSI の考え方、評価指標の読み方を一冊でつかむときの起点として使います。
>
> 読み分け: 全体像の深掘りは [[SQL Server の全体像 詳細解説]]、分離レベルの比較は [[SQL Server Snapshot Isolation と RCSI と Read Committed の違い]]、監視オブジェクトの索引は [[SQL Server DMV一覧]]、フォルダ全体の案内は [[SQL Server ドキュメントガイド]] を参照してください。

## 1. 文脈と目的

業務システムの性能問題は、単純に CPU が高い、I/O が遅い、という一言では片づきません。実際には、アプリケーションから送られた SQL が、トランザクション、ロック、メモリ、ディスク、tempdb、実行計画、監視基盤といった複数の層をまたいで処理され、そのどこかで待たされることで体感性能が悪化します。特に更新系が多い業務システムでは、読み取りと書き込みが互いにブロックし、ロック待機が連鎖して全体が遅く見えることがよくあります。

この資料の目的は、SQL Server をほとんど知らない読者でも、最終的に次の問いへ自力で答えられるようになることです。SQL Server とは何か。なぜロックや分離レベルが性能に効くのか。RCSI を有効にすると何が良くなり、何が別の場所へ押し出されるのか。そして、提示された測定項目をどの順番で見れば、RCSI の効果と副作用を筋道立てて判断できるのか、ということです。

本資料は、Microsoft SQL Server 系の Database Engine を前提にしていますが、提示された CPU 指標の取得元である `sys.server_resource_stats` は Azure SQL Managed Instance 固有です。そのため、資料全体は SQL Server の基礎から説明しつつ、監視項目の一部は Azure SQL Managed Instance を前提に補足しています。[What is SQL Server?][sqlserver-what-is] [The Microsoft SQL Database Engine][sql-database-engine] [sys.server_resource_stats (Azure SQL Managed Instance)][mi-server-resource-stats]

### この資料の対象読者

データベース専任者でなくても構いません。主な対象は、次のような人です。

- SQL Server の経験が浅いが、業務システムの性能検証に参加する人
- RCSI の有効化前後で、何をどう比較すべきか知りたい人
- DMV や Query Store の数値を、単なる監視値ではなく因果関係として理解したい人

### この資料で扱うこと

この資料で扱う中心テーマは、Database Engine の基礎、トランザクションと分離レベル、RCSI の仕組み、tempdb と version store、DMV と Query Store の見方、そして提示された各測定項目の意味と解釈です。逆に、SSIS、SSAS、可用性グループ、バックアップ設計、セキュリティ製品群そのものの詳細な設計論は扱いません。今回はあくまで、RCSI の性能検証を理解するために必要な範囲へ絞ります。

### 学習順序

この資料は最初から順に読める構成にしていますが、研修として進めるなら次の順番が理解しやすいです。

1. まず第 2 章から第 5 章で、SQL Server の正体、全体像、用語を固める。
2. 次に第 6 章から第 8 章で、RCSI が何を変えるのか、測定の原則は何かを理解する。
3. 最後に第 9 章以降で、個々の指標の読み方と、複数指標を組み合わせた判断の仕方を身につける。

## 2. SQL Server とは何か

SQL Server は、Microsoft が提供するリレーショナルデータベース管理システム、つまり RDBMS です。アプリケーションや運用ツールは SQL Server のインスタンスやデータベースへ接続し、T-SQL を使って読み書きを行います。中核にあるのは Database Engine であり、これがデータの保存、処理、保護、トランザクション制御、回復、並行実行制御を担います。[What is SQL Server?][sqlserver-what-is] [The Microsoft SQL Database Engine][sql-database-engine]

ここで重要なのは、SQL Server を単なる「データを置く箱」だと思わないことです。業務システムから見た SQL Server は、データの整合性を保ちつつ、多数のユーザーからの同時アクセスをさばき、障害時にも戻せるようにし、しかも実行中のふるまいを観測できるようにした実行基盤です。テーブルを保存するだけならファイルでもできますが、複数人が同時に更新し、途中失敗から回復し、遅い原因を追跡できるところまでまとめて面倒を見るのが Database Engine の役割です。

### 2.1 RDBMS とは何か

RDBMS は、表形式でデータを管理し、主キー、外部キー、制約、トランザクションによって整合性を保つ仕組みです。SQL Server では、単にデータを保存するだけでなく、検索を速くするインデックス、障害復旧のためのトランザクションログ、同時実行を制御するロックや行バージョン、性能調査のための DMV や Query Store までが一体になっています。

この「一体になっている」という点が重要です。アプリケーションが期待するのは、正しいデータが返ることだけではありません。更新中のデータを他の人がどう見るか、障害時にどこまで戻るか、遅いときに何が詰まっているか、という運用面まで含めて初めて業務で使える基盤になります。SQL Server はそこを Database Engine としてまとめて提供しています。[The Microsoft SQL Database Engine][sql-database-engine] [The Microsoft SQL Database Engine - ACID][sql-database-engine-acid]

### 2.2 SQL Server の「すごさ」はどこにあるか

SQL Server の強みは、単に SQL を実行できることではありません。第一に、ACID を軸にした整合性と回復性を持つことです。トランザクションの途中で失敗しても、ログに基づいて戻すか、最後までやり切るかを明確にできます。業務システムで誤差や不整合が許されない処理ほど、この性質が効きます。[The Microsoft SQL Database Engine - ACID][sql-database-engine-acid]

第二に、観測性が高いことです。SQL Server には `sys.dm_os_wait_stats`、`sys.dm_exec_requests`、`sys.dm_io_virtual_file_stats`、`sys.dm_os_sys_info` のような豊富な DMV があり、Query Store はクエリの実行履歴と集計統計を保持します。つまり、遅いという現象を、ロック待機なのか、I/O なのか、メモリなのか、クエリ応答なのかに分解して考えやすいのです。[sys.dm_os_wait_stats][dm_os_wait_stats] [How Query Store collects data][query-store-collects]

第三に、同じ SQL Database Engine が SQL Server、Azure SQL Database、Azure SQL Managed Instance など複数の提供形態の土台になっていることです。製品ごとの差はありますが、トランザクション、ロック、実行計画、DMV といった基礎概念は共通です。このため、基礎を理解しておくと、オンプレミスでもクラウドでも説明の筋道を保ちやすくなります。[The Microsoft SQL Database Engine - modern platforms][sql-database-engine-platforms]

## 3. 他の選択肢との違い

### 3.1 ファイルや共有フォルダとの違い

CSV や Excel や共有フォルダは、データを保存する手段としては簡単ですが、同時更新時の整合性、途中失敗からの回復、複雑な検索、監査、性能追跡には向いていません。SQL Server は、誰がどの行を更新したか、コミットされていない変更を他の人へどう見せるか、クラッシュ時にどこまで戻せるか、といった難しい問題を Database Engine の責務として引き受けます。

### 3.2 NoSQL との違い

NoSQL 系データストアは、スキーマ柔軟性や水平分散を優先する設計が多く、ユースケースによっては非常に有効です。一方で、複数テーブルをまたぐ厳密な整合性、複雑な結合、トランザクション制御、細かな観測性が重要な基幹系では、RDBMS の方が考えやすい場合が多くあります。SQL Server は特に、業務システムで頻出する「更新の正しさ」と「障害時の回復」を強く意識したエンジンです。

### 3.3 他の RDBMS との違い

PostgreSQL や MySQL など他の RDBMS と比べて、SQL Server だけが絶対的に優れている、という言い方は正確ではありません。実際には、組織のスキル、既存資産、クラウド方針、運用文化で選定は変わります。ただし SQL Server には、T-SQL を中心にした統合的な運用体験、豊富な DMV、Query Store による性能履歴、Azure 系サービスとの近さ、企業向けの機能群が揃っている、という特色があります。今回の資料で重要なのは、「ロック待機や version store のような内部挙動を、比較的一貫した方法で観測しやすい」という点です。

## 4. SQL Server の全体像

ここから先の性能指標を理解するには、SQL Server を一枚の絵として持つことが重要です。細かい構文に入る前に、処理の流れを大づかみに押さえます。

1. アプリケーションが SQL Server へ接続し、T-SQL を送る。
2. Query Processor が SQL を解析し、実行計画を選ぶ。
3. Storage Engine が、必要なデータページをメモリやディスクから扱う。
4. 更新処理はトランザクションログへ記録され、障害時の回復に備える。
5. 同時実行制御のために、ロックや行バージョンが使われる。
6. 一時オブジェクトや version store などで tempdb が使われる。
7. 実行中のふるまいや累積統計は、DMV や Query Store から観測する。

この流れを見ると、今回の測定項目がどこに対応しているかが見えてきます。ロック待機は並行実行制御の層、version store と tempdb 使用率は tempdb の層、I/O 待機はファイルアクセスの層、SQL メモリ使用率はメモリ管理の層、平均応答時間や P95 は最終的なユーザー体感の層を見ています。つまり、今回の指標群はばらばらではなく、同じ一本の処理経路を別の角度から見ているだけです。

> 詳しくは[[SQL Server の全体像 詳細解説]]

## 5. まず理解すべき用語

この章は、この先の説明で頻出する言葉を揃えるためのものです。ここを曖昧にしたまま DMV を読むと、数字の意味を取り違えやすくなります。

**インスタンス**

SQL Server の実行単位です。アプリケーションは通常、まずインスタンスへ接続し、その中のデータベースを使います。

**データベース**

テーブル、インデックス、ビュー、ストアドプロシージャなどが入る論理的な箱です。性能指標の中にはインスタンス全体を見るものと、データベース単位で見るものがあります。

**セッション、要求、タスク**

セッションは接続、要求はその接続上で今実行されている処理、タスクは要求を構成する実行単位です。`sys.dm_exec_requests` は要求を見ており、瞬間的なブロッキングの観測に向いています。[sys.dm_exec_requests][dm_exec_requests]

**トランザクション**

一連の処理を、全部成功か全部失敗かで扱う論理単位です。トランザクションが長引くと、ロック保持や version store のクリーンアップ遅延の原因になります。[The Microsoft SQL Database Engine - ACID][sql-database-engine-acid]

**ACID**

原子性、一貫性、独立性、永続性です。SQL Server のトランザクションを理解する基礎であり、分離レベルの議論はこの独立性の現実的な実装方法だと考えると理解しやすくなります。[The Microsoft SQL Database Engine - ACID][sql-database-engine-acid]

**ロック**

同じデータへ矛盾したアクセスが起きないようにする制御機構です。読取時の共有ロック、更新時の排他ロックなどがあり、競合すると待機が発生します。[Transaction locking and row versioning guide][locking-row-versioning]

**待機**

SQL Server のワーカーが何らかの資源を待って止まっている状態です。ロック待機だけでなく、I/O、ラッチ、メモリ、ネットワークなどさまざまな種類があります。`sys.dm_os_wait_stats` は完了した待機の累積統計を持っています。[sys.dm_os_wait_stats][dm_os_wait_stats]

**ブロッキング**

他のセッションが持っているロックや資源のせいで、自分の要求が進めない状態です。`sys.dm_exec_requests.blocking_session_id` を見ると、今まさに誰に止められているかの手掛かりが得られます。[sys.dm_exec_requests][dm_exec_requests]

**分離レベル**

あるトランザクションが、他のトランザクションの途中状態をどこまで見せるか、また見せないためにどんなロックや行バージョンを使うかを決めるルールです。SQL Server の既定は `READ COMMITTED` です。[SET TRANSACTION ISOLATION LEVEL][set-transaction-isolation] [Transaction locking and row versioning guide][locking-row-versioning]

**RCSI**

`READ_COMMITTED_SNAPSHOT` を `ON` にしたときの、行バージョンベースの `READ COMMITTED` です。読取時の共有ロック依存を減らし、文単位の一貫したスナップショットを返します。[Transaction locking and row versioning guide - basics][locking-basics-rcsi]

**tempdb**

一時テーブル、内部作業領域、version store などに使われるシステムデータベースです。RCSI の副作用を見るときに非常に重要です。[tempdb database][tempdb-database]

**version store**

行バージョンを保持する領域です。RCSI や SNAPSHOT など、行バージョンベースの読み取りで使われます。従来は tempdb が主な観測対象ですが、ADR の構成によって解釈に注意が必要です。[Transaction locking and row versioning guide - version store][locking-version-store] [tempdb database][tempdb-database]

**DMV**

Dynamic Management View の略です。稼働中の SQL Server の内部状態や統計を確認するためのビュー群です。性能分析では、DMV を「診断用の計器盤」と考えると分かりやすいです。

**Query Store**

クエリ実行の履歴と集計統計をユーザーデータベース内へ保持する機能です。平均応答時間のようなテスト後集計に向いていますが、既定では 60 分単位の集計間隔で管理される点に注意が必要です。[How Query Store collects data][query-store-collects] [Best practices for managing the Query Store][manage-query-store] [sys.query_store_runtime_stats][query-store-runtime-stats]

**累積値、瞬間値、差分値、パーセンタイル**

今回の指標を理解するうえで最重要の分類です。`wait_time_ms` や `io_stall_*_ms` は累積値なので、その場の値そのものではなく「前回との差分」で見ます。一方、`COUNT(*) WHERE blocking_session_id <> 0` は瞬間値です。P95 は「遅い側の 5% を含めた代表値」であり、平均では見えない長い待ちを拾うために使います。

## 6. なぜ RCSI を検証するのか

SQL Server の既定の `READ COMMITTED` は、SQL Server と Azure SQL Managed Instance では通常、読取時に共有ロックを使います。これによりダーティリードは防げますが、更新中の行を読むときに待たされることがあります。更新系が多い業務では、この読取ブロッキングが全体の遅さとして見えます。[SET TRANSACTION ISOLATION LEVEL][set-transaction-isolation] [Transaction locking and row versioning guide - basics][locking-basics-rcsi]

RCSI は、この既定の `READ COMMITTED` を、共有ロック中心の読取から、行バージョン中心の読取へ置き換える発想です。RCSI が `ON` になると、読取は「文の開始時点でコミット済みだった行」を参照するため、更新中の行を読むために毎回共有ロックで張り合う必要が減ります。結果として、読取と書込みのブロッキングが減り、デッドロックの可能性も下がりやすくなります。[Transaction locking and row versioning guide - basics][locking-basics-rcsi] [Snapshot isolation in SQL Server][snapshot-isolation]

ただし、RCSI は魔法ではありません。書込み同士の競合を消すわけではなく、スキーマ変更との競合も消しません。さらに、行バージョンを保持するためのコストは tempdb や version store に移ります。したがって、RCSI の評価では「ロック待機が減ったか」だけでは不十分で、「その分のコストが tempdb や I/O に出ていないか」「本当にユーザー体感が良くなったか」まで見る必要があります。これが今回の指標設計の出発点です。

### 6.1 RCSI が `OFF` のときの見え方

RCSI が `OFF` の通常の `READ COMMITTED` では、読取時に共有ロックが使われます。書込みが排他ロックを持っていると読取は待ち、逆に読取の共有ロックが残っていると更新側が待つこともあります。結果として、`LCK%` 系待機が積み上がりやすくなり、`blocking_session_id` が見つかりやすくなります。

### 6.2 RCSI が `ON` のときの見え方

RCSI が `ON` になると、既定の `READ COMMITTED` が行バージョンを使うようになります。読取は更新トランザクションの完了待ちをしにくくなるため、RCSI の効果が出ていれば、同一負荷条件でロック待機時間やブロッキング数が下がるのが自然です。[Transaction locking and row versioning guide - basics][locking-basics-rcsi]

### 6.3 RCSI の副作用

副作用の本質は、読取ブロッキングを減らす代わりに、以前の行イメージをどこかに保持しなければならないことです。この保持領域が version store であり、従来型の構成では tempdb を観測します。したがって、RCSI の副作用を見るために、version store サイズ、tempdb 使用率、tempdb I/O 待機を併せて観測します。[Transaction locking and row versioning guide - version store][locking-version-store] [tempdb database][tempdb-database] [sys.dm_db_file_space_usage][dm_db_file_space_usage]

## 7. この研修で観測する全体像

提示された測定項目は、次の四つの問いへ答えるために並んでいます。

1. RCSI によって、読取ブロッキングは本当に減ったか。
2. その効果の代わりに、tempdb や version store が苦しくなっていないか。
3. ボトルネックが CPU、メモリ、I/O の別の場所へ移っていないか。
4. 最終的に、平均応答と P95 応答は改善したか。

この四つを順番に見ると、判断の筋が通ります。最初にロック系を見るのは、RCSI の直接的な狙いがそこだからです。次に tempdb を見るのは、副作用の確認です。その次に CPU、メモリ、I/O を見るのは、ロックが減っても他の資源が足を引っ張れば全体は速くならないからです。最後に応答時間を見るのは、ユーザー体感へ本当に効いたかを確定するためです。

## 8. 測定の基本原則

この章は非常に重要です。SQL 文そのものより先に、「どう測れば誤解しにくいか」を理解しておく必要があります。

### 8.1 同じ負荷条件で前後比較する

RCSI の評価は、RCSI 前後で同じ負荷を再現しなければ意味がありません。件数、実行時間帯、並列数、テストシナリオ、キャッシュの温まり方が違うと、ロック待機や I/O の差は RCSI の効果なのか、単なる条件差なのか切り分けられなくなります。

### 8.2 累積値は必ず差分で見る

`sys.dm_os_wait_stats.wait_time_ms` や `sys.dm_io_virtual_file_stats.io_stall_read_ms`、`io_stall_write_ms` は累積値です。SQL Server 起動後、または統計リセット後からの総量なので、単発で見ても「今この 1 分で何が起きたか」は分かりません。したがって、一定間隔で 2 回取得し、後の値から前の値を引いた差分で見ます。[sys.dm_os_wait_stats][dm_os_wait_stats] [sys.dm_io_virtual_file_stats][dm_io_virtual_file_stats]

### 8.3 瞬間値は短い間隔で見る

`sys.dm_exec_requests` のブロッキング数は瞬間値です。長いブロッキングなら粗いサンプリングでも見えますが、数秒未満の短いブロッキングは見逃しやすくなります。瞬間的な止まりを拾いたいなら、ブロッキング数は短い間隔で見る必要があります。[sys.dm_exec_requests][dm_exec_requests]

### 8.4 Query Store は「後で集計する」仕組みだと考える

Query Store は個々の実行をそのまま秒単位の生ログとして持つのではなく、一定の時間窓で集計した統計を保持します。`sys.query_store_runtime_stats.avg_duration` はマイクロ秒で、`count_executions` と組み合わせると加重平均を出せますが、既定の `interval_length_minutes` は 60 分です。したがって、短時間テストで毎秒ポーリングする指標ではなく、テスト後に区間集計する指標として扱う方が現実的です。[How Query Store collects data][query-store-collects] [sys.query_store_runtime_stats][query-store-runtime-stats] [sys.database_query_store_options][database-query-store-options]

### 8.5 権限が必要な DMV がある

今回使う DMV の多くは、SQL Server 2019 以前では `VIEW SERVER STATE`、SQL Server 2022 以降では `VIEW SERVER PERFORMANCE STATE` が必要です。権限不足だと見えない指標があるので、事前に確認しておくべきです。[sys.dm_os_wait_stats][dm_os_wait_stats] [sys.dm_io_virtual_file_stats][dm_io_virtual_file_stats] [sys.dm_db_file_space_usage][dm_db_file_space_usage]

### 8.6 CPU 指標は環境差を理解して使う

提示された CPU 指標の `sys.server_resource_stats.avg_cpu_percent` は Azure SQL Managed Instance の指標であり、ホスト OS の CPU 使用率そのものではありません。サービスティア上限に対する割合で、15 秒粒度の行がありつつ、ビューの更新には 5 分から 10 分程度の遅延があります。短時間検証ではこの遅延が解釈を難しくするため、ユーザーの備考どおり、実務では別の収集方法で補うことがあります。[sys.server_resource_stats (Azure SQL Managed Instance)][mi-server-resource-stats] [Monitoring Azure SQL Managed Instance performance using dynamic management views][mi-monitoring-dmvs]

### 8.7 統計のリセットは慎重に扱う

`sys.dm_os_wait_stats` は `DBCC SQLPERF ('sys.dm_os_wait_stats', CLEAR)` でリセットできますが、共有環境でこれを実行すると他の分析者や運用監視へ影響します。研修や検証では、可能ならリセット前提ではなく「開始時と終了時の差分を引く」方式を基本にした方が安全です。[sys.dm_os_wait_stats][dm_os_wait_stats]

## 9. 指標の読み方

この章では、提示された各指標について、何を測っているか、なぜ必要か、どう読めばいいかを順番に整理します。

### 9.1 ロック・待機

#### ロック待機時間(ms)

`sys.dm_os_wait_stats` から `wait_type LIKE 'LCK%'` の `wait_time_ms` 合計差分を取る指標です。これは「観測区間のあいだに、ロック待ちでどれだけの待機時間が積み上がったか」を表します。RCSI の直接的な狙いは、読取が共有ロック中心で待たされる時間を減らすことなので、同じ負荷ならこの値が下がるかどうかは最初に見るべきポイントです。[sys.dm_os_wait_stats][dm_os_wait_stats]

ただし、この値は完了した待機の累積であり、現在進行中の待ちをそのまま示すものではありません。したがって、「今まさに止まっているか」を知りたいときは次のブロッキング数も併せて見る必要があります。また、SQL Server 全体の値であるため、同一インスタンスに別ワークロードが混在していると解釈が難しくなります。

#### ロック待機割合(%)

ロック待機時間差分を、全待機時間差分で割った比率です。この値は「システム全体の待機の中で、ロックがどれだけ支配的だったか」を見ます。絶対時間だけを見ると、単に負荷が増えたから待ちが増えたのか、ロックがボトルネック化したのか分かりにくいことがあります。割合を見ると、RCSI 前後でロック依存の度合いが下がったかを把握しやすくなります。

ただし、実務では全待機時間にアイドル系待機や背景待機が含まれると比率の意味が薄くなることがあります。今回は研修用に全待機時間で説明しますが、より厳密な分析では除外対象待機を定義してから比率を見ることもあります。重要なのは、絶対値だけでなく、比率も差分で見ることです。

#### ブロッキング数

`sys.dm_exec_requests` で `blocking_session_id <> 0` の件数を数える指標です。これは「今この瞬間に、誰かに止められている要求が何件あるか」を表します。RCSI の導入前後で、ロック待機時間が減ったとしても、瞬間的に頭を押さえている処理が残っていれば、現場の体感としてはまだ遅いままです。そのため、累積待機と瞬間ブロッキングは両方必要です。[sys.dm_exec_requests][dm_exec_requests]

ここで注意すべきなのは、`blocking_session_id` が必ずしも通常のセッション ID とは限らないことです。Microsoft Learn では `-2`、`-3`、`-4`、`-5` といった特殊値が説明されています。特に `-5` は「ブロッカーのセッション ID を追跡していないラッチ型」であり、それ自体がただちに性能問題と同義ではありません。したがって、件数を見るだけでなく、必要に応じて `blocking_session_id` の内訳や `wait_type` も確認します。[sys.dm_exec_requests][dm_exec_requests]

### 9.2 tempdb

#### version store サイズ(MB)

`tempdb.sys.dm_db_file_space_usage` の `version_store_reserved_page_count` をページ数から MB へ換算した指標です。1 ページは 8KB なので、`SUM(version_store_reserved_page_count) * 8 / 1024` で MB へ直せます。RCSI の副作用を最も直接に見る指標であり、「ロック待機の減少の裏で、行バージョンの保持量がどれだけ増えたか」を確認できます。[sys.dm_db_file_space_usage][dm_db_file_space_usage] [tempdb database][tempdb-database]

ここで重要なのは、version store はグローバル資源であり、単純にセッションへ帰属できないことです。Microsoft Learn でも、version store はファイルレベルで追跡され、セッション単位やタスク単位で単純には扱えないと説明されています。したがって、この値は「RCSI セッションが何 MB 使ったか」ではなく、「その時点で tempdb 上にどれだけの version store があるか」と読むべきです。[sys.dm_db_file_space_usage][dm_db_file_space_usage]

なお、ADR の構成によっては version store の位置づけが変わるため、この指標は環境前提を確認したうえで解釈します。本資料では、提示された監視設計どおり、tempdb 側の観測を中心に説明します。[Transaction locking and row versioning guide - version store][locking-version-store]

#### tempdb 使用率(%)

`(SUM(total_page_count) - SUM(unallocated_extent_page_count)) / SUM(total_page_count) * 100` で表す指標です。これは tempdb の厳密な「実使用率」というより、どの程度が割り当て済みかを見る比率です。ユーザー備考のとおり、監視用途としては有効ですが、純粋な使用中ページ率そのものではありません。[sys.dm_db_file_space_usage][dm_db_file_space_usage]

RCSI の評価では、この値単独で良し悪しを決めるのではなく、version store サイズや tempdb I/O 待機と合わせて見ます。たとえば、version store が増えても tempdb 使用率に十分な余裕があり、I/O 待機も増えていなければ、副作用としては管理可能と判断しやすくなります。

#### tempdb IO待機(ms)

`sys.dm_io_virtual_file_stats(DB_ID('tempdb'), NULL)` の `io_stall_read_ms + io_stall_write_ms` 差分です。tempdb のデータファイルやログファイルに対する I/O 待ち時間を合算したものであり、RCSI によって tempdb が新たなボトルネックになっていないかを見るのに使います。[sys.dm_io_virtual_file_stats][dm_io_virtual_file_stats]

この値も累積値なので差分で見る必要があります。また、Microsoft Learn の説明どおり I/O 待機はファイル単位の総和であり、同時並行の I/O が多ければ 1 秒間の差分が 1000ms を超えることも普通に起こり得ます。したがって、「1000ms を超えたから異常」と短絡せず、RCSI 前後比較と他指標との整合で判断します。

### 9.3 CPU、メモリ、I/O 負荷

#### CPU使用率(%)

提示された指標では、Azure SQL Managed Instance の `master.sys.server_resource_stats` から `avg_cpu_percent` を取得します。これは Managed Instance のサービスティア上限に対する平均 CPU 利用率であり、ホスト全体の OS CPU そのものではありません。行は 15 秒ごとですが、ビュー更新に遅延があるため、短い性能試験ではピークの瞬間を見逃しやすい点が重要です。[sys.server_resource_stats (Azure SQL Managed Instance)][mi-server-resource-stats]

この指標を見る理由は、RCSI によってロック待機が減った結果、今度は CPU が飽和していないかを確認するためです。ロックで止まっていた処理が並行に進むようになると、CPU 消費は上がることがあります。CPU 上昇が悪いとは限りませんが、応答時間改善と引き換えに上限へ張り付いているなら、次のボトルネック候補になります。

#### SQLメモリ使用率(%)

`sys.dm_os_sys_info.committed_kb / committed_target_kb * 100` で表す指標です。`committed_kb` は SQL Server メモリマネージャが現在コミットしている量、`committed_target_kb` は欲しい目標量です。Microsoft Learn では、目標が現在値より大きければ、メモリマネージャはさらにメモリを得ようとし、目標が現在値より小さければ縮小しようとすると説明されています。[sys.dm_os_sys_info][dm_os_sys_info]

この値は、100% 近いから即異常という単純なものではありません。SQL Server は、必要ならメモリを積極的に使うエンジンだからです。むしろ典型運用の中で長時間にわたり `committed_target_kb` が `committed_kb` よりかなり大きいなら、欲しい量まで確保できていない可能性を疑います。したがって、この指標は「空いているほど良い」ではなく、「SQL Server が欲しいメモリを十分に持てているか」を見る指標です。

#### OSメモリ空き率(%)

`sys.dm_os_sys_memory.available_physical_memory_kb / total_physical_memory_kb * 100` で表す指標です。これは SQL Server 外も含めた OS 側の余力を見ます。SQL Server 内部ではメモリ使用率が問題なく見えても、OS 全体が逼迫していればページングや他プロセスの干渉が起きる可能性があります。[sys.dm_os_sys_memory][dm_os_sys_memory]

Azure SQL Managed Instance ではホスト相当の OS 観点の値として解釈することになり、純粋なユーザープロセスだけを切り分けた数字ではありません。そのため、「SQL メモリ使用率」と「OS メモリ空き率」を一緒に見て、SQL Server 内部の圧迫なのか、外部要因も含めた全体圧迫なのかを区別します。

#### IO待機時間(ms)

`sys.dm_io_virtual_file_stats(NULL, NULL)` から全データベース・全ファイルの `io_stall_read_ms + io_stall_write_ms` 差分を見ます。これはインスタンス全体のディスク待機をざっくり把握するための指標です。RCSI によってロック待機が減っても、ストレージが詰まっていれば最終的な応答は良くなりません。[sys.dm_io_virtual_file_stats][dm_io_virtual_file_stats]

この指標の使いどころは、「tempdb だけでなく全体の I/O も見ておく」ことです。tempdb I/O が増えていても、全体 I/O に比べて小さければ大勢へ影響していない可能性があります。逆に tempdb が目立たなくても、ユーザーデータベース側のファイル I/O が高止まりしていれば、RCSI より先にストレージ改善やクエリ設計改善が必要かもしれません。

### 9.4 クエリ応答時間

#### 平均応答時間(ms)

Query Store の `sys.query_store_runtime_stats.avg_duration` はマイクロ秒で記録されます。したがって、平均応答時間は `SUM(avg_duration * count_executions) / SUM(count_executions) / 1000` のように加重平均でミリ秒へ換算するのが筋です。これは「全体としてどれくらい速くなったか」を見る代表指標です。[sys.query_store_runtime_stats][query-store-runtime-stats]

ただし、Query Store は集計間隔ベースで保存する仕組みなので、瞬時の揺れをそのまま追う用途には向きません。短時間テストなら、テスト区間に含まれる Query Store interval を後から集計する、という使い方が現実的です。[How Query Store collects data][query-store-collects] [sys.database_query_store_options][database-query-store-options]

#### P95応答時間(ms)

P95 は、応答時間を速い順に並べたとき、95% 地点にある値です。平均は全体の代表値として便利ですが、一部の遅い処理が埋もれやすいという欠点があります。業務体感に近いのは、平均よりも P95 や P99 のような遅い側の指標であることが多く、RCSI の導入効果を体感へ結び付けるなら非常に重要です。

一方で Query Store は、平均、最小、最大、標準偏差は持ちますが、厳密な P95 を直接は持ちません。そのため、P95 はテストツールやアプリケーション計測の生ログから出す方が素直です。Query Store だけで P95 に近い評価をしようとすると、集計窓や再集計ロジックの設計が必要になります。ここは平均応答時間と違い、最初から「別ソースが本命」と考えた方が安全です。[sys.query_store_runtime_stats][query-store-runtime-stats]

## 10. 指標同士をどう組み合わせて判断するか

性能分析では、一つの指標だけで結論を出すとほぼ必ず誤ります。ここでは、典型的な見え方を整理します。

### 10.1 RCSI がうまく効いているパターン

ロック待機時間とロック待機割合が下がり、ブロッキング数も減っている。その一方で version store は多少増えるが、tempdb 使用率と tempdb I/O 待機は管理可能範囲に収まり、平均応答時間と P95 が改善している。この形なら、RCSI は狙いどおり「読取ブロッキングを減らし、体感性能を改善した」と判断しやすくなります。

### 10.2 ロックは減ったが、副作用が大きいパターン

ロック待機は確かに減っているが、version store が大きく膨らみ、tempdb I/O 待機も増え、平均応答時間は横ばいか悪化している。この場合は、ボトルネックが lock から tempdb へ移った可能性があります。RCSI 自体が悪いのではなく、tempdb 構成や長時間トランザクション、ワークロード特性との相性が問題かもしれません。

### 10.3 そもそもロックが主因ではなかったパターン

RCSI 前後でロック待機時間もブロッキング数もあまり変わらないのに、CPU や全体 I/O が高いまま、応答も改善しない。この場合は、元の遅さの主因が実行計画、索引設計、CPU 飽和、ストレージ性能など別の場所にあります。RCSI は万能薬ではない、という典型例です。

### 10.4 平均は良くなったが P95 が悪いパターン

平均応答時間は改善したのに、P95 は改善しない、あるいは悪化することがあります。これは「大半は速くなったが、一部の遅い処理はまだ残っている」状態です。短いブロッキング、特定クエリだけのプラン不安定、長時間トランザクションの尾引きなどが候補になります。この場合、平均だけ見て成功と判断すると、現場体感とずれます。

## 11. すぐ使える取得 SQL

ここでは、提示された式を SQL に落としたときの基本形を示します。研修では「この SQL をそのまま使う」ことより、「どの値が累積で、どの値が瞬間で、どれを差分にすべきか」を理解することが大切です。

### 11.1 ロック待機時間とロック待機割合

```sql
SELECT
    SUM(CASE WHEN wait_type LIKE 'LCK%' THEN wait_time_ms ELSE 0 END) AS lck_wait_time_ms,
    SUM(wait_time_ms) AS total_wait_time_ms
FROM sys.dm_os_wait_stats;
```

この SQL を時刻 t0 と t1 で取得し、差分を計算します。

- `delta_lck_wait_time_ms = t1.lck_wait_time_ms - t0.lck_wait_time_ms`
- `delta_total_wait_time_ms = t1.total_wait_time_ms - t0.total_wait_time_ms`
- `lock_wait_ratio_pct = delta_lck_wait_time_ms * 100.0 / delta_total_wait_time_ms`

### 11.2 ブロッキング数

```sql
SELECT COUNT(*) AS blocking_request_count
FROM sys.dm_exec_requests
WHERE blocking_session_id <> 0;
```

必要なら内訳も確認します。

```sql
SELECT
    blocking_session_id,
    COUNT(*) AS blocked_requests
FROM sys.dm_exec_requests
WHERE blocking_session_id <> 0
GROUP BY blocking_session_id
ORDER BY blocked_requests DESC;
```

### 11.3 tempdb の version store と tempdb 使用率

```sql
USE tempdb;
GO

SELECT
    SUM(version_store_reserved_page_count) * 8.0 / 1024 AS version_store_mb,
    (SUM(total_page_count) - SUM(unallocated_extent_page_count)) * 100.0
        / NULLIF(SUM(total_page_count), 0) AS tempdb_allocated_ratio_pct
FROM sys.dm_db_file_space_usage;
```

### 11.4 tempdb I/O 待機と全体 I/O 待機

```sql
SELECT
    SUM(io_stall_read_ms + io_stall_write_ms) AS tempdb_io_stall_ms
FROM sys.dm_io_virtual_file_stats(DB_ID(N'tempdb'), NULL);
```

```sql
SELECT
    SUM(io_stall_read_ms + io_stall_write_ms) AS total_io_stall_ms
FROM sys.dm_io_virtual_file_stats(NULL, NULL);
```

これらも t0 と t1 の差分で評価します。

### 11.5 CPU 使用率

Azure SQL Managed Instance の場合です。

```sql
SELECT TOP (1)
    end_time,
    avg_cpu_percent
FROM master.sys.server_resource_stats
ORDER BY end_time DESC;
```

短時間テストでは遅延を考慮し、この値だけで瞬時ピークを判断しないようにします。

### 11.6 SQL メモリ使用率と OS メモリ空き率

```sql
SELECT
    committed_kb,
    committed_target_kb,
    committed_kb * 100.0 / NULLIF(committed_target_kb, 0) AS sql_memory_util_pct
FROM sys.dm_os_sys_info;
```

```sql
SELECT
    available_physical_memory_kb,
    total_physical_memory_kb,
    available_physical_memory_kb * 100.0 / NULLIF(total_physical_memory_kb, 0) AS os_free_memory_pct
FROM sys.dm_os_sys_memory;
```

### 11.7 Query Store を使った平均応答時間

```sql
DECLARE @from_utc datetimeoffset = '2026-04-09T00:00:00+00:00';
DECLARE @to_utc   datetimeoffset = '2026-04-09T01:00:00+00:00';

SELECT
    SUM(rs.avg_duration * rs.count_executions)
        / NULLIF(SUM(rs.count_executions), 0)
        / 1000.0 AS avg_response_ms
FROM sys.query_store_runtime_stats AS rs
JOIN sys.query_store_runtime_stats_interval AS rsi
    ON rs.runtime_stats_interval_id = rsi.runtime_stats_interval_id
WHERE rsi.start_time >= @from_utc
  AND rsi.end_time <= @to_utc;
```

Query Store の現在設定も事前に確認しておくと安全です。

```sql
SELECT
    actual_state_desc,
    desired_state_desc,
    interval_length_minutes,
    current_storage_size_mb,
    max_storage_size_mb
FROM sys.database_query_store_options;
```

## 12. 運用上の考慮点

監視項目は正しく選んでも、運用の仕方を誤ると解釈を誤ります。ここでは実務上の注意をまとめます。

### 12.1 サンプリング間隔は指標の性質に合わせる

ブロッキング数は瞬間値なので短い間隔が向きます。一方、累積値の差分指標は、短すぎるとノイズが増え、長すぎると変化をならしてしまいます。一般には、ブロッキングは 1 秒から数秒、累積待機や I/O は数秒から数十秒、Query Store はテスト後集計、という分担が分かりやすいです。

### 12.2 tempdb はグローバル資源だと忘れない

tempdb は一つのセッション専用ではありません。一時テーブル、内部作業領域、version store などが共用します。したがって、RCSI の副作用を tempdb で見るときは、「RCSI のせいで全てが増えた」と決め打ちせず、他の tempdb 利用要因も同時に考えるべきです。[tempdb database][tempdb-database]

### 12.3 長時間トランザクションは version store の天敵

version store のクリーンアップは、必要な最古バージョンをまだ参照するトランザクションが終わるまで進みにくくなります。そのため、RCSI 導入後に version store が膨らむ場合、単に更新量が多いだけでなく、長時間トランザクションが残っていないかも確認する必要があります。[sys.dm_db_file_space_usage][dm_db_file_space_usage]

### 12.4 Query Store の状態を確認する

Query Store はサイズ上限に達すると read-only へ切り替わり、新しいデータを集めなくなることがあります。平均応答時間の比較をする前に、Query Store が read-write 状態で継続収集中かを確認しておくべきです。[Best practices for managing the Query Store][manage-query-store] [Best practices for monitoring workloads with Query Store][best-practice-query-store-monitoring]

## 13. 使うべき場面と、これだけでは足りない場面

今回の指標セットは、RCSI の導入可否や導入効果の確認には非常に向いています。なぜなら、狙いであるロック削減、その副作用の tempdb、資源移動先の CPU/メモリ/I/O、最終結果の応答時間まで一連で追えるからです。

一方で、これだけでは足りない場面もあります。たとえば「特定の 1 クエリだけが遅い」場合は実行計画や索引を詳細に見る必要がありますし、「平均も P95 も悪いが lock は低い」場合は、クエリ設計やストレージ構成、アプリケーション側の待ち、ネットワーク遅延も視野に入れなければなりません。今回のセットは、あくまで RCSI 影響の全体診断フレームです。

## 14. よくある誤解や失敗パターン

**RCSI を入れれば全てのブロッキングがなくなる**

誤りです。読取ブロッキングは減りやすいですが、書込み同士の競合やスキーマ関連の待ちは残ります。[Transaction locking and row versioning guide - row versioning behavior][locking-row-versioning-behavior]

**`wait_time_ms` が大きいから今遅い**

誤りです。`wait_time_ms` は累積値です。起動後ずっと積み上がるので、差分を取らずに単発で見ても今の状態は分かりません。[sys.dm_os_wait_stats][dm_os_wait_stats]

**blocking_session_id が負の値なら無視してよい**

単純化しすぎです。通常のセッション ID ではないだけで、待機が存在すること自体は事実です。特に `-5` は「非同期処理待ちだから即障害ではない」という意味であり、「完全に無関係」という意味ではありません。[sys.dm_exec_requests][dm_exec_requests]

**version store はセッション単位で簡単に割り当てられる**

誤りです。version store はグローバル資源です。単純なセッション別帰属を前提にすると、解釈を誤ります。[sys.dm_db_file_space_usage][dm_db_file_space_usage]

**Query Store の avg_duration はミリ秒である**

誤りです。`avg_duration` はマイクロ秒です。ミリ秒へ直すには 1000 で割る必要があります。[sys.query_store_runtime_stats][query-store-runtime-stats]

**平均応答時間が改善したから成功である**

早計です。P95 や一部シナリオの遅さが残っていれば、現場体感は悪いままです。平均は重要ですが、平均だけで結論を出してはいけません。

## 15. 結論

SQL Server を理解するうえで大切なのは、SQL を投げる先を単なる保存箱ではなく、整合性、同時実行制御、回復性、観測性をまとめて担う Database Engine として捉えることです。その観点で見ると、RCSI は「ロックを消す機能」ではなく、「既定の `READ COMMITTED` の読み取り方法を、共有ロック依存から行バージョン依存へ寄せることで、読取ブロッキングを減らす設計変更」だと整理できます。

そして RCSI の良し悪しは、一つの数値では判断できません。ロック待機時間、ロック待機割合、ブロッキング数で直接効果を見て、version store サイズ、tempdb 使用率、tempdb I/O 待機で副作用を見て、CPU、メモリ、I/O でボトルネック移動を確認し、最後に平均応答時間と P95 でユーザー体感へ結びついたかを確定する。この順序で見れば、提示された測定項目は単なる監視表ではなく、RCSI の効果検証を支える一つの論理体系として理解できます。

## 16. 参考情報

- [What is SQL Server?][sqlserver-what-is]
- [The Microsoft SQL Database Engine][sql-database-engine]
- [The Microsoft SQL Database Engine - ACID][sql-database-engine-acid]
- [Transaction locking and row versioning guide][locking-row-versioning]
- [Transaction locking and row versioning guide - locking and row versioning basics][locking-basics-rcsi]
- [Transaction locking and row versioning guide - row versioning resource usage][locking-version-store]
- [Transaction locking and row versioning guide - behavior when reading data][locking-row-versioning-behavior]
- [SET TRANSACTION ISOLATION LEVEL (Transact-SQL)][set-transaction-isolation]
- [Snapshot isolation in SQL Server][snapshot-isolation]
- [tempdb database][tempdb-database]
- [sys.dm_os_wait_stats][dm_os_wait_stats]
- [sys.dm_exec_requests][dm_exec_requests]
- [sys.dm_db_file_space_usage][dm_db_file_space_usage]
- [sys.dm_io_virtual_file_stats][dm_io_virtual_file_stats]
- [sys.dm_os_sys_info][dm_os_sys_info]
- [sys.dm_os_sys_memory][dm_os_sys_memory]
- [sys.server_resource_stats (Azure SQL Managed Instance)][mi-server-resource-stats]
- [Monitoring Azure SQL Managed Instance performance using dynamic management views][mi-monitoring-dmvs]
- [How Query Store collects data][query-store-collects]
- [sys.query_store_runtime_stats][query-store-runtime-stats]
- [sys.database_query_store_options][database-query-store-options]
- [Best practices for managing the Query Store][manage-query-store]
- [Best practices for monitoring workloads with Query Store][best-practice-query-store-monitoring]

[sqlserver-what-is]: https://learn.microsoft.com/sql/sql-server/what-is-sql-server?view=sql-server-ver17
[sql-database-engine]: https://learn.microsoft.com/sql/database-engine/sql-database-engine?view=sql-server-ver17
[sql-database-engine-acid]: https://learn.microsoft.com/sql/database-engine/sql-database-engine?view=sql-server-ver17#database-fundamentals-acid-compliance
[sql-database-engine-platforms]: https://learn.microsoft.com/sql/database-engine/sql-database-engine?view=sql-server-ver17#modern-platforms-using-the-sql-database-engine
[locking-row-versioning]: https://learn.microsoft.com/sql/relational-databases/sql-server-transaction-locking-and-row-versioning-guide?view=sql-server-ver17
[locking-basics-rcsi]: https://learn.microsoft.com/sql/relational-databases/sql-server-transaction-locking-and-row-versioning-guide?view=sql-server-ver17#locking-and-row-versioning-basics
[locking-version-store]: https://learn.microsoft.com/sql/relational-databases/sql-server-transaction-locking-and-row-versioning-guide?view=sql-server-ver17#row-versioning-based-isolation-levels-in-the-database-engine
[locking-row-versioning-behavior]: https://learn.microsoft.com/sql/relational-databases/sql-server-transaction-locking-and-row-versioning-guide?view=sql-server-ver17#row-versioning-based-isolation-levels-in-the-database-engine
[set-transaction-isolation]: https://learn.microsoft.com/sql/t-sql/statements/set-transaction-isolation-level-transact-sql?view=sql-server-ver17
[snapshot-isolation]: https://learn.microsoft.com/sql/connect/ado-net/sql/snapshot-isolation-sql-server?view=sql-server-ver17
[tempdb-database]: https://learn.microsoft.com/sql/relational-databases/databases/tempdb-database?view=sql-server-ver17
[dm_os_wait_stats]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-os-wait-stats-transact-sql?view=sql-server-ver17
[dm_exec_requests]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-exec-requests-transact-sql?view=sql-server-ver17
[dm_db_file_space_usage]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-db-file-space-usage-transact-sql?view=sql-server-ver17
[dm_io_virtual_file_stats]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-io-virtual-file-stats-transact-sql?view=sql-server-ver17
[dm_os_sys_info]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-os-sys-info-transact-sql?view=sql-server-ver17
[dm_os_sys_memory]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-os-sys-memory-transact-sql?view=sql-server-ver17
[mi-server-resource-stats]: https://learn.microsoft.com/sql/relational-databases/system-catalog-views/sys-server-resource-stats-azure-sql-database?view=azuresqldb-current
[mi-monitoring-dmvs]: https://learn.microsoft.com/azure/azure-sql/managed-instance/monitoring-with-dmvs?view=azuresql
[query-store-collects]: https://learn.microsoft.com/sql/relational-databases/performance/how-query-store-collects-data?view=sql-server-ver17
[query-store-runtime-stats]: https://learn.microsoft.com/sql/relational-databases/system-catalog-views/sys-query-store-runtime-stats-transact-sql?view=sql-server-ver17
[database-query-store-options]: https://learn.microsoft.com/sql/relational-databases/system-catalog-views/sys-database-query-store-options-transact-sql?view=sql-server-ver17
[manage-query-store]: https://learn.microsoft.com/sql/relational-databases/performance/manage-the-query-store?view=sql-server-ver17
[best-practice-query-store-monitoring]: https://learn.microsoft.com/sql/relational-databases/performance/best-practice-with-the-query-store?view=sql-server-ver17#verify-that-query-store-collects-query-data-continuously