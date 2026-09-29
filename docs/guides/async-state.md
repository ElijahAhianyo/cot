---
title: Async execution and shared state
status: preview
---

An async handler can wait for a database or network response without keeping an executor thread occupied by that wait. This allows other requests to make progress. It doesn't make the operation faster, and it doesn't make shared data safe by itself.

For this guide, imagine two customers checking the stock of the same product while a third places an order. All three requests may be active at once. We need to consider both how they wait and which values they share.

## Waiting for I/O

An async operation returns a future. Awaiting that future allows the operation to make progress and gives us its result when it completes. A query that has not been awaited has not supplied the rows our handler needs.

```rust
use cot::db::{Auto, Database, Model, model};

#[model]
struct Product {
    #[model(primary_key)]
    id: Auto<i64>,
    name: String,
}

async fn product_names(db: Database) -> cot::Result<Vec<String>> {
    let products = Product::objects().all(&db).await?;
    Ok(products.into_iter().map(|product| product.name).collect())
}
```

The database is a handler dependency, not a pool created inside the function. The returned names are owned strings, so they can outlive the local vector of models.

This example retrieves every product to show the ownership boundary. A real catalog should limit its query; async execution doesn't make an unbounded result cheap.

## Owned data and borrowed data

A reference is valid only while the value it refers to remains available. This becomes visible when a request starts work that may outlive the request. A borrowed header or body slice cannot be kept after its owner is gone.

Usually the work needs less data than the whole request. An export can receive a customer ID and a date range. It doesn't need to retain the customer's cookies, the request body, or a database transaction held open by the handler.

Moving owned values into a task solves a lifetime problem. It doesn't solve durability: the task still disappears if its process stops. See [background tasks](../../guides/background-tasks/) for that separate concern.

## Sharing a value

Rust's `Arc<T>` lets multiple owners refer to one allocation. It does not make arbitrary mutation safe. A mutable shared value needs an appropriate synchronization strategy, and that strategy should match the work done while access is held.

For a small in-memory counter, a short critical section may be enough. A database request performed while holding a global lock can make unrelated customers wait. Prefer taking the small amount of data needed, releasing the lock, and then doing the slow work when that preserves correctness.

Never store the current authenticated user in process-wide mutable state. Each request has its own identity; sharing the storage would mix customers.

## Blocking work

Synchronous file I/O, expensive image processing, and blocking SDK calls can occupy an executor thread. Wrapping such a function in `async fn` doesn't change the underlying operation. Use an async client where appropriate or deliberately move blocking work to a suitable execution facility.

Limit concurrency as well. Starting a thousand independent requests at once can exhaust a connection pool or overload an external service even though each request is asynchronous.

## Concurrent writes

A lock in one application process cannot protect stock updates made by another process. If two customers both observe the last item in stock, correctness must be enforced where the shared record lives, using a suitable database transaction or conditional write.

The [transaction guide](../../databases/transactions/) explains Cot's database boundary. Treat thread safety, request cancellation, and database consistency as different questions; solving one doesn't answer the others.

## Cancellation and partial work

A client disconnect can happen after a write has been committed. Don't infer that a missing HTTP response means no work happened. For operations that clients may retry, use a stable operation identity and define how the application recognizes a repeat.
