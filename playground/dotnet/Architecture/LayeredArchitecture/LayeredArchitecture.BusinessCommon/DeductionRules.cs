namespace LayeredArchitecture.BusinessCommon;

/// <summary>
/// 控除計算の共通ルール。
/// </summary>
public static class DeductionRules
{
    public static decimal CalculateTaxableIncome(
        decimal salary,
        decimal socialInsurance,
        int dependants,
        decimal basicDeduction,
        decimal dependantDeduction,
        decimal additionalDeductions)
    {
        var dependantTotal = dependants * dependantDeduction;
        var taxable = salary - socialInsurance - basicDeduction - dependantTotal - additionalDeductions;
        return Math.Max(taxable, 0m);
    }
}
