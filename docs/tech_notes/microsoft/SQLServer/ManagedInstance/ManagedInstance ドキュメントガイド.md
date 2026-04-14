# ManagedInstance ドキュメントガイド

このフォルダには、Azure SQL Managed Instance 固有の観点を持つ文書だけを集めています。SQL Server 全般に共通する概念は上位の SQLServer フォルダに残し、Managed Instance 固有の挙動、監視、評価スクリプトの読み方だけをここへ寄せることで、汎用解説と環境依存の注意点が混ざりすぎないようにしています。

このガイドの目的は、Managed Instance 向け文書の責務を分けて、どこを入口に読むべきかを明確にすることです。RCSI の基礎自体は上位文書で扱い、ここでは Managed Instance で前提が変わるところだけに集中します。

## 1. このフォルダの主な文書

- 全体の入口: [[Azure SQL Managed Instance における RCSI 評価の前提整理]]
- 評価スクリプトの設計ガイド: [[Azure SQL Managed Instance における RCSI 評価スクリプト設計ガイド]]
- `sys.server_resource_stats` の個別解説: [[sys.server_resource_stats 詳細解説]]

## 2. 読む順番の目安

Managed Instance 向けの RCSI 評価をこれから整理するなら、次の順番で読むと理解がつながりやすくなります。

1. [[Azure SQL Managed Instance における RCSI 評価の前提整理]]
2. [[Azure SQL Managed Instance における RCSI 評価スクリプト設計ガイド]]
3. [[sys.server_resource_stats 詳細解説]]

そのうえで、分離レベルの基礎に戻りたいときは [[SQL Server の Isolation Level（分離レベル）詳細解説]]、RCSI と SNAPSHOT の比較を見たいときは [[SQL Server Snapshot Isolation と RCSI と Read Committed の違い]]、名前解決を確認したいときは [[SQL Server の User、Schema、tempdb.sys の名前解決]] を参照します。

## 3. このフォルダで扱う責務

Managed Instance フォルダでは、次のような内容を扱います。

- Managed Instance 固有の監視ビューやサービスメーターの読み方
- Managed Instance での RCSI 評価スクリプトの設計原則と参照実装
- ADR 常時有効、PVS、更新遅延など、Managed Instance 前提で解釈が変わる点

逆に、次の内容は原則として上位の SQLServer フォルダで扱います。

- SQL Server 全般で共通する分離レベルの基礎
- User、Schema、名前解決など製品共通の概念
- DMV / DMF の汎用索引や個票

## 4. 追加・更新時の方針

このフォルダを持続可能に保つため、Managed Instance 固有の文書を追加するときは次の方針でそろえます。

- タイトルか冒頭で、Managed Instance 固有の話であることを明示します。
- 上位フォルダの文書にある概念説明を長く再掲せず、必要に応じてリンクでつなぎます。
- 評価スクリプト文書は、個別案件の修正履歴ではなく、誤解しやすい前提と推奨観測設計を中心に書きます。
- 新しい Managed Instance 向けトップレベル文書を追加したら、このガイドと上位の [[SQL Server ドキュメントガイド]] の両方を更新します。

## 5. 迷ったときの判断基準

置き場に迷ったときは、次の基準で切り分けると整理しやすくなります。

- Managed Instance でしか成立しない注意点なら、このフォルダへ置きます。
- SQL Server 全般に共通する説明なら、上位の SQLServer フォルダへ置きます。
- `sys.server_resource_stats` のように Managed Instance 依存だが単体で繰り返し参照される対象は、個別文書として分けます。

この切り分けを守ると、後から読み返したときにも「まず上位で基礎を見てから、必要な Managed Instance 補足だけ読む」という流れを保ちやすくなります。
