# Measured GraphQL comparison

3000 background point links; 1000 sequential iterations per server; one virtual user.
Times are HTTP response duration in milliseconds, not database-only time.
Create includes two HTTP mutations on both servers; other operations use one.

| Operation | Hasura mean | Doublets mean | Hasura p95 | Doublets p95 |
|---|---:|---:|---:|---:|
| Create | 2.9257 | 0.3084 | 3.1794 | 0.4237 |
| Update | 1.4204 | 0.1456 | 1.6089 | 0.2068 |
| EachAll | 1.6278 | 2.5068 | 1.7994 | 3.4777 |
| EachIdentity | 0.7522 | 0.3697 | 0.9754 | 0.5670 |
| EachConcrete | 0.4893 | 0.2305 | 0.6308 | 0.3423 |
| EachOutgoing | 0.4151 | 0.1764 | 0.5163 | 0.2585 |
| EachIncoming | 0.4043 | 0.1571 | 0.4809 | 0.2285 |
| Delete | 1.5389 | 0.1797 | 1.7309 | 0.2290 |

Each response was checked for GraphQL errors, row count and returned values.
These local measurements describe this run and host; repeat runs before drawing performance conclusions.
