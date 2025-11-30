using LayeredArchitecture.Business.Repository;
using LayeredArchitecture.BusinessCommon;
using LayeredArchitecture.Models;

namespace LayeredArchitecture.Business;

/// <summary>
/// 控除計算に関するサービス。
/// </summary>
public sealed class DeductionService
{
    private readonly DeductionRepository _repository;

    public DeductionService(DeductionRepository repository)
    {
        _repository = repository;
    }

    public async Task<decimal> CalculateTaxableIncomeAsync(IncomeTaxInputDto input)
    {
        var setting = await _repository.GetAsync();
        return DeductionRules.CalculateTaxableIncome(
            input.Salary,
            input.SocialInsurance,
            input.Dependants,
            setting.BasicDeduction,
            setting.DependantDeduction,
            input.AdditionalDeductions);
    }

    public Task<DeductionSettingDto> GetSettingAsync() => _repository.GetAsync();

    public Task SaveSettingAsync(DeductionSettingDto setting) => _repository.SaveAsync(setting);
}
