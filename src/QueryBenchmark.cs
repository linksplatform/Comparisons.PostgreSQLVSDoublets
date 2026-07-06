using BenchmarkDotNet.Attributes;
using System.Net.Http;
using System.Text;
using System.Threading.Tasks;

namespace Comparisons.PostgreSQLVSDoublets.Benchmarks
{
    [MemoryDiagnoser]
    [RankColumn]
    public class QueryBenchmark
    {
        private static readonly HttpClient httpClient = new();
        private const string HasuraEndpoint = "http://localhost:8080/v1/graphql";
        private const string DoubletsEndpoint = "http://localhost:5000/graphql";
        private const string Query = @"
            query {
                users(limit: 10) {
                    id
                    name
                    email
                }
            }";

        [Benchmark(Baseline = true)]
        public async Task<string> HasuraQuery()
        {
            var content = new StringContent(
                "{\"query\":\"" + Query.Replace("\"", "\\\"").Replace("\n", "\\n") + "\"}",
                Encoding.UTF8,
                "application/json"
            );
            var response = await httpClient.PostAsync(HasuraEndpoint, content);
            return await response.Content.ReadAsStringAsync();
        }

        [Benchmark]
        public async Task<string> DoubletsQuery()
        {
            var content = new StringContent(
                "{\"query\":\"" + Query.Replace("\"", "\\\"").Replace("\n", "\\n") + "\"}",
                Encoding.UTF8,
                "application/json"
            );
            var response = await httpClient.PostAsync(DoubletsEndpoint, content);
            return await response.Content.ReadAsStringAsync();
        }
    }
}