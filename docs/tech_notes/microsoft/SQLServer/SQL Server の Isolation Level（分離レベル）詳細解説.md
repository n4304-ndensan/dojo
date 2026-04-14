# SQL Server の Isolation Level（分離レベル）詳細解説

> 位置づけ: この文書は、SQL Server の分離レベル全体を一枚の地図として整理するための解説です。RCSI と SNAPSHOT だけに絞った比較ではなく、`READ UNCOMMITTED` から `SERIALIZABLE` までを含めて、Isolation Level という概念そのものを説明します。
>
> 読み分け: RCSI と SNAPSHOT の比較判断に絞る場合は [[SQL Server Snapshot Isolation と RCSI と Read Committed の違い]]、フォルダ全体の案内は [[SQL Server ドキュメントガイド]] を参照してください。

## 1. 文脈と目的

SQL Server で性能や整合性を考えるとき、Isolation Level は避けて通れません。ところが実務では、`NOLOCK`、`READ COMMITTED`、RCSI、`SNAPSHOT`、`SERIALIZABLE` が断片的に語られることが多く、「どのレベルが何を防いで、何を許し、何をコストとして払うのか」が一本の体系として理解されないままになりがちです。

この文書の目的は、Isolation Level を「同時実行時に、他トランザクションの変更をどこまで見せるかを決めるルール」として整理し、各レベルの意味、行バージョニングとの関係、選び方、誤解しやすい点まで含めて説明することです。特に、SQL Server では lock-based のレベルと row-versioning-based のレベルが混在しているため、そこを明確に切り分けます。[set-isolation][locking-guide]

## 2. 背景と課題

分離レベルが必要になる背景には、同時実行があります。複数のユーザーや処理が同じデータへ同時にアクセスすると、dirty read、nonrepeatable read、phantom read、lost update のような副作用が起こり得ます。Database Engine はこれを、ロックや行バージョンによって制御します。[locking-guide]

ただし、厳密さを上げればそれで終わりではありません。より強い分離は、一般により多くのロックや競合、あるいは別の資源消費を招きます。逆に分離を緩めれば、同時実行性は上がっても、読んだデータの意味が不安定になります。Isolation Level の設計は、正しさと並行性のトレードオフ設計です。[set-isolation][locking-guide]

## 3. 全体像

SQL Server で意識すべき分離レベルは、実質的には次の六つです。

| レベル | 読み取りの基本方式 | 一貫性の単位 | 主な特徴 |
| --- | --- | --- | --- |
| `READ UNCOMMITTED` | ロックをほぼ使わない読取り | ほぼ保証なし | dirty read を許します。 |
| `READ COMMITTED` | 共有ロックによる読取り | 文ごと | SQL Server の既定です。dirty read を防ぎます。 |
| RCSI | 行バージョンによる `READ COMMITTED` | 文ごと | `READ COMMITTED` の実装だけを row versioning に切り替えます。 |
| `REPEATABLE READ` | 読んだ行に共有ロックを保持 | トランザクション内の再読 | nonrepeatable read を防ぎます。 |
| `SNAPSHOT` | 行バージョンによる読取り | トランザクション全体 | 開始時点の一貫した世界を見ます。 |
| `SERIALIZABLE` | 範囲ロックを含む最強のロック制御 | トランザクション全体 | phantom read まで防ぎます。 |

ここで重要なのは、RCSI は独立した Isolation Level 名ではなく、`READ COMMITTED` の実装変更だという点です。SQL の `SET TRANSACTION ISOLATION LEVEL` に出てくるキーワードは `READ UNCOMMITTED`、`READ COMMITTED`、`REPEATABLE READ`、`SNAPSHOT`、`SERIALIZABLE` であり、RCSI は `READ_COMMITTED_SNAPSHOT` データベースオプションで有効にします。[set-isolation][locking-guide]

## 4. 中核概念と用語

### 4.1 dirty read

他トランザクションがまだ commit していない変更を読んでしまうことです。`READ UNCOMMITTED` では許されますが、通常の業務ロジックでは危険です。[set-isolation][locking-guide]

### 4.2 nonrepeatable read

同じトランザクション内で同じ行を二回読むと、他トランザクションの commit によって値が変わって見えることです。`READ COMMITTED` では起こり得ます。[locking-guide]

### 4.3 phantom read

同じ検索条件を二回実行したとき、他トランザクションの insert や delete により、返る行集合そのものが変わることです。`REPEATABLE READ` では防げず、`SERIALIZABLE` では範囲ロックで防ぎます。[locking-guide]

### 4.4 row versioning

更新前の行イメージを保持し、読取りがロックではなくバージョンを見る仕組みです。RCSI と `SNAPSHOT` の土台になります。[locking-guide]

### 4.5 statement-level consistency と transaction-level consistency

RCSI は各 statement の開始時点で commit 済みだった世界を見ます。`SNAPSHOT` は transaction 全体の開始時点で commit 済みだった世界を見ます。この差が、実務上もっとも重要です。[set-isolation][locking-guide]

## 5. 仕組み

### 5.1 `READ UNCOMMITTED`

最も緩い分離レベルです。dirty read を許し、`NOLOCK` を全テーブルに付けたのに近い性質を持ちます。ブロッキング回避の誘惑はありますが、読んだ値や行集合が本当に意味を持つのかを慎重に判断する必要があります。[set-isolation]

### 5.2 `READ COMMITTED`

SQL Server の既定です。dirty read は防ぎますが、同一トランザクション内で二度読んだときに値が変わることや phantom read は起こり得ます。`READ_COMMITTED_SNAPSHOT` が `OFF` の場合、共有ロックでこれを実現します。[set-isolation][locking-guide]

### 5.3 RCSI

RCSI は `READ_COMMITTED_SNAPSHOT` を `ON` にしたときの `READ COMMITTED` です。読取りは shared lock ではなく row versioning を使い、各 statement の開始時点で commit 済みだった世界を見ます。読取りと更新のブロッキングを減らしやすい一方、version store や cleanup の監視が必要になります。[set-isolation][locking-guide]

### 5.4 `REPEATABLE READ`

読んだ行に対する共有ロックをトランザクション終了まで保持するため、同じ行を再読したときに値が変わることは防げます。ただし、範囲ロックまでは行わないため、phantom read は防げません。`READ COMMITTED` より並行性は下がります。[set-isolation][locking-guide]

### 5.5 `SNAPSHOT`

`ALLOW_SNAPSHOT_ISOLATION = ON` を前提に使う、transaction-level consistency の分離レベルです。transaction 開始時点で commit 済みだった世界を見続けます。読取り時に page/row lock を取らず、一貫した読み取りがしやすい一方、更新競合時にはエラーになって retry が必要になることがあります。[set-isolation][locking-guide]

### 5.6 `SERIALIZABLE`

最も強い分離レベルです。読み取った範囲に key-range lock を掛けることで phantom read まで防ぎます。その分、同時実行性は大きく下がります。範囲整合性が本当に必要な場面だけに絞って使う方が安全です。[set-isolation][locking-guide]

## 6. IsolationLevel という言葉の二つの文脈

実務で `IsolationLevel` という語が出てきたとき、二つの文脈が混ざりやすくなります。一つは T-SQL の `SET TRANSACTION ISOLATION LEVEL`、もう一つは ADO.NET などの API にある `IsolationLevel` 列挙です。

公式ガイドでは、SqlClient の `BeginTransaction` などで `IsolationLevel.Unspecified`、`Chaos`、`ReadUncommitted`、`ReadCommitted`、`RepeatableRead`、`Serializable`、`Snapshot` を指定できると説明されています。ただし、設計上の中心になるのは SQL Server 側で意味を持つ `ReadUncommitted`、`ReadCommitted`、`RepeatableRead`、`Serializable`、`Snapshot` です。`Chaos` や `Unspecified` は API 列挙としては存在しますが、通常の SQL Server 設計文書では主要選択肢として扱わない方が分かりやすくなります。[locking-guide]

## 7. 実装上の考慮点

Isolation Level は接続に対して設定され、明示的に変えるまでその接続で有効です。stored procedure や trigger の中で変更した場合、呼び出し元へ制御が戻ると元の isolation level に戻るという点も押さえておくと、挙動を読み違えにくくなります。[set-isolation]

RCSI を有効にするには `ALTER DATABASE ... SET READ_COMMITTED_SNAPSHOT ON` が必要です。この操作中は、対象データベースに他の open connection が存在できません。`SNAPSHOT` を使うには `ALLOW_SNAPSHOT_ISOLATION ON` が必要で、クロスデータベースアクセスでは参照先すべてで有効になっている必要があります。[set-isolation][locking-guide]

また、どの分離レベルでも更新時の exclusive lock が不要になるわけではありません。公式にも、分離レベルの選択は主に read operation 側の保護レベルを決めるのであって、data modification を守る lock 自体は別だと明記されています。この点を外すと「RCSI にしたから書込み競合も消える」といった誤解が起こります。[set-isolation][locking-guide]

## 8. 運用とセキュリティ上の考慮点

低い分離レベルは同時実行性に有利ですが、誤った判断をアプリへ持ち込む危険があります。特に `READ UNCOMMITTED` や `NOLOCK` は、監視やざっくりした探索用ならまだしも、業務判断や集計結果の確定に使うのは危険です。[set-isolation][locking-guide]

一方、強い分離レベルはブロッキングや retry コストを増やします。`SERIALIZABLE` は正しさが最優先の範囲整合性には有効ですが、通常の OLTP 全体へ広げると競合が増えやすくなります。RCSI や `SNAPSHOT` も万能ではなく、versioning の保持コストや更新競合の扱いを別途設計する必要があります。[set-isolation][locking-guide]

## 9. 使うべき場面と、使うべきでない場面

通常の OLTP では、まず既定の `READ COMMITTED` を起点に考えるのが基本です。読取りと書込みのブロッキングが問題なら RCSI を検討し、複数文にまたがって同じ時点の世界を見続けたいなら `SNAPSHOT` を検討します。再読一貫性だけが必要なら `REPEATABLE READ`、範囲整合性まで必要なら `SERIALIZABLE` が候補です。[set-isolation][locking-guide]

逆に、常に最も強いレベルを選べばよいわけではありません。厳密さの向上は、通常はロック、競合、retry、資源消費の増加と引き換えです。Isolation Level は「正しさの強さランキング」ではなく、「どの副作用をどのコストで防ぐかの設計選択」だと考える方が実務に合います。[locking-guide]

## 10. よくある誤解や失敗パターン

### 10.1 RCSI は独立した Isolation Level だと思う

誤りです。RCSI は `READ COMMITTED` の実装変更であり、独立した `SET TRANSACTION ISOLATION LEVEL RCSI` という文法はありません。[set-isolation][locking-guide]

### 10.2 分離レベルを下げれば更新ロックも減ると思う

誤りです。公式にも、データ変更を守る exclusive lock は分離レベルとは別だと明記されています。主に変わるのは read operation 側の保護方法です。[set-isolation]

### 10.3 `SNAPSHOT` は session で指定するだけで使えると思う

誤りです。`ALLOW_SNAPSHOT_ISOLATION` を DB 側で `ON` にする必要があります。クロスデータベースなら参照先すべてで必要です。[set-isolation][locking-guide]

### 10.4 `NOLOCK` はただ速いだけで安全だと思う

危険です。`READ UNCOMMITTED` 相当なので、dirty read や欠落行、重複読取りなどの副作用を受け得ます。[set-isolation][locking-guide]

### 10.5 高い分離レベルほど常に優れていると思う

誤りです。高い分離は整合性を強めますが、同時実行性を下げます。要件に見合わない強さは、性能問題の原因になりえます。[locking-guide]

## 11. 結論

SQL Server の Isolation Level は、「他トランザクションの変更をどのように見せるか」を決める設計パラメーターです。`READ UNCOMMITTED` から `SERIALIZABLE` までの lock-based な世界に加えて、RCSI と `SNAPSHOT` という row-versioning-based な選択肢があります。

重要なのは、Isolation Level を単なる暗記項目ではなく、dirty read、再読一貫性、phantom read、更新競合、version store コスト、ブロッキングといった具体的な結果へ結びつけて理解することです。そうすると、RCSI を入れる理由、`SNAPSHOT` を局所適用する理由、`SERIALIZABLE` を限定的に使う理由が、すべて同じ地図の上で説明できるようになります。

## 参考

- [SET TRANSACTION ISOLATION LEVEL (Transact-SQL)][set-isolation]
- [Transaction locking and row versioning guide][locking-guide]
- [SQL Server Snapshot Isolation と RCSI と Read Committed の違い][snapshot-note]

[set-isolation]: https://learn.microsoft.com/sql/t-sql/statements/set-transaction-isolation-level-transact-sql?view=sql-server-ver17
[locking-guide]: https://learn.microsoft.com/sql/relational-databases/sql-server-transaction-locking-and-row-versioning-guide?view=sql-server-ver17
[snapshot-note]: ./SQL%20Server%20Snapshot%20Isolation%20と%20RCSI%20と%20Read%20Committed%20の違い.md
