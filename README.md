# Comparisons.PostgreSQLVSDoublets

This project compares the performance of GraphQL queries between **PostgreSQL + Hasura** and **Doublets + Gql**.

## Setup

1. Ensure you have a Hasura GraphQL endpoint and a Doublets Gql endpoint running.
2. Update `config.json` with the endpoint URLs and authentication headers.
3. Place your GraphQL queries in the `queries/` directory.
4. Run `npm install` and then `node benchmark.js`.

## Results

The script executes each query multiple times (configurable via `config.json`) and outputs average execution time, min, max, and standard deviation.

## Dependencies

- Node.js 14+
- `node-fetch` for HTTP requests

## Note

This is a first version; improvements such as concurrent execution and warm-up rounds are planned.