# lab-pytest

このプロジェクトは、pytest の基本を学ぶための学習用プロジェクトです。
単にテストの書き方を覚えるだけでなく、どのようなコードがテストしやすく、
どのようなコードがテストしにくいのかを理解することを目的にしています。

## このプロジェクトで学べること

- pytest の基本的な assert ベースのテスト
- 例外テスト (`pytest.raises`)
- パラメータ化テスト (`pytest.mark.parametrize`)
- fixture によるテストデータの再利用
- テストしやすい設計と、しにくい設計の違い

## ディレクトリ構成

```text
lab-pytest/
├─ pyproject.toml
├─ README.md
├─ src/
│  └─ app/
│     ├─ __init__.py
│     ├─ calculator.py
│     ├─ user_service.py
│     └─ weekend.py
└─ tests/
   ├─ conftest.py
   ├─ test_calculator.py
   ├─ test_user_service.py
   └─ test_weekend.py
```

## セットアップ

uv を使う場合:

```powershell
cd lab-pytest
uv sync
uv run pytest
```

## 学び方のおすすめ

最初はテストを通すことよりも、各テストが何を検証しているかを読むことを重視してください。
そのうえで、わざとコードを壊してテストがどう落ちるかを見ると理解が深まります。

例えば、次のような変更を試すとよいです。

- `is_adult` の条件を変えて境界値テストが落ちるか確認する
- `divide` の例外処理を消して異常系テストが落ちるか確認する
- `is_weekend` を書き換えて、入力駆動のテストの意味を確認する

## このプロジェクトの責務

このプロジェクトの責務は、pytest の基本的な使い方と、テスト容易性のある設計の入口を理解することです。
mock の高度な使い方や、DB/HTTP を含む統合テストは含めていません。
それらは次の段階で扱うのが適切です。