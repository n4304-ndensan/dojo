namespace LayeredArchitecture.BusinessCommon;

/// <summary>
/// 期間・年度に関するルール。
/// </summary>
public static class PeriodRules
{
    public static bool IsInFiscalYear(DateTime target, int fiscalYear)
    {
        var start = new DateTime(fiscalYear, 1, 1);
        var end = new DateTime(fiscalYear, 12, 31, 23, 59, 59);
        return target >= start && target <= end;
    }
}
