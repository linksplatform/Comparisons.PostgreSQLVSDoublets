using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Hosting;
using Microsoft.Extensions.DependencyInjection;
using Platform.Data.Doublets;
using Platform.Data.Doublets.Gql;
using System;

namespace Comparisons.PostgreSQLVSDoublets.Setup
{
    public static class DoubletsServer
    {
        public static void Start()
        {
            var host = new WebHostBuilder()
                .UseKestrel()
                .UseUrls("http://localhost:5000")
                .ConfigureServices(services =>
                {
                    services.AddSingleton<ILinks>(sp =>
                    {
                        var memory = new Platform.Memory.HeapMemory();
                        return new Links<ulong>(memory);
                    });
                    services.AddGraphQLServer()
                        .AddQueryType<GqlQueries>()
                        .AddMutationType<GqlMutations>();
                })
                .Configure(app =>
                {
                    app.UseRouting();
                    app.UseEndpoints(endpoints =>
                    {
                        endpoints.MapGraphQL();
                    });
                })
                .Build();

            host.Run();
        }
    }
}
