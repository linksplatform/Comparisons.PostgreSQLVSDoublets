#!/bin/bash
# Usage: ./benchmark.sh [iterations]

ITERATIONS=${1:-100}

echo "Running benchmarks with $ITERATIONS iterations..."

# Warmup
curl -s -X POST -H "Content-Type: application/json" -d '{"query":"query { link(limit: 1) { id } }"}' http://localhost:8080/v1/graphql > /dev/null

# Hasura benchmark
echo "Benchmarking Hasura..."
TIMES_HASURA=()
for ((i=1; i<=ITERATIONS; i++)); do
  START=$(date +%s%N)
  curl -s -X POST -H "Content-Type: application/json" -H "x-hasura-admin-secret: myadminsecret" -d @queries/simple.graphql http://localhost:8080/v1/graphql > /dev/null
  END=$(date +%s%N)
  TIMES_HASURA+=($((END - START)))
done

# Doublets Gql benchmark
echo "Benchmarking Doublets Gql..."
TIMES_DOUBLETS=()
for ((i=1; i<=ITERATIONS; i++)); do
  START=$(date +%s%N)
  curl -s -X POST -H "Content-Type: application/json" -d @queries/simple.graphql http://localhost:8081/graphql > /dev/null
  END=$(date +%s%N)
  TIMES_DOUBLETS+=($((END - START)))
done

# Calculate results
echo "Hasura average: $(echo ${TIMES_HASURA[@]} | tr ' ' '\n' | awk '{sum+=$1} END {print sum/NR/1000000}') ms"
echo "Doublets average: $(echo ${TIMES_DOUBLETS[@]} | tr ' ' '\n' | awk '{sum+=$1} END {print sum/NR/1000000}') ms"

# Save results to JSON
echo "{\"hasura\": $(printf '%s\n' "${TIMES_HASURA[@]}" | jq -Rs '{times: split("\n") | map(select(length>0) | tonumber)}'), \"doublets\": $(printf '%s\n' "${TIMES_DOUBLETS[@]}" | jq -Rs '{times: split("\n") | map(select(length>0) | tonumber)}') }" > results/benchmark.json

echo "Done. Results saved to results/benchmark.json"