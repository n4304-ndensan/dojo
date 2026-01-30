using LayeredArchitecture.BusinessControl;
using LayeredArchitecture.Models;
using LayeredArchitecture.ViewCommon;

namespace LayeredArchitecture.ViewControl;

/// <summary>
/// 控除設定画面の UI ロジック。
/// </summary>
public sealed class DeductionSettingControl
{
    private readonly DeductionSettingUseCase _useCase;

    public DeductionSettingControl(DeductionSettingUseCase useCase)
    {
        _useCase = useCase;
    }

    public Task<DeductionSettingDto> LoadAsync() => _useCase.GetAsync();

    public Task SaveAsync(DeductionSettingDto dto, string userName)
    {
        if (!InputValidationHelpers.IsNonNegative(dto.BasicDeduction) ||
            !InputValidationHelpers.IsNonNegative(dto.DependantDeduction))
        {
            throw new ArgumentException("控除額は0以上を指定してください。");
        }

        return _useCase.SaveAsync(dto, userName);
    }
}
