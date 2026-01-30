using LibraryB;
using Microsoft.Data.SqlClient;
using SQLitePCL;

namespace LibraryA;

public class SqliteUser : ISqliteUser
{
    // Public method that initializes batteries and opens an in-memory connection to query the SQLite version.
    public string GetSqliteVersion()
    {
        LibraryC.Consoler.WriteLine("LibraryA.SqliteUser: Initializing Batteries...");
        // Initialize native SQLite provided by the bundle package.
        Batteries.Init();

        // Use SQLitePCL.raw to call sqlite3_libversion()
        var v = raw.sqlite3_libversion();
        // inmemory connection string
        var connString = "Data Source=:memory:;";
        var conn = new SqlConnection(connString);
        return v.utf8_to_string();
    }
}
