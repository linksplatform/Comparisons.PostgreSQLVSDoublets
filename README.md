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

### GraphQL
![Image of GraphQL benchmark (pixel scale)](https://github.com/linksplatform/Comparisons.PostgreSQLVSDoublets/blob/gh-pages/Docs/bench_graphql.png?raw=true)
![Image of GraphQL benchmark (log scale)](https://github.com/linksplatform/Comparisons.PostgreSQLVSDoublets/blob/gh-pages/Docs/bench_graphql_log_scale.png?raw=true)

### Raw benchmark results (all numbers are in nanoseconds)

| Operation     | Doublets United Volatile | Doublets United NonVolatile | Doublets Split Volatile | Doublets Split NonVolatile | PSQL NonTransaction | PSQL Transaction |
|---------------|--------------------------|-----------------------------|-------------------------|----------------------------|---------------------|------------------|
| Create        | 9204 (11056.4x faster)   | 9125 (11152.1x faster)      | 9478 (10736.8x faster)  | 8226 (12370.9x faster)     | 150151369           | 101763082        |
| Update        | 34157 (3869.8x faster)   | 33954 (3892.9x faster)      | 8291 (15942.5x faster)  | 8266 (15990.8x faster)     | 182868706           | 132179627        |
| Delete        | 17329 (2013.3x faster)   | 17451 (1999.3x faster)      | 14195 (2457.8x faster)  | 14601 (2389.5x faster)     | 58448123            | 34889131         |
| Each All      | 4708 (160.2x faster)     | 5011 (150.5x faster)        | 5316 (141.9x faster)    | 5629 (134.0x faster)       | 793277              | 754115           |
| Each Identity | 34190 (9780.9x faster)   | 34208 (9775.8x faster)      | 33923 (9857.9x faster)  | 33896 (9865.8x faster)     | 353408454           | 334410191        |
| Each Concrete | 57288 (6222.6x faster)   | 57067 (6246.7x faster)      | 44231 (8059.5x faster)  | 44280 (8050.5x faster)     | 372416796           | 356478290        |
| Each Outgoing | 91274 (3713.8x faster)   | 103925 (3261.7x faster)     | 39869 (8502.1x faster)  | 39901 (8495.3x faster)     | 358537556           | 338972159        |
| Each Incoming | 87745 (3824.4x faster)   | 104678 (3205.7x faster)     | 40089 (8370.6x faster)  | 40144 (8359.2x faster)     | 353697010           | 335569920        |

## Conclusion

As we can see in this comparison, Doublets are from 1746 to 15745 times faster than PostgreSQL in write operations, and from 100 to 9694 times faster in read operations.

To get fresh numbers, please fork the repository and rerun benchmark in GitHub Actions.
