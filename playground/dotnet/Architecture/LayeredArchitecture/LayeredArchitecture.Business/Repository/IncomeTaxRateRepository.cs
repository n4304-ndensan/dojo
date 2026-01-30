using LayeredArchitecture.BusinessCommon;

namespace LayeredArchitecture.Business.Repository;

/// <summary>
/// 税率マスタ取得のためのリポジトリ（サンプルではインメモリ）。
/// </summary>
public sealed class IncomeTaxRateRepository
{
    public Task<decimal> GetIncomeTaxRateAsync(decimal taxableIncome)
    {
        var rate = TaxRateCalculator.GetProgressiveRate(taxableIncome);
        return Task.FromResult(rate);
    }
}
