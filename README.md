# Comparisons.PostgreSQLVSDoublets

The comparison between PostgreSQL and LinksPlatform's Doublets (links) on basic database operations with links (create, read, delete, update).
All benchmarks ran with 3000 links in background to increase size of indexes and 1000 are actively created/updated/deleted.

In this particular benchmark we decided not to increase the number of links as PostgreSQL will not be able to handle it at all in timeframe what GitHub Actions limit allows to use for free. Remember that to get accurate result we ran this benchmark multiple times.

## Task

Both databases used to store and retrieve doublet-links representation. To support storage, and all basic CRUD operations that provide Turing completeness for links as in [the links theory](https://habr.com/ru/articles/895896).

## Operations
- **Create** – insert point link (link with id = source = target)
- **Update** – basic link update operation
- **Delete** – basic link delete operation
- **Each All** – take all links matching `[*, *, *]` constraint
- **Each Incoming** – take all links matching `[*, *, target]` constraint
- **Each Outgoing** – take all links matching `[*, source, *]` constraint
- **Each Concrete** – take all links matching `[*, source, target]` constraint
- **Each Identity** – take all links matching `[id, *, *]` constraint

## Results
The results below represent the amount of time (ns) the operation takes per iteration.
- First picture shows time in a pixel scale (for doublets just minimum value is shown, otherwise it will be not present on the graph).
- Second picture shows time in a logarithmic scale (to see diffrence clearly, because it is around 2-3 orders of magnitude).

### Rust
![Image of Rust benchmark (pixel scale)](https://github.com/linksplatform/Comparisons.PostgreSQLVSDoublets/blob/gh-pages/Docs/bench_rust.png?raw=true)
![Image of Rust benchmark (log scale)](https://github.com/linksplatform/Comparisons.PostgreSQLVSDoublets/blob/gh-pages/Docs/bench_rust_log_scale.png?raw=true)

## Conclusion

As we can see in this comparison, Doublets are around 200+ faster than PostgreSQL in write operations, and 1000+ faster in read operations.

To get fresh numbers, please fork the repository and rerun benchmark in GitHub Actions.
