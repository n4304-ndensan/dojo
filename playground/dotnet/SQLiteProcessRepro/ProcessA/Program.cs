using LibraryB;

class Program
{
    static int Main(string[] args)
    {
        try
        {
            Console.WriteLine("ProcessA: started.");
            Console.WriteLine("ProcessA: args:");
            Console.WriteLine(string.Join(", ", args));
            // 第一引数dll 第二引数クラス名　第三引数メソッド名
            // LibraryA.dll SqliteUser GetSqliteVersion
            var dllPath = args.Length > 0 ? args[0] : "LibraryA.dll";
            var className = args.Length > 1 ? args[1] : "LibraryA.SqliteUser";
            var methodName = args.Length > 2 ? args[2] : "GetSqliteVersion";
            Console.WriteLine("ProcessA: calling LibraryA.GetSqliteVersion()...");
            ISqliteUser sqliteUser;
            var assembly = System.Reflection.Assembly.LoadFrom(dllPath);
            var type = assembly.GetType(className);
            sqliteUser = (ISqliteUser)Activator.CreateInstance(type!)!;
            var ver = sqliteUser.GetSqliteVersion();
            Console.WriteLine($"SQLite version: {ver}");
            return 0;
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine("ProcessA: Exception occurred:\n" + ex);
            return 1;
        }
    }
}
