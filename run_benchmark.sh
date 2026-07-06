#!/bin/bash
set -e

echo "Running benchmark with graphql-bench..."
# graphql-bench must be installed: npm install -g graphql-bench-cli
graphql-bench --config benchmark-config.yaml --output results.json

echo "Benchmark completed. Results in results.json"