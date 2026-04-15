# SQL Server で ADR ON のとき PVS は更新のたびに必ず増えるのか

> 位置づけ: この文書は、ADR 有効時の row versioning と PVS の関係を整理し、「更新が起きたのに `pvs_size_mb` が増えないのはなぜか」を説明するための技術解説です。特に、in-row version storage と off-row PVS を混同しないための補助資料として使います。
>
> 読み分け: lock / blocking / version store の全体像は [[SQL Server の Lock と Blocking と Version Store の整理]]、分離レベル全般は [[SQL Server の Isolation Level（分離レベル）詳細解説]]、Managed Instance の測定列に落として読む場合は [[../ManagedInstance/Azure SQL Managed Instance における RCSI 評価スクリプトの測定項目整理]] を参照してください。

## 1. 文脈と目的

ADR を使う環境で row versioning を観測していると、よくぶつかる疑問があります。`ADR = ON` なら PVS を使うはずなのに、更新を流しても `pvs_size_mb` が思ったほど増えない、あるいはまったく増えないことがある、という疑問です。ここで「増えないなら versioning は起きていない」と読むと、かなりの確率で誤ります。

この誤解が起きやすい理由は、Microsoft Learn が使う「PVS」という言葉と、実際に現場で見ている `pvs_size_mb` の意味が完全には一致しないためです。公式は PVS を、ユーザーデータベース内で row version を保持する仕組み全体として説明していますが、`sys.dm_tran_persistent_version_store_stats.persistent_version_store_size_kb` が表すのは off-row version だけです。[adr-concepts][locking-row-versioning][pvs-stats]

この文書の目的は、次の三点を明確にすることです。

- `ADR ON` で更新が起きたとき、何が必ず起きて、何が必ずしも起きないのか
- in-row version storage と off-row PVS がどう違うのか
- `pvs_size_mb` や `effective_version_store_mb` をどう読めばよいのか

## 2. まず結論

結論から言うと、`ADR = ON` だからといって、更新のたびに必ず off-row の PVS サイズが増えるわけではありません。Microsoft Learn の row versioning ガイドは、ADR 有効時の row version が三通りの保存方式を取りうると説明しています。小さい行は旧行全体を更新後の行の一部として保持し、中程度の行は差分を更新後の行の一部として保持し、大きい行は別の内部テーブルに保持します。前二者が in-row version storage、最後だけが off-row version storage です。[locking-row-versioning]

したがって、`ADR ON` の更新で重要なのは「versioning が使われるか」ではなく、「その version が in-row なのか off-row なのか」です。`pvs_size_mb` が見ているのは off-row だけなので、更新が行われても in-row で吸収されれば `pvs_size_mb` は増えません。逆に、`pvs_size_mb` が増えていれば off-row version が発生していることは言えますが、増えていないことから「versioning がなかった」とは言えません。[locking-row-versioning][pvs-stats]

さらに、公式は ADR 下で version store が満杯になると `UPDATE` と `DELETE` は失敗しうる一方、`INSERT` はデータベースに十分な空きがあれば成功すると説明しています。このため、PVS 容量圧力を強く意識すべき主役は `UPDATE` / `DELETE` であり、`INSERT` を同列には扱わない方が実務上は整理しやすくなります。[locking-row-versioning]

## 3. なぜ誤解しやすいか

この論点がややこしいのは、「PVS」という言葉が二つの粒度で使われるからです。ADR の概念説明では、PVS は「row version を `tempdb` ではなくユーザーデータベースに保持する仕組み」として説明されます。この文脈では、行の中に保持される in-row version も、別の内部テーブルに置かれる off-row version も、どちらも PVS の一部です。[adr-concepts][locking-row-versioning]

一方、監視でよく使う `sys.dm_tran_persistent_version_store_stats.persistent_version_store_size_kb` は、名前から PVS 全体のサイズに見えますが、実際には off-row version のサイズしか含みません。Microsoft Learn の DMV 説明も、`persistent_version_store_size_kb` は off-row versions のサイズであり、in-row versions は含まないと明記しています。[pvs-stats]

このズレのため、次のような読み違いが起きます。

- ADR ON なのに `pvs_size_mb` が増えないから、versioning は起きていない
- `pvs_size_mb` が小さいから、ADR のコストはほぼゼロだ
- `effective_version_store_mb = pvs_size_mb` だから、これで versioning コスト全体を完全に表せる

実際には、これらはどれも強すぎる結論です。正しくは、「off-row の PVS は大きく増えていない」が言えるだけです。

## 4. 中核概念と用語

このテーマでは、似た言葉を少し丁寧に切り分けておく必要があります。

まず ADR は、クラッシュ復旧とロールバックを高速化するために、物理更新を versioning し、PVS と logical revert を使って undo を行う仕組みです。ADR 概念ページは、ADR が physical database modifications を versioning すると説明しています。[adr-concepts]

次に row versioning は、更新前の行イメージを保持して、後からその時点の整合した状態を読めるようにする枠組みです。RCSI や SNAPSHOT だけでなく、trigger、MARS、online index operations でも使われます。[locking-row-versioning]

そして PVS は、ADR 有効時に row version をユーザーデータベース側へ保持する仕組みです。ここでいう PVS は、次の二つを含みます。

- in-row version storage: 更新後の行の中に旧行全体または差分を保持する方式
- off-row version storage: 別の内部テーブルに旧行全体を保持する方式

最後に `pvs_size_mb` は、たいてい `persistent_version_store_size_kb / 1024.0` のように作られた観測列です。これは「PVS 全体」ではなく、「off-row versions のサイズ」を見ている列です。この点を取り違えると、観測の意味が大きくずれます。[locking-row-versioning][pvs-stats]

## 5. ADR ON で更新が起きたとき、内部では何が起きるか

まず確実に押さえるべきなのは、ADR を含む row versioning 系機能が有効なデータベースでは、行に versioning 用の追加情報が入ることです。Microsoft Learn は、各データベース行が行末に最大 14 バイトの row versioning information を持ちうると説明しており、この 14 バイトは ADR が有効な場合、行が最初に更新されたとき、または新しい行が挿入されたときに追加されます。[locking-row-versioning]

そのうえで、更新前イメージの保持方法が分かれます。ADR 有効時には、行サイズに応じて三通りの保存方式があります。

- 行が小さい場合: 旧行全体を更新後の行の一部として保持します。
- 行が中程度の場合: 旧行との差分だけを更新後の行の一部として保持します。
- 行が大きい場合: 旧行全体を別の内部テーブルに置きます。

つまり、`UPDATE` が実行されたときに「古い状態を保持する」という責務自体は発生しても、その保存先が常に off-row とは限りません。ここが「ADR ON なら更新のたびに PVS が増える」と単純化できない理由です。[locking-row-versioning]

また、ADR 概念ページは、ADR が physical database modifications を versioning すると説明しています。さらに row versioning ガイドは、row versioning-based isolation を有効にしたデータベースでは、active transaction が row versioning-based isolation を使っていなくても、更新前コピーが version store に保持されると説明しています。したがって、少なくとも ADR または row versioning-based isolation が有効な文脈では、「その瞬間に SNAPSHOT 読み取りがいないから version は作られない」という説明は、一次資料とは合いません。[adr-concepts][locking-row-versioning]

## 6. UPDATE / DELETE / INSERT をどう分けて考えるか

この論点では、`UPDATE` / `DELETE` と `INSERT` を分けて考えた方が実務で迷いません。

`UPDATE` と `DELETE` は、既存行の以前の状態をどこかに保持しないと、ADR の logical revert や row versioning 読み取りで過去の整合状態を再構成できません。そのため、PVS や in-row version storage の観点で最も重要なのは、この二つです。[adr-concepts][locking-row-versioning]

`INSERT` は少し性質が違います。公式は、ADR 使用時に version store が満杯になると `UPDATE` と `DELETE` は失敗し、`INSERT` はデータベースに十分な空きがあれば成功すると説明しています。ここから実務的には、「`INSERT` も versioning まわりのメタデータを持ちうるが、更新前イメージの保持という意味では `UPDATE` / `DELETE` と同じ経路を踏むわけではない」と読むのが妥当です。これは Microsoft Learn の挙動説明からの読み取りであり、`INSERT` の内部表現を完全に同一視しているわけではありません。[locking-row-versioning]

したがって、`ADR ON` で PVS 圧力や cleanup 停滞を議論するときは、まず `UPDATE` / `DELETE` がどれだけ走ったかを優先的に見る方が筋が通ります。

## 7. `pvs_size_mb` が増えないのに versioning が起きていることはあるか

あります。しかも、それは珍しいケースではありません。

最も単純なのは、更新が in-row version storage で吸収されたケースです。Microsoft Learn が明示しているとおり、旧行全体や差分が更新後の行の一部として保持されるなら、off-row internal table は増えません。この場合、`persistent_version_store_size_kb` は増えないか、ほとんど増えませんが、versioning 自体は使われています。[locking-row-versioning]

次に、off-row version は作られたが、観測時点では cleanup が追いついていて、瞬間値としては大きく見えないケースがあります。PVS cleaner は不要になった off-row version を非同期に片づけるため、サンプリング間隔が粗いと、生成と掃除のあいだを見逃すことがあります。[adr-concepts][pvs-stats]

さらに、`pvs_size_mb` は off-row の容量であり、in-row の payload やレコード数は別の観測手段を使わないと見えません。row versioning ガイドと `sys.dm_db_index_physical_stats` の説明では、`inrow_version_record_count`、`inrow_diff_version_record_count`、`total_inrow_version_payload_size_in_bytes`、`offrow_regular_version_record_count` を使って、index / partition 単位の in-row / off-row の状況を見られます。[locking-row-versioning][dm-db-index-physical-stats]

つまり、`pvs_size_mb` が増えないことから安全に言えるのは、「off-row version の残量が大きく増えていない」までです。それ以上、たとえば「ADR の versioning は起きていない」「RCSI の副作用は出ていない」とまでは言えません。

## 8. 観測するときに見るべき DMV

実際に切り分けるなら、off-row だけを見る DMV と、in-row / off-row を index 単位で見る DMV を並べて使うのが安全です。

まず off-row の瞬間残量を見るには、`sys.dm_tran_persistent_version_store_stats` を使います。

```sql
SELECT
    DB_NAME(database_id) AS database_name,
    persistent_version_store_size_kb / 1024.0 AS offrow_pvs_mb,
    current_aborted_transaction_count,
    oldest_active_transaction_id,
    pvs_off_row_page_skipped_oldest_active_xdesid
FROM sys.dm_tran_persistent_version_store_stats
WHERE database_id = DB_ID();
```

この結果は、off-row 側の残量と cleanup 停滞の気配を見るには有効ですが、in-row は見えません。[pvs-stats]

次に、in-row と off-row を object / index 単位で見るには `sys.dm_db_index_physical_stats` を使います。

```sql
SELECT
    OBJECT_SCHEMA_NAME(object_id) AS schema_name,
    OBJECT_NAME(object_id) AS object_name,
    index_id,
    partition_number,
    inrow_version_record_count,
    inrow_diff_version_record_count,
    total_inrow_version_payload_size_in_bytes,
    offrow_regular_version_record_count
FROM sys.dm_db_index_physical_stats(DB_ID(), NULL, NULL, NULL, 'DETAILED')
WHERE total_inrow_version_payload_size_in_bytes > 0
   OR offrow_regular_version_record_count > 0;
```

このクエリは重いので、実運用では対象 object_id を絞る方が安全です。ただし、`pvs_size_mb` が増えない理由を本気で見たいなら、この種の index / partition レベルの観測が必要になります。[dm-db-index-physical-stats]

Managed Instance の評価スクリプトで使う `effective_version_store_mb` や `pvs_size_mb` は、短時間比較には便利です。ただし、その便利さは「off-row を主指標として素早く見られる」ことにあり、「versioning 全体を完全表現する」ことではありません。この責務差を理解したうえで使うべきです。[[../ManagedInstance/Azure SQL Managed Instance における RCSI 評価スクリプトの測定項目整理]]

## 9. 使うべき場面と、使うべきでない場面

この整理が特に役立つのは、次のような場面です。

- `ADR ON` なのに `pvs_size_mb` が増えず、versioning の有無を判断できなくなっているとき
- Managed Instance や SQL Server 2022 で、`tempdb` ではなく PVS を見始めたばかりのとき
- RCSI の効果は見えているが、副作用の観測先が合っているか不安なとき

逆に、この文書だけでは足りない場面もあります。

- 実際にどのトランザクションが cleanup を止めているかを追いたいとき
- PVS filegroup の I/O 影響まで設計に落としたいとき
- lock / blocking / wait と version store をまとめて切り分けたいとき

その場合は、上位の概念整理として [[SQL Server の Lock と Blocking と Version Store の整理]]、Managed Instance の評価設計として [[../ManagedInstance/Azure SQL Managed Instance における RCSI 評価スクリプト設計ガイド]] を併読した方がよくなります。

## 10. よくある誤解や失敗パターン

### 10.1 `ADR ON` なら更新のたびに必ず `pvs_size_mb` が増えると思う

これは最も多い誤解です。正しくは、更新に伴う versioning は起きうるが、その保存が毎回 off-row とは限りません。small / intermediate row では in-row で吸収されます。[locking-row-versioning]

### 10.2 `pvs_size_mb` が増えないなら versioning は起きていないと思う

これも誤りです。`pvs_size_mb` は off-row しか見ません。in-row version storage や cleanup のタイミングによって、値が増えないことは普通にあります。[pvs-stats][dm-db-index-physical-stats]

### 10.3 `INSERT` も `UPDATE` と同じ意味で PVS を増やすと思う

公式の挙動説明は、version store が満杯のとき `UPDATE` / `DELETE` が失敗し、`INSERT` は空きがあれば成功するとしています。したがって、容量圧力の読み方としては `UPDATE` / `DELETE` を主役に置く方が適切です。[locking-row-versioning]

### 10.4 `effective_version_store_mb` ひとつで versioning コスト全体を判断できると思う

これは測定列の責務を超えています。`effective_version_store_mb` は「いま主としてどこを見るか」を抽象化するには便利ですが、in-row / off-row の内訳や cleanup 停滞の理由までは表しません。必要に応じて `sys.dm_db_index_physical_stats` と `sys.dm_tran_persistent_version_store_stats` を併用すべきです。[[../ManagedInstance/Azure SQL Managed Instance における RCSI 評価スクリプトの測定項目整理]]

## 11. 結論

`ADR = ON` のとき、更新に versioning が関与すること自体は本質ですが、その結果が毎回 off-row の PVS サイズ増加として見えるとは限りません。Microsoft Learn が示すとおり、ADR 下の row version は in-row と off-row の両方式を取りえます。そして、`persistent_version_store_size_kb` が表すのは off-row だけです。[locking-row-versioning][pvs-stats]

したがって、現場での正しい読み方は、「ADR ON で更新が走ったのに `pvs_size_mb` が増えない」ではなく、「ADR ON で更新は走ったが、今回観測できた off-row version の残量は大きくなっていない」です。この違いを押さえておくと、PVS の見え方と実際の versioning の有無を取り違えにくくなります。

## 参考

- [Transaction locking and row versioning guide][locking-row-versioning]
- [Accelerated database recovery][adr-concepts]
- [sys.dm_tran_persistent_version_store_stats][pvs-stats]
- [sys.dm_db_index_physical_stats][dm-db-index-physical-stats]
- [Server configuration: ADR Preallocation Factor][adr-prealloc]

[locking-row-versioning]: https://learn.microsoft.com/sql/relational-databases/sql-server-transaction-locking-and-row-versioning-guide?view=sql-server-ver17
[adr-concepts]: https://learn.microsoft.com/sql/relational-databases/accelerated-database-recovery-concepts?view=sql-server-ver17
[pvs-stats]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-tran-persistent-version-store-stats?view=sql-server-ver17
[dm-db-index-physical-stats]: https://learn.microsoft.com/sql/relational-databases/system-dynamic-management-views/sys-dm-db-index-physical-stats-transact-sql?view=sql-server-ver17
[adr-prealloc]: https://learn.microsoft.com/sql/database-engine/configure-windows/adr-preallocation-factor-server-configuration-option?view=sql-server-ver17
