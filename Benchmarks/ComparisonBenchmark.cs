using BenchmarkDotNet.Attributes;
using GraphQL;
using GraphQL.Client.Http;
using GraphQL.Client.Serializer.SystemTextJson;
using System;
using System.Threading.Tasks;

namespace Comparisons.PostgreSQLVSDoublets.Benchmarks
{
    [MemoryDiagnoser]
    public class ComparisonBenchmark
    {
        private GraphQLHttpClient _doubletsClient;
        private GraphQLHttpClient _hasuraClient;

        [GlobalSetup]
        public async Task Setup()
        {
            _doubletsClient = new GraphQLHttpClient("http://localhost:5000/graphql", new SystemTextJsonSerializer());
            _hasuraClient = new GraphQLHttpClient("http://localhost:8080/v1/graphql", new SystemTextJsonSerializer());

            // Initialize data
            await InsertData(_doubletsClient, "Doublets");
            await InsertData(_hasuraClient, "Hasura");
        }

        private async Task InsertData(GraphQLHttpClient client, string source)
        {
            var request = new GraphQLRequest
            {
                Query = @"
                    mutation {
                        createLink(source: ""TestNode"", target: ""TestNode"", value: ""1"") {
                            id
                        }
                    }
                "
            };
            await client.SendMutationAsync<dynamic>(request);
        }

        [Benchmark]
        public async Task<double> DoubletsGetAllLinks()
        {
            var request = new GraphQLRequest
            {
                Query = @"
                    query {
                        links {
                            id
                            source
                            target
                            value
                        }
                    }
                "
            };
            var response = await _doubletsClient.SendQueryAsync<dynamic>(request);
            return Convert.ToDouble(response.Data.links.Count);
        }

        [Benchmark]
        public async Task<double> HasuraGetAllLinks()
        {
            var request = new GraphQLRequest
            {
                Query = @"
                    query {
                        links {
                            id
                            source
                            target
                            value
                        }
                    }
                "
            };
            var response = await _hasuraClient.SendQueryAsync<dynamic>(request);
            return Convert.ToDouble(response.Data.links.Count);
        }

        [GlobalCleanup]
        public void Cleanup()
        {
            _doubletsClient?.Dispose();
            _hasuraClient?.Dispose();
        }
    }
}
