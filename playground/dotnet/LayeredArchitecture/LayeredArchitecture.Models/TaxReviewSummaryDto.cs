namespace LayeredArchitecture.Models;

public class TaxReviewSummaryDto
{
    public decimal TotalTax { get; set; }
    public decimal TotalTaxableIncome { get; set; }
    public decimal AverageAppliedRate { get; set; }
    public DateTime? LastCalculatedAt { get; set; }
}
