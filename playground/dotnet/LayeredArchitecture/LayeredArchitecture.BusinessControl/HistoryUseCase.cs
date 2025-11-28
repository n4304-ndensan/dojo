using LayeredArchitecture.Business;
using LayeredArchitecture.Models;

namespace LayeredArchitecture.BusinessControl;

/// <summary>
/// 履歴参照ユースケース。
/// </summary>
public sealed class HistoryUseCase
{
    private readonly HistoryService _historyService;

    public HistoryUseCase(HistoryService historyService)
    {
        _historyService = historyService;
    }

    public Task<IReadOnlyList<HistoryDto>> GetAllAsync() => _historyService.GetAllAsync();
}
