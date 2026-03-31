"""
test_weekend.py

このファイルでは、「入力を与えれば安定して判定できる関数」のテストを書きます。
あえて is_weekend_now は直接テストしていません。
理由は、その関数が現在時刻に依存しており、実行タイミングによって結果が変わるからです。

ここで学ぶべきことは、pytest のテクニックだけではありません。
それ以上に、「どういう設計にすればテストしやすくなるのか」を理解することが重要です。
"""

import pytest

from app.weekend import is_weekend


@pytest.mark.parametrize(
    "day,expected",
    [
        (0, False),  # 月曜
        (1, False),  # 火曜
        (4, False),  # 金曜
        (5, True),   # 土曜
        (6, True),   # 日曜
    ],
)
def test_is_weekend(day: int, expected: bool) -> None:
    """
    曜日番号ごとに、土日判定が期待通りになることを確認します。

    day を外から受け取る設計にしたことで、
    テストの入力と期待結果が明確になり、
    実行日によって結果が揺れない安定したテストになっています。
    """
    assert is_weekend(day) is expected