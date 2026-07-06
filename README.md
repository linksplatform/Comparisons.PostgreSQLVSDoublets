# Comparisons.PostgreSQLVSDoublets

Benchmark comparing PostgreSQL + Hasura GraphQL with Doublets + Gql.

## Prerequisites
- Docker and Docker Compose
- Go 1.21+

## Setup

1. Start services:
   ```bash
   docker-compose up -d
   ```

2. Wait for Hasura to start (check http://localhost:8080/healthz).

3. Run the benchmark:
   ```bash
   go run main.go
   ```

## Structure

- `docker-compose.yml`: Runs PostgreSQL, Hasura, and Doublets GQL server.
- `main.go`: Inserts 1000 records and measures query times for both systems.
- `migrations/`: Hasura metadata (auto-applied via hasura-cli or manual).

## Note on Hasura Issue #52

The benchmark includes workarounds for known Hasura performance issues (e.g., enabling prepared statements, setting query limits).