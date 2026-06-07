#!/bin/bash
# ============================================================================
# PostgreSQL Airlines Demo - Benchmark Script
# ============================================================================
# This script runs timetable queries against the PostgreSQL Airlines demo
# database and collects timing measurements for benchmarking.
#
# Usage: ./run.sh <durability_mode> <dataset_size> [num_runs]
#   durability_mode: durable or embedded
#   dataset_size: 3m, 6m, 1y, or 2y
#   num_runs: number of iterations per query (default: 10)
#
# Example:
#   ./run.sh durable 6m 10
#   ./run.sh embedded 1y 20
#
# Output:
#   - CSV file: ../results/pg_<mode>_<dataset>_<timestamp>.csv
#   - EXPLAIN logs: ../results/pg_<mode>_<dataset>_<timestamp>_explain.txt
# ============================================================================

set -euo pipefail

# Configuration
DURABILITY_MODE="${1:-durable}"
DATASET_SIZE="${2:-6m}"
NUM_RUNS="${3:-10}"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
RESULTS_DIR="../results"
OUTPUT_CSV="${RESULTS_DIR}/pg_${DURABILITY_MODE}_${DATASET_SIZE}_${TIMESTAMP}.csv"
EXPLAIN_LOG="${RESULTS_DIR}/pg_${DURABILITY_MODE}_${DATASET_SIZE}_${TIMESTAMP}_explain.txt"

# Database connection
PGHOST="${PGHOST:-localhost}"
PGPORT="${PGPORT:-5432}"
PGUSER="${PGUSER:-postgres}"
PGDATABASE="${PGDATABASE:-demo}"
export PGPASSWORD="${PGPASSWORD:-postgres}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Create results directory
mkdir -p "${RESULTS_DIR}"

# Initialize CSV
echo "system,durability_mode,dataset,query_id,run,rows,ms" > "${OUTPUT_CSV}"

echo -e "${GREEN}=== PostgreSQL Benchmark ===${NC}"
echo "Mode: ${DURABILITY_MODE}"
echo "Dataset: ${DATASET_SIZE}"
echo "Runs per query: ${NUM_RUNS}"
echo "Output: ${OUTPUT_CSV}"
echo ""

# Function to execute a query and measure time
benchmark_query() {
  local query_id="$1"
  local query="$2"
  local run="$3"

  # Execute query and measure wall-clock time
  local start_ms=$(date +%s%3N)
  local row_count=$(psql -h "${PGHOST}" -p "${PGPORT}" -U "${PGUSER}" -d "${PGDATABASE}" \
    -t -c "${query}" | wc -l)
  local end_ms=$(date +%s%3N)
  local elapsed_ms=$((end_ms - start_ms))

  # Trim whitespace from row_count
  row_count=$(echo "${row_count}" | xargs)

  # Write to CSV
  echo "pg,${DURABILITY_MODE},${DATASET_SIZE},${query_id},${run},${row_count},${elapsed_ms}" >> "${OUTPUT_CSV}"

  echo "  Run ${run}: ${elapsed_ms}ms (${row_count} rows)"
}

# Function to run EXPLAIN ANALYZE for a query
explain_query() {
  local query_id="$1"
  local query="$2"

  echo "" >> "${EXPLAIN_LOG}"
  echo "========================================" >> "${EXPLAIN_LOG}"
  echo "Query: ${query_id}" >> "${EXPLAIN_LOG}"
  echo "========================================" >> "${EXPLAIN_LOG}"
  echo "" >> "${EXPLAIN_LOG}"

  psql -h "${PGHOST}" -p "${PGPORT}" -U "${PGUSER}" -d "${PGDATABASE}" \
    -c "EXPLAIN (ANALYZE, BUFFERS, TIMING) ${query}" >> "${EXPLAIN_LOG}" 2>&1 || true

  echo "" >> "${EXPLAIN_LOG}"
}

# Define queries
declare -A QUERIES

QUERIES["departures_svo"]="SELECT flight_id, route_no, departure_airport, arrival_airport, scheduled_departure, scheduled_arrival, status FROM bookings.timetable WHERE departure_airport = 'SVO' AND (scheduled_departure AT TIME ZONE 'UTC')::date = DATE '2025-10-07' ORDER BY scheduled_departure;"

QUERIES["arrivals_svo"]="SELECT flight_id, route_no, departure_airport, arrival_airport, scheduled_departure, scheduled_arrival, status FROM bookings.timetable WHERE arrival_airport = 'SVO' AND (scheduled_arrival AT TIME ZONE 'UTC')::date = DATE '2025-10-07' ORDER BY scheduled_arrival;"

QUERIES["next_flight_svx_wuh"]="SELECT flight_id, route_no, departure_airport, arrival_airport, scheduled_departure, scheduled_arrival, status FROM bookings.timetable WHERE departure_airport = 'SVX' AND arrival_airport = 'WUH' AND scheduled_departure > bookings.now() ORDER BY scheduled_departure LIMIT 1;"

QUERIES["manual_departures_svo"]="SELECT f.flight_id, r.route_no, r.departure_airport, r.arrival_airport, f.status, f.scheduled_departure, f.scheduled_arrival FROM bookings.flights AS f JOIN bookings.routes AS r ON r.route_no = f.route_no AND r.validity @> f.scheduled_departure WHERE r.departure_airport = 'SVO' AND f.scheduled_departure::date = DATE '2025-10-07' ORDER BY f.scheduled_departure;"

QUERIES["manual_arrivals_svo"]="SELECT f.flight_id, r.route_no, r.departure_airport, r.arrival_airport, f.status, f.scheduled_departure, f.scheduled_arrival FROM bookings.flights AS f JOIN bookings.routes AS r ON r.route_no = f.route_no AND r.validity @> f.scheduled_departure WHERE r.arrival_airport = 'SVO' AND f.scheduled_arrival::date = DATE '2025-10-07' ORDER BY f.scheduled_arrival;"

QUERIES["route_details"]="SELECT f.flight_id, f.route_no, dep.airport_code AS dep_code, arr.airport_code AS arr_code, f.scheduled_departure, f.scheduled_arrival, f.status FROM bookings.flights f JOIN bookings.routes r ON r.route_no = f.route_no AND r.validity @> f.scheduled_departure JOIN bookings.airports dep ON dep.airport_code = r.departure_airport JOIN bookings.airports arr ON arr.airport_code = r.arrival_airport WHERE f.route_no = 'PG0001' ORDER BY f.scheduled_departure LIMIT 20;"

QUERIES["status_counts"]="SELECT f.status, COUNT(*) AS flight_count FROM bookings.flights f JOIN bookings.routes r ON r.route_no = f.route_no AND r.validity @> f.scheduled_departure WHERE r.departure_airport = 'SVO' GROUP BY f.status ORDER BY flight_count DESC;"

QUERIES["busiest_routes"]="SELECT r.departure_airport, r.arrival_airport, COUNT(*) AS num_flights FROM bookings.flights f JOIN bookings.routes r ON r.route_no = f.route_no AND r.validity @> f.scheduled_departure WHERE r.departure_airport = 'SVO' GROUP BY r.departure_airport, r.arrival_airport ORDER BY num_flights DESC LIMIT 10;"

QUERIES["date_range"]="SELECT DATE(f.scheduled_departure) AS flight_date, COUNT(*) AS num_flights FROM bookings.flights f JOIN bookings.routes r ON r.route_no = f.route_no AND r.validity @> f.scheduled_departure WHERE f.scheduled_departure >= DATE '2025-10-01' AND f.scheduled_departure < DATE '2025-10-08' GROUP BY DATE(f.scheduled_departure) ORDER BY flight_date;"

# Check database connection
echo -e "${YELLOW}Checking database connection...${NC}"
if ! psql -h "${PGHOST}" -p "${PGPORT}" -U "${PGUSER}" -d "${PGDATABASE}" -c "SELECT 1;" >/dev/null 2>&1; then
  echo -e "${RED}ERROR: Cannot connect to database${NC}"
  echo "Host: ${PGHOST}:${PGPORT}"
  echo "Database: ${PGDATABASE}"
  echo "User: ${PGUSER}"
  exit 1
fi
echo -e "${GREEN}Connected successfully${NC}"
echo ""

# Verify database has data
echo -e "${YELLOW}Verifying database...${NC}"
FLIGHT_COUNT=$(psql -h "${PGHOST}" -p "${PGPORT}" -U "${PGUSER}" -d "${PGDATABASE}" -t -c "SELECT COUNT(*) FROM bookings.flights;" | xargs)
echo "Flights in database: ${FLIGHT_COUNT}"
if [ "${FLIGHT_COUNT}" -eq 0 ]; then
  echo -e "${RED}ERROR: Database has no flights${NC}"
  exit 1
fi
echo ""

# Run EXPLAIN ANALYZE for each query (once)
echo -e "${YELLOW}Collecting EXPLAIN ANALYZE plans...${NC}"
for query_id in "${!QUERIES[@]}"; do
  echo "  ${query_id}..."
  explain_query "${query_id}" "${QUERIES[$query_id]}"
done
echo -e "${GREEN}EXPLAIN plans saved to: ${EXPLAIN_LOG}${NC}"
echo ""

# Warm-up: run each query once
echo -e "${YELLOW}Warming up (running each query once)...${NC}"
for query_id in "${!QUERIES[@]}"; do
  echo "  ${query_id}..."
  psql -h "${PGHOST}" -p "${PGPORT}" -U "${PGUSER}" -d "${PGDATABASE}" \
    -t -c "${QUERIES[$query_id]}" >/dev/null 2>&1 || true
done
echo -e "${GREEN}Warm-up complete${NC}"
echo ""

# Run benchmarks
echo -e "${YELLOW}Running benchmarks...${NC}"
for query_id in "${!QUERIES[@]}"; do
  echo -e "${GREEN}Query: ${query_id}${NC}"
  for run in $(seq 1 "${NUM_RUNS}"); do
    benchmark_query "${query_id}" "${QUERIES[$query_id]}" "${run}"
  done
  echo ""
done

echo -e "${GREEN}=== Benchmark Complete ===${NC}"
echo "Results saved to: ${OUTPUT_CSV}"
echo "EXPLAIN logs saved to: ${EXPLAIN_LOG}"
echo ""

# Generate summary statistics
echo -e "${YELLOW}Generating summary statistics...${NC}"
python3 - <<EOF
import csv
import statistics

data = {}
with open('${OUTPUT_CSV}', 'r') as f:
    reader = csv.DictReader(f)
    for row in reader:
        query_id = row['query_id']
        ms = int(row['ms'])
        if query_id not in data:
            data[query_id] = []
        data[query_id].append(ms)

print("\n=== Summary Statistics ===")
print(f"{'Query ID':<30} {'Min (ms)':<12} {'Median (ms)':<12} {'P95 (ms)':<12} {'Max (ms)':<12}")
print("-" * 80)

for query_id in sorted(data.keys()):
    times = sorted(data[query_id])
    min_time = min(times)
    median_time = statistics.median(times)
    p95_time = times[int(len(times) * 0.95)]
    max_time = max(times)
    print(f"{query_id:<30} {min_time:<12} {median_time:<12.1f} {p95_time:<12} {max_time:<12}")

print("\nDetailed results available in: ${OUTPUT_CSV}")
EOF

echo ""
echo -e "${GREEN}Done!${NC}"
