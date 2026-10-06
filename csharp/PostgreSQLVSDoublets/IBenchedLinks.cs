namespace Comparisons.PostgreSQLVSDoublets;

public readonly record struct Link(ulong Id, ulong Source, ulong Target);

public interface IBenchedLinks : IDisposable
{
    ulong Any { get; }
    void Fork(int backgroundLinks);
    void Unfork();
    ulong CreatePoint();
    void Update(ulong id, ulong source, ulong target);
    void Delete(ulong id);
    void Each(ulong id, ulong source, ulong target, Action<Link> visit);
    ulong Count();
}
