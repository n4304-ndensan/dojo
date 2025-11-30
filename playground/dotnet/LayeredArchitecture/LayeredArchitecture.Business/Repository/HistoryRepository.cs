using LayeredArchitecture.Models;

namespace LayeredArchitecture.Business.Repository;

/// <summary>
/// 計算履歴の保管/参照（インメモリ）。
/// </summary>
public sealed class HistoryRepository
{
    private readonly List<HistoryDto> _histories = new();

    public Task SaveAsync(HistoryDto history)
    {
        _histories.Add(history);
        return Task.CompletedTask;
    }

    public Task<IReadOnlyList<HistoryDto>> GetAllAsync()
    {
        return Task.FromResult((IReadOnlyList<HistoryDto>)_histories.ToList());
    }
}
