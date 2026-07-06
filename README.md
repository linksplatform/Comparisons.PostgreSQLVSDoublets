# Comparisons.PostgreSQLVSDoublets

Benchmark comparing PostgreSQL + Hasura with Doublets + Gql via GraphQL queries.

## Setup

1. Have a PostgreSQL instance running with Hasura GraphQL engine connected.
2. Have Doublets running with Gql endpoint.
3. Install Python dependencies: `pip install -r requirements.txt`
4. Update `config.json` with your endpoints.

## Run

```bash
python benchmark.py
```

## Notes

This benchmark addresses issue #52 of hasura/graphql-bench. Ensure your Hasura version includes necessary fixes.
