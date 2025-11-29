using LayeredArchitecture.Models;
using LayeredArchitecture.ViewControl;
using Microsoft.AspNetCore.Components;

namespace LayeredArchitecture.View;

public partial class IncomeTaxResult : ComponentBase
{
    private IncomeTaxResultDto? _result;
    private string _summary = string.Empty;

    [Inject] public IncomeTaxResultControl Control { get; set; } = default!;

    protected override void OnInitialized()
    {
        _result = Control.GetLatestResult();
        if (_result is not null)
        {
            _summary = Control.BuildSummary(_result);
        }
    }
}
