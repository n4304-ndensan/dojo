using LayeredArchitecture.Models;
using LayeredArchitecture.ViewCommon;
using LayeredArchitecture.ViewControl;
using Microsoft.AspNetCore.Components;

namespace LayeredArchitecture.View;

public partial class IncomeTaxInput : ComponentBase
{
    private readonly IncomeTaxInputDto _input = new();
    private string _errorMessage = string.Empty;
    private string _infoMessage = "所得税の計算を開始できます。";

    [Inject] public IncomeTaxInputControl Control { get; set; } = default!;
    [Inject] public NavigationManager Navigation { get; set; } = default!;

    protected override void OnInitialized()
    {
        _input.Dependants = 0;
        base.OnInitialized();
    }

    private async Task OnCalculateAsync()
    {
        try
        {
            var result = await Control.CalculateAsync(_input);
            _errorMessage = string.Empty;
            _infoMessage = $"課税所得: {FormatHelpers.Currency(result.TaxableIncome)} / 税額: {FormatHelpers.Currency(result.TaxAmount)}";
            Navigation.NavigateTo("/income-tax/result");
        }
        catch (Exception ex)
        {
            _errorMessage = UiMessageHelpers.ToUserMessage(ex);
        }
    }
}
