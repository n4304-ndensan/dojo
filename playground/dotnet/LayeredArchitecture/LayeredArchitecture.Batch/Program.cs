using LayeredArchitecture.Business;
using LayeredArchitecture.Business.Repository;
using LayeredArchitecture.BusinessControl;
using LayeredArchitecture.Models;

// バッチプロセス: 同じ BusinessControl を呼び出し、夜間処理の一括計算を行うデモ。
// 実務想定: ファイルやDBから入力を読み込み、共通ユースケースを呼び出すだけに徹する。

var deductionRepo = new DeductionRepository();
var rateRepo = new IncomeTaxRateRepository();
var historyRepo = new HistoryRepository();

var deductionService = new DeductionService(deductionRepo);
var incomeTaxService = new IncomeTaxService(rateRepo);
var historyService = new HistoryService(historyRepo);

var useCase = new IncomeTaxUseCase(deductionService, incomeTaxService, historyService);
var historyUseCase = new HistoryUseCase(historyService);

Console.WriteLine("=== バッチ開始: IncomeTaxUseCase を利用して一括計算 ===");

// サンプル: 外部入力を模した複数レコード
var inputs = new[]
{
    new IncomeTaxInputDto { Salary = 5_000_000m, SocialInsurance = 800_000m, Dependants = 0, AdditionalDeductions = 50_000m },
    new IncomeTaxInputDto { Salary = 7_200_000m, SocialInsurance = 1_000_000m, Dependants = 2, AdditionalDeductions = 0m },
    new IncomeTaxInputDto { Salary = 4_500_000m, SocialInsurance = 700_000m, Dependants = 1, AdditionalDeductions = 120_000m }
};

foreach (var input in inputs)
{
    var result = await useCase.CalculateAsync(input);
    Console.WriteLine($"[calculated] Salary={input.Salary}, Taxable={result.TaxableIncome}, Rate={result.TaxRate:P1}, Tax={result.TaxAmount}");
}

Console.WriteLine("=== バッチ終了: 履歴を確認（同じユースケース経由） ===");
var histories = await historyUseCase.GetAllAsync();
foreach (var h in histories)
{
    Console.WriteLine($"[history] {h.Timestamp:yyyy/MM/dd HH:mm} | Taxable={h.TaxableIncome}, Tax={h.TaxAmount}");
}

Console.WriteLine("※ バッチもアプリも BusinessControl の IncomeTaxUseCase を共通利用。ロジック重複なし。");
