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

### Raw benchmark results (all numbers are in nanoseconds)

| Operation     | Doublets United Volatile | Doublets United NonVolatile | Doublets Split Volatile | Doublets Split NonVolatile | PSQL NonTransaction | PSQL Transaction |
|---------------|--------------------------|-----------------------------|-------------------------|----------------------------|---------------------|------------------|
| Create        | 98074 (10163.6+ times faster) | 101753 (9796.1+ times faster) | 83158 (11986.6+ times faster) | 83555 (11929.7+ times faster) | 1418444409          | 996785265        |
| Update        | 393717 (3328.3+ times faster) | 395531 (3313.0+ times faster) | 83090 (15770.8+ times faster) | 83222 (15745.8+ times faster) | 1708932179          | 1310397499       |
| Delete        | 193643 (1746.4+ times faster) | 184686 (1831.1+ times faster) | 144773 (2336.0+ times faster) | 144322 (2343.3+ times faster) | 516439248           | 338185472        |
| Each All      | 14991 (110.0+ times faster) | 16263 (101.4+ times faster) | 16860 (97.8+ times faster) | 15970 (103.2+ times faster) | 1717107             | 1648433          |
| Each Identity | 102612 (9614.5+ times faster) | 102701 (9606.2+ times faster) | 101771 (9694.0+ times faster) | 101764 (9694.7+ times faster) | 1030793967          | 986567072        |
| Each Concrete | 232170 (4482.9+ times faster) | 231432 (4497.2+ times faster) | 133134 (7817.6+ times faster) | 132956 (7828.0+ times faster) | 1085583098          | 1040785456       |
| Each Outgoing | 384706 (2590.4+ times faster) | 382143 (2607.8+ times faster) | 120126 (8295.9+ times faster) | 121173 (8224.2+ times faster) | 1048393281          | 996555891        |
| Each Incoming | 386936 (2551.7+ times faster) | 390969 (2525.4+ times faster) | 120659 (8182.9+ times faster) | 120620 (8185.6+ times faster) | 1039836582          | 987344012        |

## Conclusion

As we can see in this comparison, Doublets are from 1746 to 15745 times faster than PostgreSQL in write operations, and from 100 to 9694 times faster in read operations.

To get fresh numbers, please fork the repository and rerun benchmark in GitHub Actions.
