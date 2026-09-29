---
title: Seeding and test data
status: proposed
---

Seeding supplies a known set of data for an environment. We might need three products for a demonstration, standard order states for an installation, or carefully chosen customers for a test. Those uses look similar, but they should not all run the same way.

This page describes the proposed seeding design. The examples are data and behavior contracts; they do not imply that a `cot seed` command or factory API is available today.

## Reference data and demonstration data

Reference data is part of the application's meaning. For example, an order-state code used by application logic may need a stable value. Demonstration data exists to make a development environment useful and can be replaced more freely.

For our shop, the split could look like this:

| Dataset | Examples | Expected environment |
| --- | --- | --- |
| Reference | Stable delivery-method codes | Environments that use those methods |
| Demonstration | Sample notebooks and customers | Local development and demos |
| Test fixture | One out-of-stock product | The test that needs that case |

A production database should not receive demonstration customer accounts merely because both datasets are called seeds.

## Stable identity

A repeatable seed needs a way to recognize a row it created earlier. For a demonstration product, an application-controlled code such as `DEMO-NOTEBOOK` can serve that purpose. A display name is a poor identity because someone may edit it.

The intended operation can be expressed without choosing a framework API:

```text
Find product by code DEMO-NOTEBOOK.
If absent, create it with the demonstration defaults.
If present, update only the fields owned by this seed.
Report whether the row was created, updated, or unchanged.
```

The important part is the ownership rule. Rerunning a seed should not overwrite a staff member's edited description unless the seed explicitly owns that description.

## Dependencies and transactions

Orders depend on customers and products. Seed those prerequisites before the order rows, and keep a useful record of failures. A seed that stops halfway through should be safe to rerun rather than requiring someone to manually guess which rows were created.

Use transactions when a group must become visible together. For larger datasets, bounded batches can reduce lock duration and memory use, but the recovery behavior needs to be defined across batch boundaries.

## Deterministic test data

Tests benefit from small, explicit fixtures. A pagination test needs enough rows to cross a page boundary; it doesn't need a random copy of the whole shop. Name the important conditions so a failure can be understood without examining a thousand generated records.

Random generation can be useful for broader exploration, but record its seed and retain the failing input. Otherwise an intermittent failure may be impossible to reproduce.

## Side effects

Creating sample orders should not send real emails, charge payment methods, or notify production webhooks. Use isolated service configuration and make the side-effect policy part of the seed operation.

A test factory should also distinguish building an in-memory value from persisting it. A helper whose name hides a database write can make tests unexpectedly slow or dependent on global setup.

## Production safeguards and reporting

A production data operation should identify its target environment, require the intended dataset explicitly, and report useful counts without exposing personal values. A dry-run mode is only trustworthy when it exercises the same selection rules without performing writes.

Until the proposed seeding interface exists, small application-owned management commands and test setup can use Cot's verified model APIs. See [management commands](../../guides/management-commands/) and [database tests](../../guides/database-tests/).
