using LayeredArchitecture.BusinessControl;
using LayeredArchitecture.Models;
using LayeredArchitecture.ViewCommon;

namespace LayeredArchitecture.ViewControl;

/// <summary>
/// ダッシュボード画面向けに複数ユースケースを束ねる ViewControl。
/// </summary>
public sealed class DashboardControl
{
    private readonly IncomeTaxUseCase _incomeTaxUseCase;
    private readonly HistoryUseCase _historyUseCase;
    private readonly DeductionSettingUseCase _settingUseCase;
    private readonly TaxReviewUseCase _reviewUseCase;

    public DashboardControl(
        IncomeTaxUseCase incomeTaxUseCase,
        HistoryUseCase historyUseCase,
        DeductionSettingUseCase settingUseCase,
        TaxReviewUseCase reviewUseCase)
    {
        _incomeTaxUseCase = incomeTaxUseCase;
        _historyUseCase = historyUseCase;
        _settingUseCase = settingUseCase;
        _reviewUseCase = reviewUseCase;
    }

    public async Task<IncomeTaxResultDto> RunQuickCalcAsync(IncomeTaxInputDto input)
    {
        if (!InputValidationHelpers.IsValid(input))
        {
            throw new ArgumentException("ダッシュボード入力が不足しています。");
        }

        var result = await _incomeTaxUseCase.CalculateAsync(input);
        IncomeTaxState.LatestResult = result;
        return result;
    }

    public async Task<DashboardSnapshot> LoadSnapshotAsync()
    {
        var settingTask = _settingUseCase.GetAsync();
        var historyTask = _historyUseCase.GetAllAsync();
        var reviewTask = _reviewUseCase.SummarizeAsync();

        await Task.WhenAll(settingTask, historyTask, reviewTask);

        return new DashboardSnapshot(
            await settingTask,
            IncomeTaxState.LatestResult,
            await reviewTask,
            await historyTask);
    }
}

public sealed record DashboardSnapshot(
    DeductionSettingDto Setting,
    IncomeTaxResultDto? LatestResult,
    TaxReviewSummaryDto Summary,
    IReadOnlyList<HistoryDto> Histories);
