---
title: Pagination
status: preview
---

Pagination returns a bounded part of a result set. A catalog with 20 products and one with 200,000 products should not require the browser to download every row before it can show the first page.

We'll use a product list ordered by ID. Cot's query interface exposes `limit` and `offset`; an application can build its page contract on those operations. Cursor pagination below describes an application design, not a built-in paginator API.

## Limiting and skipping rows

```rust
use cot::db::{Auto, Database, Model, model};
use cot::db::query::expr::ExprSort;

#[model]
struct Product {
    #[model(primary_key)]
    id: Auto<i64>,
    name: String,
}

async fn second_page(db: &Database) -> cot::Result<Vec<Product>> {
    let products = Product::objects()
        .order_by([<Product as Model>::Fields::id.asc()])
        .limit(20)
        .offset(20)
        .all(db)
        .await?;
    Ok(products)
}
```

This skips the first 20 ordered rows and retrieves at most the next 20. The database applies the limit; the application doesn't load the complete table and discard most of it afterward.

## Page numbers and bounds

For one-based page numbers, the conceptual offset is `(page - 1) * page_size`. Validate before calculating it. Page zero, an enormous page number, and an overflowing multiplication need a defined response.

A page-size parameter also needs an upper bound. A client asking for a million rows shouldn't bypass the limit that makes the endpoint usable. Choose the bound around row size and expected use, and document whether excessive values are rejected or reduced.

## Stable ordering

Pagination needs deterministic ordering. Sorting by product name alone leaves ties: two products called “Notebook” can move relative to one another. Add a unique tie-breaker such as the primary key.

When a user changes a filter or sort order, start a new page sequence. A cursor or offset from a price-sorted catalog cannot be meaningfully reused against a name-sorted catalog without defining that behavior.

## Changes between pages

Even stable ordering doesn't freeze the dataset. If a product is inserted before the current offset, a later page can repeat a row. If an earlier row is removed, a later page can skip one.

For a casual catalog, that tradeoff may be acceptable. For an audit export, it may not be. A consistent snapshot, a cutoff, or a cursor-based design may be more appropriate, depending on the database and business requirement.

## Cursor pagination

A cursor describes where to continue in an ordered result. For an ascending unique ID, it can represent “return rows after ID 42.” A compound sort needs all relevant values, such as the last price and ID, so ties are handled correctly.

Treat a cursor as untrusted input. Validate its shape and bind it to the filters and ordering it represents. Encoding data in base64 doesn't make it tamper-proof or establish permission to read the rows it selects.

## Counts and empty pages

An exact total can require another query and may be expensive. Sometimes “next page available” is enough; an application can request one additional row to determine that without computing the full count.

An empty first page means no matching results. An empty later page can mean that the data changed or the client requested a page beyond the end. Provide a useful way back rather than presenting the situation as a server failure.

For browser pages, preserve active filters in next/previous links. For APIs, document the item order and continuation fields. [Query performance](../../guides/query-performance/) explains how to measure the database cost of either design.
