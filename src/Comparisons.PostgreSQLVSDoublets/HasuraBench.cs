using GraphQL;
using GraphQL.Client.Http;
using GraphQL.Client.Serializer.SystemTextJson;

namespace Comparisons.PostgreSQLVSDoublets
{
    public class HasuraBench
    {
        private readonly GraphQLHttpClient _client;
        private readonly GraphQLRequest _request;

        public HasuraBench(string endpoint)
        {
            _client = new GraphQLHttpClient(endpoint, new SystemTextJsonSerializer());
            _request = new GraphQLRequest
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
        }

        public async Task ExecuteQuery()
        {
            var response = await _client.SendQueryAsync<dynamic>(_request);
            if (response.Errors != null && response.Errors.Any())
                throw new Exception("Hasura query failed");
        }
    }
}