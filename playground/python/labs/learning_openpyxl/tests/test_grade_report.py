"""
test_grade_report.py

このファイルでは、Python 側で計算した結果を Workbook に反映し、
さらに save したファイルを読み戻して確認する流れを扱います。
"""

from pathlib import Path

from openpyxl import load_workbook

from src.app.grade_report import build_grade_report, save_grade_report, StudentScore


def test_build_grade_report_calculates_total_average_and_result(
    student_scores: list[StudentScore],
) -> None:
    """合計、平均、判定が期待通りに書き込まれることを確認します。"""
    workbook = build_grade_report(student_scores)
    sheet = workbook["Grades"]

    assert sheet["E2"].value == 240
    assert sheet["F2"].value == 80.0
    assert sheet["G2"].value == "合格"
    assert sheet["E3"].value == 160
    assert sheet["F3"].value == 53.3
    assert sheet["G3"].value == "要復習"


def test_build_grade_report_marks_review_rows(
    student_scores: list[StudentScore],
) -> None:
    """要復習の行だけ判定セルが強調表示されることを確認します。"""
    workbook = build_grade_report(student_scores)
    sheet = workbook["Grades"]

    assert sheet["G2"].fill.fill_type is None
    assert sheet["G3"].fill.fill_type == "solid"
    assert sheet["F2"].number_format == "0.0"


def test_save_grade_report_creates_excel_file(
    tmp_path: Path,
    student_scores: list[StudentScore],
) -> None:
    """保存した xlsx を読み戻せることを確認します。"""
    path = tmp_path / "grade_report.xlsx"

    save_grade_report(path, student_scores)
    loaded_workbook = load_workbook(path)
    sheet = loaded_workbook["Grades"]

    assert path.exists() is True
    assert sheet["A2"].value == "Aiko"
    assert sheet["G3"].value == "要復習"