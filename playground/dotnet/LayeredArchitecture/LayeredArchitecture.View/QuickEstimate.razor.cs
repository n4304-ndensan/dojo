using LayeredArchitecture.Models;
using LayeredArchitecture.ViewControl;
using Microsoft.AspNetCore.Components;

namespace LayeredArchitecture.View;

public partial class QuickEstimate : ComponentBase
{
    private readonly IncomeTaxInputDto _input = new();
    private string _message = string.Empty;

    [Inject] public IncomeTaxInputControl Control { get; set; } = default!;

    private async Task OnCalculateAsync()
    {
        try
        {
            var result = await Control.CalculateAsync(_input);
            _message = $"課税所得: {result.TaxableIncome} / 税額: {result.TaxAmount}";
        }
        catch (Exception ex)
        {
            _message = ex.Message;
        }
    }
}
