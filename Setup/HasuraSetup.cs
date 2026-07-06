using System;
using System.Diagnostics;
using System.IO;

namespace Comparisons.PostgreSQLVSDoublets.Setup
{
    public static class HasuraSetup
    {
        public static void Start()
        {
            // Assumes docker-compose.yml with Hasura + PostgreSQL is present
            var process = new Process
            {
                StartInfo = new ProcessStartInfo
                {
                    FileName = "docker-compose",
                    Arguments = "-f docker-compose.hasura.yml up -d",
                    UseShellExecute = false,
                    RedirectStandardOutput = true,
                    CreateNoWindow = true
                }
            };
            process.Start();
            process.WaitForExit();

            // Wait for Hasura to be ready
            System.Threading.Thread.Sleep(5000);
        }

        public static void Stop()
        {
            var process = new Process
            {
                StartInfo = new ProcessStartInfo
                {
                    FileName = "docker-compose",
                    Arguments = "-f docker-compose.hasura.yml down",
                    UseShellExecute = false,
                    RedirectStandardOutput = true,
                    CreateNoWindow = true
                }
            };
            process.Start();
            process.WaitForExit();
        }
    }
}
