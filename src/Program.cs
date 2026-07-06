using BenchmarkDotNet.Running;
using Comparisons.PostgreSQLVSDoublets.Benchmarks;

namespace Comparisons.PostgreSQLVSDoublets
{
    class Program
    {
        static void Main(string[] args)
        {
            BenchmarkRunner.Run<QueryBenchmark>();
        }
    }
}