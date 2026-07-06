# PostgreSQL+Hasura vs Doublets+Gql Comparison

This project implements the first version of a performance comparison between PostgreSQL with Hasura GraphQL and Doublets with its GQL interface.

## Prerequisites

- Docker and Docker Compose
- Python 3.8+ with `requests` library
- Hasura GraphQL Engine (will be run via Docker)
- Doublets GQL service (see [linksplatform/Data.Doublets.Gql](https://github.com/linksplatform/Data.Doublets.Gql))

## Setup

1. Start Hasura with PostgreSQL:
   ```bash
   docker-compose -f docker-compose-hasura.yml up -d
   ```
2. Start Doublets GQL service:
   ```bash
   docker-compose -f docker-compose-doublets.yml up -d
   ```
3. Apply the Hasura metadata (if needed) to create the `items` table.

## Running the Benchmark

```bash
python src/benchmark.py
```

Results will be saved to `results.csv`.

## Notes

- This benchmark addresses issue [#52](https://github.com/hasura/graphql-bench/issues/52) from Hasura GraphQL Bench by providing a comparable test for Doublets.
- The `items` table has columns: `id` (serial primary key) and `name` (text).
