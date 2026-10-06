using System.Diagnostics;

namespace Comparisons.PostgreSQLVSDoublets;

/// <summary>Matches the operations and timed boundaries in rust/benches/benchmarks.</summary>
public static class Benchmarks
{
    public static readonly string[] Operations =
        ["Create", "Update", "Delete", "Each_All", "Each_Identity", "Each_Concrete", "Each_Outgoing", "Each_Incoming"];

    public static TimeSpan Iteration(IBenchedLinks store, string operation, int background, int links)
    {
        var elapsed = TimeSpan.Zero;
        void Timed(Action action)
        {
            var started = Stopwatch.GetTimestamp();
            action();
            elapsed += Stopwatch.GetElapsedTime(started);
        }
        static void Visit(Link link) { }
        store.Fork(background);
        try
        {
            switch (operation)
            {
                case "Create":
                    for (var i = 0; i < links; i++)
                    {
                        Timed(() => store.CreatePoint());
                    }
                    break;
                case "Update":
                    for (var id = (ulong)(background - links + 1); id <= (ulong)background; id++)
                    {
                        Timed(() => store.Update(id, 0, 0));
                        Timed(() => store.Update(id, id, id));
                    }
                    break;
                case "Delete":
                    for (var i = 0; i < links; i++)
                    {
                        store.CreatePoint();
                    }
                    for (var id = (ulong)(background + links); id > (ulong)background; id--)
                    {
                        Timed(() => store.Delete(id));
                    }
                    break;
                case "Each_All":
                    Timed(() => store.Each(store.Any, store.Any, store.Any, Visit));
                    break;
                default:
                    for (ulong id = 1; id <= (ulong)background; id++)
                    {
                        var query = operation switch
                        {
                            "Each_Identity" => (id, store.Any, store.Any),
                            "Each_Concrete" => (store.Any, id, id),
                            "Each_Outgoing" => (store.Any, id, store.Any),
                            "Each_Incoming" => (store.Any, store.Any, id),
                            _ => throw new ArgumentException($"Unknown operation {operation}"),
                        };
                        Timed(() => store.Each(query.Item1, query.Item2, query.Item3, Visit));
                    }
                    break;
            }
            return elapsed;
        }
        finally
        {
            store.Unfork();
        }
    }
}
