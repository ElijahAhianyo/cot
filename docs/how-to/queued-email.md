---
title: Send email after a committed change
status: proposed
---

Send an order confirmation only after the order is committed, and retain enough state to retry if delivery fails. This recipe describes the target workflow for a durable background-task integration. Cot's integrated queue is proposed; use the procedure as an integration design, not as an available dispatch API.

You need a transactional database, a durable work dispatcher, an email transport, and a worker process. The example uses an outbox: a database table recording messages that still need dispatch. The outbox is application-owned until a supported integration defines it.

## Record the order and its notification together

Within one database transaction, insert the order and an outbox row. Give the outbox entry a stable event identifier and enough information to identify the order and notification type. Commit both changes together.

If the transaction rolls back, neither the order nor its notification should become visible. Do not send the email inside that transaction: a delivered message cannot be undone by a database rollback.

Decide whether the email describes the order at checkout or its state when the worker runs. For a checkout receipt, retain the relevant immutable order details. Loading mutable prices later can produce a receipt that disagrees with what the customer bought.

## Dispatch pending entries

Run a dispatcher that finds committed pending entries and submits work to the durable queue. Mark an entry dispatched only according to the integration's confirmed acceptance contract.

A crash can occur after acceptance but before the dispatcher records it. Expect that entry to be submitted again. The stable event identifier lets the worker recognize repeated delivery; relying on the dispatcher to run exactly once does not close this gap.

## Send from the worker

Load the event and its delivery state, construct the message, and send through the configured transport. Keep recipient addresses and message content out of routine logs. Record an event ID, attempt count, and a useful outcome instead.

Classify failures. A temporary transport outage may be retryable; an invalid recipient or configuration error needs a different resolution. Use bounded retries and delay repeated attempts so an unavailable service is not overwhelmed.

## Define duplicate behavior

A worker can send successfully and crash before marking the event complete. Retrying may send another email. A local “sent” flag alone cannot make a remote side effect exactly once.

If the provider supports an idempotency key, use the event's stable identity and understand the provider's retention window. Otherwise, decide how your application handles possible duplicates. A repeated receipt is often less harmful than a missing one; a one-time action link needs its own single-use server-side semantics.

## Exercise the failure boundaries

| Test | Expected observation |
| --- | --- |
| Roll back the order transaction | No order confirmation becomes dispatchable |
| Stop the dispatcher after queue acceptance | Repeated dispatch remains safe under the chosen policy |
| Stop the worker after sending | Duplicate-delivery behavior is understood and tested |
| Make the mail transport unavailable | Attempts are visible and retries are bounded |
| Exhaust retries | Operators can inspect and deliberately retry or dismiss the event |

Track the age of the oldest pending notification as well as failures. A dispatcher that never runs can produce no errors while customers receive no mail.

See [transactions](../../databases/transactions/), [background tasks](../../guides/background-tasks/), and [sending email](../../sending-emails/) for the separate responsibilities in this workflow.
