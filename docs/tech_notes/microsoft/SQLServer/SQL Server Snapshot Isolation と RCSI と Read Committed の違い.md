# SQL Server Snapshot Isolation と RCSI と Read Committed の違い

> 位置づけ: この文書は、RCSI と SNAPSHOT と従来の READ COMMITTED を選び分けるための比較ノートです。設計判断や既存システムへの影響範囲を整理したいときに使います。
>
> 読み分け: 基礎理解と評価指標の全体像は [[SQL Server 入門から始める RCSI 性能評価研修]]、Azure SQL Managed Instance 固有の前提整理は [[ManagedInstance/Azure SQL Managed Instance における RCSI 評価の前提整理]]、フォルダ全体の案内は [[SQL Server ドキュメントガイド]] を参照してください。

## 1. 文脈と目的

SQL Server の分離レベルを調べていると、`SNAPSHOT` と `READ_COMMITTED_SNAPSHOT` がどちらも「行バージョンを使って読む」仕組みに見えるため、両者の違いが曖昧になりやすいです。さらにややこしいのは、RCSI は独立した分離レベルではなく、`READ COMMITTED` の読み取り実装をロック方式から行バージョン方式へ切り替えるデータベースオプションだという点です。[SET TRANSACTION ISOLATION LEVEL][set-transaction-isolation] [Transaction locking and row versioning guide][locking-row-versioning]

このノートの目的は、従来の `READ COMMITTED`、RCSI を有効化した `READ COMMITTED`、そして `SNAPSHOT` を同じ土俵で比較し、何が違うのかを設計判断の視点で整理することです。違いだけでなく、メリットとデメリット、導入時の注意点、どの場面で選ぶべきかまで含めて判断できる形にまとめます。

> 注記: オンプレミスの SQL Server では通常 `READ_COMMITTED_SNAPSHOT` は既定で `OFF` ですが、Azure SQL Database では `ON` が既定です。[SET TRANSACTION ISOLATION LEVEL][set-transaction-isolation]
> 関連: [[SQL Server 入門から始める RCSI 性能評価研修]]

## 2. まず結論

最初に押さえるべきポイントは三つです。第一に、RCSI は新しい分離レベルではなく、`READ COMMITTED` の読み取り方法を変える仕組みです。第二に、RCSI は文単位のスナップショットを返し、`SNAPSHOT` はトランザクション単位のスナップショットを返します。第三に、RCSI はデータベース内の `READ COMMITTED` 利用全体へ波及しますが、`SNAPSHOT` は明示的に指定したセッションだけに効きます。[SET TRANSACTION ISOLATION LEVEL][set-transaction-isolation] [Snapshot isolation in SQL Server][snapshot-isolation] [Transaction locking and row versioning guide][locking-row-versioning]

この違いを一言で言い換えると、RCSI は「既定の読取ブロッキングを減らすための広域スイッチ」、`SNAPSHOT` は「必要なトランザクションだけ、開始時点の一貫した見え方へ固定するための明示指定」です。

| 観点 | READ COMMITTED（RCSI OFF） | READ COMMITTED（RCSI ON） | SNAPSHOT |
| --- | --- | --- | --- |
| 位置づけ | SQL Server の既定分離レベル | `READ COMMITTED` の実装変更 | 明示指定する独立した分離レベル |
| 読み取りが見る時点 | 各行を読む瞬間にコミット済みの値 | 各文の開始時点でコミット済みの値 | トランザクション開始時点でコミット済みの値 |
| 読み取り時の共有ロック | 使う | 使わない | 使わない |
| 読み取りと更新の相互ブロック | 起こりやすい | 大きく減る | 大きく減る |
| 同一トランザクション内の複数 `SELECT` の一貫性 | 弱い | 文ごとに変わり得る | 強い |
| 更新競合の基本挙動 | ロック待ち | ロック待ち | 競合時にコミット失敗し得る |
| 影響範囲 | その接続の `READ COMMITTED` | DB 内の `READ COMMITTED` 全体 | `SNAPSHOT` を指定した接続だけ |
| 主なコスト | ブロッキング、デッドロック | version store の領域と監視 | version store に加え、競合時のリトライ設計 |
| 向く場面 | 従来挙動を維持したい | OLTP の読取ブロッキングを広く減らしたい | 連携処理、レポート、複数文で同じ時点を見たい処理 |

## 3. 三つの方式を順に整理する

比較表だけでは設計判断に必要な手触りが足りません。この章では、`READ COMMITTED`、RCSI、`SNAPSHOT` を順に見て、何が保たれ、どこが変わり、何を代償として払うのかを整理します。

### 3.1 READ COMMITTED（RCSI OFF）

従来の `READ COMMITTED` は、ダーティリードを防ぐ代わりに、読み取り時に共有ロックを使って整合性を守ります。そのため、更新中の行を読むと待たされることがあり、逆に読取側が更新側を待たせることもあります。重要なのは、これは「文全体の時点を固定する」方式ではなく、読める瞬間にコミット済みであることを保証する方式だということです。したがって、同じトランザクション内で二度読んだときに結果が変わることは普通に起こります。[SET TRANSACTION ISOLATION LEVEL][set-transaction-isolation]

この方式のメリットは、挙動が古典的で分かりやすく、既存アプリケーションが前提としてきたロックベースのふるまいをそのまま維持しやすいことです。追加の version store 領域も不要であり、tempdb や PVS の監視負荷も増えません。一方で、読取と更新がぶつかる OLTP ワークロードでは、ロック待機やデッドロックが性能問題の入り口になりやすいです。[Transaction locking and row versioning guide][locking-row-versioning]

`READ COMMITTED` を維持する利点は次の通りです。

- 既存アプリケーションのロック前提を変えずに済む。
- version store のための追加領域や監視を原則として気にしなくてよい。
- 読み取り時の挙動が「いまコミット済みのものだけを見る」という意味で直感的である。

一方で、弱みは次の通りです。

- 読取と更新が相互に待ちやすく、混雑時のスループットが落ちやすい。
- 同一トランザクション内で複数回読んだときに、結果が変わり得る。
- 大きな参照系クエリや連携処理が、更新系ワークロードへ悪影響を与えやすい。

### 3.2 READ COMMITTED（RCSI ON）

RCSI は `ALTER DATABASE ... SET READ_COMMITTED_SNAPSHOT ON` で有効化する、`READ COMMITTED` の実装切り替えです。ここで重要なのは、RCSI を使うために `ALLOW_SNAPSHOT_ISOLATION ON` は必須ではない、という点です。RCSI を有効化したデータベースでは、`READ COMMITTED` で動く読み取りが、共有ロックではなく行バージョンを使って「各文の開始時点」でコミット済みだったデータを見るようになります。これにより、読取と更新のブロッキングは大きく減ります。[SET TRANSACTION ISOLATION LEVEL][set-transaction-isolation] [Transaction locking and row versioning guide][locking-row-versioning]

ただし、RCSI は文単位でしか時点を固定しません。つまり、一つ目の `SELECT` と二つ目の `SELECT` の間に他トランザクションがコミットすれば、同じトランザクション内でも二回目の `SELECT` は新しいコミット済みデータを見る可能性があります。この点が、後述する `SNAPSHOT` との本質的な違いです。また、読み取りに共有ロックを使わなくなるだけで、更新処理が排他ロックを必要とする事実は変わりません。[SET TRANSACTION ISOLATION LEVEL][set-transaction-isolation]

RCSI のメリットは次の通りです。

- `READ COMMITTED` のまま、読取と更新のブロッキングを広く減らせる。
- 既存 SQL を大きく書き換えずに効果を出しやすい。
- 読み取り由来のロック待機やデッドロックを減らしやすい。

RCSI のデメリットは次の通りです。

- データベース内の `READ COMMITTED` 全体へ影響するため、既存アプリへの波及範囲が広い。
- 行バージョン保持のために version store の領域と監視が必要になる。
- 文ごとにスナップショットが更新されるため、トランザクション全体で同じ世界を見続けることはできない。

なお、RCSI を有効化していても、個別文では `READCOMMITTEDLOCK` ヒントを使って共有ロック方式へ戻すことができます。つまり、RCSI は全体の既定を行バージョンへ寄せつつ、必要な箇所だけロック方式へ戻す設計も可能です。[SET TRANSACTION ISOLATION LEVEL][set-transaction-isolation]

### 3.3 SNAPSHOT

`SNAPSHOT` は独立した分離レベルです。使うには、まずデータベースで `ALLOW_SNAPSHOT_ISOLATION ON` を有効にし、そのうえで対象セッションが `SET TRANSACTION ISOLATION LEVEL SNAPSHOT` を実行する必要があります。この分離レベルでは、トランザクション内のすべての読み取りが、トランザクション開始時点のコミット済みデータを見ます。したがって、同じトランザクションで二度読んでも、他トランザクションのコミットを途中で取り込まず、より安定した読み取りができます。[SET TRANSACTION ISOLATION LEVEL][set-transaction-isolation] [Snapshot isolation in SQL Server][snapshot-isolation]

もう一つ重要なのは、`SNAPSHOT` が楽観的同時実行制御に寄ることです。読み取り時にはロックを取りませんが、トランザクション開始後に他のトランザクションが同じ行を更新していた場合、その行へ自分が更新をコミットしようとすると競合としてロールバックされることがあります。つまり、RCSI の更新競合が「待つ」方向に寄りやすいのに対し、`SNAPSHOT` の更新競合は「失敗してやり直す」方向へ寄りやすい、という違いがあります。[Snapshot isolation in SQL Server][snapshot-isolation] [SET TRANSACTION ISOLATION LEVEL][set-transaction-isolation]

`SNAPSHOT` のメリットは次の通りです。

- トランザクション全体で同じ時点のデータを見られるため、複数文にまたがる参照処理と相性がよい。
- 読取と更新の相互ブロックを大きく減らせる。
- セッション単位の明示指定なので、影響範囲を局所化しやすい。

`SNAPSHOT` のデメリットは次の通りです。

- 競合時にコミット失敗が起こり得るため、アプリ側でリトライや例外処理が必要になる。
- 長時間トランザクションが version store の掃除を遅らせやすい。
- 複数データベースをまたぐ場合は、アクセス先すべてで `ALLOW_SNAPSHOT_ISOLATION` が有効であることを確認する必要がある。[Transaction locking and row versioning guide][locking-row-versioning]

## 4. Read Committed と比べて何が変わるのか

`READ COMMITTED` と比較したときの差分を短く言うと、RCSI は「読取の待ち方」を変え、`SNAPSHOT` は「読取の見え方そのもの」を変えます。従来の `READ COMMITTED` は、他トランザクションの未コミット変更を見ない代わりに、必要なら待ちます。RCSI は、その待ちを行バージョン参照へ置き換えることで、読取ブロッキングを減らします。`SNAPSHOT` はさらに踏み込み、トランザクション全体の読み取り時点を固定します。[SET TRANSACTION ISOLATION LEVEL][set-transaction-isolation] [Snapshot isolation in SQL Server][snapshot-isolation]

設計上の違いとしては、次の三点が特に重要です。

1. 読み取り一貫性の強さは、`READ COMMITTED` < RCSI < `SNAPSHOT` の順で高くなるわけではなく、RCSI と `SNAPSHOT` は性質が違います。RCSI は文単位、`SNAPSHOT` はトランザクション単位です。
2. 導入影響の広さは、`READ COMMITTED` を維持 < `SNAPSHOT` を個別採用 < RCSI を全体採用、の順で大きくなりやすいです。
3. 競合時の失敗モードは、従来 `READ COMMITTED` と RCSI が「待機」寄り、`SNAPSHOT` が「競合エラーとリトライ」寄りです。

## 5. どう選ぶべきか

選び分けは、整合性の強さだけでなく、既存アプリへの波及範囲と運用コストまで含めて考えるべきです。特に「ブロッキングを減らしたい」という同じ目的でも、全体最適なら RCSI、局所最適なら `SNAPSHOT` になりやすいです。

判断の目安は次の通りです。

1. 既存の `READ COMMITTED` ベースのアプリを大きく書き換えず、読取ブロッキングを全体的に減らしたいなら、まず RCSI を検討します。
2. 一部のバッチ、連携、レポート、ETL のように「このトランザクションだけ同じ時点を見たい」要件があるなら、`SNAPSHOT` を個別適用する方が安全です。
3. ロックベースのふるまい自体が既存業務ロジックの前提になっている、または version store の追加コストを避けたいなら、従来の `READ COMMITTED` を維持する判断もあり得ます。

ユーザーが示した ZOZO の記事では、既存アプリへの影響を局所化したいことと、課題が読み取り主体の連携ワークロードに限定されていたことから、RCSI ではなく `SNAPSHOT` を選んでいます。この判断は、影響範囲を絞りたいケースではかなり筋がよいです。[ZOZO TECH BLOG][zozo-blog]

## 6. 導入と運用の注意

行バージョン系の方式は、ロック待機を減らす代わりに、コストを別の場所へ移します。導入判断では「待たなくなる」ことだけでなく、「何を監視すべきか」「どこが新しい故障点になるか」をあわせて見なければなりません。

まず RCSI の有効化では、`ALTER DATABASE ... SET READ_COMMITTED_SNAPSHOT ON` の実行中、そのデータベースには実行接続以外のオープン接続が存在できません。この制約は、導入手順を考えるうえでかなり重要です。[SET TRANSACTION ISOLATION LEVEL][set-transaction-isolation]

次に version store の置き場所です。古い説明では tempdb に蓄積されると理解されがちですが、現在の SQL Server では ADR が有効なら version store はユーザーデータベース側の PVS に作られ、ADR が無効なら tempdb に作られます。したがって、監視対象は環境に応じて tempdb か PVS かを見分ける必要があります。[Transaction locking and row versioning guide][locking-row-versioning]

さらに、version store は長時間トランザクションが残ると掃除が遅れます。行バージョンが必要な最古のトランザクションが終わらない限り、古いバージョンを消せないためです。RCSI でも `SNAPSHOT` でも、長時間開きっぱなしのトランザクションは運用上の危険信号になります。[Transaction locking and row versioning guide][locking-row-versioning]

実務で監視したい項目は次の通りです。

- version store のサイズと生成速度、クリーンアップ速度
- tempdb もしくは PVS の空き容量
- 長時間トランザクションの有無
- ロック待機が減った代わりに、別の I/O 待機やログ肥大が起きていないか

実例として、ZOZO の記事では `ALLOW_SNAPSHOT_ISOLATION` 有効化後に、一部の DB でトランザクションログのバックアップサイズが急増した事例が紹介されています。特に「実質的には値が変わらない大量 `UPDATE`」のような処理は、行バージョン管理の副作用がログ量として表面化しやすく、tempdb だけ見ていれば十分とは言えません。これは全環境で必ず起こるわけではありませんが、大量更新バッチがある DB では事前に確認する価値があります。[ZOZO TECH BLOG][zozo-blog]

## 7. 状態確認に使うクエリ

導入後にまず確認したいのは、RCSI と `SNAPSHOT` の状態です。`sys.databases` を見れば、そのデータベースで RCSI が有効か、`SNAPSHOT` が許可状態かを確認できます。[sys.databases][sys-databases]

```sql
SELECT
    name,
    is_read_committed_snapshot_on,
    snapshot_isolation_state_desc
FROM sys.databases
WHERE name = N'YourDatabase';
```

読み方は単純です。`is_read_committed_snapshot_on = 1` なら、そのデータベースの `READ COMMITTED` は行バージョン方式です。`snapshot_isolation_state_desc = ON` なら、そのデータベースで `SET TRANSACTION ISOLATION LEVEL SNAPSHOT` を使える状態です。[sys.databases][sys-databases]

## 8. まとめ

RCSI と `SNAPSHOT` は、どちらも「読取で待たない」ための技術に見えますが、同じものではありません。RCSI は `READ COMMITTED` を文単位スナップショットへ置き換える広域スイッチであり、`SNAPSHOT` は必要なトランザクションだけを開始時点の一貫した世界へ固定する分離レベルです。設計判断では、整合性の粒度、影響範囲、競合時の失敗モード、version store の運用コストをセットで見るべきです。

迷ったときは、まず「全体の既定を変えたいのか」「一部の処理だけ明示的に強い読取一貫性を持たせたいのか」を切り分けると判断しやすくなります。前者なら RCSI、後者なら `SNAPSHOT` が基本線になります。

## 参考

- [SET TRANSACTION ISOLATION LEVEL][set-transaction-isolation]
- [Transaction locking and row versioning guide][locking-row-versioning]
- [Snapshot isolation in SQL Server][snapshot-isolation]
- [sys.databases][sys-databases]
- [ZOZO TECH BLOG][zozo-blog]

[set-transaction-isolation]: https://learn.microsoft.com/sql/t-sql/statements/set-transaction-isolation-level-transact-sql?view=sql-server-ver17
[locking-row-versioning]: https://learn.microsoft.com/sql/relational-databases/sql-server-transaction-locking-and-row-versioning-guide?view=sql-server-ver17
[snapshot-isolation]: https://learn.microsoft.com/sql/connect/ado-net/sql/snapshot-isolation-sql-server?view=sql-server-ver17
[sys-databases]: https://learn.microsoft.com/sql/relational-databases/system-catalog-views/sys-databases-transact-sql?view=sql-server-ver17
[zozo-blog]: https://techblog.zozo.com/entry/sqlserver-transaction-isolation-level-snapshot
