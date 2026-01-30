using LayeredArchitecture.Business;
using LayeredArchitecture.Business.Repository;
using LayeredArchitecture.BusinessControl;
using LayeredArchitecture.Models;

// コンソールでユースケースオーケストレーションをざっくり検証するデモ。
var deductionRepo = new DeductionRepository();
var rateRepo = new IncomeTaxRateRepository();
var historyRepo = new HistoryRepository();

var deductionService = new DeductionService(deductionRepo);
var incomeTaxService = new IncomeTaxService(rateRepo);
var historyService = new HistoryService(historyRepo);

var incomeTaxUseCase = new IncomeTaxUseCase(deductionService, incomeTaxService, historyService);
var historyUseCase = new HistoryUseCase(historyService);

var sampleInput = new IncomeTaxInputDto
{
    Salary = 6_000_000m,
    SocialInsurance = 900_000m,
    Dependants = 1,
    AdditionalDeductions = 100_000m
};

var result = await incomeTaxUseCase.CalculateAsync(sampleInput);
Console.WriteLine($"課税所得: {result.TaxableIncome}, 税率: {result.TaxRate:P1}, 税額: {result.TaxAmount}");

var histories = await historyUseCase.GetAllAsync();
foreach (var h in histories)
{
    Console.WriteLine($"履歴: {h.Timestamp:yyyy/MM/dd HH:mm} - 税額 {h.TaxAmount}");
}
