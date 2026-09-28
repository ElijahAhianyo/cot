---
title: Seeding and test data
status: proposed
---

Seed data gives an application a useful starting state. Required permission records, sample projects for development, and test fixtures have different purposes and should not be treated as one dataset.

## Required and sample data

A required lookup value belongs to the application’s operational data. A sample customer exists only to make development convenient. Production setup should not accidentally load demonstration accounts.

## Repeatable execution

A seed operation may run more than once. A future seeding interface should define whether existing records are preserved, updated, or rejected. Repeating the operation should not silently duplicate required data.

## Factories and fixtures

Factories generate records for tests; fixtures describe known records. Neither automatically belongs in a production seed. Relationships and unique constraints need to remain valid in each case.

## Related reading

- [Migrations](../../databases/migrations/).
- [Database tests](../../guides/database-tests/).
