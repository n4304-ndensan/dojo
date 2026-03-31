"""
calculator.py

このモジュールは、pytest の最も基本的なテスト対象として使うための計算関数を置いています。
関数自体はシンプルですが、正常系と異常系を両方扱えるため、
pytest の assert、raises、parametrize を学ぶ題材として扱いやすい構成です。
"""


def add(a: int, b: int) -> int:
    """
    2つの整数を加算して返します。

    この関数は副作用を持たない純粋関数です。
    純粋関数は、入力に対して出力が安定するため、テストが非常に書きやすいです。
    pytest 学習の最初の対象として適しています。
    """
    return a + b



def divide(a: int, b: int) -> float:
    """
    2つの数値の割り算を行います。

    0除算は禁止とし、b が 0 の場合は ValueError を送出します。
    これにより、正常系だけでなく異常系のテストも学べるようにしています。
    """
    if b == 0:
        raise ValueError("b must not be zero")
    return a / b