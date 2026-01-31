# Schema Mapping: PostgreSQL Airlines Demo to Doublets

This document describes how to map the PostgreSQL Airlines demo database schema to Doublets data structures for equivalent benchmark operations.

## Overview

The Airlines demo database uses a traditional relational model with tables, foreign keys, and typed columns. Doublets uses a link-based model where all data is represented as triplets (links) with source, target, and optional meaning.

## Core Mapping Strategy

### 1. Entity Representation

Each entity type (Airport, Flight, Route, etc.) is represented as:
- A **type link** that identifies the entity class
- **Instance links** for each entity record
- **Attribute links** connecting instances to their property values

### 2. Relationships

Foreign key relationships become directional links:
- `Flight -> Route` relationship = link from flight instance to route instance
- Many-to-many relationships = pairs of links through junction instances

### 3. Temporal Data

Timestamps and date ranges are represented as:
- Numeric values (Unix timestamps or custom encoding)
- Range types (validity periods) as pairs of start/end links

## Schema Mapping Tables

### Airports Table

**PostgreSQL Schema:**
```sql
CREATE TABLE bookings.airports (
  airport_code char(3) PRIMARY KEY,
  airport_name jsonb NOT NULL,
  city jsonb NOT NULL,
  coordinates point NOT NULL,
  timezone text NOT NULL
);
```

**Doublets Representation:**
```
Type: Airport
Attributes:
  - airport_code (string → numeric ID)
  - airport_name_en (string)
  - airport_name_ru (string)
  - city_en (string)
  - city_ru (string)
  - latitude (float → integer encoding)
  - longitude (float → integer encoding)
  - timezone (string → numeric ID)

Link Structure:
  [Airport Type] → [Airport Instance: SVO]
  [Airport Instance: SVO] → [Code] → [SVO_numeric_id]
  [Airport Instance: SVO] → [Name_EN] → ["Sheremetyevo"]
  [Airport Instance: SVO] → [Latitude] → [encoded_value]
  [Airport Instance: SVO] → [Longitude] → [encoded_value]
```

### Routes Table

**PostgreSQL Schema:**
```sql
CREATE TABLE bookings.routes (
  route_no char(6) PRIMARY KEY,
  departure_airport char(3) REFERENCES airports,
  arrival_airport char(3) REFERENCES airports,
  aircraft_code char(3) REFERENCES aircraft,
  duration interval NOT NULL,
  validity tstzrange NOT NULL
);
```

**Doublets Representation:**
```
Type: Route
Relationships:
  - departure_airport → Airport instance
  - arrival_airport → Airport instance
  - aircraft_code → Aircraft instance
Attributes:
  - route_no (string → numeric ID)
  - duration_seconds (integer)
  - validity_start (timestamp as integer)
  - validity_end (timestamp as integer)

Link Structure:
  [Route Type] → [Route Instance: PG0001]
  [Route Instance: PG0001] → [RouteNo] → [PG0001_numeric_id]
  [Route Instance: PG0001] → [DepartureAirport] → [Airport Instance: SVO]
  [Route Instance: PG0001] → [ArrivalAirport] → [Airport Instance: LED]
  [Route Instance: PG0001] → [ValidityStart] → [timestamp_value]
  [Route Instance: PG0001] → [ValidityEnd] → [timestamp_value]
```

### Flights Table

**PostgreSQL Schema:**
```sql
CREATE TABLE bookings.flights (
  flight_id serial PRIMARY KEY,
  route_no char(6) REFERENCES routes,
  aircraft_code char(3) REFERENCES aircraft,
  status varchar(20) NOT NULL,
  scheduled_departure timestamptz NOT NULL,
  scheduled_arrival timestamptz NOT NULL,
  actual_departure timestamptz,
  actual_arrival timestamptz
);
```

**Doublets Representation:**
```
Type: Flight
Relationships:
  - route_no → Route instance
  - aircraft_code → Aircraft instance
Attributes:
  - flight_id (integer, direct mapping)
  - status (enum → numeric ID: Scheduled=1, OnTime=2, Delayed=3, etc.)
  - scheduled_departure (timestamp as integer)
  - scheduled_arrival (timestamp as integer)
  - actual_departure (timestamp as integer or NULL)
  - actual_arrival (timestamp as integer or NULL)

Link Structure:
  [Flight Type] → [Flight Instance: 123456]
  [Flight Instance: 123456] → [FlightID] → [123456]
  [Flight Instance: 123456] → [Route] → [Route Instance: PG0001]
  [Flight Instance: 123456] → [Status] → [Status: OnTime]
  [Flight Instance: 123456] → [ScheduledDeparture] → [timestamp_value]
  [Flight Instance: 123456] → [ScheduledArrival] → [timestamp_value]
```

### Aircraft Table

**PostgreSQL Schema:**
```sql
CREATE TABLE bookings.aircraft (
  aircraft_code char(3) PRIMARY KEY,
  model jsonb NOT NULL,
  range integer NOT NULL
);
```

**Doublets Representation:**
```
Type: Aircraft
Attributes:
  - aircraft_code (string → numeric ID)
  - model_en (string)
  - model_ru (string)
  - range (integer)

Link Structure:
  [Aircraft Type] → [Aircraft Instance: 773]
  [Aircraft Instance: 773] → [Code] → [773_numeric_id]
  [Aircraft Instance: 773] → [Model_EN] → ["Boeing 777-300"]
  [Aircraft Instance: 773] → [Range] → [11100]
```

## Query Mapping

### Query 1: Departures from Airport on Date

**PostgreSQL:**
```sql
SELECT *
FROM bookings.timetable
WHERE departure_airport = 'SVO'
  AND scheduled_departure::date = DATE '2025-10-07';
```

**Doublets Equivalent:**
```
1. Find all Flight instances
2. Filter by:
   - Flight → Route → DepartureAirport = [Airport Instance: SVO]
   - Flight → ScheduledDeparture between [date_start, date_end)
3. For each matching flight:
   - Traverse Flight → Route to get departure/arrival airports
   - Traverse Flight → ScheduledDeparture/Arrival for times
   - Traverse Flight → Status for status
4. Return result set
```

### Query 2: Next Available Flight

**PostgreSQL:**
```sql
SELECT *
FROM bookings.timetable
WHERE departure_airport = 'SVX'
  AND arrival_airport = 'WUH'
  AND scheduled_departure > bookings.now()
ORDER BY scheduled_departure
LIMIT 1;
```

**Doublets Equivalent:**
```
1. Find all Flight instances where:
   - Flight → Route → DepartureAirport = [Airport Instance: SVX]
   - Flight → Route → ArrivalAirport = [Airport Instance: WUH]
   - Flight → ScheduledDeparture > [current_model_time]
2. Sort by ScheduledDeparture (ascending)
3. Return first match
```

### Query 3: Manual Join with Validity Check

**PostgreSQL:**
```sql
SELECT f.flight_id, r.route_no
FROM bookings.flights f
JOIN bookings.routes r
  ON r.route_no = f.route_no
  AND r.validity @> f.scheduled_departure
WHERE r.departure_airport = 'SVO';
```

**Doublets Equivalent:**
```
1. Find all Flight instances
2. For each flight:
   - Traverse Flight → Route
   - Check Route → ValidityStart <= Flight → ScheduledDeparture
   - Check Route → ValidityEnd >= Flight → ScheduledDeparture
   - Check Route → DepartureAirport = [Airport Instance: SVO]
3. Return matching flights with route info
```

## Data Type Encoding

### Strings
- Convert to numeric IDs using a string interning table
- Store mapping: String → Numeric ID
- Use numeric IDs in all links

### Timestamps
- Store as Unix timestamps (seconds since epoch)
- Or use custom encoding (days since base date + seconds within day)
- Range queries use numeric comparisons

### JSON/JSONB
- Extract relevant fields (e.g., `airport_name->>'en'`)
- Store each field as separate attribute link
- Ignore unused fields

### NULL Values
- Option 1: Omit the attribute link (absence = NULL)
- Option 2: Create special NULL link target

### Enums (Status, etc.)
- Map each value to numeric ID
- Store as: `[Instance] → [AttributeType] → [EnumValue_ID]`

## Implementation Considerations

### 1. Data Loading
- Parse PostgreSQL dump or query results
- Convert to Doublets links in batches
- Build indexes for common access patterns

### 2. Query Translation
- Implement query builder that translates SQL-like operations to link traversals
- Support common patterns: filter, join, aggregate, order, limit

### 3. Performance Optimizations
- Index on departure_airport for fast filtering
- Index on scheduled_departure for date range queries
- Consider denormalization for frequent joins (e.g., cache airport codes on flights)

### 4. Validity Checks
- Implement range overlap efficiently:
  - `validity_start <= scheduled_departure <= validity_end`
- Consider creating validity index

### 5. Benchmark Equivalence
- Ensure result sets match PostgreSQL exactly (same rows, same order)
- Compute checksums (hash of sorted result set)
- Report any discrepancies

## Storage Estimates

Assuming 1-year dataset (~5.4GB in PostgreSQL):

- **Airports**: ~100 records → ~2,000 links (20 attributes each)
- **Routes**: ~500 records → ~20,000 links (40 attributes each)
- **Flights**: ~500,000 records → ~4,000,000 links (8 attributes each)
- **Aircraft**: ~10 records → ~100 links

**Total estimate**: ~4-5 million links

With Doublets split storage (index + data):
- Index: ~200MB (assuming 48 bytes per link)
- Data: ~200MB
- **Total**: ~400MB (10x compression vs PostgreSQL)

## Testing Strategy

1. **Data Integrity**:
   - Load subset of data (3 months)
   - Verify all entities and relationships present
   - Check attribute values match

2. **Query Correctness**:
   - Run same query on both systems
   - Compare result sets (row counts, values, order)
   - Report mismatches

3. **Performance Baseline**:
   - Run benchmarks on small dataset (3m)
   - Verify Doublets is faster (as expected)
   - Scale to larger datasets

4. **Durability Modes**:
   - Test Doublets volatile (in-memory) mode
   - Test Doublets non-volatile (persistent) mode
   - Compare with PostgreSQL durable/embedded modes

## Open Questions

1. **How to handle JSONB fields efficiently?**
   - Extract all fields vs. only used fields
   - Dynamic schema vs. fixed schema

2. **String interning strategy?**
   - Global intern table vs. per-type
   - Hash-based vs. sequential IDs

3. **Timestamp precision?**
   - Seconds vs. milliseconds
   - Timezone handling

4. **Query API design?**
   - SQL-like DSL vs. native link API
   - Type safety vs. flexibility

## Next Steps

1. Implement basic entity mapping (Airport, Flight, Route)
2. Create data loader script (PostgreSQL → Doublets)
3. Implement first query (departures by airport/date)
4. Verify correctness against PostgreSQL
5. Add timing measurements
6. Iterate on remaining queries
