namespace LayeredArchitecture.Models;

public class DeductionSettingDto
{
    public decimal BasicDeduction { get; set; } = 480_000m;
    public decimal DependantDeduction { get; set; } = 380_000m;
    public decimal SocialInsuranceRate { get; set; } = 0m;
    public string LastUpdatedBy { get; set; } = "system";
    public DateTime LastUpdatedAt { get; set; } = DateTime.Now;
}
