using System;
using System.Threading.Tasks;
using LinksPlatform.Data.Doublets;
using LinksPlatform.Data.Doublets.Gql;

namespace DoubletsBenchmark
{
    class Program
    {
        static async Task Main(string[] args)
        {
            if (args.Length > 0 && args[0] == "--init")
            {
                await InitializeDoublets();
                Console.WriteLine("Doublets data initialized.");
            }
            else
            {
                Console.WriteLine("Use --init to initialize data.");
            }
        }

        static async Task InitializeDoublets()
        {
            // Initialize doublets storage and create schema/data
            // This is a placeholder - implement actual Doublets initialization here.
            // The Doublets GQL library should be used to set up the graph.
            // For simplicity, we assume it's done via configuration.
            
            Console.WriteLine("Doublets initialization would go here.");
        }
    }
}
