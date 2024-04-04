# Comparisons.PostgreSQLVSDoublets

The comparison between PostgreSQL and LinksPlatform's Doublets (links) on basic database operations with objects (create table, read table, delete table, update table).
All benchmarks ran with 3000 links in background and 1000 in active use (i.e. insert, delete, update).

## Operations
- **Update** – basic update operation
- **Each All** – take all links matching `[*, *, *]` constraint
- **Each Incoming** – take all links matching `[*, *, target]` constraint
- **Each Outgoing** – take all links matching `[*, source, *]` constraint
- **Each Concrete** – take all links matching `[*, source, target]` constraint
- **Each Identity** – take all links matching `[id, *, *]` constraint
- **Delete** – basic delete operation
- **Create** – insert point (link with id = source = target)

## Results
The results below represent the amount of time (ns) the operation takes per iteration.
- First picture shows time in a pixel scale.
- Second picture shows time in a logarithmic scale.

### Rust
![Image of Rust benchmark (pixel scale)](https://github.com/linksplatform/Comparisons.PostgreSQLVSDoublets/blob/gh-pages/Docs/bench_rust.png?raw=true)
![Image of Rust benchmark (log scale)](https://github.com/linksplatform/Comparisons.PostgreSQLVSDoublets/blob/gh-pages/Docs/bench_rust_log_scale.png?raw=true)

## Conclusion
As we can see in this comparison, Doublets are blazingly faster than PostgreSQL in basic database operations.