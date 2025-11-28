using LayeredArchitecture.Models;
using LayeredArchitecture.ViewCommon;

namespace LayeredArchitecture.ViewControl;

/// <summary>
/// 結果画面の UI ロジック。
/// </summary>
public sealed class IncomeTaxResultControl
{
    public IncomeTaxResultDto? GetLatestResult() => IncomeTaxState.LatestResult;

    public string BuildSummary(IncomeTaxResultDto result)
    {
        return $"課税所得 {FormatHelpers.Currency(result.TaxableIncome)} に税率 {FormatHelpers.Percent(result.TaxRate)} を適用 → 税額 {FormatHelpers.Currency(result.TaxAmount)}";
    }
}
