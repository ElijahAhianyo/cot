---
title: Pagination
status: preview
---

Pagination divides a result set into smaller responses. Its correctness depends on ordering: without a stable order, a record can appear twice or disappear between pages even when the page size stays the same.

## Offsets and cursors

Offset pagination describes where a page begins in a result set. Cursor pagination describes a position using record values. They have different tradeoffs for random access, large datasets, and changing data.

## Stable ordering

Ordering issues by creation time can leave ties. A unique secondary key makes the order deterministic. The continuation position must account for the same ordering used by the query.

## Boundaries and changes

Empty pages, deleted records, and new inserts are normal conditions. This preview explains the contract a future pagination guide should establish; it does not introduce a Cot paginator API.

## Related reading

- [Queries](../../databases/queries/).
