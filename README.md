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
| Create        | 9035 (12162.4x faster)   | 8807 (12477.2x faster)      | 7005 (15686.9x faster)  | 7086 (15507.6x faster)     | 145975967           | 109886881        |
| Update        | 33210 (4323.7x faster)   | 32724 (4387.9x faster)      | 7826 (18347.9x faster)  | 7756 (18513.5x faster)     | 181785688           | 143590948        |
| Delete        | 17427 (2096.8x faster)   | 17237 (2119.9x faster)      | 12612 (2897.4x faster)  | 12659 (2886.6x faster)     | 54113187            | 36541411         |
| Each All      | 1903 (394.1x faster)     | 1903 (394.1x faster)        | 2520 (297.6x faster)    | 2220 (337.8x faster)       | 772260              | 750025           |
| Each Identity | 32364 (10720.2x faster)  | 32376 (10716.2x faster)     | 32675 (10618.2x faster) | 32678 (10617.2x faster)    | 365907001           | 346948519        |
| Each Concrete | 47228 (7710.2x faster)   | 46435 (7841.9x faster)      | 42978 (8472.6x faster)  | 43041 (8460.2x faster)     | 388005509           | 364137549        |
| Each Outgoing | 60270 (5830.3x faster)   | 59978 (5858.7x faster)      | 36132 (9725.2x faster)  | 36206 (9705.3x faster)     | 369609106           | 351390605        |
| Each Incoming | 59812 (5849.0x faster)   | 59956 (5835.0x faster)      | 36733 (9523.9x faster)  | 36753 (9518.7x faster)     | 362811363           | 349842592        |

## Conclusion

As we can see in this comparison, Doublets are from 1746 to 15745 times faster than PostgreSQL in write operations, and from 100 to 9694 times faster in read operations.

To get fresh numbers, please fork the repository and rerun benchmark in GitHub Actions.
