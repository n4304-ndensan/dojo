using Microsoft.AspNetCore.Components;
using Microsoft.AspNetCore.Components.Web;
using LayeredArchitecture.Web.Data;

using LayeredArchitecture.Business;
using LayeredArchitecture.Business.Repository;
using LayeredArchitecture.BusinessControl;
using LayeredArchitecture.View;
using LayeredArchitecture.ViewControl;

var builder = WebApplication.CreateBuilder(args);

// Razor/Blazor
builder.Services.AddRazorPages();
builder.Services.AddServerSideBlazor();

// DI: repositories
builder.Services.AddSingleton<DeductionRepository>();
builder.Services.AddSingleton<IncomeTaxRateRepository>();
builder.Services.AddSingleton<HistoryRepository>();

// DI: business services
builder.Services.AddSingleton<DeductionService>();
builder.Services.AddSingleton<IncomeTaxService>();
builder.Services.AddSingleton<HistoryService>();

// DI: use cases
builder.Services.AddSingleton<IncomeTaxUseCase>();
builder.Services.AddSingleton<DeductionSettingUseCase>();
builder.Services.AddSingleton<HistoryUseCase>();
builder.Services.AddSingleton<TaxReviewUseCase>();

// DI: view controls
builder.Services.AddScoped<IncomeTaxInputControl>();
builder.Services.AddScoped<IncomeTaxResultControl>();
builder.Services.AddScoped<DeductionSettingControl>();
builder.Services.AddScoped<CalculationHistoryControl>();
builder.Services.AddScoped<DashboardControl>();

var app = builder.Build();

if (!app.Environment.IsDevelopment())
{
    app.UseExceptionHandler("/Error");
    app.UseHsts();
}

app.UseHttpsRedirection();
app.UseStaticFiles();
app.UseRouting();

app.MapBlazorHub();
app.MapFallbackToPage("/_Host");

app.Run();
