# SQL Server の User、Schema、tempdb.sys の名前解決

> 位置づけ: この文書は、`master.sys.server_resource_stats` や `tempdb.sys.dm_db_file_space_usage` のような名前を読むための概念整理です。SQL Server の名前空間、スキーマ、データベースユーザーの関係を一度分解して説明します。
>
> 読み分け: `sys.server_resource_stats` 自体の解説は [[ManagedInstance/sys.server_resource_stats 詳細解説]]、Isolation Level の全体像は [[SQL Server の Isolation Level（分離レベル）詳細解説]]、フォルダ全体の案内は [[SQL Server ドキュメントガイド]] を参照してください。

## 1. 文脈と目的

SQL Server のクエリを読み始めたとき、多くの人が最初につまずくのが名前の読み方です。`tempdb.sys.dm_db_file_space_usage`、`master.sys.server_resource_stats`、`dbo.Customers`、`sys.objects` のような名前が並ぶと、「`sys` は namespace なのか」「`user` は schema のことなのか」「なぜ `tempdb.sys` と書くときと書かないときがあるのか」が曖昧になりやすくなります。

この文書の目的は、SQL Server における名前解決の基本単位を揃えることです。結論から言うと、SQL Server の現代的な整理では、オブジェクトは基本的に `server.database.schema.object` という多部名で解決され、schema が名前空間の役割を持ちます。`user` は schema そのものではなく、データベースプリンシパルです。`tempdb.sys` や `master.sys` は、データベース名とスキーマ名を明示しているだけです。[tsql-syntax][user-schema-separation][sys-schemas]

## 2. 背景と課題

この話がややこしいのは、SQL Server が長い歴史の中で「user が所有者のように見えた時代」と「schema と user を分離した現在」の両方の文脈を持っているからです。SQL Server 2005 以降は user と schema が分離され、schema は独立したコンテナー、すなわち namespace として扱うのが基本になりました。それでも、古い資料や慣用表現では user と schema が混ざって語られることがあります。[user-schema-separation][create-schema]

さらに、名前解決では現在のデータベースコンテキストと既定スキーマが影響します。省略した部分は、SQL Server が自動的に補って解決します。便利ではありますが、そのぶん誤解や事故も起きやすくなります。特に、監視用クエリや運用 SQL では、どのデータベース、どのスキーマのオブジェクトを参照しているかを意識しないと、意図しないオブジェクトに解決される危険があります。[tsql-syntax][user-schema-separation]

## 3. 全体像

SQL Server のオブジェクト名は、原則として次の多部名で表せます。

`server_name.database_name.schema_name.object_name`

ただし、常に四部すべてを書く必要はありません。現在の接続先、現在のデータベース、既定スキーマが分かっている場合は、一部を省略できます。公式には、四部名、三部名、二部名、一部名のいずれも有効な場合があります。[tsql-syntax]

この構造を具体例へ落とすと、次のように読めます。

| 表記 | 読み方 |
| --- | --- |
| `tempdb.sys.dm_db_file_space_usage` | `tempdb` データベースの `sys` スキーマにある `dm_db_file_space_usage` |
| `master.sys.server_resource_stats` | `master` データベースの `sys` スキーマにある `server_resource_stats` |
| `dbo.Customers` | 現在のデータベースの `dbo` スキーマにある `Customers` |
| `sys.objects` | 現在のデータベースの `sys` スキーマにある `objects` |
| `Orders` | 現在のデータベースの既定スキーマ、なければ `dbo` を候補に解決する一部名 |

このうち namespace の役割を担うのは schema です。database はその上位のコンテナーであり、user はセキュリティ主体です。同じものではありません。[sys-schemas][principals][user-schema-separation]

## 4. 中核概念と用語

### 4.1 Server、Database、Schema、Object

Transact-SQL の公式構文では、データベースオブジェクト名は四部名まで持てます。`server` はリンクサーバーやリモートサーバー、`database` はデータベース、`schema` はそのデータベース内の namespace、`object` はテーブルやビューや関数などの実体です。[tsql-syntax]

### 4.2 Schema

`sys.schemas` の説明では、database schema は tables、views、procedures、functions などを収める namespace または container です。各 schema は owner を持ち、その owner は database principal です。つまり、schema はセキュリティ主体ではなく、名前空間兼コンテナーです。[sys-schemas]

### 4.3 User

SQL Server でいう user は、データベースレベルの principal です。`dbo` user のような特別な user もありますが、schema とは別物です。ユーザーは default schema を持てますが、それは「スキーマと同一」という意味ではなく、「オブジェクト名を省略したとき最初に探す schema」を持つ、という意味です。[principals][user-schema-separation][alter-user]

### 4.4 `dbo`

`dbo` は特別な user でもあり、特別な schema でもあります。`dbo` schema はすべてのデータベースに存在し、多くのユーザーにとって既定スキーマとして使われます。ただし、`dbo` schema を既定スキーマに持つことと、`dbo` user の権限を持つことは同じではありません。[principals][user-schema-separation]

### 4.5 `sys` と `INFORMATION_SCHEMA`

`sys` と `INFORMATION_SCHEMA` はシステムオブジェクトのために予約された schema です。`sys.system_views` の説明では、システムビューは `sys` か `INFORMATION_SCHEMA` schema に入っています。これらの schema にユーザーオブジェクトを作ることはできません。[sys-system-views][user-schema-separation]

## 5. 仕組み

### 5.1 `tempdb.sys` は何を意味しているか

`tempdb.sys.dm_db_file_space_usage` という表記を分解すると、`tempdb` は database name、`sys` は schema name、`dm_db_file_space_usage` は object name です。特別な構文ではありません。SQL Server の多部名のうち、database と schema を明示しているだけです。[tsql-syntax]

この書き方の利点は、どのデータベースのオブジェクトを見たいかを明示できることです。RCSI 評価で tempdb の空き領域や version store を見たいなら、`tempdb.sys` と書くことで、現在の接続先 DB に依存せず tempdb 側の system view を読む意図が明確になります。

### 5.2 `master.sys` は何を意味しているか

`master.sys.server_resource_stats` も同じです。`master` という database name を明示し、その中の `sys` schema にある object を読むという意味です。実務で三部名がよく使われるのは、「現在の DB コンテキストに依存したくないから」です。監視クエリや運用手順では、どこを見ているかを表記に残しておく方が安全です。

### 5.3 schema は namespace、user は principal

`Ownership and user-schema separation` の公式説明では、schema は named container であり、database object permissions の管理を柔軟にするための namespace です。user は principal であり、schema の owner になれますが、schema そのものではありません。複数の user が同じ schema を既定にすることもできますし、user を削除しても schema は残せます。[user-schema-separation][sys-schemas]

### 5.4 一部名の名前解決

ユーザーが schema を省略して一部名でオブジェクトを参照した場合、SQL Server はまずその user の default schema を探し、見つからなければ `dbo` schema を探します。ここが、一部名を多用すると事故が起きやすい理由です。同名オブジェクトが複数 schema にあると、想定と違うものへ解決される可能性があります。[user-schema-separation]

### 5.5 metadata 関数と current database context

`OBJECT_SCHEMA_NAME` のような関数では、database_id を省略すると current database context を前提に解決します。公式の例でも、`master.sys.objects` を FROM しているのに `OBJECT_SCHEMA_NAME(object_id)` とだけ書くと誤った結果になる可能性があると説明されています。クロスデータベースでメタデータを見るときは、関数側にも database_id を渡す方が安全です。[object-schema-name]

## 6. アーキテクチャと設計上の含意

この整理を持っていると、監視クエリ、移行スクリプト、権限設計の読みやすさがかなり変わります。まず、運用 SQL では schema 名を省略しない方が安全です。公式にも、schema-scoped object を参照するときは schema を明示することが推奨されています。[tsql-syntax]

次に、権限設計では「user に権限をばらまく」のではなく、「schema を単位に権限を付ける」という設計がしやすくなります。schema は namespace であり、container でもあるため、同じ責務のオブジェクト群をまとめやすいからです。[user-schema-separation]

さらに、システムオブジェクトの参照でも、`sys` を単なる慣習的接頭辞だと誤解しない方がよいです。`sys` は schema であり、`sys.objects` や `sys.system_views` はその schema に配置された system catalog / system view です。[sys-system-views]

## 7. 実装上の考慮点

実務で最低限覚えておくとよいのは、schema と user の差をカタログビューで確認する方法です。schema の一覧は `sys.schemas`、database user の一覧と default schema は `sys.database_principals` で確認できます。[sys-schemas][sys-database-principals]

```sql
SELECT name, schema_id, principal_id
FROM sys.schemas
ORDER BY name;
```

```sql
SELECT name, type_desc, default_schema_name
FROM sys.database_principals
WHERE type IN ('S', 'U', 'G', 'A', 'E', 'X')
ORDER BY name;
```

この二つを分けて見れば、schema が namespace、user が principal だという構造がかなり直感的になります。

また、オブジェクト作成時に schema を明示しないと、場合によっては implicit に user と schema が作られる挙動もあります。公式には、これを避けるために database principal と default schema を明示的に作るか、既存 schema を明示してオブジェクトを作ることが推奨されています。[create-schema]

## 8. 運用とセキュリティ上の考慮点

schema は owner を持つため、権限設計と所有権設計を切り分けて考える必要があります。schema owner は principal であり、object owner とも関係します。特に `dbo` は特別な principal なので、「`dbo` schema を既定にしている user」と「`dbo` user として入っている管理者」を同一視しない方が安全です。[principals][user-schema-separation]

また、`sys` と `INFORMATION_SCHEMA` は予約済み schema です。ユーザーオブジェクトをここへ置けないため、アプリケーション用の object は `dbo` か独自 schema へ分ける設計が基本になります。[user-schema-separation][sys-system-views]

## 9. 使うべき場面と、使うべきでない場面

この整理が特に役立つのは、監視 SQL を読むとき、クロスデータベース参照を書くとき、default schema を使うアプリの権限設計をするときです。`tempdb.sys` や `master.sys` を見て戸惑わなくなり、一部名がどこへ解決されるかも説明できるようになります。

一方で、アプリケーション SQL で一部名を多用し続ける設計は避けた方がよいです。名前解決の暗黙性に依存するほど、将来の schema 追加や移行時の事故が増えやすくなるからです。運用 SQL と同様に、業務 SQL でも schema を明示する方が持続可能です。[tsql-syntax][user-schema-separation]

## 10. よくある誤解や失敗パターン

### 10.1 user と schema は同じだと思う

誤りです。schema は namespace、user は principal です。user は default schema を持てますが、schema と同一ではありません。[user-schema-separation][sys-schemas]

### 10.2 `tempdb.sys` は特別な keyword だと思う

誤りです。単に `database.schema.object` の三部名です。`master.sys` も同じ構造です。[tsql-syntax]

### 10.3 `sys` はユーザーオブジェクトも置ける普通の schema だと思う

誤りです。`sys` は system object のために予約された schema です。ユーザーオブジェクトを作る場所ではありません。[user-schema-separation][sys-system-views]

### 10.4 schema を省略しても常に安全だと思う

危険です。一部名は default schema と `dbo` に依存して解決されます。意図しない object へ解決される可能性があります。[user-schema-separation]

### 10.5 `dbo` schema を既定に持つ user は `dbo` 権限を継承すると思う

誤りです。公式には、`dbo` schema を default schema に持つ user が `dbo` user の権限を継承するわけではないと明記されています。[user-schema-separation]

## 11. 結論

SQL Server の名前解決を理解する鍵は、schema を namespace として捉え、user を principal として分けて考えることです。`tempdb.sys.dm_db_file_space_usage` や `master.sys.server_resource_stats` は、ただ database と schema を明示しているだけです。特別な別言語ではありません。

この構造が見えるようになると、SQL の読み方だけでなく、権限設計、オブジェクト配置、監視クエリの安全性まで一貫して説明できるようになります。SQL Server で「どこにある何を読んでいるか」を曖昧にしないための基礎として、この整理はかなり重要です。

## 参考

- [Transact-SQL syntax conventions: Multipart names][tsql-syntax]
- [Ownership and user-schema separation in SQL Server][user-schema-separation]
- [Schema catalog view - sys.schemas][sys-schemas]
- [sys.system_views][sys-system-views]
- [Principals (Database Engine)][principals]
- [ALTER USER (Transact-SQL)][alter-user]
- [CREATE SCHEMA (Transact-SQL)][create-schema]
- [sys.database_principals][sys-database-principals]
- [OBJECT_SCHEMA_NAME (Transact-SQL)][object-schema-name]

[tsql-syntax]: https://learn.microsoft.com/sql/t-sql/language-elements/transact-sql-syntax-conventions-transact-sql?view=sql-server-ver17#multipart-names
[user-schema-separation]: https://learn.microsoft.com/sql/relational-databases/security/authentication-access/ownership-and-user-schema-separation?view=sql-server-ver17
[sys-schemas]: https://learn.microsoft.com/sql/relational-databases/system-catalog-views/schemas-catalog-views-sys-schemas?view=sql-server-ver17
[sys-system-views]: https://learn.microsoft.com/sql/relational-databases/system-catalog-views/sys-system-views-transact-sql?view=sql-server-ver17
[principals]: https://learn.microsoft.com/sql/relational-databases/security/authentication-access/principals-database-engine?view=sql-server-ver17
[alter-user]: https://learn.microsoft.com/sql/t-sql/statements/alter-user-transact-sql?view=sql-server-ver17
[create-schema]: https://learn.microsoft.com/sql/t-sql/statements/create-schema-transact-sql?view=sql-server-ver17#remarks
[sys-database-principals]: https://learn.microsoft.com/sql/relational-databases/system-catalog-views/sys-database-principals-transact-sql?view=sql-server-ver17
[object-schema-name]: https://learn.microsoft.com/sql/t-sql/functions/object-schema-name-transact-sql?view=sql-server-ver17#remarks
