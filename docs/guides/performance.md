---
title: Performance and scaling
status: preview
---

Scaling starts with knowing which resource limits useful work. More application instances do not fix every slow query, and a faster handler does not make a slow external service respond sooner.

## Measurement

Measure latency distributions and throughput under representative requests. Separate compilation time, startup time, and steady-state request behavior; they describe different user experiences.

## Concurrency and shared resources

Async execution allows requests to overlap while waiting. Pools and downstream services still have finite capacity. Excess concurrency can increase waiting and memory use without increasing completed work.

## Several instances

Shared sessions, cache consistency, uploads, and background work need explicit coordination when requests can reach different processes. A deployment should be tested under the failure conditions it is intended to tolerate.

## Related reading

- [Async and shared state](../../guides/async-state/).
- [Query performance](../../guides/query-performance/).
- [Caching](../../caching/).
