using LayeredArchitecture.Business.Repository;
using LayeredArchitecture.BusinessCommon;

namespace LayeredArchitecture.Business;

/// <summary>
/// 税率取得と税額計算を担うサービス。
/// </summary>
public sealed class IncomeTaxService
{
    private readonly IncomeTaxRateRepository _rateRepository;

    public IncomeTaxService(IncomeTaxRateRepository rateRepository)
    {
        _rateRepository = rateRepository;
    }

    public Task<decimal> GetTaxRateAsync(decimal taxableIncome) =>
        _rateRepository.GetIncomeTaxRateAsync(taxableIncome);

    public decimal CalculateTax(decimal taxableIncome, decimal taxRate) =>
        TaxRateCalculator.CalculateIncomeTax(taxableIncome, taxRate);
}
