using System.Diagnostics;
using System.Text;
using Xunit.Abstractions;

namespace dojo.playground.dotnet.System.Runtime.InteropServices.DllImportAttribute;
public class DllImportSearchFromCurrentDirectoryTests
{
    ITestOutputHelper _output;
    public DllImportSearchFromCurrentDirectoryTests(ITestOutputHelper output)
    {
        _output = output;
        // Arrange
        // カレントディレクトリにmy.dllをコピーしておく
        // 実行前にmy.csprojをdotnet publish -c Release -r win-x64しておくこと。
        var currentDir = Environment.CurrentDirectory;
        File.Copy(
            @"..\..\..\..\my\bin\Release\net8.0\win-x64\publish\my.dll",
            Path.Combine(currentDir, "my.dll"),
            overwrite: true
        );
    }

    [Fact]
    public void DllImport_mydll_FromCurrentDirectory_Works()
    {
        // Act
        var result = Program.Invoke();

        // Assert
        Assert.Equal(5, result);
    }

    [Fact]
    public void DllImport_ResolvesNativeLibrary_FromCurrentDirectory_WhenWorkingDirectoryIsSet()
    {
        // Arrange
        var currentDir = Environment.CurrentDirectory;
        var psi = new ProcessStartInfo
        {
            FileName = "dotnet",
            Arguments = "../../../../DllImportInvoker/bin/Debug/net8.0/DllImportInvoker.dll",
            WorkingDirectory = currentDir,   // ★ 重要
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            UseShellExecute = false,
            StandardOutputEncoding = Encoding.UTF8,
            StandardErrorEncoding = Encoding.UTF8
        };
        using var proc = Process.Start(psi)!;

        proc.OutputDataReceived += (_, e) => { if (e.Data != null) Console.Out.WriteLine(e.Data); };
        proc.ErrorDataReceived += (_, e) => { if (e.Data != null) Console.Error.WriteLine(e.Data); };

        // 出力の非同期読み取り開始
        proc.BeginOutputReadLine();
        proc.BeginErrorReadLine();

        // Act
        proc.WaitForExit();

        // Assert
        Assert.Equal(0, proc.ExitCode);
    }
}
