# GraphQL Benchmark Implementation

This directory contains the GraphQL comparison implementation between PostgreSQL + Hasura and Doublets + GQL.

## Overview

The GraphQL benchmark compares the performance of two GraphQL implementations:

1. **PostgreSQL + Hasura**: Traditional SQL database with Hasura GraphQL layer
2. **Doublets + GQL**: LinksPlatform's Doublets storage with custom GraphQL server

## GraphQL Operations Benchmarked

Based on the original benchmark operations, we test these GraphQL operations:

### Mutations (Write Operations)
- `createPointLink`: Creates a point link (id = source = target)
- `createLink`: Creates a regular link with source and target
- `updateLink`: Updates an existing link
- `deleteLink`: Deletes a link by ID

### Queries (Read Operations)  
- `allLinks`: Get all links (equivalent to Each All `[*, *, *]`)
- `linkById`: Get link by ID (equivalent to Each Identity `[id, *, *]`)
- `concreteLinks`: Get links by source and target (equivalent to Each Concrete `[*, source, target]`)
- `outgoingLinks`: Get outgoing links from source (equivalent to Each Outgoing `[*, source, *]`)
- `incomingLinks`: Get incoming links to target (equivalent to Each Incoming `[*, *, target]`)

## Schema

The `schema.graphql` file defines the common GraphQL schema used by both implementations, ensuring fair comparison.

## Running Benchmarks

### Prerequisites

- Docker and Docker Compose
- k6 load testing tool
- Python 3 with matplotlib and numpy

### Local Execution

```bash
./run-benchmarks.sh
```

### GitHub Actions

Benchmarks run automatically on push/PR and publish results to the `gh-pages` branch.

## Results

Results are processed and visualized similar to the existing Rust/C++ benchmarks:
- Linear scale chart showing absolute performance
- Logarithmic scale chart for easier comparison
- Tabular results with speedup factors

The benchmark maintains the same testing conditions as the original benchmarks:
- 3000 background links for realistic index sizes
- 1000 active operations per benchmark run
- Multiple scenarios to test different operation types