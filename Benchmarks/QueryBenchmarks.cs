using BenchmarkDotNet.Attributes;
using GraphQL.Client.Http;
using GraphQL.Client.Serializer.SystemTextJson;
using System;
using System.Net.Http;
using System.Threading.Tasks;

namespace Benchmarks
{
    [MemoryDiagnoser]
    public class QueryBenchmarks
    {
        private GraphQLHttpClient _hasuraClient;
        private GraphQLHttpClient _doubletsClient;

        [GlobalSetup]
        public void Setup()
        {
            _hasuraClient = new GraphQLHttpClient("http://localhost:8080/v1/graphql", new SystemTextJsonSerializer());
            _hasuraClient.HttpClient.DefaultRequestHeaders.Add("x-hasura-admin-secret", ""); // adjust if needed

            // Doublets GQL endpoint (assume running on port 5000)
            _doubletsClient = new GraphQLHttpClient("http://localhost:5000/graphql", new SystemTextJsonSerializer());
        }

        [Benchmark]
        public async Task<string> HasuraQuery()
        {
            var request = new GraphQLHttpRequest
            {
                Query = @"
                    query {
                        users(limit: 10) {
                            id
                            name
                            email
                        }
                    }"
            };
            var response = await _hasuraClient.SendQueryAsync<object>(request);
            return response.Data.ToString();
        }

        [Benchmark]
        public async Task<string> DoubletsQuery()
        {
            var request = new GraphQLHttpRequest
            {
                Query = @"
                    query {
                        get(limit: 10) {
                            id
                            name
                            email
                        }
                    }"
            };
            var response = await _doubletsClient.SendQueryAsync<object>(request);
            return response.Data.ToString();
        }
    }
}
