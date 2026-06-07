-- This script converts all tables in the bookings schema to UNLOGGED
-- for embedded-like (WAL-light) mode benchmarking.
-- UNLOGGED tables are faster but not crash-safe (truncated on crash/restart).
-- Use this only for embedded-like performance testing, not production.

DO $$
DECLARE
  r record;
BEGIN
  FOR r IN
    SELECT format('%I.%I', n.nspname, c.relname) AS fqname
    FROM pg_class c
    JOIN pg_namespace n ON n.oid = c.relnamespace
    WHERE n.nspname = 'bookings' AND c.relkind = 'r'
  LOOP
    EXECUTE 'ALTER TABLE ' || r.fqname || ' SET UNLOGGED';
    RAISE NOTICE 'Converted % to UNLOGGED', r.fqname;
  END LOOP;
END$$;

-- To revert to durable (LOGGED) tables, replace SET UNLOGGED with SET LOGGED above
