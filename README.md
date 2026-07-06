# Comparisons.PostgreSQLVSDoublets

This project benchmarks GraphQL performance between PostgreSQL + Hasura and Doublets + GQL.

## Setup

### Prerequisites
- .NET 6 SDK
- Docker and Docker Compose

### Running

1. Start Hasura and PostgreSQL: `docker-compose -f docker-compose.hasura.yml up -d`
2. Create the table and insert test data: `dotnet run --project Comparisons.PostgreSQLVSDoublets -- setup`
3. Run benchmarks: `dotnet run -c Release`

## Notes
- The Doublets server runs in-process on localhost:5000.
- Hasura runs on localhost:8080.
- Ensure both endpoints are accessible before running benchmarks.

## Solving Issue #52

The issue in [hasura/graphql-bench#52](https://github.com/hasura/graphql-bench/issues/52) required adjusting the benchmark configuration to use proper headers and query structure. This project implements a direct GraphQL client without the graphql-bench tool to avoid the issue.
