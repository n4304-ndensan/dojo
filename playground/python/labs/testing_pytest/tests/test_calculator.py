"""
test_calculator.py

このテストファイルでは、pytest の基本的な3つの機能を扱います。

1. assert を使った基本テスト
2. pytest.raises を使った例外テスト
3. parametrize を使った複数ケースの整理

最初に読むならこのファイルから始めるのが分かりやすいです。
"""

import pytest

from app.calculator import add, divide



def test_add_returns_sum() -> None:
    """
    add 関数が正しく合計値を返すことを確認します。

    pytest では通常の assert をそのまま使えます。
    特別なアサーションAPIを覚えなくても始めやすいことが pytest の利点です。
    """
    assert add(2, 3) == 5



def test_divide_by_zero_raises_value_error() -> None:
    """
    divide 関数に 0 を渡したとき、ValueError が送出されることを確認します。

    これは異常系テストの基本です。
    正常系だけでは仕様を守れているとは言えないため、
    例外を投げるべき条件も明示的にテストします。
    """
    with pytest.raises(ValueError):
        divide(10, 0)


@pytest.mark.parametrize(
    "a,b,expected",
    [
        (10, 2, 5),
        (9, 3, 3),
        (5, 2, 2.5),
    ],
)
def test_divide_with_multiple_cases(a: int, b: int, expected: float) -> None:
    """
    divide 関数を複数ケースでまとめて検証します。

    同じ構造のテストをコピペで増やす代わりに、
    入力と期待値だけを列挙して pytest に繰り返し実行させます。

    これにより、テストコードの重複が減り、
    「どのケースを検証対象としているのか」が見えやすくなります。
    """
    assert divide(a, b) == expected