using BenchmarkDotNet.Running;

namespace Comparisons.PostgreSQLVSDoublets
{
    public class Program
    {
        public static void Main(string[] args)
        {
            BenchmarkRunner.Run<ComparisonBenchmark>();
        }
    }
}
