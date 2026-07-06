using GraphQL;
using GraphQL.Client.Http;
using GraphQL.Client.Serializer.SystemTextJson;

namespace Comparisons.PostgreSQLVSDoublets
{
    public class DoubletsBench
    {
        private readonly GraphQLHttpClient _client;
        private readonly GraphQLRequest _request;

        public DoubletsBench(string endpoint)
        {
            _client = new GraphQLHttpClient(endpoint, new SystemTextJsonSerializer());
            _request = new GraphQLRequest
            {
                Query = @"
                    query {
                        links(limit: 10) {
                            id
                            source
                            target
                        }
                    }"
            };
        }

        public async Task ExecuteQuery()
        {
            var response = await _client.SendQueryAsync<dynamic>(_request);
            if (response.Errors != null && response.Errors.Any())
                throw new Exception("Doublets query failed");
        }
    }
}