# Measured sample — 2026-09-28 UTC

These files are actual outputs from the local Docker/k6 implementation at the
script SHA-256 in `run.json`. Both servers completed 1,000 sequential iterations
with 3,000 background point links, validating returned rows during every operation.

- `hasura.json` and `doublets.json`: raw measured operation summaries.
- `comparison.md`: derived table in milliseconds.
- `run.json`: source revisions, official image digests, script hash, runtime and workload.

This is one host and one sequential run. Startup/seeding and correctness-checking
requests are excluded from the operation trends. Create sums two HTTP requests
on both servers. These results describe this workload; they do not establish
broad storage-engine or production performance. The result-validation unit tests
use separate synthetic fixtures and did not produce any of these measurements.

To reproduce a fresh run, follow `../README.md`; output goes into a separate
ignored run directory. This sample is retained as evidence, not overwritten.
