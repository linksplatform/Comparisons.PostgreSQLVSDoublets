using BenchmarkDotNet.Attributes;
using BenchmarkDotNet.Configs;
using BenchmarkDotNet.Jobs;

namespace Comparisons.PostgreSQLVSDoublets
{
    [MemoryDiagnoser]
    [SimpleJob(RunStrategy.ColdStart, launchCount: 1, warmupCount: 2, iterationCount: 5, invocationCount: 100)]
    public class BenchmarkConfig
    {
        private readonly HasuraBench _hasura;
        private readonly DoubletsBench _doublets;

        public BenchmarkConfig()
        {
            var config = new ConfigurationBuilder()
                .SetBasePath(Directory.GetCurrentDirectory())
                .AddJsonFile("appsettings.json")
                .Build();
            _hasura = new HasuraBench(config["HasuraEndpoint"]!);
            _doublets = new DoubletsBench(config["DoubletsEndpoint"]!);
        }

        [Benchmark(Baseline = true)]
        public async Task HasuraQuery()
        {
            await _hasura.ExecuteQuery();
        }

        [Benchmark]
        public async Task DoubletsQuery()
        {
            await _doublets.ExecuteQuery();
        }
    }
}