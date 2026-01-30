using LayeredArchitecture.Models;
using LayeredArchitecture.ViewControl;
using Microsoft.AspNetCore.Components;

namespace LayeredArchitecture.View;

public partial class CalculationHistory : ComponentBase
{
    private IReadOnlyList<HistoryDto>? _histories;

    [Inject] public CalculationHistoryControl Control { get; set; } = default!;

    protected override async Task OnInitializedAsync()
    {
        _histories = await Control.GetAllAsync();
    }
}
