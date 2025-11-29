namespace LayeredArchitecture.Models;

public class IncomeTaxResultDto
{
    public decimal TaxableIncome { get; set; }
    public decimal TaxRate { get; set; }
    public decimal TaxAmount { get; set; }
}
