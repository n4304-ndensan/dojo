using LayeredArchitecture.Business;
using LayeredArchitecture.Models;

namespace LayeredArchitecture.BusinessControl;

/// <summary>
/// 控除設定の更新/取得ユースケース。
/// </summary>
public sealed class DeductionSettingUseCase
{
    private readonly DeductionService _deductionService;

    public DeductionSettingUseCase(DeductionService deductionService)
    {
        _deductionService = deductionService;
    }

    public Task<DeductionSettingDto> GetAsync() => _deductionService.GetSettingAsync();

    public async Task SaveAsync(DeductionSettingDto setting, string userName)
    {
        setting.LastUpdatedBy = userName;
        setting.LastUpdatedAt = DateTime.Now;
        await _deductionService.SaveSettingAsync(setting);
    }
}
