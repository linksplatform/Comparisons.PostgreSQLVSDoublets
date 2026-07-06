using System;
using System.Threading.Tasks;
using Npgsql;

namespace HasuraBenchmark
{
    class Program
    {
        static async Task Main(string[] args)
        {
            if (args.Length > 0 && args[0] == "--init")
            {
                await InitializeDatabase();
                Console.WriteLine("Database initialized.");
            }
            else
            {
                Console.WriteLine("Use --init to initialize the database.");
            }
        }

        static async Task InitializeDatabase()
        {
            var connectionString = "Host=localhost;Port=5432;Database=hasura_bench;Username=bench;Password=benchpass";
            using var conn = new NpgsqlConnection(connectionString);
            await conn.OpenAsync();
            
            // Create table
            var createTableCmd = new NpgsqlCommand(@"
                CREATE TABLE IF NOT EXISTS ""users"" (
                    ""id"" SERIAL PRIMARY KEY,
                    ""name"" TEXT NOT NULL,
                    ""email"" TEXT NOT NULL
                );
            ", conn);
            await createTableCmd.ExecuteNonQueryAsync();

            // Insert sample data (1000 rows)
            for (int i = 0; i < 1000; i++)
            {
                var insertCmd = new NpgsqlCommand(
                    "INSERT INTO \"users\" (\"name\", \"email\") VALUES (@name, @email)",
                    conn);
                insertCmd.Parameters.AddWithValue("@name", $"User{i}");
                insertCmd.Parameters.AddWithValue("@email", $"user{i}@example.com");
                await insertCmd.ExecuteNonQueryAsync();
            }

            Console.WriteLine("Inserted 1000 users.");
        }
    }
}
