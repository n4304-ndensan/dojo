namespace LayeredArchitecture.ViewCommon;

/// <summary>
/// 表示整形ヘルパー。
/// </summary>
public static class FormatHelpers
{
    public static string Currency(decimal value) => value.ToString("C0");

    public static string Percent(decimal value) => $"{value:P1}";
}
