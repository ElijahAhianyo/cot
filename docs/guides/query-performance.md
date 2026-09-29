---
title: Query performance
status: preview
---

A slow page often spends its time waiting for data rather than rendering it. The useful question is not whether a query looks short in Rust, but what work the database and application perform to answer it.

We'll use an order list that shows the customer's name and a total. Start with a representative number of orders and customers; a query that feels immediate against five rows may behave differently against a year's history.

## Measure the work

Record the number of queries, their duration, the rows returned, and the total request time. These observations distinguish several problems: one expensive query, many small queries, connection-pool waits, and application work after fetching the data.

A database execution plan helps explain how a query is executed. Use the tools for your actual database backend; this page does not assume that Cot exposes an ORM method with the same name as another framework's plan viewer.

## Bound the result

A first improvement is often to ask for only the rows the page can show:

```rust
use cot::db::{Auto, Database, Model, model};
use cot::db::query::expr::ExprSort;

#[model]
struct Order {
    #[model(primary_key)]
    id: Auto<i64>,
    total_cents: i64,
}

async fn recent_orders(db: &Database) -> cot::Result<Vec<Order>> {
    Ok(Order::objects()
        .order_by([<Order as Model>::Fields::id.desc()])
        .limit(20)
        .all(db)
        .await?)
}
```

Here “recent” means descending generated ID for this example. If the application defines recency by a timestamp, query and index that actual field instead. An ID is not a universal substitute for event time.

## Repeated relationship queries

Suppose the page loads 20 orders, then performs one customer lookup for every row. That is 21 queries for one list. Repeated customers can make the waste especially visible.

Possible approaches include retrieving the required related records in a bounded query, using an appropriate join, or changing the representation so the list doesn't need that data. Choose an approach supported by the current query API; don't assume that a similarly named eager-loading helper exists in Cot.

If raw SQL is appropriate, bind external values through the parameterized APIs described in [queries](../../databases/queries/). A performance improvement must not replace safe parameter binding with string interpolation.

## Indexes follow access patterns

An index can help the database find or order rows, but it also takes storage and adds work to writes. Choose it for a measured access pattern. A list filtered by customer and ordered by creation time may need a different index from a global list ordered by total.

Inspect the generated migration and database plan. An index that exists but isn't useful for the actual predicate won't solve the slow query.

## Counts, exports, and memory

Counting all matching rows can be more expensive than retrieving the first page. Large exports need a bounded reading strategy rather than an unbounded `all` followed by one large serialization step.

Pagination, batching, and streaming each have consistency implications when rows change during the operation. Define whether an export represents a fixed cutoff or a live view before choosing a faster implementation.

## Caching after understanding the query

A cache can avoid repeated work, but it adds invalidation and freshness decisions. Caching a private order list under a shared key can produce a fast information leak. Include the relevant scope and authorization considerations in the cache design.

After changing the query, measure again with the same workload. Keep the change only when the improvement is meaningful and the resulting behavior is still correct. See [pagination](../../guides/pagination/) and [performance](../../guides/performance/) for the surrounding request-level decisions.
