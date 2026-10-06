using Comparisons.PostgreSQLVSDoublets;
using Xunit;

[assembly: CollectionBehavior(DisableTestParallelization = true)]

public class SameBehaviorTests
{
    private static List<Link> Rows(IBenchedLinks store, ulong id, ulong source, ulong target)
    {
        var rows = new List<Link>();
        store.Each(id, source, target, rows.Add);
        return rows.OrderBy(link => link.Id).ToList();
    }

    private static void Exercise(IBenchedLinks store)
    {
        store.Fork(2);
        try
        {
            var any = store.Any;
            Assert.Equal(2UL, store.Count());
            Assert.Equal([new Link(1, 1, 1), new Link(2, 2, 2)], Rows(store, any, any, any));
            Assert.Equal([new Link(1, 1, 1)], Rows(store, 1, any, any));
            Assert.Empty(Rows(store, 999, any, any));
            foreach (var query in new[] { (any, 2UL, 2UL), (any, 2UL, any), (any, any, 2UL) })
            {
                Assert.Equal([new Link(2, 2, 2)], Rows(store, query.Item1, query.Item2, query.Item3));
            }
            store.Update(2, 0, 0);
            Assert.Equal([new Link(2, 0, 0)], Rows(store, 2, any, any));
            store.Update(2, 2, 2);
            var created = store.CreatePoint();
            Assert.Equal(3UL, created);
            store.Delete(created);
            Assert.Equal(2UL, store.Count());
        }
        finally
        {
            store.Unfork();
        }
        store.Fork(1);
        Assert.Equal([new Link(1, 1, 1)], Rows(store, store.Any, store.Any, store.Any));
        store.Unfork();
    }

    [Theory]
    [InlineData("Doublets_United_Volatile")]
    [InlineData("Doublets_United_NonVolatile")]
    [InlineData("Doublets_Split_Volatile")]
    [InlineData("Doublets_Split_NonVolatile")]
    public void Doublets(string variant)
    {
        var directory = Path.Combine(Path.GetTempPath(), Guid.NewGuid().ToString());
        Directory.CreateDirectory(directory);
        try
        {
            using var store = DoubletsLinks.Open(variant, directory);
            Exercise(store);
            foreach (var operation in Benchmarks.Operations)
            {
                Benchmarks.Iteration(store, operation, 10, 2);
                Assert.Equal(0UL, store.Count());
            }
        }
        finally
        {
            Directory.Delete(directory, true);
        }
    }

    [PostgresTheory]
    [InlineData(false)]
    [InlineData(true)]
    public void Postgres(bool transactional)
    {
        var connection = Environment.GetEnvironmentVariable("NPGSQL_CONNECTION");
        using var store = new PostgresLinks(connection!, transactional);
        Exercise(store);
        foreach (var operation in Benchmarks.Operations)
        {
            Benchmarks.Iteration(store, operation, 10, 2);
        }
    }

    [Fact]
    public void StatisticsAndSampling()
    {
        var estimate = Estimate.Of([1, 2, 3, 4]);
        Assert.Equal(2.5, estimate.Median);
        Assert.Equal(Math.Sqrt(5.0 / 3), estimate.StandardDeviation, 8);
        Assert.Equal(Enumerable.Repeat(10L, 10), new Sampling(10, true, TimeSpan.FromSeconds(1), TimeSpan.FromSeconds(1)).IterationCounts(10_000_000));
        var measured = Harness.Measure(new Sampling(10, true, TimeSpan.FromTicks(1), TimeSpan.FromTicks(1)), () => TimeSpan.FromTicks(2));
        Assert.Equal(200, measured.Median);
        Assert.Equal(0, measured.StandardDeviation);
        Assert.Contains("bench:           3 ns/iter", new Estimate(3, 0).Bencher("Create", "PSQL_Transaction"));
    }
}

public sealed class PostgresTheoryAttribute : TheoryAttribute
{
    public PostgresTheoryAttribute()
    {
        if (Environment.GetEnvironmentVariable("NPGSQL_CONNECTION") is null)
        {
            Skip = "Set NPGSQL_CONNECTION to a disposable PostgreSQL database";
        }
    }
}
