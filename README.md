# PostgreSQL+Hasura vs Doublets+Gql Comparison

This project benchmarks the performance of GraphQL queries against two backends:
- PostgreSQL with Hasura GraphQL Engine
- Doublets with Gql (GraphQL layer for Doublets)

Based on [linksplatform/Comparisons.SQLiteVSDoublets](https://github.com/linksplatform/Comparisons.SQLiteVSDoublets).

## Prerequisites
- .NET 7 SDK
- Running instances of both backends

## Setup
1. Configure endpoints in `appsettings.json`.
2. Run `dotnet run` to execute benchmarks.

## Results
Benchmarks measure query latency (p50, p95, p99) and throughput.