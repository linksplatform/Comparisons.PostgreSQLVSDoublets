using Npgsql;
using System;
using System.Threading.Tasks;

namespace Comparisons.PostgreSQLVSDoublets.Setup
{
    public static class DatabaseInitializer
    {
        public static async Task Initialize()
        {
            using var conn = new NpgsqlConnection("Host=localhost;Port=5432;Database=postgres;Username=postgres;Password=postgrespassword");
            await conn.OpenAsync();

            var createTableCommand = @"
                CREATE TABLE IF NOT EXISTS links (
                    id SERIAL PRIMARY KEY,
                    source BIGINT NOT NULL,
                    target BIGINT NOT NULL,
                    value BIGINT NOT NULL
                );
            ";
            using var cmd = new NpgsqlCommand(createTableCommand, conn);
            await cmd.ExecuteNonQueryAsync();

            // Insert test data
            for (int i = 0; i < 100; i++)
            {
                var insertCommand = @"INSERT INTO links (source, target, value) VALUES (1, 2, 1);";
                using var insertCmd = new NpgsqlCommand(insertCommand, conn);
                await insertCmd.ExecuteNonQueryAsync();
            }

            Console.WriteLine("Database initialized with test data.");
        }
    }
}
