using Npgsql;

namespace Comparisons.PostgreSQLVSDoublets;

/// <summary>The schema and statement sequence used by the Rust PostgreSQL adapter.</summary>
public sealed class PostgresLinks : IBenchedLinks
{
    private readonly NpgsqlDataSource _source;
    private readonly NpgsqlConnection _connection;
    private readonly bool _transactional;
    private NpgsqlTransaction? _transaction;
    public ulong Any => ulong.MaxValue;

    public PostgresLinks(string connection, bool transactional)
    {
        _source = NpgsqlDataSource.Create(connection);
        _connection = _source.OpenConnection();
        _transactional = transactional;
    }

    private NpgsqlCommand Command(string sql, params ulong[] parameters)
    {
        var command = new NpgsqlCommand(sql, _connection, _transaction);
        foreach (var value in parameters)
        {
            command.Parameters.Add(new NpgsqlParameter<long> { TypedValue = checked((long)value) });
        }
        return command;
    }

    private void Execute(string sql, params ulong[] parameters)
    {
        using var command = Command(sql, parameters);
        command.ExecuteNonQuery();
    }

    public void Fork(int backgroundLinks)
    {
        if (_transactional)
        {
            _transaction = _connection.BeginTransaction();
        }
        Execute("CREATE TABLE Links (id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY, from_id bigint, to_id bigint)");
        Execute("CREATE INDEX source ON Links USING btree(from_id)");
        Execute("CREATE INDEX target ON Links USING btree(to_id)");
        for (var i = 0; i < backgroundLinks; i++)
        {
            CreatePoint();
        }
    }

    public void Unfork()
    {
        Execute("DROP TABLE Links");
        _transaction?.Commit();
        _transaction?.Dispose();
        _transaction = null;
    }

    public ulong CreatePoint()
    {
        using var command = Command("INSERT INTO Links(to_id, from_id) VALUES (0, 0) RETURNING id");
        var id = checked((ulong)(long)command.ExecuteScalar()!);
        Update(id, id, id);
        return id;
    }

    public void Update(ulong id, ulong source, ulong target)
    {
        using (var previous = Command("SELECT * FROM Links WHERE id = $1", id))
        using (var reader = previous.ExecuteReader())
        {
            if (!reader.Read())
            {
                throw new InvalidOperationException($"Link {id} does not exist");
            }
        }
        Execute("UPDATE Links SET from_id = $1, to_id = $2 WHERE id = $3", source, target, id);
    }

    public void Delete(ulong id)
    {
        using var command = Command("DELETE FROM Links WHERE id = $1 RETURNING from_id, to_id", id);
        using var reader = command.ExecuteReader();
        if (!reader.Read())
        {
            throw new InvalidOperationException($"Link {id} does not exist");
        }
    }

    public void Each(ulong id, ulong source, ulong target, Action<Link> visit)
    {
        var conditions = new List<string>();
        var parameters = new List<ulong>();
        foreach (var (column, value) in new[] { ("id", id), ("from_id", source), ("to_id", target) })
        {
            if (value != Any)
            {
                parameters.Add(value);
                conditions.Add($"{column} = ${parameters.Count}");
            }
        }
        var where = conditions.Count == 0 ? "" : " WHERE " + string.Join(" AND ", conditions);
        using var command = Command("SELECT * FROM Links" + where, parameters.ToArray());
        using var reader = command.ExecuteReader();
        while (reader.Read())
        {
            visit(new Link((ulong)reader.GetInt64(0), (ulong)reader.GetInt64(1), (ulong)reader.GetInt64(2)));
        }
    }

    public ulong Count()
    {
        using var command = Command("SELECT COUNT(*) FROM Links");
        return (ulong)(long)command.ExecuteScalar()!;
    }

    public void Dispose()
    {
        _transaction?.Dispose();
        _connection.Dispose();
        _source.Dispose();
    }
}
