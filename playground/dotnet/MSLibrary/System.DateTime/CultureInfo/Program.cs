using System;
using System.Globalization;
using System.Threading;

class Program
{
    static void Main()
    {
        // 比較用に固定日時（好きに変えてOK）
        var dt = new DateTime(2026, 1, 30, 21, 5, 7);

        var cultures = new[]
        {
            "ja-JP", // 日本
            "en-US", // 米国
            "en-GB", // 英国
            "de-DE", // ドイツ
            "fr-FR", // フランス
            "it-IT", // イタリア
            "es-ES", // スペイン
            "sv-SE", // スウェーデン
            "ru-RU", // ロシア
            "zh-CN", // 中国(簡体)
            "ko-KR", // 韓国
            "ar-SA", // サウジアラビア（数字・暦も特徴出やすい）
            "th-TH", // タイ（年の扱いが特徴出ることあり）
            "vi-VN", // ベトナム
        };

        Console.WriteLine("=== CurrentCulture 影響（引数なし ToString） ===");
        Console.WriteLine($"Default Culture: {CultureInfo.CurrentCulture.Name}");
        Console.WriteLine($"dt.ToString(): {dt.ToString()}");

        foreach (var name in cultures)
        {
            var ci = CultureInfo.GetCultureInfo(name);

            // このスレッドのカルチャを切り替える
            CultureInfo.CurrentCulture = ci;
            CultureInfo.CurrentUICulture = ci;

            Console.WriteLine($"[{name}]");
            Console.WriteLine($"  dt.ToString():   {dt.ToString()}");
            Console.WriteLine($"  dt.ToString(\"F\"): {dt.ToString("F", ci)}");  // 長い日付 + 長い時刻
            Console.WriteLine($"  dt.ToString(\"d\"): {dt.ToString("d", ci)}");  // 短い日付
            Console.WriteLine($"  dt.ToString(\"T\"): {dt.ToString("T", ci)}");  // 長い時刻
            Console.WriteLine();
        }

        Console.WriteLine("=== カルチャ非依存の例 ===");
        Console.WriteLine($"Invariant: {dt.ToString(CultureInfo.InvariantCulture)}");
        Console.WriteLine($"ISO 8601(o): {dt.ToString("o")}");
    }
}
