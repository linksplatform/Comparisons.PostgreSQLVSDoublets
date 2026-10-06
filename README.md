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
<!-- markdownlint-disable MD013 MD024 -->

### Rust

#### Rust: 100 background links, 10 links per iteration

_Median time of one iteration with 10 links. PostgreSQL 17.6 (Debian 17.6-2.pgdg13+1) through postgres 0.19.7; doublets 0.3.0. CPU AMD EPYC 7763 64-Core Processor (PostgreSQL) and AMD EPYC 9V45 96-Core Processor (Doublets), [GitHub Actions run](https://github.com/linksplatform/Comparisons.PostgreSQLVSDoublets/actions/runs/37548090422) on 2026-10-06T23:46:07+00:00._

| Operation | Doublets United Volatile | Doublets United NonVolatile | Doublets Split Volatile | Doublets Split NonVolatile | PostgreSQL NonTransaction | PostgreSQL Transaction |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Create | 514 ns (21,300× faster) | 512 ns (21,400× faster) | 421 ns (26,000× faster) | 416 ns (26,300× faster) | 13.8 ms | **10.9 ms** |
| Update | 1.69 µs (8,440× faster) | 1.62 µs (8,760× faster) | 583 ns (24,400× faster) | 583 ns (24,400× faster) | 17.5 ms | **14.2 ms** |
| Delete | 729 ns (5,000× faster) | 736 ns (4,960× faster) | 771 ns (4,730× faster) | 796 ns (4,580× faster) | 5.02 ms | **3.65 ms** |
| Each All | 36 ns (10,300× faster) | 35 ns (10,600× faster) | 143 ns (2,600× faster) | 142 ns (2,620× faster) | 390 µs | **371 µs** |
| Each Identity | 2.42 µs (14,600× faster) | 2.39 µs (14,800× faster) | 2.44 µs (14,500× faster) | 2.44 µs (14,500× faster) | 36.5 ms | **35.4 ms** |
| Each Concrete | 2.85 µs (12,800× faster) | 2.83 µs (12,900× faster) | 3.12 µs (11,700× faster) | 3.13 µs (11,700× faster) | 37.7 ms | **36.6 ms** |
| Each Outgoing | 3.39 µs (10,400× faster) | 3.34 µs (10,600× faster) | 2.72 µs (13,000× faster) | 2.73 µs (13,000× faster) | 36.3 ms | **35.4 ms** |
| Each Incoming | 3.42 µs (10,400× faster) | 3.37 µs (10,600× faster) | 2.76 µs (12,900× faster) | 2.75 µs (13,000× faster) | 36.9 ms | **35.6 ms** |

![Rust, 100 background links, 10 links per iteration, linear scale](Docs/bench_rust_100.png)
![Rust, 100 background links, 10 links per iteration, log scale](Docs/bench_rust_log_scale_100.png)

### C#

#### C#: 100 background links, 10 links per iteration

_Median time of one iteration with 10 links. PostgreSQL 17.6 (Debian 17.6-2.pgdg13+1) through Npgsql 10.0.0; Platform.Data.Doublets 0.18.1. CPU AMD EPYC 7763 64-Core Processor (PostgreSQL) and Intel(R) Xeon(R) 6973P-C (Doublets), [GitHub Actions run](https://github.com/linksplatform/Comparisons.PostgreSQLVSDoublets/actions/runs/37548090422) on 2026-10-06T23:45:47+00:00._

| Operation | Doublets United Volatile | Doublets United NonVolatile | Doublets Split Volatile | Doublets Split NonVolatile | PostgreSQL NonTransaction | PostgreSQL Transaction |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Create | 2.79 µs (2,600× faster) | 2.87 µs (2,530× faster) | 418 ns (17,400× faster) | 1.24 µs (5,830× faster) | 9.85 ms | **7.26 ms** |
| Update | 3.39 µs (2,920× faster) | 3.39 µs (2,920× faster) | 235 ns (42,100× faster) | 223 ns (44,400× faster) | 12.4 ms | **9.9 ms** |
| Delete | 1.33 µs (1,860× faster) | 1.41 µs (1,760× faster) | 525 ns (4,710× faster) | 1.17 µs (2,110× faster) | 3.66 ms | **2.47 ms** |
| Each All | 1.13 µs (245× faster) | 1.12 µs (248× faster) | 906 ns (306× faster) | 908 ns (305× faster) | **277 µs** | 284 µs |
| Each Identity | 606 ns (39,500× faster) | 619 ns (38,700× faster) | 815 ns (29,400× faster) | 789 ns (30,400× faster) | 24.4 ms | **24 ms** |
| Each Concrete | 705 ns (36,000× faster) | 842 ns (30,100× faster) | 763 ns (33,300× faster) | 756 ns (33,600× faster) | 25.8 ms | **25.4 ms** |
| Each Outgoing | 1.08 µs (22,300× faster) | 1.12 µs (21,500× faster) | 911 ns (26,500× faster) | 977 ns (24,700× faster) | 24.9 ms | **24.1 ms** |
| Each Incoming | 2.1 µs (11,500× faster) | 2.21 µs (11,000× faster) | 844 ns (28,800× faster) | 840 ns (28,900× faster) | 24.7 ms | **24.3 ms** |

![C#, 100 background links, 10 links per iteration, linear scale](Docs/bench_csharp_100.png)
![C#, 100 background links, 10 links per iteration, log scale](Docs/bench_csharp_log_scale_100.png)

<!-- markdownlint-restore -->
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
