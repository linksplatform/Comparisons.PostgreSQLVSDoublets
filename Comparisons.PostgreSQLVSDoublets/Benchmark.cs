using System;
using System.Net.Http;
using System.Threading.Tasks;
using BenchmarkDotNet.Attributes;
using BenchmarkDotNet.Engines;

namespace Comparisons.PostgreSQLVSDoublets
{
    [SimpleJob(RunStrategy.ColdStart, targetCount: 10)]
    [MinColumn, MaxColumn, MeanColumn, MedianColumn]
    public class Benchmark
    {
        private static readonly HttpClient httpClient = new HttpClient();
        private static readonly string hasuraEndpoint = Config.HasuraEndpoint;
        private static readonly string doubletsEndpoint = Config.DoubletsEndpoint;
        private static readonly string graphqlQuery = @"{ __typename }"; // Simple introspection query

        [Benchmark]
        public async Task<HttpResponseMessage> HasuraGraphQL()
        {
            var request = new HttpRequestMessage(HttpMethod.Post, hasuraEndpoint);
            request.Content = new StringContent(
                $"{{\"query\": \"{graphqlQuery}\"}}",
                System.Text.Encoding.UTF8,
                "application/json");
            request.Headers.Add("X-Hasura-Admin-Secret", Config.HasuraAdminSecret);
            return await httpClient.SendAsync(request);
        }

        [Benchmark]
        public async Task<HttpResponseMessage> DoubletsGql()
        {
            var request = new HttpRequestMessage(HttpMethod.Post, doubletsEndpoint);
            request.Content = new StringContent(
                $"{{\"query\": \"{graphqlQuery}\"}}",
                System.Text.Encoding.UTF8,
                "application/json");
            return await httpClient.SendAsync(request);
        }
    }
}
