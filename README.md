# Comparisons.PostgreSQLVSDoublets

This project compares the performance of two GraphQL implementations:
- **PostgreSQL + Hasura**: Using Hasura as a GraphQL engine over PostgreSQL.
- **Doublets + Gql**: Using Doublets as a storage layer with the GQL library for GraphQL.

The goal is to measure latency and throughput for common queries under various loads.

## Setup

### Prerequisites
- Docker & Docker Compose
- Node.js (for running graphql-bench)
- wrk or autocannon (for load generation)

### Running

1. Start the PostgreSQL+Hasura stack:
   ```bash
   cd postgresql-hasura
   docker-compose up -d
   ```

2. Start the Doublets+Gql stack:
   ```bash
   cd doublets-gql
   docker-compose up -d
   ```

3. Run the benchmark:
   ```bash
   cd benchmark
   ./run.sh
   ```

4. Results will be saved in `benchmark/results/`.

## Configuration

- See `benchmark/config/config.yml` for benchmark parameters.
- GraphQL queries are in `benchmark/config/queries/`.

## References

- [Comparisons.SQLiteVSDoublets](https://github.com/linksplatform/Comparisons.SQLiteVSDoublets)
- [Data.Doublets.Gql](https://github.com/linksplatform/Data.Doublets.Gql)
- [Hasura GraphQL Bench](https://github.com/hasura/graphql-bench)

## Issue #52

This work includes a workaround/solution for [hasura/graphql-bench#52](https://github.com/hasura/graphql-bench/issues/52) – see `benchmark/graphql-bench-patch.js` for details.
