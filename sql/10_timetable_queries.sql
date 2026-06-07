-- ============================================================================
-- PostgresPro Airlines Demo - Timetable Queries
-- ============================================================================
-- These queries demonstrate flight timetable operations using the Airlines
-- demo database. They include both queries using the built-in timetable view
-- and manual queries with explicit validity checks.
--
-- Prerequisites:
-- - PostgreSQL 18
-- - Airlines demo database loaded (demo database)
-- - bookings schema with flights, routes, airports tables
-- ============================================================================

\timing on

-- ============================================================================
-- Query 1: Departures from an Airport (Using Timetable View)
-- ============================================================================
-- Get all departures from Sheremetyevo (SVO) on a specific date
-- The timetable view handles timezone conversions and validity checks

\echo '=== Query 1: Departures from SVO on 2025-10-07 (using view) ==='
SELECT
  flight_id,
  route_no,
  departure_airport,
  arrival_airport,
  scheduled_departure,
  scheduled_arrival,
  status
FROM bookings.timetable
WHERE departure_airport = 'SVO'
  AND (scheduled_departure AT TIME ZONE 'UTC')::date = DATE '2025-10-07'
ORDER BY scheduled_departure;

-- ============================================================================
-- Query 2: Arrivals to an Airport (Using Timetable View)
-- ============================================================================
-- Get all arrivals to Sheremetyevo (SVO) on a specific date

\echo '=== Query 2: Arrivals to SVO on 2025-10-07 (using view) ==='
SELECT
  flight_id,
  route_no,
  departure_airport,
  arrival_airport,
  scheduled_departure,
  scheduled_arrival,
  status
FROM bookings.timetable
WHERE arrival_airport = 'SVO'
  AND (scheduled_arrival AT TIME ZONE 'UTC')::date = DATE '2025-10-07'
ORDER BY scheduled_arrival;

-- ============================================================================
-- Query 3: Next Available Flight on a Route
-- ============================================================================
-- Find the next available flight from Yekaterinburg (SVX) to Wuhan (WUH)
-- after the current model time

\echo '=== Query 3: Next available flight SVX -> WUH after model time ==='
SELECT
  flight_id,
  route_no,
  departure_airport,
  arrival_airport,
  scheduled_departure,
  scheduled_arrival,
  status,
  bookings.now() AS model_time
FROM bookings.timetable
WHERE departure_airport = 'SVX'
  AND arrival_airport = 'WUH'
  AND scheduled_departure > bookings.now()
ORDER BY scheduled_departure
LIMIT 1;

-- ============================================================================
-- Query 4: Manual Timetable (Without View, Explicit Validity Check)
-- ============================================================================
-- This demonstrates the manual approach without using the timetable view.
-- IMPORTANT: Always include the validity check when joining routes to flights!
-- The validity check ensures the route was valid at the flight's scheduled time.

\echo '=== Query 4: Manual departures from SVO (explicit validity) ==='
SELECT
  f.flight_id,
  r.route_no,
  r.departure_airport,
  r.arrival_airport,
  f.status,
  f.scheduled_departure,
  f.scheduled_arrival,
  r.validity AS route_validity_period
FROM bookings.flights AS f
JOIN bookings.routes AS r
  ON r.route_no = f.route_no
  AND r.validity @> f.scheduled_departure  -- CRITICAL: validity check!
WHERE r.departure_airport = 'SVO'
  AND f.scheduled_departure::date = DATE '2025-10-07'
ORDER BY f.scheduled_departure;

-- ============================================================================
-- Query 5: Manual Arrivals (Explicit Validity Check)
-- ============================================================================
-- Same as Query 4 but for arrivals

\echo '=== Query 5: Manual arrivals to SVO (explicit validity) ==='
SELECT
  f.flight_id,
  r.route_no,
  r.departure_airport,
  r.arrival_airport,
  f.status,
  f.scheduled_departure,
  f.scheduled_arrival,
  r.validity AS route_validity_period
FROM bookings.flights AS f
JOIN bookings.routes AS r
  ON r.route_no = f.route_no
  AND r.validity @> f.scheduled_departure  -- CRITICAL: validity check!
WHERE r.arrival_airport = 'SVO'
  AND f.scheduled_arrival::date = DATE '2025-10-07'
ORDER BY f.scheduled_arrival;

-- ============================================================================
-- Query 6: Flights by Route Number with Full Details
-- ============================================================================
-- Get all flights for a specific route with aircraft and airport details

\echo '=== Query 6: All flights for a specific route with details ==='
SELECT
  f.flight_id,
  f.route_no,
  dep.airport_code AS dep_code,
  dep.airport_name->>'en' AS dep_name,
  dep.city->>'en' AS dep_city,
  arr.airport_code AS arr_code,
  arr.airport_name->>'en' AS arr_name,
  arr.city->>'en' AS arr_city,
  f.scheduled_departure,
  f.scheduled_arrival,
  f.status,
  a.aircraft_code,
  a.model->>'en' AS aircraft_model
FROM bookings.flights f
JOIN bookings.routes r
  ON r.route_no = f.route_no
  AND r.validity @> f.scheduled_departure  -- validity check
JOIN bookings.airports dep ON dep.airport_code = r.departure_airport
JOIN bookings.airports arr ON arr.airport_code = r.arrival_airport
LEFT JOIN bookings.aircraft a ON a.aircraft_code = f.aircraft_code
WHERE f.route_no = 'PG0001'
ORDER BY f.scheduled_departure
LIMIT 20;

-- ============================================================================
-- Query 7: Count Flights by Status for an Airport
-- ============================================================================
-- Aggregate query: count flights by status for a specific departure airport

\echo '=== Query 7: Flight status distribution for departures from SVO ==='
SELECT
  f.status,
  COUNT(*) AS flight_count,
  COUNT(DISTINCT f.route_no) AS unique_routes
FROM bookings.flights f
JOIN bookings.routes r
  ON r.route_no = f.route_no
  AND r.validity @> f.scheduled_departure
WHERE r.departure_airport = 'SVO'
GROUP BY f.status
ORDER BY flight_count DESC;

-- ============================================================================
-- Query 8: Busiest Routes from an Airport
-- ============================================================================
-- Find the busiest routes (most flights) from a specific airport

\echo '=== Query 8: Top 10 busiest routes from SVO ==='
SELECT
  r.departure_airport,
  r.arrival_airport,
  arr.city->>'en' AS destination_city,
  COUNT(*) AS num_flights,
  COUNT(DISTINCT f.aircraft_code) AS num_aircraft_types
FROM bookings.flights f
JOIN bookings.routes r
  ON r.route_no = f.route_no
  AND r.validity @> f.scheduled_departure
JOIN bookings.airports arr ON arr.airport_code = r.arrival_airport
WHERE r.departure_airport = 'SVO'
GROUP BY r.departure_airport, r.arrival_airport, arr.city
ORDER BY num_flights DESC
LIMIT 10;

-- ============================================================================
-- Query 9: Flights by Date Range
-- ============================================================================
-- Get all flights in a specific date range for a route

\echo '=== Query 9: Flights in date range (2025-10-01 to 2025-10-07) ==='
SELECT
  DATE(f.scheduled_departure) AS flight_date,
  COUNT(*) AS num_flights,
  COUNT(DISTINCT r.route_no) AS num_routes,
  COUNT(DISTINCT r.departure_airport) AS num_dep_airports,
  COUNT(DISTINCT r.arrival_airport) AS num_arr_airports
FROM bookings.flights f
JOIN bookings.routes r
  ON r.route_no = f.route_no
  AND r.validity @> f.scheduled_departure
WHERE f.scheduled_departure >= DATE '2025-10-01'
  AND f.scheduled_departure < DATE '2025-10-08'
GROUP BY DATE(f.scheduled_departure)
ORDER BY flight_date;

-- ============================================================================
-- Query 10: EXPLAIN ANALYZE Example
-- ============================================================================
-- This query shows how to profile query performance
-- Use this pattern for benchmark measurements

\echo '=== Query 10: EXPLAIN ANALYZE for departures ==='
EXPLAIN (ANALYZE, BUFFERS, TIMING)
SELECT
  f.flight_id,
  r.route_no,
  r.departure_airport,
  r.arrival_airport,
  f.scheduled_departure
FROM bookings.flights AS f
JOIN bookings.routes AS r
  ON r.route_no = f.route_no
  AND r.validity @> f.scheduled_departure
WHERE r.departure_airport = 'SVO'
  AND f.scheduled_departure::date = DATE '2025-10-07'
ORDER BY f.scheduled_departure;

-- ============================================================================
-- Validation Queries
-- ============================================================================

\echo '=== Validation: Check model time ==='
SELECT bookings.now() AS current_model_time;

\echo '=== Validation: Check if routes/flights validity is respected ==='
-- This should return 0 (no flights without valid routes)
SELECT COUNT(*) AS invalid_flights
FROM bookings.flights f
LEFT JOIN bookings.routes r
  ON r.route_no = f.route_no
  AND r.validity @> f.scheduled_departure
WHERE r.route_no IS NULL;

\echo '=== Validation: Date range in dataset ==='
SELECT
  MIN(scheduled_departure) AS earliest_flight,
  MAX(scheduled_arrival) AS latest_flight,
  MAX(scheduled_arrival) - MIN(scheduled_departure) AS total_span
FROM bookings.flights;

\echo '=== Validation: Count airports and routes ==='
SELECT
  (SELECT COUNT(DISTINCT airport_code) FROM bookings.airports) AS num_airports,
  (SELECT COUNT(*) FROM bookings.routes) AS num_routes,
  (SELECT COUNT(*) FROM bookings.flights) AS num_flights;

\timing off

\echo '=== All queries completed ==='
