# PostgreSQL+Hasura vs Doublets+Gql Comparison

This project benchmarks GraphQL performance between PostgreSQL via Hasura and Doublets via its GQL server.

## Setup

1. Ensure Docker and Docker Compose are installed.
2. Run `bash setup.sh` to start services and load data.
3. Run `bash run_benchmark.sh` to execute benchmarks.

## Structure

- `docker-compose.postgres-hasura.yaml`: PostgreSQL + Hasura.
- `docker-compose.doublets.yaml`: Doublets with GQL.
- `queries/`: GraphQL queries used in benchmarks.
- `benchmark-config.yaml`: Configuration for graphql-bench.

## Notes

The benchmark uses [graphql-bench](https://github.com/hasura/graphql-bench) (patched for PostgreSQL support per issue #52).