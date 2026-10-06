using Comparisons.PostgreSQLVSDoublets;

var background = ReadSize("BENCHMARK_BACKGROUND_LINKS", 1000);
var links = ReadSize("BENCHMARK_LINKS", 100);
if (links > background)
{
    throw new ArgumentException("BENCHMARK_LINKS must not exceed BENCHMARK_BACKGROUND_LINKS");
}
var backend = Environment.GetEnvironmentVariable("BENCHMARK_BACKEND") ?? "all";
if (backend is not ("all" or "psql" or "doublets"))
{
    throw new ArgumentException($"Unknown BENCHMARK_BACKEND: {backend}");
}
var connection = Environment.GetEnvironmentVariable("NPGSQL_CONNECTION") ??
    "Host=localhost;Port=5432;Username=postgres;Password=postgres;Database=postgres";
var variants = new List<string>();
if (backend is "all" or "psql")
{
    variants.AddRange(["PSQL_NonTransaction", "PSQL_Transaction"]);
}
if (backend is "all" or "doublets")
{
    variants.AddRange(DoubletsLinks.Variants);
}
var directory = Path.Combine(Path.GetTempPath(), "postgres-doublets-" + Guid.NewGuid());
Directory.CreateDirectory(directory);
try
{
    foreach (var operation in Benchmarks.Operations)
    {
        foreach (var variant in variants)
        {
            using IBenchedLinks store = variant.StartsWith("PSQL", StringComparison.Ordinal)
                ? new PostgresLinks(connection, variant == "PSQL_Transaction")
                : DoubletsLinks.Open(variant, directory);
            var estimate = Harness.Measure(variant.StartsWith("PSQL", StringComparison.Ordinal) ? Sampling.PostgreSQL : Sampling.Doublets, () => Benchmarks.Iteration(store, operation, background, links));
            Console.WriteLine(estimate.Bencher(operation, variant));
        }
    }
}
finally
{
    Directory.Delete(directory, true);
}

static int ReadSize(string key, int fallback)
{
    var text = Environment.GetEnvironmentVariable(key);
    if (text is null) return fallback;
    if (!int.TryParse(text, out var value) || value <= 0)
    {
        throw new ArgumentException($"{key} must be a positive integer");
    }
    return value;
}
