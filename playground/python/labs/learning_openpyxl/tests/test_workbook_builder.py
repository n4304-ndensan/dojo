"""
test_workbook_builder.py

このファイルでは、メモリ上で作った Workbook をどう検証するかを学びます。
ファイル保存を伴わないため、openpyxl 学習の最初の一歩として扱いやすい題材です。
"""

import pytest

from src.app.workbook_builder import build_sales_workbook


def test_build_sales_workbook_writes_headers_and_freeze_panes(
    sales_rows: list[dict[str, int | str]],
) -> None:
    """ヘッダー行と基本設定が正しく入ることを確認します。"""
    workbook = build_sales_workbook(sales_rows)
    sheet = workbook["Sales"]

    assert sheet["A1"].value == "商品名"
    assert sheet["B1"].value == "数量"
    assert sheet["C1"].value == "単価"
    assert sheet["D1"].value == "売上"
    assert sheet.freeze_panes == "A2"


@pytest.mark.parametrize(
    "row_index,expected_amount",
    [
        (2, 360),
        (3, 300),
    ],
)
def test_build_sales_workbook_calculates_amounts(
    sales_rows: list[dict[str, int | str]],
    row_index: int,
    expected_amount: int,
) -> None:
    """数量と単価から売上金額が計算されることを確認します。"""
    workbook = build_sales_workbook(sales_rows)
    sheet = workbook["Sales"]

    assert sheet[f"D{row_index}"].value == expected_amount


def test_build_sales_workbook_applies_styles(
    sales_rows: list[dict[str, int | str]],
) -> None:
    """ヘッダーの強調表示と数値書式が設定されることを確認します。"""
    workbook = build_sales_workbook(sales_rows)
    sheet = workbook["Sales"]

    assert sheet["A1"].font.bold is True
    assert sheet["A1"].fill.fill_type == "solid"
    assert sheet["C2"].number_format == "#,##0"
    assert sheet["D2"].number_format == "#,##0"