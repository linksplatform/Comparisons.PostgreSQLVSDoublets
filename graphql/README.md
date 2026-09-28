# PostgreSQL + Hasura versus Doublets + GraphQL

This comparison uses the real PostgreSQL/Hasura and LinksPlatform Doublets GraphQL
servers. It runs a correctness-checked sequence through k6, with 3,000 background
point links and 1,000 sequential iterations per server by default. It does not
call a remote benchmark endpoint or access an existing database.

## Run locally

Prerequisites: Git, Python 3.12 or newer, Docker and Docker Compose. No hosted k6
account, API keys or broker account is used. Images and NuGet packages are
fetched from their official registries on the first build.

```sh
# Separate, public source checkout; this does not modify this benchmark repository.
git clone --recurse-submodules https://github.com/linksplatform/Data.Doublets.Gql.git /tmp/doublets-gql
python3 graphql/prepare-source.py /tmp/doublets-gql

# Fast correctness smoke test, with real servers and real k6:
BACKGROUND=30 ITERATIONS=3 ./graphql/run.sh

# Full configured workload:
./graphql/run.sh
```

`prepare-source.py` exports unmodified Doublets commit
`b11f33b4080a7ef6b6d1c056c40bbf758d6cdd7e` and its Settings submodule commit
`f08ce8fc84f8ab7ccdcf9293b5566007305254c1`. It performs no network calls. If your
existing checkout lacks either Git object, fetch the named public revision into
that checkout first. An already prepared `graphql/source` directory is not
overwritten. Docker publishes the upstream .NET 6 server target explicitly;
this preserves the pinned server rather than silently changing its runtime.

Each invocation creates a unique Compose project, private Docker network and
fresh volumes. No ports are exposed to the host. The `EXIT` trap removes only
that invocation's containers, network and database volumes. A force-killed shell
cannot run a cleanup trap; use the printed `doublets-gql-bench-<pid>` project name
to clean that run with `docker compose -p <project> -f graphql/compose.yaml down -v`.

Measured results are retained separately for each run under
`graphql/results/doublets-gql-bench-<pid>/`:

- `hasura.json` and `doublets.json`: actual k6 summary metrics and sample counts.
- `comparison.md`: mean and p95 HTTP durations, in milliseconds.
- `run.json`: source, script, runtime and workload metadata.

A failed run exits nonzero. An incomplete run is never converted into a comparison
table. No chart or result in the repository is claimed to be a measurement from
this GraphQL harness until the actual run has completed.

## Workload and correctness

The initial dataset contains exactly 3,000 point links: `id = from_id = to_id`.
The default 1,000 iterations each perform the following sequence using one k6
virtual user; the two servers are measured sequentially:

| Operation | Behavior / verified result |
|---|---|
| Create | Insert an empty link, then update both endpoints to its returned ID; verify a point link. Two HTTP mutations on **both** systems. |
| Update | Set the new link to `(id, 1)` and check the returned row. |
| Each All | Fetch every row; verify 3,001 unique IDs, exact background values and the active row. |
| Each Identity | Filter by the active ID; verify the exact row. |
| Each Concrete | Filter by source and target; verify the exact row. |
| Each Outgoing | Filter by source; verify the exact row. |
| Each Incoming | Filter by target 1; verify point1 and the active row. |
| Delete | Delete the active ID, check the returned row, and separately verify that it is absent. |

The default operation timings therefore contain 1,000 samples each, with 3,000
background links and one active link during reads. This is a single-client CRUD
comparison, not a throughput or saturation test. `BACKGROUND` may be 1..3000 and
`ITERATIONS` 1..1000. The request timeout is 15s, setup is bounded to 5 min, and each
server's iteration phase is bounded to 10 min.

All measured requests use the servers' actual common fields:
`insert_links`, `update_links`, `delete_links`, and `links(where: ...)`, with
`id`, `from_id`, and `to_id`. PostgreSQL has a primary key, individual source and
target indexes, and a unique source/target pair to match the native link model.
Both endpoints receive equivalent data and the same operations. Inserted IDs
are read from responses rather than assumed for the active workload.

Every response is checked for HTTP status, GraphQL errors, expected cardinality,
unique IDs and expected field values. The run aborts at the first mismatch and
prints the operation, request and response for request failures. Setup refuses
an already populated database. No error-swallowing continuation or fake response
adapter is used. Report unit tests use clearly synthetic fixtures; these are never
written to the measured-results directory.

## Relation to graphql-bench #52

[The prerequisite issue](https://github.com/hasura/graphql-bench/issues/52) reports
HTTP 400/500 responses and missing request/response diagnostics. Its maintainer
suggested JSON Content-Type headers and debug output. The comparison's sponsor
[also suggested using k6 directly](https://github.com/linksplatform/Comparisons.PostgreSQLVSDoublets/issues/1#issuecomment-910735999).

This implementation takes that direct k6 path: it explicitly posts JSON with
`Content-Type: application/json`, uses the actual endpoint/schema, and reports
failed requests and response bodies. Successful real-server runs verify that
these requests work. It does not claim to modify, reproduce every old executor's
bug in, or close the separate graphql-bench issue.

## Reproducibility and interpretation

The pinned Doublets source is compiled without source changes; runtime image
versions/digests and source revision are recorded by the harness. The baseline
.NET 6 target is inherited from that source. These isolated containers are for
local benchmark work, not a deployment configuration.

Numbers measure HTTP response time (two durations summed for Create), not pure
storage-engine latency. Validation, seeding, deletion verification and startup
are outside the operation metrics. Container scheduling, caching, host load and
run order can affect results. Repeat runs and compare like-for-like workloads
before drawing performance conclusions. Neither one run nor 1,000 sequential
iterations establishes broad performance superiority.

The existing Rust benchmark results elsewhere in the repository are separate
and are not replaced by this comparison.

## Automated verification

```sh
python3 -m unittest discover -s graphql -v
```

The GraphQL workflow runs a real Docker/k6 smoke test on changes and permits the
full bounded workload through manual dispatch. It uploads run artifacts and does
not publish to `gh-pages` or commit generated results.
