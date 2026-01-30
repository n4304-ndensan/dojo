using LayeredArchitecture.Business;
using LayeredArchitecture.Models;

namespace LayeredArchitecture.BusinessControl;

/// <summary>
/// 所得税計算ユースケース（業務フローオーケストレーション）。
/// </summary>
public sealed class IncomeTaxUseCase
{
    private readonly DeductionService _deductionService;
    private readonly IncomeTaxService _incomeTaxService;
    private readonly HistoryService _historyService;

    public IncomeTaxUseCase(
        DeductionService deductionService,
        IncomeTaxService incomeTaxService,
        HistoryService historyService)
    {
        _deductionService = deductionService;
        _incomeTaxService = incomeTaxService;
        _historyService = historyService;
    }

    public async Task<IncomeTaxResultDto> CalculateAsync(IncomeTaxInputDto input)
    {
        var taxableIncome = await _deductionService.CalculateTaxableIncomeAsync(input);
        var taxRate = await _incomeTaxService.GetTaxRateAsync(taxableIncome);
        var taxAmount = _incomeTaxService.CalculateTax(taxableIncome, taxRate);

        await _historyService.SaveAsync(new HistoryDto
        {
            Salary = input.Salary,
            TaxableIncome = taxableIncome,
            TaxRate = taxRate,
            TaxAmount = taxAmount,
            Timestamp = DateTime.Now
        });

        return new IncomeTaxResultDto
        {
            TaxableIncome = taxableIncome,
            TaxRate = taxRate,
            TaxAmount = taxAmount
        };
    }
}
