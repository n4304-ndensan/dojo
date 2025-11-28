using LayeredArchitecture.Models;

namespace LayeredArchitecture.Business.Repository;

/// <summary>
/// 控除設定の保存/取得を扱うリポジトリ（インメモリ）。
/// </summary>
public sealed class DeductionRepository
{
    private DeductionSettingDto _current = new();

    public Task<DeductionSettingDto> GetAsync() => Task.FromResult(_current);

    public Task SaveAsync(DeductionSettingDto setting)
    {
        _current = setting;
        _current.LastUpdatedAt = DateTime.Now;
        return Task.CompletedTask;
    }
}
