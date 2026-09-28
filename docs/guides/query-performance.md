---
title: Query performance
status: preview
---

A fast handler can still wait on an expensive query. Query performance starts with the work the database performs and how often the application asks it to perform that work.

## Query counts

Rendering a list may trigger more queries than the handler visibly contains. Measuring query counts reveals repeated related-record lookups and work hidden in helper functions.

## Indexes and query shape

Indexes help some filters and sort orders, but also consume space and affect writes. An execution plan provides evidence about a particular query and dataset; an index is not a universal speed switch.

## Batching and measurement

A batch can reduce round trips while increasing memory use or transaction duration. Compare the behavior under representative data rather than treating a microbenchmark as a production guarantee.

## Related reading

- [Queries](../../databases/queries/).
- [Transactions](../../databases/transactions/).
