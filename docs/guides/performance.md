---
title: Performance and scaling
status: preview
---

Performance is the cost of delivering a useful result. For a catalog page, that includes database work, rendering, response size, and the time the browser spends loading assets. A faster handler alone may not make the page noticeably faster.

Start with a concrete goal. “The first catalog page should remain responsive with the expected concurrent visitors” gives us a workload and an outcome to measure. “Make Cot fast” does not.

## Establish a baseline

Measure a representative release build with realistic data. Record response latency, error rate, throughput, memory, and the dependencies involved. Keep the workload repeatable so a later measurement can be compared fairly.

Include cold and warm behavior when both matter. A page that is fast only after a cache is populated may still be slow after every deployment or idle restart.

## Find where time goes

| Observation | Possible next investigation |
| --- | --- |
| Many short queries per page | Relationship loading and repeated lookups |
| Few queries but long database time | Execution plans, indexes, and locks |
| Low CPU with slow responses | I/O waits, connection pools, or external calls |
| High CPU with small responses | Serialization, templates, compression, or computation |
| Rising memory with traffic | Unbounded bodies, result sets, caches, or buffers |

These are hypotheses, not diagnoses. Use traces and controlled changes to confirm them. [Query performance](../../guides/query-performance/) covers database-specific work.

## Bound work per request

Limit collection queries, upload sizes, and external calls. A single endpoint that accepts an arbitrarily large batch can consume the capacity intended for many ordinary visitors.

Async execution helps overlap waits; it does not remove resource limits. Starting more concurrent work can increase contention and latency once the database or provider is saturated.

## Cache with a freshness policy

A cache saves repeated work at the cost of deciding when a result is stale. For a catalog, a short delay in a descriptive field may be acceptable; a checkout stock decision may need authoritative data.

Key cached results by every dimension that changes their meaning, including authorization scope or locale where relevant. Define the miss path and backend-failure path. A cache outage shouldn't trigger unlimited identical expensive recomputations without a plan.

See [caching](../../caching/) for the implemented interfaces. A cache should follow an understood access pattern, not hide an unexplained query problem.

## Scale the limiting resource

Adding web instances helps only if the web layer is the constraint. If all instances wait on one saturated database, adding more can make the database queue longer. Connection limits, background workers, and request concurrency must fit the shared capacity.

Process-local state also changes with replication. A local cache may duplicate work; a local session store may break login continuity. Performance and correctness need to be reviewed together.

## Measure the browser too

Large images, blocking scripts, and excessive asset requests can dominate perceived page time. Serve appropriately sized media and use release-compatible asset URLs. A server benchmark that discards the response body doesn't measure this experience.

## Keep improvements accountable

After a change, rerun the same workload and compare both speed and correctness. Keep the measurements with the change's rationale. Avoid broad claims such as “twice as fast” without stating the workload, environment, and metric.

The [observability guide](../../guides/observability/) explains the evidence, while [deployment](../../guides/deployment/) covers resource boundaries.
