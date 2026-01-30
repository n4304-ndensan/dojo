namespace LayeredArchitecture.Models;

public class IncomeTaxInputDto
{
    public decimal Salary { get; set; }
    public decimal SocialInsurance { get; set; }
    public int Dependants { get; set; }
    public decimal AdditionalDeductions { get; set; }
}
