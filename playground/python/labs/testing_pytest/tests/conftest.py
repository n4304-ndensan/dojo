"""
conftest.py

pytest では、複数のテストで共有したい fixture を conftest.py に置くことがよくあります。
このファイルに書かれた fixture は、同じディレクトリ配下のテストから自動的に利用できます。

この学習用プロジェクトでは、User を返す fixture を置き、
「共通の前提データを毎回書かずに再利用する」感覚を掴むことを目的としています。
"""

import pytest

from src.app.user_service import User


@pytest.fixture
def adult_user() -> User:
    """
    成人ユーザーを返す fixture です。

    テスト関数の引数に `adult_user` と書くだけで、
    pytest がこの関数を実行して値を注入してくれます。

    これは依存性注入に近い感覚で理解すると分かりやすいです。
    """
    return User(id=1, name="Taro", age=25)


@pytest.fixture
def minor_user() -> User:
    """
    未成年ユーザーを返す fixture です。

    成人ケースと未成年ケースの両方を読みやすく比較するために用意しています。
    """
    return User(id=2, name="Jiro", age=18)