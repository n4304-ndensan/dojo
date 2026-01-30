namespace LayeredArchitecture.ViewCommon;

/// <summary>
/// UI メッセージ生成ヘルパー。
/// </summary>
public static class UiMessageHelpers
{
    public static string Required(string label) => $"{label} を入力してください。";

    public static string ToUserMessage(Exception ex) => ex.Message;
}
