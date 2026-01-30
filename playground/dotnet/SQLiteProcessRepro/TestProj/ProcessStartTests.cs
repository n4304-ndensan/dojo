using LibraryA;
using System.Diagnostics;
using System.Text;
using Xunit;

namespace ndensan.framework.uf.privatemodule.real.webapi.hogehoge;
public class ProcessStartTests
{
    [Fact]
    public void Start_ProcessA_From_TestBin_Should_Show_SqliteMissingError()
    {
        // Arrange: We'll assume the Test project's bin/Debug/net8.0 is the working dir.
        var workingDir = Environment.CurrentDirectory; // when test runs, this is test project's bin output dir

        // Copy ProcessA.dll and LibraryA's runtime files into this working dir to simulate scenario.
        // In an automated scenario you'd publish ProcessA and copy native libs. Here we explain steps in comments.
        var liba = new SqliteUser();
        var dllPath = typeof(SqliteUser).Assembly.Location;
        var className = "LibraryA.SqliteUser";
        var methodName = "GetSqliteVersion";

        var psi = new ProcessStartInfo
        {
            FileName = "dotnet",
            WorkingDirectory = workingDir,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            UseShellExecute = false,
            StandardOutputEncoding = Encoding.UTF8,
            StandardErrorEncoding = Encoding.UTF8
        };

        psi.ArgumentList.Add("../../../../ProcessA/bin/Debug/net8.0/ProcessA.dll");
        psi.ArgumentList.Add(dllPath);
        psi.ArgumentList.Add(className);
        psi.ArgumentList.Add(methodName);

        using var proc = Process.Start(psi)!;

        proc.OutputDataReceived += (_, e) => { if (e.Data != null) Console.WriteLine(e.Data); };
        proc.ErrorDataReceived += (_, e) => { if (e.Data != null) Console.Error.WriteLine(e.Data); };

        proc.BeginOutputReadLine();
        proc.BeginErrorReadLine();

        proc.WaitForExit();

        // We expect the process to fail with exit code 1 due to missing native sqlite dependency because ProcessA lacks package reference.
        Assert.Equal(1, proc.ExitCode);
    }

    [Fact]
    public void Start_ProcessA_From_TestBin_Should_Show_Success()
    {
        // project fileにProcessAの参照を追加してビルドした場合のテスト 
        var workingDir = Environment.CurrentDirectory; // when test runs, this is test project's bin output dir

        // Copy ProcessA.dll and LibraryA's runtime files into this working dir to simulate scenario.
        // In an automated scenario you'd publish ProcessA and copy native libs. Here we explain steps in comments.
        var liba = new SqliteUser();
        var dllPath = typeof(SqliteUser).Assembly.Location;
        var className = "LibraryA.SqliteUser";
        var methodName = "GetSqliteVersion";

        var psi = new ProcessStartInfo
        {
            FileName = "dotnet",
            WorkingDirectory = workingDir,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            UseShellExecute = false,
            StandardOutputEncoding = Encoding.UTF8,
            StandardErrorEncoding = Encoding.UTF8
        };

        psi.ArgumentList.Add("./ProcessA.dll");
        psi.ArgumentList.Add(dllPath);
        psi.ArgumentList.Add(className);
        psi.ArgumentList.Add(methodName);

        using var proc = Process.Start(psi)!;

        proc.OutputDataReceived += (_, e) => { if (e.Data != null) Console.WriteLine(e.Data); };
        proc.ErrorDataReceived += (_, e) => { if (e.Data != null) Console.Error.WriteLine(e.Data); };

        proc.BeginOutputReadLine();
        proc.BeginErrorReadLine();

        proc.WaitForExit();

        // We expect the process to fail with exit code 1 due to missing native sqlite dependency because ProcessA lacks package reference.
        Assert.Equal(0, proc.ExitCode);
    }

    [Fact]
    public void Start_LibraryA_Should_Show_Success()
    {
        var liba = new SqliteUser();
        var ver = liba.GetSqliteVersion();
        Console.WriteLine($"SQLite version: {ver}");
    }
}
