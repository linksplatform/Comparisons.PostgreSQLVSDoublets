#!/usr/bin/env bash
# Reproduce the PR benchmark on a disposable local PostgreSQL database.
set -euo pipefail
cd "$(dirname "$0")/.."
export BENCHMARK_BACKGROUND_LINKS="${BENCHMARK_BACKGROUND_LINKS:-100}"
export BENCHMARK_LINKS="${BENCHMARK_LINKS:-10}"
export POSTGRES_CONNECTION="${POSTGRES_CONNECTION:-host=localhost port=5432 user=postgres password=postgres dbname=postgres}"
export NPGSQL_CONNECTION="${NPGSQL_CONNECTION:-Host=localhost;Port=5432;Username=postgres;Password=postgres;Database=postgres}"
export POSTGRES_VERSION
POSTGRES_VERSION=$(psql "$POSTGRES_CONNECTION" -Atc 'SHOW server_version')
output_directory="${1:-results}"
mkdir -p "$output_directory"
cargo bench --locked --manifest-path rust/Cargo.toml --bench bench --no-run
dotnet restore csharp/PostgreSQLVSDoublets/PostgreSQLVSDoublets.csproj --locked-mode
dotnet build csharp/PostgreSQLVSDoublets/PostgreSQLVSDoublets.csproj -c Release --no-restore --warnaserror
for language in rust csharp; do
    for backend in psql doublets; do
        export BENCHMARK_BACKEND="$backend"
        output="$output_directory/$language-$BENCHMARK_BACKGROUND_LINKS-$backend.txt"
        python3 scripts/benchmark_header.py "$language" "$backend" > "$output"
        if [ "$language" = rust ]; then
            (cd rust && cargo bench --locked --bench bench -- --output-format bencher --noplot) >> "$output"
        else
            dotnet run --project csharp/PostgreSQLVSDoublets -c Release --no-build >> "$output"
        fi
    done
done
python3 scripts/benchmark_report.py "$output_directory" --charts "$output_directory/charts" > "$output_directory/report.md"
