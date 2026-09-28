---
title: Background tasks
status: proposed
---

Some work does not need to finish before an HTTP response is returned. Sending a notification or generating a large export can continue independently, provided the work is recorded durably and its outcome can be observed.

## Tasks, queues, and workers

A task describes work. A queue records pending work. A worker claims and executes it. Returning a successful enqueue response should mean that the work has been accepted, not that it has already succeeded.

## Retries and idempotency

A worker can finish an external operation and fail before recording completion. A retry may then run the operation again. An idempotency key or another domain-specific check can prevent duplicate effects; retries alone cannot guarantee exactly-once delivery.

## Transactions and dispatch

Enqueuing an email before its order transaction commits can expose a record that is later rolled back. A mature integration needs a deliberate boundary between committed data and dispatched work.

## Failure and visibility

Operators need to see attempts, terminal failures, and work waiting too long. An export should have a visible status and a recoverable failure, rather than leave the user waiting indefinitely.

## Illustrative lifecycle

| Stage | Export example |
| --- | --- |
| Accepted | Record the requested date range and requester. |
| Running | A worker produces the export. |
| Retrying | A temporary storage failure schedules another attempt. |
| Completed | Save the result and notify the requester. |
| Failed | Record an actionable terminal error. |

## Related reading

- [Sending email](../../sending-emails/).
- [Transactions](../../databases/transactions/).
- [Scheduled tasks](../../guides/scheduling/).
