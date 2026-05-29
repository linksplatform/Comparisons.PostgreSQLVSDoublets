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
- Second picture shows time in a logarithmic scale (to see difference clearly, because it is around 2-3 orders of magnitude).

### Rust
![Image of Rust benchmark (pixel scale)](https://github.com/linksplatform/Comparisons.PostgreSQLVSDoublets/blob/main/Docs/bench_rust.png?raw=true)
![Image of Rust benchmark (log scale)](https://github.com/linksplatform/Comparisons.PostgreSQLVSDoublets/blob/main/Docs/bench_rust_log_scale.png?raw=true)

### Raw benchmark results (all numbers are in nanoseconds)

| Operation     | Doublets United Volatile | Doublets United NonVolatile | Doublets Split Volatile | Doublets Split NonVolatile | PSQL NonTransaction | PSQL Transaction |
|---------------|--------------------------|-----------------------------|-------------------------|----------------------------|---------------------|------------------|
| Create        | 8780 (12614.9x faster)   | 9125 (12137.9x faster)      | 8296 (13350.9x faster)  | 8270 (13392.8x faster)     | 144051115           | 110758727        |
| Update        | 33773 (4264.4x faster)   | 33670 (4277.4x faster)      | 8208 (17546.3x faster)  | 8236 (17486.7x faster)     | 179650014           | 144020140        |
| Delete        | 16799 (2200.1x faster)   | 17008 (2173.1x faster)      | 14901 (2480.4x faster)  | 16508 (2238.9x faster)     | 52939761            | 36960301         |
| Each All      | 5012 (156.4x faster)     | 5627 (139.3x faster)        | 5307 (147.7x faster)    | 5589 (140.2x faster)       | 795313              | 783693           |
| Each Identity | 34225 (10100.2x faster)  | 34243 (10094.9x faster)     | 34256 (10091.1x faster) | 34245 (10094.3x faster)    | 364256566           | 345680880        |
| Each Concrete | 58809 (6206.2x faster)   | 58118 (6279.9x faster)      | 44626 (8178.6x faster)  | 44674 (8169.8x faster)     | 384697270           | 364978052        |
| Each Outgoing | 89110 (3918.3x faster)   | 89121 (3917.9x faster)      | 40241 (8676.8x faster)  | 40243 (8676.4x faster)     | 368437596           | 349163623        |
| Each Incoming | 86241 (4053.8x faster)   | 86548 (4039.5x faster)      | 40536 (8624.6x faster)  | 40889 (8550.1x faster)     | 360478187           | 349606577        |

## Conclusion

As we can see in this comparison, Doublets are from 1746 to 15745 times faster than PostgreSQL in write operations, and from 100 to 9694 times faster in read operations.

To get fresh numbers, please fork the repository and rerun benchmark in GitHub Actions.
