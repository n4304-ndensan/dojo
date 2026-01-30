using System.Globalization;

class Program
{
    static void Main()
    {
        var cultures = new[]
        {
            CultureInfo.InvariantCulture,
            CultureInfo.GetCultureInfo("en-US"),
            CultureInfo.GetCultureInfo("ja-JP"),
            CultureInfo.GetCultureInfo("tr-TR"), // ToUpper/ToLower差分が有名
            CultureInfo.GetCultureInfo("vi-VN"), // ベトナム オフショア
            CultureInfo.GetCultureInfo("zh-CN"), // 中国（簡体）オフショア
        };

        Console.OutputEncoding = System.Text.Encoding.UTF8;

        Console.WriteLine("=== Culture-dependent ToUpper/ToLower & Compare test (.NET 8) ===");
        Console.WriteLine();

        // 英字（トルコ語で差が出る）
        var latinSamples = new[] { "i", "I", "id", "ID", "file", "FILE" };

        // 日本語（かな/カナ、全角/半角）※日本語・英語・数字のみ
        // ひらがな: あ, かたかな: ア, 半角ｶﾀｶﾅ: ｱ
        var jpPairs = new[]
        {
            ("あ", "ア"),
            ("ア", "ｱ"),
            ("あ1", "ア1"),
        };

        // 数字混じり（文化差は小さいが、Ordinal/CurrentCulture 比較確認用）
        var numericPairs = new[]
        {
            ("2", "10"),
            ("A2", "A10"),
        };

        foreach (var ci in cultures)
        {
            // このプロセス（スレッド）のカルチャを切替
            CultureInfo.CurrentCulture = ci;
            CultureInfo.CurrentUICulture = ci;

            Console.WriteLine($"--- Culture: {ci.Name,-10}  DisplayName: {ci.DisplayName} ---");
            Console.WriteLine($"DecimalSep='{ci.NumberFormat.NumberDecimalSeparator}', DateSep sample='{DateTime.Now.ToString("d", ci)}'");
            Console.WriteLine();

            // 1) ToUpper/ToLower
            Console.WriteLine("[1] ToUpper/ToLower");
            foreach (var s in latinSamples)
            {
                var upper = s.ToUpper();                 // CurrentCulture 依存
                var lower = s.ToLower();                 // CurrentCulture 依存
                var invU  = s.ToUpperInvariant();        // 文化非依存
                var invL  = s.ToLowerInvariant();        // 文化非依存

                Console.WriteLine($"  '{s}' -> Upper='{upper}', Lower='{lower}' | UpperInv='{invU}', LowerInv='{invL}'");
            }
            Console.WriteLine();

            // 2) Compare: string.Compare (CurrentCulture) と Ordinal の違い
            Console.WriteLine("[2] string.Compare differences");
            CompareBoth(ci, "i", "I");
            CompareBoth(ci, "id", "ID");
            CompareBoth(ci, "FILE", "file");
            Console.WriteLine();

            // 3) CompareInfo.Compare + CompareOptions（日本語で差を作りやすい）
            Console.WriteLine("[3] CompareInfo.Compare with options (Japanese samples)");
            var cmp = ci.CompareInfo;

            foreach (var (a, b) in jpPairs)
            {
                // デフォルト（カルチャの標準的な比較）
                int def = cmp.Compare(a, b, CompareOptions.None);

                // かな種別無視（ひらがな/カタカナを同一視）
                int kana = cmp.Compare(a, b, CompareOptions.IgnoreKanaType);

                // 全角/半角無視
                int width = cmp.Compare(a, b, CompareOptions.IgnoreWidth);

                // 両方無視
                int both = cmp.Compare(a, b, CompareOptions.IgnoreKanaType | CompareOptions.IgnoreWidth);

                Console.WriteLine($"  a='{a}', b='{b}'");
                Console.WriteLine($"    None={def}, IgnoreKanaType={kana}, IgnoreWidth={width}, IgnoreKana+Width={both}");
            }
            Console.WriteLine();

            // 4) 数字混じり：Ordinal と CurrentCulture の比較（参考）
            Console.WriteLine("[4] Numeric-ish compare (Ordinal vs CurrentCulture) just to observe");
            foreach (var (a, b) in numericPairs)
            {
                int ord = string.Compare(a, b, StringComparison.Ordinal);
                int cur = string.Compare(a, b, ignoreCase: false, culture: ci);
                Console.WriteLine($"  a='{a}', b='{b}' -> Ordinal={ord}, CurrentCulture({ci.Name})={cur}");
            }

            Console.WriteLine();
        }

        Console.WriteLine("=== Notes ===");
        Console.WriteLine("- 'tr-TR' では 'i'/'I' の Upper/Lower が Invariant と変わるのが確認ポイント。");
        Console.WriteLine("- 日本語は CompareOptions（IgnoreKanaType/IgnoreWidth）を使うと、比較結果が変わるのが確認しやすい。");
        Console.WriteLine("- キー/ID 比較は Ordinal / OrdinalIgnoreCase を使うのが定石（文化差を排除）。");
    }

    static void CompareBoth(CultureInfo ci, string a, string b)
    {
        int curCase = string.Compare(a, b, ignoreCase: false, culture: ci); // CurrentCulture(指定)
        int curIg   = string.Compare(a, b, ignoreCase: true,  culture: ci); // カルチャ依存の ignoreCase
        int ord     = string.Compare(a, b, StringComparison.Ordinal);
        int ordIg   = string.Compare(a, b, StringComparison.OrdinalIgnoreCase);

        Console.WriteLine($"  a='{a}', b='{b}'");
        Console.WriteLine($"    CurrentCulture(case)={curCase}, CurrentCulture(ignoreCase)={curIg}");
        Console.WriteLine($"    Ordinal={ord}, OrdinalIgnoreCase={ordIg}");
    }
}
