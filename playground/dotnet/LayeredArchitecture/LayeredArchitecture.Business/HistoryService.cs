using LayeredArchitecture.Business.Repository;
using LayeredArchitecture.Models;

namespace LayeredArchitecture.Business;

/// <summary>
/// 計算履歴管理サービス。
/// </summary>
public sealed class HistoryService
{
    private readonly HistoryRepository _repository;

    public HistoryService(HistoryRepository repository)
    {
        _repository = repository;
    }

    public Task SaveAsync(HistoryDto history) => _repository.SaveAsync(history);

    public Task<IReadOnlyList<HistoryDto>> GetAllAsync() => _repository.GetAllAsync();
}
