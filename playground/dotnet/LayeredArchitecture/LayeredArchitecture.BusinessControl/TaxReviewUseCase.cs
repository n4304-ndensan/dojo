using LayeredArchitecture.Business;
using LayeredArchitecture.Models;

namespace LayeredArchitecture.BusinessControl;

/// <summary>
/// 履歴を元に集計・レビューを行うユースケース。
/// </summary>
public sealed class TaxReviewUseCase
{
    private readonly HistoryService _historyService;
    private readonly IncomeTaxService _incomeTaxService;

    public TaxReviewUseCase(HistoryService historyService, IncomeTaxService incomeTaxService)
    {
        _historyService = historyService;
        _incomeTaxService = incomeTaxService;
    }

    public async Task<TaxReviewSummaryDto> SummarizeAsync()
    {
        var histories = await _historyService.GetAllAsync();
        if (histories.Count == 0)
        {
            return new TaxReviewSummaryDto();
        }

        var totalTax = histories.Sum(h => h.TaxAmount);
        var totalTaxable = histories.Sum(h => h.TaxableIncome);
        var weightedRate = totalTaxable == 0 ? 0m : totalTax / totalTaxable;
        var last = histories.MaxBy(h => h.Timestamp);

        // 参考: 最終課税所得に現在の税率を適用した場合
        var lastRate = last is null ? 0m : _incomeTaxService.CalculateTax(last.TaxableIncome, last.TaxRate);

        return new TaxReviewSummaryDto
        {
            TotalTax = totalTax,
            TotalTaxableIncome = totalTaxable,
            AverageAppliedRate = weightedRate,
            LastCalculatedAt = last?.Timestamp
        };
    }
}
