# sys.dm_db_file_space_usage

この DMV は、データベース内の各データ ファイルごとの領域使用量を返します。RCSI や Snapshot Isolation の副作用として tempdb の version store を見たいとき、またはユーザー オブジェクト・内部オブジェクト・未割り当て領域の比率を確認したいときに便利です。[official]

とくに tempdb 監視では、この DMV が中心になります。セッション単位やタスク単位では見えない version store の消費を、ファイル単位で安定して追えるためです。

## 1. 主な使いどころ

この DMV は、次のような問いに向いています。

- tempdb の空き領域はどれくらいか。
- version store がどれだけ膨らんでいるか。
- 内部オブジェクトやユーザー オブジェクトがどれだけ領域を使っているか。
- 差分バックアップ対象となる変更ページがどれくらい増えたか。

## 2. 列の整理

ページ数の列は多いですが、見るべき意味は比較的整理しやすいです。

| 列 | 意味 | 実務での見方 |
| --- | --- | --- |
| `database_id` | データベース ID | 対象 DB の識別に使います。 |
| `file_id` | ファイル ID | `sys.database_files` や `sys.master_files` と結合します。 |
| `filegroup_id` | ファイル グループ ID | どのファイルグループかを区別するときに使います。 |
| `total_page_count` | ファイル内の総ページ数 | ファイルの総容量です。 |
| `allocated_extent_page_count` | 割り当て済みエクステント内の総ページ数 | 使われている側の大枠を見る列です。 |
| `unallocated_extent_page_count` | 未割り当てエクステント内の総ページ数 | まだ使える空き領域の基本指標です。 |
| `version_store_reserved_page_count` | version store 用に確保されたページ数 | RCSI / Snapshot の副作用を見る主列です。 |
| `user_object_reserved_page_count` | ユーザー オブジェクト用に確保されたページ数 | 一時テーブル、テーブル変数、ユーザー作成オブジェクトの使用量を見ます。 |
| `internal_object_reserved_page_count` | 内部オブジェクト用に確保されたページ数 | ソート、ハッシュ、スプール、作業テーブルの消費を見ます。 |
| `mixed_extent_page_count` | 混合エクステント内の総ページ数 | 頻繁な変動は SGAM 競合や `PAGELATCH_UP` の一因を示すことがあります。 |
| `modified_extent_page_count` | 前回フル バックアップ以降に変更されたページ数 | 差分バックアップの必要性判断の補助に使います。 |

## 3. 読み方のコツ

tempdb 監視では、`version_store_reserved_page_count`、`internal_object_reserved_page_count`、`unallocated_extent_page_count` の三つをまず見ます。version store が増えているのか、ソートやハッシュのような内部作業が膨らんでいるのか、単純に空きが尽きかけているのかをここで切り分けられます。[official]

差分バックアップの観点では、`modified_extent_page_count` が重要です。前回フルバックアップ以降にどれだけ変更ページが溜まったかを見ることで、差分バックアップがどれくらい肥大化しそうかの判断材料になります。[official]

## 4. 注意点

ページ数は常にエクステント レベルで管理されるため、値は 8 の倍数になります。さらに、version store はグローバル資源であり、セッションやタスク単位ではなくファイル単位で追跡されます。[official]

`mixed_extent_page_count` が激しく変動している場合は、SGAM ページ利用が集中している可能性があり、`PAGELATCH_UP` 待機の増加と関連することがあります。[official]

権限は、SQL Server 2022 以降では `VIEW SERVER PERFORMANCE STATE`、それ以前では `VIEW SERVER STATE` が基本です。[official]

[official]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-db-file-space-usage-transact-sql?view=sql-server-ver17
