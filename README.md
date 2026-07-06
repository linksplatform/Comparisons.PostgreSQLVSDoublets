# Comparisons.PostgreSQLVSDoublets

Comparison of PostgreSQL + Hasura vs Doublets + Gql using GraphQL.

Based on [Comparisons.SQLiteVSDoublets](https://github.com/linksplatform/Comparisons.SQLiteVSDoublets).

## Prerequisites

- .NET 6 SDK
- Docker (for Hasura + PostgreSQL)
- Doublets server with Gql (e.g., from [Data.Doublets.Gql](https://github.com/linksplatform/Data.Doublets.Gql))

## Setup

1. Start Hasura + PostgreSQL:
```bash
docker-compose up -d
```

2. Set up Hasura tables (via console at http://localhost:8080) for doublets: id (serial), source (bigint), target (bigint).

3. Start the Doublets Gql server on port 5000.

## Running the benchmark

```bash
cd Benchmark
dotnet run [hasura-url] [doublets-url]
```

Default URLs: `http://localhost:8080/v1/graphql` and `http://localhost:5000/graphql`.

The benchmark runs 100 iterations per query and prints average times.

## Queries tested

- Introspection (`__typename`)
- Create a doublet (mutation)
- List all doublets

## Issue Reference

This task requires solving [hasura/graphql-bench#52](https://github.com/hasura/graphql-bench/issues/52) for production-ready benchmarking.
