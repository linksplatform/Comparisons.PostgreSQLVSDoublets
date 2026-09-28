#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
if [[ ! -d source/csharp/Platform.Data.Doublets.Gql.Server ]]; then
  echo 'Prepare the pinned Doublets source first; see graphql/README.md.' >&2
  exit 1
fi
export BENCH_UID="$(id -u)" BENCH_GID="$(id -g)"
# Unique project name gives this run its own network and fresh database volumes.
project="doublets-gql-bench-$$"
export BENCH_RESULTS="$PWD/results/$project"
mkdir -p "$BENCH_RESULTS"
compose=(docker compose --project-name "$project" --file compose.yaml)
cleanup() { "${compose[@]}" down --volumes --remove-orphans; }
trap cleanup EXIT
"${compose[@]}" pull k6
"${compose[@]}" up --detach --build postgres hasura doublets
python3 record-run.py "$BENCH_RESULTS"
"${compose[@]}" run --rm -e TARGET=hasura k6 run /scripts/benchmark.js
"${compose[@]}" run --rm -e TARGET=doublets k6 run /scripts/benchmark.js
python3 report.py "$BENCH_RESULTS/hasura.json" "$BENCH_RESULTS/doublets.json" > "$BENCH_RESULTS/comparison.md"
cat "$BENCH_RESULTS/comparison.md"
printf 'Saved results: %s\n' "$BENCH_RESULTS"
