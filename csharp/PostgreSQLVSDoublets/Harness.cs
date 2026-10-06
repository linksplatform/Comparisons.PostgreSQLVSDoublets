using System.Diagnostics;
using System.Globalization;

namespace Comparisons.PostgreSQLVSDoublets;

/// <summary>
/// How a benchmark is sampled. The values are the ones of the Rust benchmarks (<c>rust/benches/bench.rs</c>),
/// and the iteration counts are chosen as Criterion 0.4 chooses them, so both languages measure the same way.
/// </summary>
/// <param name="SampleSize">Number of samples.</param>
/// <param name="Flat">Every sample has the same number of iterations (Criterion's flat sampling);
/// otherwise sample <c>i</c> has <c>i × d</c> iterations (linear sampling).</param>
/// <param name="WarmUp">How long the benchmark runs before it is measured.</param>
/// <param name="MeasurementTime">The measured time that all samples take together.</param>
public sealed record Sampling(int SampleSize, bool Flat, TimeSpan WarmUp, TimeSpan MeasurementTime)
{
    /// <summary>One PostgreSQL iteration takes milliseconds, so 10 samples of the same number of iterations already give stable results.</summary>
    public static Sampling PostgreSQL { get; } = new(10, true, TimeSpan.FromSeconds(1), TimeSpan.FromSeconds(1));

    /// <summary>One Doublets iteration takes microseconds, so the same flat sampling is used.</summary>
    public static Sampling Doublets { get; } = new(10, true, TimeSpan.FromSeconds(1), TimeSpan.FromSeconds(1));

    /// <summary>The number of iterations of every sample, for iterations that take <paramref name="nanoseconds"/> each.</summary>
    public long[] IterationCounts(double nanoseconds)
    {
        var measurement = MeasurementTime.TotalNanoseconds;
        if (Flat)
        {
            var iterations = Math.Max(1, (long)Math.Ceiling(measurement / (nanoseconds * SampleSize)));
            return Enumerable.Repeat(iterations, SampleSize).ToArray();
        }
        var totalRuns = (double)SampleSize * (SampleSize + 1) / 2;
        var step = Math.Max(1, (long)Math.Ceiling(measurement / nanoseconds / totalRuns));
        return Enumerable.Range(1, SampleSize).Select(sample => sample * step).ToArray();
    }
}

/// <summary>The median time of one iteration and the standard deviation of the samples, in nanoseconds.</summary>
public readonly record struct Estimate(double Median, double StandardDeviation)
{
    /// <summary>Estimates the time of one iteration from the average iteration time of every sample.</summary>
    public static Estimate Of(IReadOnlyCollection<double> samples)
    {
        var sorted = samples.Order().ToArray();
        var middle = sorted.Length / 2;
        var median = sorted.Length % 2 == 0 ? (sorted[middle - 1] + sorted[middle]) / 2 : sorted[middle];
        var mean = sorted.Average();
        var variance = sorted.Length > 1 ? sorted.Sum(sample => (sample - mean) * (sample - mean)) / (sorted.Length - 1) : 0;
        return new Estimate(median, Math.Sqrt(variance));
    }

    /// <summary>
    /// The line that Criterion prints with <c>--output-format bencher</c>, for example
    /// <c>test Create/PSQL_Transaction ... bench:  44,055,505 ns/iter (+/- 5,345,991)</c>.
    /// </summary>
    public string Bencher(string group, string id) =>
        string.Create(CultureInfo.InvariantCulture,
            $"test {group}/{id} ... bench: {Math.Round(Median),11:N0} ns/iter (+/- {Math.Round(StandardDeviation):N0})");
}

public static class Harness
{
    /// <summary>
    /// Measures <paramref name="iteration"/>, which runs one iteration and returns its measured time:
    /// first warms up for <see cref="Sampling.WarmUp"/>, doubling the number of iterations each round,
    /// then takes <see cref="Sampling.SampleSize"/> samples.
    /// </summary>
    public static Estimate Measure(Sampling sampling, Func<TimeSpan> iteration)
    {
        TimeSpan Run(long iterations)
        {
            var elapsed = TimeSpan.Zero;
            for (long i = 0; i < iterations; i++)
            {
                elapsed += iteration();
            }
            return elapsed;
        }

        var warmUp = TimeSpan.Zero;
        long warmUpIterations = 0;
        for (long iterations = 1; warmUp <= sampling.WarmUp; iterations *= 2)
        {
            var started = Stopwatch.GetTimestamp();
            Run(iterations);
            warmUp += Stopwatch.GetElapsedTime(started);
            warmUpIterations += iterations;
        }
        var counts = sampling.IterationCounts(Math.Max(1, warmUp.TotalNanoseconds / warmUpIterations));
        return Estimate.Of(counts.Select(count => Run(count).TotalNanoseconds / count).ToArray());
    }

}
