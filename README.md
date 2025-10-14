# Comparisons.PostgreSQLVSDoublets

The comparison between PostgreSQL and LinksPlatform's Doublets (links) on basic database operations with links (create, read, delete, update).
All benchmarks ran with 3000 links in background to increase size of indexes and 1000 are actively created/updated/deleted.

In this particular benchmark we decided not to increase the number of links as PostgreSQL will not be able to handle it at all in timeframe what GitHub Actions limit allows to use for free. Remember that to get accurate result we ran this benchmark multiple times.

## Task

Both databases used to store and retrieve doublet-links representation. To support storage, and all basic CRUD operations that provide Turing completeness for links as in [the links theory](https://habr.com/ru/articles/895896).

## Operations
- **Create** – insert point link (link with id = source = target)
- **Update** – basic link update operation
- **Delete** – basic link delete operation
- **Each All** – take all links matching `[*, *, *]` constraint
- **Each Incoming** – take all links matching `[*, *, target]` constraint
- **Each Outgoing** – take all links matching `[*, source, *]` constraint
- **Each Concrete** – take all links matching `[*, source, target]` constraint
- **Each Identity** – take all links matching `[id, *, *]` constraint

## Results
The results below represent the amount of time (ns) the operation takes per iteration.
- First picture shows time in a pixel scale (for doublets just minimum value is shown, otherwise it will be not present on the graph).
- Second picture shows time in a logarithmic scale (to see diffrence clearly, because it is around 2-3 orders of magnitude).

### Rust
![Image of Rust benchmark (pixel scale)](https://github.com/linksplatform/Comparisons.PostgreSQLVSDoublets/blob/gh-pages/Docs/bench_rust.png?raw=true)
![Image of Rust benchmark (log scale)](https://github.com/linksplatform/Comparisons.PostgreSQLVSDoublets/blob/gh-pages/Docs/bench_rust_log_scale.png?raw=true)

### Raw benchmark results (all numbers are in nanoseconds)

| Operation     | Doublets United Volatile | Doublets United NonVolatile | Doublets Split Volatile | Doublets Split NonVolatile | PSQL NonTransaction | PSQL Transaction |
|---------------|--------------------------|-----------------------------|-------------------------|----------------------------|---------------------|------------------|
| Create        | 98074 (10163.6+ times faster) | 101753 (9796.1+ times faster) | 83158 (11986.6+ times faster) | 83555 (11929.7+ times faster) | 1418444409          | 996785265        |
| Update        | 393717 (3328.3+ times faster) | 395531 (3313.0+ times faster) | 83090 (15770.8+ times faster) | 83222 (15745.8+ times faster) | 1708932179          | 1310397499       |
| Delete        | 193643 (1746.4+ times faster) | 184686 (1831.1+ times faster) | 144773 (2336.0+ times faster) | 144322 (2343.3+ times faster) | 516439248           | 338185472        |
| Each All      | 14991 (110.0+ times faster) | 16263 (101.4+ times faster) | 16860 (97.8+ times faster) | 15970 (103.2+ times faster) | 1717107             | 1648433          |
| Each Identity | 102612 (9614.5+ times faster) | 102701 (9606.2+ times faster) | 101771 (9694.0+ times faster) | 101764 (9694.7+ times faster) | 1030793967          | 986567072        |
| Each Concrete | 232170 (4482.9+ times faster) | 231432 (4497.2+ times faster) | 133134 (7817.6+ times faster) | 132956 (7828.0+ times faster) | 1085583098          | 1040785456       |
| Each Outgoing | 384706 (2590.4+ times faster) | 382143 (2607.8+ times faster) | 120126 (8295.9+ times faster) | 121173 (8224.2+ times faster) | 1048393281          | 996555891        |
| Each Incoming | 386936 (2551.7+ times faster) | 390969 (2525.4+ times faster) | 120659 (8182.9+ times faster) | 120620 (8185.6+ times faster) | 1039836582          | 987344012        |

## Conclusion

As we can see in this comparison, Doublets are from 1746 to 15745 times faster than PostgreSQL in write operations, and from 100 to 9694 times faster in read operations.

To get fresh numbers, please fork the repository and rerun benchmark in GitHub Actions.

---

## Benchmark: Flight Timetable (Airlines Demo)

A new benchmark comparing PostgreSQL 18 and Doublets on realistic airline timetable queries using the [PostgresPro Airlines demo database](https://postgrespro.ru/education/demodb).

### What's Being Tested

This benchmark evaluates both systems on:
- **Complex relational queries**: Multi-table joins with temporal validity checks
- **Large datasets**: 6-month to 1-year flight schedules (~500k flights)
- **Real-world operations**: Airport departures/arrivals, route searches, aggregations
- **Two durability modes**:
  - **Durable** (production-like): Full ACID with WAL
  - **Embedded-like**: WAL-light configuration (similar to embedded databases)

### Queries

The benchmark includes 9 timetable queries:
1. Departures from airport by date
2. Arrivals to airport by date
3. Next available flight on a route
4. Manual join with temporal validity checks
5. Route details with airport information
6. Flight status distribution
7. Busiest routes analysis
8. Flights by date range
9. And more...

See [`sql/10_timetable_queries.sql`](sql/10_timetable_queries.sql) for details.

### Getting Started

#### Prerequisites
- Docker and Docker Compose
- ~10GB free disk space (for 1-year dataset)
- Python 3 (for result analysis)

#### Quick Start

```bash
# 1. Start PostgreSQL 18 with Airlines demo data (6 months)
cd docker
docker compose up -d

# Wait for database to load (~5 minutes)
docker compose logs -f pg

# 2. Run PostgreSQL benchmarks
cd ../bench/pg
./run.sh durable 6m 10      # Durable mode
./run.sh embedded 6m 10     # Embedded-like mode

# 3. Run Doublets benchmarks (TODO: implement)
cd ../doublets
./run.sh volatile 6m 10
./run.sh nonvolatile 6m 10

# 4. Compare results
ls -lh ../results/*.csv
```

#### Durability Modes

**Durable Mode** (PostgreSQL default):
- Full ACID guarantees
- WAL enabled with fsync
- Production-safe
- Baseline for comparison

**Embedded-Like Mode** (PostgreSQL optimized):
- `fsync=off`, `synchronous_commit=off`
- `wal_level=minimal`
- Optional: UNLOGGED tables
- Trades durability for speed (matches embedded DB behavior)

To run in embedded-like mode:
```bash
cd docker
docker compose -f docker-compose.yml -f compose.embedded.yml up -d
```

### Directory Structure

```
docker/
  docker-compose.yml        # PostgreSQL 18 setup (durable mode)
  compose.embedded.yml      # Override for embedded-like mode
  init/
    01_download_demo.sh     # Auto-download Airlines demo DB
    99_unlogged.sql         # Optional: convert to UNLOGGED tables

sql/
  10_timetable_queries.sql  # All benchmark queries

bench/
  pg/
    run.sh                  # PostgreSQL benchmark script
  doublets/
    run.sh                  # Doublets benchmark script (placeholder)
  results/                  # CSV output and EXPLAIN logs
  schema-mapping.md         # How to map Airlines schema to Doublets

docs/
  HOWTO.md                  # Detailed setup and usage guide
```

### Documentation

- **[HOWTO.md](docs/HOWTO.md)** - Complete setup guide, dataset options, troubleshooting
- **[schema-mapping.md](bench/schema-mapping.md)** - Mapping Airlines entities to Doublets links
- **[10_timetable_queries.sql](sql/10_timetable_queries.sql)** - All queries with explanations

### Dataset Sizes

| Size | Period   | Flights | PostgreSQL | Compressed | Download Time |
|------|----------|---------|------------|------------|---------------|
| 3m   | 3 months | ~125k   | ~1.3 GB    | 133 MB     | ~2 min        |
| 6m   | 6 months | ~250k   | ~2.7 GB    | 276 MB     | ~5 min        |
| 1y   | 1 year   | ~500k   | ~5.4 GB    | 558 MB     | ~10 min       |
| 2y   | 2 years  | ~1M     | ~11 GB     | 1137 MB    | ~20 min       |

Default: **6 months** (good balance of size and completeness)

### Implementation Status

- [x] PostgreSQL 18 Docker setup
- [x] Airlines demo database integration
- [x] Timetable queries (10 queries)
- [x] PostgreSQL benchmark script
- [x] Durability modes (durable + embedded-like)
- [x] Schema mapping documentation
- [ ] **Doublets implementation** (TODO)
- [ ] Results comparison and visualization

### Next Steps

To complete this benchmark:

1. **Implement Doublets data model** (see `bench/schema-mapping.md`)
   - Map Airports, Routes, Flights to links
   - Handle temporal data (validity ranges)
   - Support NULL values and enums

2. **Implement equivalent queries**
   - Ensure exact same result sets as PostgreSQL
   - Validate with checksums

3. **Run comparative benchmarks**
   - Two durability modes
   - Two dataset sizes (6m, 1y)
   - 10 runs per query

4. **Analyze and visualize results**
   - Compare median times
   - Identify bottlenecks
   - Generate comparison charts

### References

- [PostgresPro Airlines Demo](https://postgrespro.ru/education/demodb) - Official documentation
- [PostgreSQL 18 Release Notes](https://www.postgresql.org/docs/18/) - What's new
- [Doublets Documentation](https://github.com/linksplatform/Data.Doublets) - Link storage system
- [Issue #11](https://github.com/linksplatform/Comparisons.PostgreSQLVSDoublets/issues/11) - Original requirements
