# Comparisons.PostgreSQLVSDoublets

Comparison of query performance between PostgreSQL + Hasura GraphQL engine and Doublets + Doublets GQL.

## How to Run

### Prerequisites
- .NET 6 SDK
- Docker (for PostgreSQL + Hasura)
- Doublets GQL library (see [Data.Doublets.Gql](https://github.com/linksplatform/Data.Doublets.Gql))

### Setup

1. Start PostgreSQL and Hasura:
   ```
   docker-compose up -d
   ```
2. Initialize the database:
   ```
   dotnet run --project HasuraBenchmark -- --init
   ```
3. Initialize Doublets data:
   ```
   dotnet run --project DoubletsBenchmark -- --init
   ```

### Run Benchmarks

```
dotnet run -c Release --project Benchmarks
```

## Results

_To be filled after running benchmarks._

## GraphQL Query Examples

### Hasura
```graphql
query {
  users(limit: 10) {
    id
    name
    email
  }
}
```

### Doublets GQL
```graphql
query {
  get(limit: 10) {
    id
    name
    email
  }
}
```
