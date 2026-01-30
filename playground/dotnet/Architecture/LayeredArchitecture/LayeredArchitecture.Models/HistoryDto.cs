namespace LayeredArchitecture.Models;

public class HistoryDto
{
    public decimal Salary { get; set; }
    public decimal TaxableIncome { get; set; }
    public decimal TaxRate { get; set; }
    public decimal TaxAmount { get; set; }
    public DateTime Timestamp { get; set; } = DateTime.Now;
}
