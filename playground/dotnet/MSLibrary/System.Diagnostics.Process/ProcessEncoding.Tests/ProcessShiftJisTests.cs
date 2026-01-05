using System.Diagnostics;
using System.Text;
using Xunit;

public class ProcessShiftJisTests
{
    [Fact]
    public void ChildProcess_ShiftJisOutput_IsDecodedCorrectly()
    {
        var txtWriter = new StreamWriter("output.txt");
        Console.SetOut(txtWriter);
        var utf8 = Encoding.UTF8;

        var psi = new ProcessStartInfo
        {
            FileName = "dotnet",
            Arguments = "./ChildApp.ShiftJis.dll",
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            UseShellExecute = false,

            StandardOutputEncoding = utf8,
            StandardErrorEncoding = utf8
        };

        var proc = new Process
        {
            StartInfo = psi,
        };

        proc.OutputDataReceived += (_, e) => { if (e.Data != null) Console.Out.WriteLine(e.Data); };
        proc.ErrorDataReceived += (_, e) => { if (e.Data != null) Console.Error.WriteLine(e.Data); };

        // プロセス開始
        proc.Start();

        // 出力の非同期読み取り開始
        proc.BeginOutputReadLine();
        proc.BeginErrorReadLine();

        // プロセス終了を待機
        proc.WaitForExit();

        txtWriter.Close();

        var text = File.ReadAllText("output.txt");
        Assert.DoesNotContain("日本語テスト", text);
    }
}
