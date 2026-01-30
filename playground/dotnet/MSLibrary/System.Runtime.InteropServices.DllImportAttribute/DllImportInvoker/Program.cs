using System.Runtime.InteropServices;

public static class Program
{
    [DllImport("my", ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
    private static extern int add(int a, int b);

    public static void Main()
    {
        // 10秒待機
        try
        {
            var result = Invoke();

            if (result == 5)
            {
                Console.WriteLine("Success: 2 + 3 = 5");
            }
            else
            {
                Console.WriteLine($"Failure: 2 + 3 != {result}");
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine("Exception occurred:");
            Console.WriteLine(ex.ToString());
        }
    }
    public static int Invoke() => add(2, 3);
}
