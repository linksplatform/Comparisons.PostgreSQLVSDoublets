using BenchmarkDotNet.Running;

namespace Comparisons.PostgreSQLVSDoublets
{
    class Program
    {
        static void Main(string[] args)
        {
            BenchmarkRunner.Run<Benchmark>();
        }
    }
}
