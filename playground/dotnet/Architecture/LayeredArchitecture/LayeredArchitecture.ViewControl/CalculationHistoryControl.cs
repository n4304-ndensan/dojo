using LayeredArchitecture.BusinessControl;
using LayeredArchitecture.Models;
using LayeredArchitecture.ViewCommon;

namespace LayeredArchitecture.ViewControl;

/// <summary>
/// 履歴画面の UI ロジック。
/// </summary>
public sealed class CalculationHistoryControl
{
    private readonly HistoryUseCase _useCase;

    public CalculationHistoryControl(HistoryUseCase useCase)
    {
        _useCase = useCase;
    }

    public Task<IReadOnlyList<HistoryDto>> GetAllAsync() => _useCase.GetAllAsync();

    public string Describe(HistoryDto history)
    {
        return $"{history.Timestamp:yyyy/MM/dd HH:mm} | 課税所得 {FormatHelpers.Currency(history.TaxableIncome)} → 税額 {FormatHelpers.Currency(history.TaxAmount)}";
    }
}
