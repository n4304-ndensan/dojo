using LayeredArchitecture.Models;

namespace LayeredArchitecture.ViewCommon;

/// <summary>
/// 入力系の共通ヘルパー。
/// </summary>
public static class InputValidationHelpers
{
    public static bool HasRequired(decimal value) => value != 0m;

    public static bool IsPositive(decimal value) => value > 0m;

    public static bool IsNonNegative(decimal value) => value >= 0m;

    public static bool IsValid(IncomeTaxInputDto dto)
    {
        return IsPositive(dto.Salary)
               && IsNonNegative(dto.SocialInsurance)
               && dto.Dependants >= 0;
    }
}
