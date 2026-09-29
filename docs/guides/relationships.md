---
title: Model relationships
status: preview
---

A relationship connects records that have meaning together. In our shop, an order belongs to a customer, and each order line refers to a product. Storing those links lets us ask which customer placed an order without copying all the customer's details into every row.

Cot provides `ForeignKey` for model relationships. The model definition describes the stored link; loading related values and choosing deletion behavior are additional decisions.

## A customer and their orders

Here is a small relationship between a customer and an order:

```rust
use cot::db::{Auto, ForeignKey, model};

#[model]
struct Customer {
    #[model(primary_key)]
    id: Auto<i64>,
    name: String,
}

#[model]
struct Order {
    #[model(primary_key)]
    id: Auto<i64>,
    customer: ForeignKey<Customer>,
    total_cents: i64,
}
```

The foreign key represents which customer the order belongs to. The database schema needs the corresponding migration; adding a field to Rust source doesn't update an existing database by itself.

A relationship is not an authorization rule. Knowing that order 42 points to customer 7 doesn't establish that the current request belongs to customer 7. The handler must make that comparison against trusted authentication data.

## Identity and snapshots

Some values should follow the current related record; others should preserve history. A customer's current display name might be suitable for an account page. The address used for a shipped order needs to remain the address used at purchase time.

Likewise, an order line should usually retain its purchased unit price. If it only reads the product's current price, a later catalog update would change the apparent value of an old order. The foreign key keeps identity; a snapshot field preserves a historical fact.

## One-to-many and many-to-many data

One customer can have many orders because many rows can point to the same customer. A product can also appear in many orders. An order-line model makes that second relationship explicit and provides a home for quantity and purchased price.

An association that has its own data deserves its own model. Membership in a team may need a role and joined-at timestamp; a bare list of user IDs doesn't express those facts clearly.

Use the current model and query APIs to represent and retrieve these rows. Do not assume that a relationship automatically creates Django-style reverse managers or Laravel-style dynamic properties in Cot.

## Loading related records

A page that displays 20 orders and independently fetches one customer per order can issue many small queries. Fetching relationships deliberately makes this cost visible. [Query performance](../../guides/query-performance/) discusses how to measure and reduce that repeated work.

Choose the loading strategy around the page's actual needs. A summary list may need only an order ID and customer name. A detail page may need the customer, lines, and delivery information. Loading every relationship for every view can waste more work than it saves.

## Missing and deleted records

Decide what should happen when the referenced record is removed. Preventing deletion, deleting dependent records, and retaining an optional link express different business rules. Confirm the constraints emitted by the migration rather than assuming a default from another ORM.

For order history, deleting a customer account may require retaining some order facts while removing personal information. That is a data-retention design, not merely a choice of foreign-key type.

## Keeping related writes consistent

Creating an order header and its lines is one logical operation. If only half the lines are saved, the stored order can be misleading. Use a transaction where those writes must succeed or fail together, and keep unrelated network calls outside the database transaction where possible.

Continue with [queries](../../databases/queries/), [migrations](../../databases/migrations/), and [transactions](../../databases/transactions/) for the supported operations.
