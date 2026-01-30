using System.Runtime.InteropServices;
using System.Runtime.CompilerServices;

public static class MyExports
{
    // ★ C ABI で export される
    [UnmanagedCallersOnly(
        EntryPoint = "add",
        CallConvs = new[] { typeof(CallConvCdecl) }
    )]
    public static int Add(int a, int b)
    {
        return a + b;
    }
}

