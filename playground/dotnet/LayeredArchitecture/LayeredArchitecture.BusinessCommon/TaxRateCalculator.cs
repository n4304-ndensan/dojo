namespace LayeredArchitecture.BusinessCommon;

/// <summary>
/// 税率および税額計算の共通ロジック。
/// </summary>
public static class TaxRateCalculator
{
    public static decimal CalculateIncomeTax(decimal taxableIncome, decimal rate)
    {
        if (taxableIncome <= 0) return 0m;
        return decimal.Round(taxableIncome * rate, 0, MidpointRounding.AwayFromZero);
    }

    public static decimal GetProgressiveRate(decimal taxableIncome)
    {
        return taxableIncome switch
        {
            <= 1_950_000m => 0.05m,
            <= 3_300_000m => 0.10m,
            <= 6_950_000m => 0.20m,
            <= 9_000_000m => 0.23m,
            <= 18_000_000m => 0.33m,
            <= 40_000_000m => 0.40m,
            _ => 0.45m
        };
    }
}
