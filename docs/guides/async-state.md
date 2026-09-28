---
title: Async execution and shared state
status: preview
---

An async handler can wait for I/O without occupying a thread for the duration of that wait. It does not make every operation inside the handler nonblocking, and it does not make shared mutable data safe automatically.

## Waiting versus blocking

Waiting for a database query and performing a large CPU calculation use runtime resources differently. CPU-heavy processing or a blocking library call can delay unrelated requests when it occupies an async worker thread.

## Shared and request-local state

A connection pool is shared across requests. A submitted form and a path parameter belong to one request. Rust’s ownership and `Send`/`Sync` requirements help make these boundaries explicit, but the application still needs a concurrency policy.

## Cancellation and durable work

A disconnected client may no longer need a response, but an external side effect may already have happened. Retriable operations need a way to distinguish an operation that never started from one that succeeded before the connection ended.

## Related reading

- [Transactions](../../databases/transactions/).
- [Background tasks](../../guides/background-tasks/).
