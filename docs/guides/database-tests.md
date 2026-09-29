---
title: Database and service tests
status: preview
---

A database test verifies behavior that depends on persistence: constraints, queries, transactions, and the schema the application actually uses. A unit test with an in-memory Rust collection cannot prove that a database rejects a duplicate key or rolls back a transaction.

We'll use an order with two lines. The important invariant is that a failed second line must not leave a misleading half-created order when the operation promises atomic creation.

## Test the intended database behavior

Choose the backend around the behavior being tested. SQLite is convenient for many local tests, but a passing SQLite test doesn't establish PostgreSQL-specific locking, SQL syntax, or migration behavior.

For critical backend-specific paths, run tests against the deployment backend too. Keep those service dependencies explicit so a missing database produces a useful setup error rather than a mysterious timeout.

## Cot's test database

`TestDatabase` provides helpers for creating an isolated test database, registering migrations, and cleaning up. A minimal setup looks like this:

```rust
use cot::test::TestDatabase;

let mut test_db = TestDatabase::new_sqlite().await?;
test_db.run_migrations().await;
let db = test_db.database();
// Register the application's migrations before running them in a real test.
test_db.cleanup().await?;
```

This demonstrates the lifecycle only; it registers no application models. Tests of order queries need the order app's actual migrations added before `run_migrations`. See the [`TestDatabase`](struct@cot::test::TestDatabase) reference for the supported migration setup.

## Fixtures describe the case

A cancellation test may need one customer, one unshipped order, and one shipped order. It doesn't need the entire demonstration shop. Keep the important values close to the assertions so a reader can understand why the result should differ.

Avoid hard-coding generated primary keys unless the fixture deliberately assigns them. Use the identity returned by persistence. Otherwise adding an unrelated setup row can change which record the test accesses.

## Transactions and rollback

Test both the successful commit and a failure after an earlier write. Read the database afterward to confirm the promised state. An error return alone doesn't prove that a partial write was rolled back.

For concurrency behavior, use genuinely competing operations and the isolation model of the target backend. Running two functions one after another cannot expose the same race as overlapping transactions.

## Isolation and cleanup

Tests must not point at development or production databases. Use clearly separated configuration and unique resources when tests run in parallel. A cleanup operation should target only the resource created for that test.

Application tests using a real server may use separate connections from the test process. A transaction opened only around test setup may not isolate writes performed by those server connections. Confirm the helper's actual behavior rather than assuming rollback covers every request.

## Testing schema changes

A fresh database test proves that the current migration chain can create a schema. It doesn't prove that an existing installation with real data can upgrade. Important migrations also need representative pre-migration data and post-migration assertions.

Include values that stress the change: nulls, duplicates where relevant, long strings, and rows referenced by other tables. The goal is to test the data transformation and compatibility, not merely that a migration command exits successfully.

For application requests around the database, see [HTTP tests](../../guides/http-tests/). For deployment sequencing, see [releases and recovery](../../guides/recovery/).
