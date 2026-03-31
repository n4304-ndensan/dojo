"""
user_service.py

このモジュールでは、簡単なドメインオブジェクトと、その判定関数を用意しています。
pytest では、単なる数値計算だけでなく、「意味のあるデータ構造」を相手にしたテストを書くことが重要です。
その入口として dataclass を利用したユーザー判定を扱います。
"""

from dataclasses import dataclass


@dataclass
class User:
    """
    学習用の簡易ユーザーモデルです。

    dataclass を使うことで、初期化や表示用のメソッドを簡潔に持てるため、
    テストデータを読みやすく作れます。
    """

    id: int
    name: str
    age: int



def is_adult(user: User) -> bool:
    """
    ユーザーが成人かどうかを判定します。

    ここでは「20歳以上なら成人」と定義しています。
    条件が単純なため、境界値テストの題材に向いています。
    例えば age=19, 20, 21 を比較することで、
    どこを境界として仕様が成立しているかを確認できます。
    """
    return user.age >= 20