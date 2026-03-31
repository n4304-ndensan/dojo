# lab-openpyxl

このプロジェクトは、openpyxl の基本を学ぶための学習用プロジェクトです。
Excel ファイルをただ作るだけでなく、
「どのように workbook を組み立て、どう保存し、どう読み戻すのか」を小さな題材で理解することを目的にしています。

## このプロジェクトで学べること

- Workbook と Worksheet の基本操作
- セルへの値の書き込みと読み取り
- ヘッダー装飾、数値書式、freeze panes などの基本書式設定
- xlsx ファイルの保存と読み込み
- pytest の fixture と tmp_path を使った Excel ファイルのテスト

## ディレクトリ構成

```text
lab-openpyxl/
├─ pyproject.toml
├─ README.md
├─ src/
│  └─ app/
│     ├─ __init__.py
│     ├─ grade_report.py
│     ├─ inventory_reader.py
│     └─ workbook_builder.py
└─ tests/
   ├─ conftest.py
   ├─ test_grade_report.py
   ├─ test_inventory_reader.py
   └─ test_workbook_builder.py
```

## セットアップ

uv を使う場合:

```powershell
cd learning_openpyxl
uv sync
uv run pytest
```

## 学び方のおすすめ

最初は README よりも tests を先に読むと、
「何を期待して workbook を作っているのか」が掴みやすくなります。

次に、実装側を見て以下を確認してください。

- ヘッダー行にどのようなスタイルを適用しているか
- 保存先の path を引数で受け取ると、なぜテストしやすくなるか
- load_workbook で読んだ値を、どのように Python のデータへ戻しているか

慣れてきたら、次の変更を試すと理解が深まります。

- 売上シートに列を追加して、テストも一緒に更新する
- 判定基準を変更して、成績表のテストがどう落ちるか見る
- 在庫判定の条件を変えて、読み取りロジックの影響範囲を確認する

## このプロジェクトの責務

このプロジェクトの責務は、openpyxl の入口を理解することです。
グラフ作成、複雑な数式、既存テンプレートの高度な編集までは扱っていません。
まずは workbook の生成・保存・読み込みという基本サイクルを安定して扱えるようになることを優先しています。
