#!/bin/bash
set -e

# Run graphql-bench with custom config
npx graphql-bench --config ./config/config.yml --output ./results/

echo "Benchmark complete. Results in ./results/"
