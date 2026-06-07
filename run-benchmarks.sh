#!/bin/bash
# Local script to run GraphQL benchmarks

set -e

echo "Starting GraphQL benchmark comparison..."

# Check if k6 is installed
if ! command -v k6 &> /dev/null; then
    echo "k6 is not installed. Please install it first:"
    echo "https://k6.io/docs/get-started/installation/"
    exit 1
fi

# Check if docker-compose is available
if ! command -v docker-compose &> /dev/null; then
    echo "docker-compose is not installed. Please install it first."
    exit 1
fi

echo "Starting PostgreSQL + Hasura stack..."
docker-compose -f docker-compose.postgresql-hasura.yml up -d

echo "Waiting for Hasura to be ready..."
timeout 120s bash -c 'until curl -f http://localhost:8080/healthz; do sleep 2; done' || {
    echo "Hasura failed to start within 120 seconds"
    docker-compose -f docker-compose.postgresql-hasura.yml logs
    exit 1
}

echo "Building and starting Doublets GraphQL stack..."
docker-compose -f docker-compose.doublets-gql.yml up -d --build

echo "Waiting for Doublets GraphQL to be ready..."
timeout 180s bash -c 'until curl -f http://localhost:60341/v1/graphql; do sleep 5; done' || {
    echo "Doublets GraphQL failed to start within 180 seconds"
    docker-compose -f docker-compose.doublets-gql.yml logs
    exit 1
}

echo "Running PostgreSQL + Hasura benchmarks..."
cd benchmarks/k6
k6 run --out json=hasura-results.json hasura-benchmark.js || echo "Hasura benchmark completed with errors"

echo "Running Doublets GraphQL benchmarks..."
k6 run --out json=doublets-results.json doublets-benchmark.js || echo "Doublets benchmark completed with errors"

echo "Processing results..."
cd ..
python3 process-results.py k6/hasura-results.json k6/doublets-results.json

echo "Benchmark completed! Check the generated charts:"
echo "- bench_graphql.png"
echo "- bench_graphql_log_scale.png"

echo "Stopping services..."
cd ..
docker-compose -f docker-compose.postgresql-hasura.yml down
docker-compose -f docker-compose.doublets-gql.yml down

echo "Done!"