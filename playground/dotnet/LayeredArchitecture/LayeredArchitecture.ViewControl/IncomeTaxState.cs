using LayeredArchitecture.Models;

namespace LayeredArchitecture.ViewControl;

/// <summary>
/// 画面間で共有したい状態を保持する簡易ステート。
/// </summary>
public static class IncomeTaxState
{
    public static IncomeTaxResultDto? LatestResult { get; set; }
}
