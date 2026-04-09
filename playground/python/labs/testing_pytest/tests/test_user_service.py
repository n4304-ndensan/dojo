"""
test_user_service.py

このファイルでは、dataclass を使ったオブジェクトを相手にしたテストと、
fixture を利用した共通テストデータの再利用を学びます。

数値計算だけでなく、意味のあるデータ構造を扱うことで、
実務に近い見え方になります。
"""

from src.app.user_service import User, is_adult



def test_is_adult_returns_true_for_age_20_or_more() -> None:
    """
    年齢が20以上のユーザーは成人と判定されることを確認します。

    ここでは最も基本的な正常系を扱っています。
    """
    user = User(id=10, name="Hanako", age=20)
    assert is_adult(user) is True



def test_is_adult_returns_false_for_under_20() -> None:
    """
    年齢が20未満のユーザーは未成年と判定されることを確認します。

    正常系の反対側も明示しておくことで、
    条件の意味がよりはっきりします。
    """
    user = User(id=11, name="Ken", age=19)
    assert is_adult(user) is False



def test_is_adult_with_adult_fixture(adult_user: User) -> None:
    """
    fixture から注入された成人ユーザーでテストします。

    fixture を使うことで、前提データ作成の重複を減らせます。
    ただし、fixture を増やしすぎると依存関係が見えにくくなるため、
    最初は分かりやすい範囲で使うのが重要です。
    """
    assert is_adult(adult_user) is True



def test_is_adult_with_minor_fixture(minor_user: User) -> None:
    """
    fixture から注入された未成年ユーザーでテストします。

    成人用 fixture と未成年用 fixture を分けておくことで、
    テストの意図が名前から読み取れるようになります。
    """
    assert is_adult(minor_user) is False