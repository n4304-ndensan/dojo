using LayeredArchitecture.BusinessControl;
using LayeredArchitecture.Models;
using LayeredArchitecture.ViewCommon;

namespace LayeredArchitecture.ViewControl;

/// <summary>
/// 入力画面の UI ロジック。
/// </summary>
public sealed class IncomeTaxInputControl
{
    private readonly IncomeTaxUseCase _useCase;

    public IncomeTaxInputControl(IncomeTaxUseCase useCase)
    {
        _useCase = useCase;
    }

    public async Task<IncomeTaxResultDto> CalculateAsync(IncomeTaxInputDto input)
    {
        if (!InputValidationHelpers.IsValid(input))
        {
            throw new ArgumentException(UiMessageHelpers.Required("入力値"));
        }

        var result = await _useCase.CalculateAsync(input);
        IncomeTaxState.LatestResult = result;
        return result;
    }
}
