# SQL Server の全体像 詳細解説

> 位置づけ: この文書は [[SQL Server 入門から始める RCSI 性能評価研修]] の第4章を掘り下げる補足資料です。接続から観測までの流れを一本で理解したいときに読みます。
>
> 読み分け: 分離レベルの比較判断は [[SQL Server Snapshot Isolation と RCSI と Read Committed の違い]]、監視オブジェクトの索引は [[SQL Server DMV一覧]]、フォルダ全体の案内は [[SQL Server ドキュメントガイド]] を参照してください。

この文書は、[[SQL Server 入門から始める RCSI 性能評価研修]] の第4章「SQL Server の全体像」を詳しく説明するための補足資料です。元の章では、接続、Query Processor、Storage Engine、トランザクションログ、ロックと行バージョン、tempdb、DMV と Query Store という七つの観点を一枚の絵として並べました。本資料では、その七つがそれぞれ何を意味し、どこでつながっていて、なぜ性能指標の読み解きに効くのかを、基礎から順に説明します。

SQL Server の性能分析が難しく見えるのは、現象が一つの場所で完結しないからです。ユーザーから見ると「画面が遅い」だけでも、実際には接続の張り直し、プラン選択の失敗、ページ I/O の増加、ログ書き込み待ち、ロック競合、tempdb 膨張、観測粒度の不足など、まったく違う原因が同じ遅さとして現れます。逆に言えば、SQL Server を一本の処理経路として理解できるようになると、どの性能指標がどの層を見ているのかが整理され、判断が一気にしやすくなります。

この資料の対象は、SQL Server の経験が浅い読者から、性能分析や設計判断へ踏み込みたい読者までを想定しています。構文の暗記よりも、Database Engine の中で何が起きているかを、文脈から理解することを目的にしています。細かなチューニング手順や製品設定の網羅は扱わず、全体像を正しく持つことに責務を絞ります。

## 1. 背景と課題

業務アプリケーションは、通常、データベースをブラックボックスとして扱いがちです。アプリケーションは接続文字列を使って SQL を投げ、必要な結果を返してもらうだけに見えます。しかし、データベースは単なる保存箱ではありません。SQL Server の Database Engine は、保存、検索、更新、並行実行制御、障害回復、観測性をまとめて引き受ける実行基盤です。[What is SQL Server?][sqlserver-what-is] [The Microsoft SQL Database Engine][sql-database-engine]

この基盤性を理解しないまま性能指標を見ると、数字がばらばらに見えます。たとえば `LCK_M_X` はロックの話、`io_stall_read_ms` は I/O の話、`avg_duration` は応答時間の話、と個別には分かっても、それらが同じ処理の別断面だと捉えられないと、原因と結果を取り違えます。RCSI のような並行実行制御の変更を評価するときに特に重要なのは、ロック待機が減ったことだけではなく、その効果が Query Processor、Storage Engine、tempdb、最終応答時間へどう波及したかを一本で説明できることです。

そのため、この文書では SQL Server の内部を「接続してから観測されるまでの連続した流れ」として扱います。これにより、性能指標の所在を、構造と役割の両方から理解できるようにします。

## 2. 全体像

SQL Server を一枚の絵として捉えるなら、次の七段階で考えると理解しやすくなります。

1. アプリケーションが SQL Server へ接続し、要求を送る。
2. Query Processor が SQL を解析し、実行計画を作るか再利用する。
3. Storage Engine が必要なページをメモリまたはディスクから扱う。
4. 更新処理はトランザクションログへ記録され、回復可能性を担保する。
5. 複数要求が同時に走るため、ロックや行バージョンで整合性を保つ。
6. 一時作業や version store のために tempdb が使われる。
7. 実行中や履歴の様子は DMV や Query Store で観測される。

この七つは、独立した部品一覧ではありません。Query Processor が決めた実行計画に従って Storage Engine がページを読み、更新ならログを出し、同時実行ならロックや行バージョンを使い、その副作用が tempdb に現れ、最後に DMV や Query Store がその足跡を見せます。性能分析では、どこか一つだけを見ても不十分で、全体のどこで詰まったかを見分ける必要があります。

もう少し短く言えば、SQL Server は「要求を受ける層」「計画を決める層」「データを触る層」「整合性を守る層」「痕跡を見せる層」が重なったシステムです。これを理解しておくと、たとえばロック待機が高いときに、ただちに索引設計だけを疑うのではなく、そもそも Query Processor がどのアクセス方法を選んだか、長いトランザクションがないか、tempdb の version store が膨らんでいないかまで自然に視野に入ります。

## 3. 中核概念と用語

この先の説明を安定して読むため、先に重要語をそろえます。

**セッション**

クライアント接続に対応する実行単位です。ユーザーやアプリケーションが接続すると、SQL Server 側にはその接続に紐づくセッションができます。

**要求**

セッション上で実行されるクエリやバッチの論理表現です。`sys.dm_exec_requests` は、いま走っている要求を見せる DMV です。[Thread and task architecture guide][thread-task-guide] [sys.dm_exec_requests][dm-exec-requests]

**タスク**

要求を満たすために実行される作業単位です。並列実行では一つの要求に複数タスクがぶら下がります。[Thread and task architecture guide][thread-task-guide]

**Query Processor**

SQL を解析し、どの順番で、どのアクセス方法で、どの演算子を使って実行するかを決める側です。最終成果物は実行計画です。[query-processing-architecture][query-processing-architecture]

**Storage Engine**

実行計画に従って、実際にページを読み書きし、バッファやファイル、ログと連携する側です。ページ、エクステント、バッファキャッシュ、I/O の理解はここに属します。[memory-management-architecture][memory-management-architecture] [pages-extents-guide][pages-extents-guide]

**ページ**

Database Engine における基本的なデータ保存単位です。サイズは 8 KiB です。データファイルへの I/O はページ単位で行われます。[pages-extents-guide][pages-extents-guide]

**エクステント**

8 個の連続したページをまとめた単位です。サイズは 64 KiB です。ページ割り当てを効率化するために使われます。[pages-extents-guide][pages-extents-guide]

**トランザクションログ**

各トランザクションとデータ変更を記録する仕組みです。障害時に一貫した状態へ戻すための土台です。データファイルと違い、ログファイルはページではなく可変長のログレコード列で構成されます。[transaction-log-guide][transaction-log-guide] [the-transaction-log][the-transaction-log]

**ロック**

矛盾する同時アクセスを防ぐための制御機構です。読み取りには共有ロック、更新には排他ロックなどが関わります。競合すると待機が発生します。[locking-row-versioning][locking-row-versioning]

**行バージョン**

更新前の行のコピーを保持し、読み取りが途中の更新に引きずられないようにする仕組みです。RCSI や SNAPSHOT で重要になります。[locking-row-versioning][locking-row-versioning]

**tempdb**

一時テーブル、内部作業領域、version store を収容するグローバル資源です。RCSI の副作用やソート、ハッシュ、スプールと強く関係します。[tempdb-database][tempdb-database]

**DMV**

Dynamic Management View の略で、現在の内部状態を見せるビュー群です。サーバー状態、待機、要求、メモリ、I/O などを観測できます。[system-dmvs][system-dmvs]

**Query Store**

クエリ、実行計画、実行統計、待機統計を時間窓で保持する機能です。現在の一瞬よりも、時間をまたいだ変化や回帰の追跡に向きます。[monitor-query-store][monitor-query-store]

## 4. 仕組み

ここからは、七つの流れを実際の中身として追います。重要なのは、各段階を単独で覚えるのではなく、前の段階の出力が次の段階の入力になることです。

### 4.1 アプリケーションが接続し、要求を送る

すべてはクライアント接続から始まります。アプリケーション、バッチ、管理ツールは SQL Server へ接続し、T-SQL のクエリやバッチを送ります。SQL Server から見ると、この時点でまずセッションができ、そのセッション上で要求が発生します。要求は論理的な実行単位であり、後続の解析、最適化、実行、待機の対象になります。[sql-database-engine][sql-database-engine] [thread-task-guide][thread-task-guide]

ここで初心者が見落としやすいのは、ユーザーが一人でも、SQL Server の内部では「接続」「要求」「タスク」が分かれていることです。たとえば、一つのセッション上で一つの SELECT を実行しているつもりでも、並列実行になれば内部では複数タスクが動きます。したがって、画面上では一つの操作でも、内部では複数の実行主体が資源を奪い合っている可能性があります。

性能指標の観点では、この段階は接続数、セッション数、実行中要求数、待機中要求の把握につながります。特に `sys.dm_exec_requests` は「いま何が走っているか」を見る入口であり、ブロッキングや待機時間を読むときの起点になります。[sys.dm_exec_requests][dm-exec-requests]

### 4.2 Query Processor が SQL を解析し、実行計画を決める

要求を受けた SQL Server は、ただちにテーブルを読みに行くわけではありません。まず Query Processor が SQL 文を解析します。Microsoft Learn の Query processing architecture guide では、基本的な SELECT 処理として、パーサーが文を論理単位へ分解し、query tree を作り、Query Optimizer が複数のアクセス方法を比較し、最終的な実行計画を選ぶ流れが説明されています。[query-processing-architecture][query-processing-architecture]

ここで重要なのは、SQL は「何をほしいか」を書く宣言的言語であり、「どう取るか」は Query Processor が決めるという点です。同じ結果を返す SQL でも、索引シーク、索引スキャン、テーブルスキャン、結合順序、並列化の有無など、内部のやり方はいくつもありえます。Query Optimizer は、統計情報や推定コストに基づいて、その時点で最も効率がよいと思われる計画を選びます。

この設計の利点は、データ量や分布が変わっても、エンジンが自動的によりよい計画を選び直せることです。一方の弱点は、統計やパラメータ値の偏りによって望ましくない計画が選ばれることがある点です。性能問題のかなりの割合は、ストレージが遅いのではなく、入口のここで不利な計画を選んだことから始まります。

また、実行計画は毎回ゼロから作るとは限りません。SQL Server には実行計画のキャッシュ再利用機構があり、同じ文に対して既存計画を使える場合があります。これは CPU 節約に有利ですが、パラメータ感度のように、ある値では最適でも別の値では不利な計画が使い回される問題も生みます。[query-processing-architecture][query-processing-architecture]

並列実行も Query Processor が関与する重要な論点です。クエリが並列化されると、一つの要求に対して親タスクと複数の子タスクが作られます。したがって、性能分析で一つの要求しか見ていないつもりでも、実際には複数ワーカーが CPU、メモリ、I/O を使っていることがあります。[thread-task-guide][thread-task-guide]

### 4.3 Storage Engine がページをメモリやディスクから扱う

実行計画が決まると、今度は Storage Engine が必要なデータへ実際に触ります。ここで重要なのは、SQL Server が行を直接ファイルから一件ずつ読むのではなく、ページという単位でデータを扱うことです。ページは 8 KiB で、エクステントはその8ページ分、つまり 64 KiB です。データファイルへの I/O はページ単位で行われます。[pages-extents-guide][pages-extents-guide]

ページにはデータページ、インデックスページ、LOB ページ、各種システムページがあります。つまり、テーブルの中身だけでなく、割り当て情報や空き領域の情報もページで管理されています。Storage Engine は、必要な行を見つけるために、まずどのページを読むべきかを決め、そのページがメモリにあるか、ディスクから読む必要があるかを判断します。[pages-extents-guide][pages-extents-guide]

ここでバッファ管理が効いてきます。Memory management architecture guide では、Buffer Manager がデータページをディスクから buffer cache に読み込み、更新済みページを必要に応じて書き戻すと説明されています。バッファはページと同じ 8-KB 単位で、buffer cache はそのページ群で構成されます。ページがすでに buffer cache にあれば論理読み取りで済み、なければ物理読み取りが発生します。[memory-management-architecture][memory-management-architecture] [read-data-pages][read-data-pages]

この違いは性能に直結します。Query Processor が同じ計画を選んでも、必要ページの多くがメモリ内にあれば速く、毎回ディスクから読まなければ遅くなります。そのため、I/O 待機やバッファキャッシュの状態は、Storage Engine の層を見ている指標だと理解すると整理しやすくなります。

さらに、Storage Engine は Query Processor の選択を具体化する側でもあります。Query Processor が「この索引をシークし、その後ネストループ結合する」と決めれば、Storage Engine はそのために必要なページの読み取りパターンを最適化します。逆に言えば、Storage Engine の負荷は単独では生まれず、上流の実行計画に強く依存します。[query-processing-architecture][query-processing-architecture] [read-data-pages][read-data-pages]

### 4.4 更新はトランザクションログへ記録される

SELECT だけなら主にページ読取が中心ですが、INSERT、UPDATE、DELETE のような変更処理は、必ずトランザクションログと結びつきます。SQL Server のすべてのデータベースにはトランザクションログがあり、各トランザクションと、そのトランザクションが行った変更が記録されます。これはシステム障害時にデータベースを一貫した状態へ戻すための中核です。[transaction-log-guide][transaction-log-guide] [the-transaction-log][the-transaction-log]

ここで初心者が混同しやすいのは、データファイルとログファイルは同じような構造ではないという点です。データファイルはページとエクステントで管理されますが、ログファイルは固定サイズページではなく、可変長のログレコード列です。さらに、ログキャッシュはデータページのバッファキャッシュとは別管理です。[pages-extents-guide][pages-extents-guide] [the-transaction-log][the-transaction-log]

この構造が意味するのは、更新処理の遅さが、必ずしもデータページの書き込みだけで決まらないということです。ログ書き込みのレイテンシや、ログ肥大、VLF の過多、切り捨て遅延も更新性能へ影響します。したがって、「更新が遅い」という現象に対しては、データ I/O とログ I/O を分けて考える必要があります。

また、トランザクションログは回復性の土台でもあるため、短縮や削除を気軽に考えてはいけません。ログは単なる作業ファイルではなく、Database Engine が ACID の durability を実現するための本体の一部です。[sql-database-engine-acid][sql-database-engine-acid] [the-transaction-log][the-transaction-log]

### 4.5 同時実行制御としてロックや行バージョンが使われる

複数の要求が同時に走る以上、SQL Server は「正しさ」と「並行性」のバランスを取らなければなりません。ここで使われる代表的な仕組みがロックと行バージョンです。ロックは、他トランザクションと矛盾する読み書きを防ぐための直接的な制御です。行バージョンは、更新前の値を保持して、読み取りが更新中のデータに引きずられないようにする仕組みです。[locking-row-versioning][locking-row-versioning]

既定の `READ COMMITTED` では、SQL Server は共有ロックを使ってダーティリードを防ぎます。これにより整合性は保たれますが、書き込みと読み取りがぶつかるとブロッキングが発生します。RCSI はこの読み取り側の実装を行バージョンベースへ寄せることで、読取ブロッキングを減らす考え方です。[locking-row-versioning][locking-row-versioning]

重要なのは、ロックが悪で、行バージョンが善、という単純な話ではないことです。ロックは ACID の isolation を支える基本機構であり、消せばよいものではありません。行バージョンもコストゼロではなく、保持場所やクリーンアップの責務が必要です。設計としては、どこで待たせ、どこへコストを逃がすかの選択です。

性能指標の観点では、この層が `LCK%` 待機、`blocking_session_id`、デッドロック、Query Store の wait categories に対応します。RCSI 評価でロック待機時間やブロッキング数を見るのは、この層の挙動が直接変わるからです。[sys.dm_exec_requests][dm-exec-requests] [monitor-query-store][monitor-query-store]

### 4.6 tempdb は内部作業と version store の置き場である

tempdb は、初心者が「一時テーブルの置き場」程度に理解しがちな領域ですが、実際にはもっと重要です。Microsoft Learn では、tempdb はグローバル資源であり、ユーザーが明示的に作る一時オブジェクトだけでなく、スプール、ソート、ハッシュ、カーソル、LOB の一時領域、そして version store を保持すると説明されています。[tempdb-database][tempdb-database]

これは何を意味するかというと、tempdb は「SQL Server の裏方処理の吹きだまり」だということです。Query Processor や Storage Engine がある処理を効率的に実行しようとすると、その副作用や中間結果が tempdb へ逃がされることがあります。RCSI では特に version store がここへ入るため、ロック待機を減らした代わりに tempdb の容量や I/O が圧迫されることがあります。

しかも tempdb はグローバル資源です。あるセッションだけの専用領域ではないため、別ワークロードのソートやハッシュと、RCSI の version store が同じ場所で競合しえます。このため、tempdb を見る指標は「ある処理だけの責任量」を単純に表すのではなく、インスタンス全体の裏方負荷を映していると解釈する必要があります。

運用面では、tempdb の初期サイズ、ファイル数、ファイル成長設定、配置先の I/O 性能が非常に重要です。Microsoft Learn でも、適切な事前割り当てと、同サイズ・同成長設定の複数データファイルが推奨されています。[tempdb-database][tempdb-database]

### 4.7 DMV と Query Store が観測の役割を分担する

SQL Server の内部が見えるといっても、すべてを一つの仕組みで見られるわけではありません。現在状態を見るのに向くのが DMV、時間をまたいだ履歴を見るのに向くのが Query Store です。[system-dmvs][system-dmvs] [monitor-query-store][monitor-query-store]

DMV は server state information を返す内部ビュー群で、現在の要求、待機、メモリ、I/O、バッファなどを観測できます。たとえば `sys.dm_exec_requests` は今走っている要求、`sys.dm_os_wait_stats` は完了した待機の累積、`sys.dm_io_virtual_file_stats` はファイル I/O の累積、`sys.dm_os_sys_info` はメモリ目標とコミット量、といった具合に、かなり直接的に内部状態へ触れます。[system-dmvs][system-dmvs]

一方、Query Store はクエリ、プラン、実行統計、待機統計を時間窓で保持します。これは「いま何が起きているか」よりも、「昨日より遅くなったのはどのクエリか」「この時間帯にどの待機カテゴリが増えたか」「プラン変更の前後で何が変わったか」といった分析に向きます。[monitor-query-store][monitor-query-store]

この分担を誤ると、分析が空振りします。短いブロッキングを Query Store だけで追おうとしても粒度が合わず、逆に数日前から続く回帰を DMV だけで追おうとしても過去がありません。全体像の最後に DMV と Query Store が置かれているのは、これらが処理そのものを担当するのではなく、前段までのふるまいを観測可能にする層だからです。

### 4.8 七つを一本につなぐと何が見えるか

ここまでの話を一つのストーリーにすると、次のようになります。アプリケーションが接続し、要求が作られる。Query Processor が SQL を解析し、実行計画を選ぶ。Storage Engine がその計画を具体的なページ I/O とメモリアクセスに変換する。更新ならログが記録される。同時実行ならロックや行バージョンが関わる。補助的な作業や version store は tempdb を使う。最後に、それらの結果が DMV や Query Store に現れる。

つまり、待機統計は「別の問題」ではなく、内部フローの末端に現れた痕跡です。たとえば Query Processor が広いスキャン計画を選ぶと、Storage Engine の I/O が増え、I/O 待機が増え、処理時間が伸び、ロック保持時間も長くなり、最終的にブロッキングや Query Store の duration 上昇につながることがあります。このように、全体像を持つと一つの指標の変化を単独イベントではなく、因果連鎖として読めるようになります。

## 5. アーキテクチャと設計上の含意

SQL Server の全体像を知る価値は、説明ができることそのものではなく、設計判断の質が上がることにあります。

第一に、Query Processor と Storage Engine の境界を理解すると、SQL の書き方と物理設計の責任分担が見えます。アプリケーションは「必要な結果」を表現し、Database Engine は「最も効率のよい取り方」を探します。ただし、統計、索引、パラメータ分布、データ量の変化次第では、最適化が期待どおり働かないこともあります。したがって、アプリケーション SQL、索引設計、統計管理は別々ではなく、同じ最適化問題の異なる入力だと考えるべきです。

第二に、トランザクションログとロック機構を理解すると、性能と正しさのトレードオフが見えます。ログは更新コストを増やしますが、回復性と durability のために不可欠です。ロックは並行性を下げることがありますが、整合性のために必要です。RCSI のような仕組みは、そのコストを tempdb や version store 側へ動かす選択であり、無料の高速化ではありません。

第三に、tempdb がグローバル資源だと理解すると、局所最適の危険が見えます。あるクエリのソート改善、別機能のオンライン索引操作、RCSI 導入などが、それぞれ単独では正しい判断でも、同じ tempdb を消費して干渉することがあります。Database Engine の中では、複数の賢い仕組みが同じ裏方資源を共有しているのです。

第四に、観測層が独立していることを理解すると、監視設計の責務分担が明確になります。現在の異常検知は DMV、過去比較と回帰分析は Query Store、という分担が自然になります。この視点がないと、どの道具でどの問いに答えるべきかが曖昧になります。

## 6. 実装上と検証上の考慮点

この文書は実装手順書ではありませんが、全体像を理解したあとで何を意識すべきかは整理しておく価値があります。

### 6.1 性能指標をどの層へ対応づけるか

性能を見るときは、まず指標を層へ対応づけます。たとえば次の対応です。

1. `blocking_session_id` や `LCK%` 待機は、ロックと同時実行制御の層を見ている。
2. `io_stall_read_ms` や `io_stall_write_ms` は、Storage Engine とファイル I/O の層を見ている。
3. `committed_kb` や `committed_target_kb` は、バッファとメモリ管理の層を見ている。
4. `version_store_reserved_page_count` や tempdb 使用量は、行バージョンと tempdb の層を見ている。
5. Query Store の `avg_duration` や wait stats は、Query Processor の結果が時間軸でどう現れたかを見ている。

この対応づけができると、たとえばロック待機が減ったのに応答時間が改善しないとき、次に見るべき層が自然に決まります。

### 6.2 並列実行の見え方に注意する

並列クエリでは、一つの要求に複数タスクがぶら下がります。`sys.dm_exec_requests` では coordinator thread の情報中心に見える部分があるため、必要に応じて `sys.dm_os_tasks` まで見ないと、実際の並列度や待機の分布を見誤ることがあります。[thread-task-guide][thread-task-guide] [sys.dm_exec_requests][dm-exec-requests]

### 6.3 DMV は内部実装に近い情報である

Microsoft Learn でも、DMV は implementation-specific な内部状態を返し、将来のリリースで列が増減し得るため、運用コードで `SELECT *` を使うべきではないと明示しています。分析用の便利ビューである一方、契約が固定された業務 API ではありません。[system-dmvs][system-dmvs]

### 6.4 Query Store は時間窓を持つ

Query Store は万能な履歴装置ではなく、時間窓で集計した統計を保持します。したがって、一瞬のブロッキング観測には向かず、時間をまたいだ比較や plan regression 追跡に向きます。設計時にこの守備範囲を区別しておくべきです。[monitor-query-store][monitor-query-store]

## 7. 運用とセキュリティ上の考慮点

運用で重要なのは、SQL Server の全体像を構造だけでなく資源管理として見ることです。

tempdb は十分な初期サイズ、適切な複数ファイル構成、安定した I/O 基盤が必要です。ログファイルは適切な初期サイズと FILEGROWTH を持たせ、VLF の過剰生成を避ける必要があります。バッファキャッシュはメモリ余力と密接に関係します。つまり、Database Engine の各層は論理的には分かれていても、運用上は同じ資源予算の中で折り合いをつけています。[tempdb-database][tempdb-database] [transaction-log-guide][transaction-log-guide] [memory-management-architecture][memory-management-architecture]

権限面では、DMV は server-scoped と database-scoped があり、`VIEW SERVER STATE` または `VIEW DATABASE STATE` 系の権限が必要です。性能分析のためにどこまで見せるかは、運用設計と合わせて考える必要があります。[system-dmvs][system-dmvs]

また、Query Store は read-only 化や容量上限の影響を受けることがあるため、履歴が自動的に常に取れている前提にしない方が安全です。計画回帰の分析基盤として使うなら、状態監視も含めて運用対象にすべきです。[monitor-query-store][monitor-query-store]

## 8. 使うべき場面と、これだけでは足りない場面

この全体像モデルは、性能分析、RCSI 評価、障害切り分け、監視設計、設計レビューに非常に向いています。理由は、接続から観測までの一本の流れとして問題を説明できるからです。

一方で、このモデルだけでは足りない場面もあります。たとえば、アプリケーション接続プールの設定不備、ネットワーク遅延、ORM の発行 SQL の癖、ストレージ製品固有の挙動、OS レベルの圧迫などは、SQL Server の内部像だけでは完結しません。全体像は強力な土台ですが、境界の外にある要因まで自動で説明してくれるわけではありません。

## 9. よくある誤解や失敗パターン

**Query Processor と Storage Engine を同じものだと思う**

実務では非常に多い誤解です。計画を決める層と、ページを触る層を分けて考えないと、遅い原因をすべて「ディスクが遅い」か「SQL が悪い」の二択で見てしまいます。

**データファイルとログファイルは同じように読めばよいと思う**

誤りです。データファイルはページ・エクステント管理、ログファイルはログレコード管理で、役割も性能影響も異なります。[pages-extents-guide][pages-extents-guide] [transaction-log-guide][transaction-log-guide]

**tempdb は一時テーブルだけの領域だと思う**

誤りです。tempdb は内部オブジェクトと version store の重要な置き場であり、RCSI やソート、ハッシュの性能に直結します。[tempdb-database][tempdb-database]

**DMV があれば過去も分かると思う**

不正確です。DMV の多くは現在状態や起動後累積を返しますが、履歴管理機能ではありません。時間をまたいだ比較は Query Store など別の仕組みが必要です。[system-dmvs][system-dmvs] [monitor-query-store][monitor-query-store]

**Query Store があるから現在の短いブロッキングも十分に追えると思う**

誤りです。Query Store は時間窓で集計された履歴情報に強く、一瞬の詰まりや瞬間的な head blocker の観測は DMV 側の役割です。

**RCSI はロックをなくす機能だと思う**

誤りです。RCSI は読み取りの実装を行バージョン側へ寄せる仕組みであり、書き込み競合やスキーマロックを消すわけではありません。コストは tempdb や version store へ移ります。[locking-row-versioning][locking-row-versioning] [tempdb-database][tempdb-database]

## 10. 結論

SQL Server の全体像を理解するというのは、部品名を暗記することではありません。アプリケーションの要求が、Query Processor で実行計画になり、Storage Engine でページ操作になり、更新ならログに記録され、同時実行ならロックや行バージョンに守られ、補助処理は tempdb を使い、その結果が DMV や Query Store に現れる、という一本の因果を持つことです。

この因果が見えていると、性能指標は単なるメーターの羅列ではなくなります。どの指標がどの層の痕跡なのかが分かり、RCSI のような設計変更が何を良くし、何を別の場所へ動かすのかを説明できるようになります。SQL Server の性能分析で本当に必要なのは、この地図です。個々の DMV や待機種別は、その地図の上で初めて意味を持ちます。

## 11. 参考情報

- [What is SQL Server?][sqlserver-what-is]
- [The Microsoft SQL Database Engine][sql-database-engine]
- [The Microsoft SQL Database Engine - ACID][sql-database-engine-acid]
- [Query processing architecture guide][query-processing-architecture]
- [Thread and task architecture guide][thread-task-guide]
- [sys.dm_exec_requests][dm-exec-requests]
- [Memory management architecture guide][memory-management-architecture]
- [Read data pages in the Database Engine][read-data-pages]
- [Page and extent architecture guide][pages-extents-guide]
- [SQL Server transaction log architecture and management guide][transaction-log-guide]
- [The transaction log][the-transaction-log]
- [Transaction locking and row versioning guide][locking-row-versioning]
- [tempdb database][tempdb-database]
- [System dynamic management views][system-dmvs]
- [Monitor performance by using the Query Store][monitor-query-store]

[sqlserver-what-is]: https://learn.microsoft.com/sql/sql-server/what-is-sql-server?view=sql-server-ver17
[sql-database-engine]: https://learn.microsoft.com/sql/database-engine/sql-database-engine?view=sql-server-ver17
[sql-database-engine-acid]: https://learn.microsoft.com/sql/database-engine/sql-database-engine?view=sql-server-ver17#database-fundamentals-acid-compliance
[query-processing-architecture]: https://learn.microsoft.com/sql/relational-databases/query-processing-architecture-guide?view=sql-server-ver17
[thread-task-guide]: https://learn.microsoft.com/sql/relational-databases/thread-and-task-architecture-guide?view=sql-server-ver17
[dm-exec-requests]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-exec-requests-transact-sql?view=sql-server-ver17
[memory-management-architecture]: https://learn.microsoft.com/sql/relational-databases/memory-management-architecture-guide?view=sql-server-ver17
[read-data-pages]: https://learn.microsoft.com/sql/relational-databases/reading-pages?view=sql-server-ver17
[pages-extents-guide]: https://learn.microsoft.com/sql/relational-databases/pages-and-extents-architecture-guide?view=sql-server-ver17
[transaction-log-guide]: https://learn.microsoft.com/sql/relational-databases/sql-server-transaction-log-architecture-and-management-guide?view=sql-server-ver17
[the-transaction-log]: https://learn.microsoft.com/sql/relational-databases/logs/the-transaction-log-sql-server?view=sql-server-ver17
[locking-row-versioning]: https://learn.microsoft.com/sql/relational-databases/sql-server-transaction-locking-and-row-versioning-guide?view=sql-server-ver17
[tempdb-database]: https://learn.microsoft.com/sql/relational-databases/databases/tempdb-database?view=sql-server-ver17
[system-dmvs]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/system-dynamic-management-views?view=sql-server-ver17
[monitor-query-store]: https://learn.microsoft.com/sql/relational-databases/performance/monitoring-performance-by-using-the-query-store?view=sql-server-ver17
