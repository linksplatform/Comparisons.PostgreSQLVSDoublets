using System;
using System.Diagnostics;
using System.Net.Http;
using System.Threading.Tasks;

namespace GqlComparison
{
    class Program
    {
        static async Task Main(string[] args)
        {
            var hasuraUrl = args.Length > 0 ? args[0] : "http://localhost:8080/v1/graphql";
            var doubletsUrl = args.Length > 1 ? args[1] : "http://localhost:5000/graphql";
            var iterations = 100;

            Console.WriteLine("GraphQL Comparison Benchmark");
            Console.WriteLine($"Hasura endpoint: {hasuraUrl}");
            Console.WriteLine($"Doublets endpoint: {doubletsUrl}");
            Console.WriteLine($"Iterations: {iterations}\n");

            var client = new HttpClient();

            // Warm-up
            await Benchmark(client, hasuraUrl, "query { __typename }", 1);
            await Benchmark(client, doubletsUrl, "query { __typename }", 1);

            // Benchmark queries
            var queries = new (string Name, string Gql)[]
            {
                ("SimpleQuery", "query { __typename }"),
                ("CreateDoublet", "mutation { createDoublet(source: 1, target: 2) { id } }"),
                ("GetDoublets", "query { doublets { id source target } }")
            };

            foreach (var (name, gql) in queries)
            {
                Console.WriteLine($"Query: {name}");
                var hasuraTime = await Benchmark(client, hasuraUrl, gql, iterations);
                var doubletsTime = await Benchmark(client, doubletsUrl, gql, iterations);
                Console.WriteLine($"Hasura avg: {hasuraTime.TotalMilliseconds / iterations:F3} ms");
                Console.WriteLine($"Doublets avg: {doubletsTime.TotalMilliseconds / iterations:F3} ms");
                Console.WriteLine();
            }
        }

        static async Task<TimeSpan> Benchmark(HttpClient client, string url, string gql, int iterations)
        {
            var content = new StringContent(
                $"{{\"query\":\"{gql.Replace("\"", "\\\"")}\"}}",
                System.Text.Encoding.UTF8,
                "application/json");

            var sw = Stopwatch.StartNew();
            for (int i = 0; i < iterations; i++)
            {
                var response = await client.PostAsync(url, content);
                response.EnsureSuccessStatusCode();
            }
            sw.Stop();
            return sw.Elapsed;
        }
    }
}
