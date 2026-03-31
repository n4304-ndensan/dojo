"""
conftest.py

このプロジェクトでは、Excel の元データを fixture でまとめて再利用します。
毎回テストの中で同じ行データを書くよりも、
「題材となるデータ」を名前付きで共有した方が学習しやすいためです。
"""

from pathlib import Path

import pytest

from src.app.grade_report import StudentScore
from src.app.inventory_reader import InventoryItem, save_inventory_workbook


@pytest.fixture
def sales_rows() -> list[dict[str, int | str]]:
    """売上シート作成に使うサンプル行です。"""
    return [
        {"item_name": "Apple", "quantity": 3, "unit_price": 120},
        {"item_name": "Orange", "quantity": 2, "unit_price": 150},
    ]


@pytest.fixture
def student_scores() -> list[StudentScore]:
    """成績表作成に使うサンプル成績です。"""
    return [
        StudentScore(name="Aiko", japanese=80, math=75, science=85),
        StudentScore(name="Ren", japanese=50, math=60, science=50),
    ]


@pytest.fixture
def inventory_items() -> list[InventoryItem]:
    """在庫表作成に使うサンプル商品です。"""
    return [
        InventoryItem(item_name="Notebook", stock=3, reorder_level=5),
        InventoryItem(item_name="Pen", stock=10, reorder_level=5),
        InventoryItem(item_name="Marker", stock=1, reorder_level=4),
    ]


@pytest.fixture
def inventory_file(tmp_path: Path, inventory_items: list[InventoryItem]) -> Path:
    """サンプル在庫表を一時ディレクトリへ保存して返します。"""
    path = tmp_path / "inventory.xlsx"
    save_inventory_workbook(path, inventory_items)
    return path