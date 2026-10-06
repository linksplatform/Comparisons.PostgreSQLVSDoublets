# Comparisons.PostgreSQLVSDoublets

PostgreSQL and LinksPlatform Doublets compared on the same basic link operations in Rust and C#. A doublet link has an ID, a source and a target. See [the links theory](https://habr.com/ru/articles/895896) for the representation.

Each language runs four Doublets stores (united/split, in RAM/memory-mapped files) and two PostgreSQL modes (autocommit/explicit transaction). PostgreSQL uses a `bigint` identity primary key and separate B-tree indexes on source and target. Doublets runs inside the benchmark process; PostgreSQL uses a local server and includes driver and network costs. Memory-mapped Doublets writes use the OS page cache without an explicit disk flush; their durability differs from PostgreSQL commits.

## Operations

- **Create**: create `BENCHMARK_LINKS` point links (`id = source = target`).
- **Update**: change the last `BENCHMARK_LINKS` background points to `(0, 0)` and back, timing both updates.
- **Delete**: prepare `BENCHMARK_LINKS` additional points, then delete those points in reverse order.
- **Each All**: enumerate all background links once (`[*, *, *]`).
- **Each Identity**: enumerate `[id, *, *]` for each background point.
- **Each Concrete**: enumerate `[*, source, target]` for each background point.
- **Each Outgoing**: enumerate `[*, source, *]` for each background point.
- **Each Incoming**: enumerate `[*, *, target]` for each background point.

Both languages recreate `BENCHMARK_BACKGROUND_LINKS` points before each iteration and clear the store afterwards. Background creation, delete preparation, table/index setup and cleanup are excluded from the measured time. In transaction mode the timed SQL operations run inside an explicit transaction; its begin and commit happen outside the timed interval. Thus these numbers measure operation batches, rather than committed transactions or individual links. Indexed reads perform one query per background point, independently of `BENCHMARK_LINKS`.

Both harnesses use one second of warm-up, ten flat samples and a one-second measurement target. Iteration counts are calibrated using total wall time including preparation, as Criterion 0.4 does for `iter_custom`; reported samples include only timed operations. The report shows medians and compares each Doublets median with the fastest PostgreSQL median. Medians within 5%, or overlapping ± one standard deviation ranges, are labelled `≈ same`. Other cells show `× faster` or `× slower`. Linear charts give very short bars a minimum visible width; log charts use the measured values.

## Results

Every generated table records the server and library versions, CPU, UTC date and source Actions run. CI runs on separate runners for each language, backend and size. PRs use 100 background / 10 active links. Default-branch runs also measure 1,000 background / 100 active links. A report job assembles all tables and linear/log charts, uploads them for PR review, and commits results after default-branch pushes. Changes only to this README or the charts do not start benchmarks.

<!-- results:start -->
### Rust

_Updated results will be recorded after the first run of the new workflow._

### C#

_Updated results will be recorded after the first run of the new workflow._
<!-- results:end -->

Results depend on workload, server configuration, versions and runner hardware. Use the provenance and sizes above each table when comparing runs.

## Running locally

Install stable Rust, .NET 10, Python 3.11+ and PostgreSQL. The CI server image is pinned to `postgres:17.6`. Use a disposable database: the benchmarks create and drop a table named `links`, and correctness tests reset that table.

```bash
docker run -d --name comparison-postgres -p 5432:5432 \
  -e POSTGRES_PASSWORD=postgres postgres:17.6
export POSTGRES_CONNECTION='host=localhost port=5432 user=postgres password=postgres dbname=postgres'
export NPGSQL_CONNECTION='Host=localhost;Port=5432;Username=postgres;Password=postgres;Database=postgres'
export POSTGRES_VERSION=17.6
export BENCHMARK_BACKGROUND_LINKS=100
export BENCHMARK_LINKS=10
python3 -m pip install -r scripts/requirements.txt

# Report regressions and backend correctness.
python3 -m unittest discover -s scripts -v
cargo fmt --manifest-path rust/Cargo.toml -- --check
cargo clippy --locked --manifest-path rust/Cargo.toml --all-targets --all-features -- -D warnings
cargo test --locked --manifest-path rust/Cargo.toml -- --include-ignored
dotnet restore csharp/PostgreSQLVSDoublets.slnx --locked-mode
dotnet format csharp/PostgreSQLVSDoublets.slnx --verify-no-changes --no-restore
dotnet build csharp/PostgreSQLVSDoublets.slnx -c Release --no-restore --warnaserror
dotnet test csharp/PostgreSQLVSDoublets.slnx -c Release --no-build --blame-hang-timeout 2m

# Run backends sequentially to avoid CPU contention.
mkdir -p results
for language in rust csharp; do
  for backend in psql doublets; do
    export BENCHMARK_BACKEND="$backend"
    output="results/$language-$BENCHMARK_BACKGROUND_LINKS-$backend.txt"
    python3 scripts/benchmark_header.py "$language" "$backend" > "$output"
    if [ "$language" = rust ]; then
      (cd rust && cargo bench --locked --bench bench -- --output-format bencher --noplot) >> "$output"
    else
      dotnet run --project csharp/PostgreSQLVSDoublets -c Release --no-build >> "$output"
    fi
  done
done
python3 scripts/benchmark_report.py results --readme README.md --charts Docs
```

`rust/out.py` remains an entry point to the same report generator and accepts the same arguments. Output files must be named `<rust|csharp>-<background>-<psql|doublets>.txt`; a report requires both complete backends for each language and size it publishes. The C++ prototype and its dormant workflow have been removed.

| Variable | Default | Meaning |
| --- | --- | --- |
| `BENCHMARK_BACKGROUND_LINKS` | `1000` | Background point links per iteration; must be positive. |
| `BENCHMARK_LINKS` | `100` | Active links for writes; must be positive and no greater than the background. |
| `BENCHMARK_BACKEND` | `all` | `psql`, `doublets` or `all`; select one for isolated measurements. |
| `POSTGRES_CONNECTION` | `user=postgres dbname=postgres password=postgres host=localhost port=5432` | Rust connection string in postgres/libpq format, including custom hosts, ports and credentials. |
| `NPGSQL_CONNECTION` | `Host=localhost;Port=5432;Username=postgres;Password=postgres;Database=postgres` | C# Npgsql connection string. |
| `POSTGRES_VERSION` | `unknown version` | Server version recorded by the header script; CI reads `SHOW server_version`. |

Rust PostgreSQL tests are ignored unless `--include-ignored` is supplied. C# PostgreSQL tests are explicitly skipped without `NPGSQL_CONNECTION`; CI configures both connections and runs all variants. Report tracing is off by default; set `DEBUG = True` in `scripts/benchmark_report.py` when investigating parsing.
