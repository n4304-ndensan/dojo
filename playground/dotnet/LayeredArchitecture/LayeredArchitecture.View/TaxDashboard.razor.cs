using LayeredArchitecture.Models;
using LayeredArchitecture.ViewControl;
using Microsoft.AspNetCore.Components;

namespace LayeredArchitecture.View;

public partial class TaxDashboard : ComponentBase
{
    private readonly IncomeTaxInputDto _input = new();
    private DashboardSnapshot? _snapshot;
    private bool _loading = true;
    private string _message = string.Empty;

    [Inject] public DashboardControl Control { get; set; } = default!;
    [Inject] public CalculationHistoryControl ControlHistory { get; set; } = default!;

    protected override async Task OnInitializedAsync()
    {
        _snapshot = await Control.LoadSnapshotAsync();
        _loading = false;
    }

    private async Task OnQuickCalcAsync()
    {
        try
        {
            var result = await Control.RunQuickCalcAsync(_input);
            _message = $"課税所得: {result.TaxableIncome} / 税額: {result.TaxAmount}";
            _snapshot = await Control.LoadSnapshotAsync();
        }
        catch (Exception ex)
        {
            _message = ex.Message;
        }
    }
}
