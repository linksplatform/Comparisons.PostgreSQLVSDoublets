# Airlines Demo Database - Benchmark HOWTO

This guide explains how to set up the PostgresPro Airlines demo database, run timetable queries, and benchmark PostgreSQL vs Doublets in both durable and embedded-like modes.

## Prerequisites

- Docker and Docker Compose installed
- At least 10GB free disk space (for 1-year dataset)
- `psql` client (optional, for manual queries)
- `gh` CLI (GitHub CLI) for CI integration

## Quick Start

### 1. Start PostgreSQL 18 (Durable Mode)

```bash
cd docker
docker compose up -d
```

This will:
- Pull PostgreSQL 18 image
- Create a persistent volume for data
- Download and load the Airlines demo database (6 months by default)
- Start the database on port 5432

Wait for the database to be ready:
```bash
docker compose logs -f pg
```

### 2. Connect to the Database

```bash
docker compose exec pg psql -U postgres -d demo
```

### 3. Verify the Setup

```sql
-- Check model time
SELECT bookings.now();

-- Check table sizes
\dt+ bookings.*

-- Check airports
SELECT COUNT(*) FROM bookings.airports;

-- Check flights
SELECT COUNT(*) FROM bookings.flights;
```

## Dataset Sizes

You can choose different dataset sizes by setting the `DEMO_SIZE` environment variable:

- `3m` - 3 months (1.3 GB, ~133 MB compressed)
- `6m` - 6 months (2.7 GB, ~276 MB compressed) **[Default]**
- `1y` - 1 year (5.4 GB, ~558 MB compressed)
- `2y` - 2 years (11 GB, ~1137 MB compressed)

To use a different size:

```bash
# Option 1: Set environment variable
export DEMO_SIZE=1y
docker compose up -d

# Option 2: Inline
DEMO_SIZE=1y docker compose up -d
```

## Durability Modes

### Durable Mode (Default)

This is the standard, production-like mode with full ACID guarantees:

```bash
docker compose -f docker-compose.yml up -d
```

Configuration:
- Full WAL (Write-Ahead Logging)
- `fsync=on` (default)
- `synchronous_commit=on` (default)
- All tables are LOGGED (persistent)

### Embedded-Like Mode (WAL-Light)

This mode trades durability for performance, similar to embedded databases:

```bash
docker compose -f docker-compose.yml -f compose.embedded.yml up -d
```

Configuration:
- `fsync=off` - No forced disk syncs
- `synchronous_commit=off` - Async commit
- `full_page_writes=off` - Skip full-page writes
- `wal_level=minimal` - Minimal WAL logging

#### Optional: UNLOGGED Tables

For maximum speed (no WAL at all for data tables), convert tables to UNLOGGED:

```sql
-- Connect to the database
docker compose exec pg psql -U postgres -d demo

-- Run the unlogged conversion script
\i /docker-entrypoint-initdb.d/99_unlogged.sql
```

**Warning**: UNLOGGED tables:
- Are truncated after crashes or unclean shutdowns
- Cannot be replicated
- Are NOT suitable for production
- Should only be used for benchmarking

To revert to LOGGED tables:

```sql
DO $$
DECLARE r record;
BEGIN
  FOR r IN
    SELECT format('%I.%I', n.nspname, c.relname) AS fqname
    FROM pg_class c
    JOIN pg_namespace n ON n.oid = c.relnamespace
    WHERE n.nspname = 'bookings' AND c.relkind = 'r'
  LOOP
    EXECUTE 'ALTER TABLE ' || r.fqname || ' SET LOGGED';
  END LOOP;
END$$;
```

### Verify Durability Mode

Check table persistence:

```sql
SELECT relname,
       CASE relpersistence
         WHEN 'p' THEN 'LOGGED'
         WHEN 'u' THEN 'UNLOGGED'
         WHEN 't' THEN 'TEMPORARY'
       END AS persistence
FROM pg_class
WHERE relnamespace = 'bookings'::regnamespace AND relkind='r'
ORDER BY relname;
```

Check WAL settings:

```sql
SHOW fsync;
SHOW synchronous_commit;
SHOW full_page_writes;
SHOW wal_level;
```

## Custom Data Generation

Instead of loading a pre-generated snapshot, you can generate your own data:

1. Start the container without the download script:

```bash
# Temporarily disable the download script
mv docker/init/01_download_demo.sh docker/init/01_download_demo.sh.disabled
docker compose up -d
```

2. Download the generator from PostgresPro:

```bash
wget https://edu.postgrespro.ru/demo-generator.tar.gz
tar -xzf demo-generator.tar.gz
```

3. Install in PostgreSQL:

```bash
docker compose exec -T pg psql -U postgres < install.sql
```

4. Generate data:

```sql
-- Connect to demo database
docker compose exec pg psql -U postgres -d demo

-- Generate 1 year of data with 4 worker processes
CALL bookings.generate(now(), now() + interval '1 year', 4);

-- Monitor progress
SELECT bookings.busy();

-- Run post-generation checks
\i check.sql
```

## Running Timetable Queries

All timetable queries are in `sql/10_timetable_queries.sql`.

### From Host

```bash
docker compose exec -T pg psql -U postgres -d demo < ../sql/10_timetable_queries.sql
```

### From Inside Container

```bash
docker compose exec pg psql -U postgres -d demo
\i /path/to/sql/10_timetable_queries.sql
```

## Running Benchmarks

### PostgreSQL Benchmarks

The benchmark script runs all timetable queries with timing and EXPLAIN ANALYZE:

```bash
cd bench/pg
./run.sh durable 6m  # Durable mode, 6-month dataset
./run.sh embedded 6m  # Embedded mode, 6-month dataset
```

Results are saved to `bench/results/pg_*.csv`.

### Doublets Benchmarks

```bash
cd bench/doublets
# TODO: Add Doublets benchmark instructions
```

Results are saved to `bench/results/doublets_*.csv`.

## Benchmark Procedure

1. **Prepare environment**: Start Docker with desired durability mode
2. **Warm-up**: Run each query once (results discarded)
3. **Measure**: Run each query 10 times, record wall-clock time
4. **Collect**: Save results to CSV with columns:
   - `system` (pg or doublets)
   - `durability_mode` (durable or embedded)
   - `dataset` (3m, 6m, 1y, 2y)
   - `query_id` (departures_svo, arrivals_svo, etc.)
   - `run` (1-10)
   - `rows` (result count)
   - `ms` (wall-clock milliseconds)

5. **Analyze**: Compute min/median/p95 for each query

## Database Schema Overview

The Airlines demo database contains:

### Main Tables

- `bookings.airports` - Airport codes and locations
- `bookings.aircraft` - Aircraft models and seat configurations
- `bookings.routes` - Flight routes with validity periods
- `bookings.flights` - Scheduled flights (references routes)
- `bookings.bookings` - Passenger bookings
- `bookings.tickets` - Individual tickets
- `bookings.ticket_flights` - Ticket-to-flight associations
- `bookings.boarding_passes` - Boarding pass assignments

### Key Concepts

- **Model Time**: `bookings.now()` returns the current "time" in the simulation
- **Validity Period**: Routes have `validity` ranges (tstzrange) that must overlap with flight scheduled times
- **Airport-Local Times**: The `bookings.timetable` view handles timezone conversions
- **Temporal Joins**: When joining routes to flights, always check `r.validity @> f.scheduled_departure`

## Validation Checks

### Data Consistency

```sql
-- Count flights by status
SELECT status, COUNT(*) FROM bookings.flights GROUP BY status;

-- Check date ranges
SELECT
  MIN(scheduled_departure) AS first_flight,
  MAX(scheduled_arrival) AS last_flight
FROM bookings.flights;

-- Verify airports
SELECT COUNT(DISTINCT departure_airport) FROM bookings.routes;
SELECT COUNT(DISTINCT arrival_airport) FROM bookings.routes;
```

### Validity Checks

```sql
-- All flights should have valid routes
SELECT COUNT(*) AS invalid_flights
FROM bookings.flights f
LEFT JOIN bookings.routes r
  ON r.route_no = f.route_no
  AND r.validity @> f.scheduled_departure
WHERE r.route_no IS NULL;
-- Should return 0
```

## Troubleshooting

### Container Won't Start

```bash
docker compose logs pg
docker compose down -v  # Remove volumes
docker compose up -d
```

### Database Not Loading

Check init script logs:
```bash
docker compose logs pg | grep -A 20 "downloading\|extracting"
```

### Out of Disk Space

Use a smaller dataset (3m or 6m) or clean up:
```bash
docker compose down -v
docker system prune -a
```

### Queries Too Slow

1. Check if indexes exist:
```sql
\di bookings.*
```

2. Analyze query plans:
```sql
EXPLAIN (ANALYZE, BUFFERS)
SELECT * FROM bookings.timetable
WHERE departure_airport = 'SVO'
  AND scheduled_departure::date = DATE '2025-10-07';
```

3. Consider adding indexes (note: this changes the benchmark):
```sql
CREATE INDEX idx_flights_dep_time ON bookings.flights(scheduled_departure);
CREATE INDEX idx_flights_route ON bookings.flights(route_no);
```

## Stopping and Cleaning Up

### Stop (preserve data)
```bash
docker compose down
```

### Stop and remove all data
```bash
docker compose down -v
```

## References

- [PostgresPro Airlines Demo Database](https://postgrespro.ru/education/demodb)
- [PostgreSQL 18 Documentation](https://www.postgresql.org/docs/18/)
- [Docker Compose Documentation](https://docs.docker.com/compose/)
- [Doublets Documentation](https://github.com/linksplatform/Data.Doublets)
