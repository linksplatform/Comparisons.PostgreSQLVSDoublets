# PostgreSQL + Hasura vs Doublets + Gql

This project benchmarks GraphQL query performance between PostgreSQL with Hasura and Doublets with its GQL layer.

## Setup

1. Start Hasura connected to a PostgreSQL instance.
2. Start a Doublets server with GQL enabled.
3. Update the endpoint URLs in `appsettings.json`.
4. Run the benchmark: `dotnet run -c Release`.

## Results

Benchmark results will be output in the console and can be exported as markdown or CSV.