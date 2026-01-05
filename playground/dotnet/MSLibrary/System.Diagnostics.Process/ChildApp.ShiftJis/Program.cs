using System.Text;

namespace dojo.playground.dotnet.ChildApp.ShiftJis;
class Program
{
    static void Main()
    {
        // 明示的に Shift-JIS（CP932）
        Encoding.RegisterProvider(CodePagesEncodingProvider.Instance);
        Console.OutputEncoding = Encoding.GetEncoding("shift_jis");

        Console.WriteLine("日本語テスト：子プロセスはShift-JIS");
    }
}
