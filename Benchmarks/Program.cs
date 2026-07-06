using BenchmarkDotNet.Running;
using System.Threading.Tasks;

namespace Benchmarks
{
    class Program
    {
        static async Task Main(string[] args)
        {
            var summary = BenchmarkRunner.Run<QueryBenchmarks>();
        }
    }
}
