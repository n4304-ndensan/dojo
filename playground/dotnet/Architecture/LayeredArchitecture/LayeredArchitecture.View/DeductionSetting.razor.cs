using LayeredArchitecture.Models;
using LayeredArchitecture.ViewControl;
using Microsoft.AspNetCore.Components;

namespace LayeredArchitecture.View;

public partial class DeductionSetting : ComponentBase
{
    private DeductionSettingDto _setting = new();
    private string _message = string.Empty;

    [Inject] public DeductionSettingControl Control { get; set; } = default!;

    protected override async Task OnInitializedAsync()
    {
        _setting = await Control.LoadAsync();
    }

    private async Task OnSaveAsync()
    {
        await Control.SaveAsync(_setting, "demo-user");
        _message = "保存しました。";
    }
}
