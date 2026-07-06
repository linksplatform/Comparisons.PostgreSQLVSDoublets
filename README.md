# Comparisons.PostgreSQLVSDoublets

This project benchmarks **PostgreSQL + Hasura** against **Doublets + Gql**.

## Prerequisites

- Docker and Docker Compose
- Node.js (for Gql client if needed)

## Setup

1. Start PostgreSQL and Hasura:
   ```bash
   docker-compose up -d postgres hasura
   ```
2. Start Doublets with Gql support:
   ```bash
   docker-compose up -d doublets-gql
   ```
3. Run benchmarks:
   ```bash
   ./benchmark.sh
   ```

## Queries

- `queries/simple.graphql`: Fetch first 10 links.
- `queries/complex.graphql`: Join and filter across link types.

## Results

Results are stored in `results/` as JSON files.

## Workaround for Hasura issue #52

The benchmark includes a small delay between requests to avoid connection pooling issues (see `benchmark.js`).