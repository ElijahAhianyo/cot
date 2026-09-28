---
title: Send email after a committed change
status: proposed
---

This task outline describes sending an order confirmation through a future background-task integration. Cot does not currently provide the background-task framework assumed by the outline.

## Commit the application change

Record the order before making it visible to a worker. An email task that sees an uncommitted or rolled-back order cannot reliably describe what happened.

## Record durable work

A mature implementation needs a supported way to associate committed data with work that will be dispatched. The exact transaction or outbox interface is intentionally left open in this prototype.

## Verify retries

Simulate a temporary delivery failure and a duplicate task attempt. Confirm that an operator can distinguish pending, completed, and failed work and that repeated attempts follow the application's duplicate-delivery policy.

See [background tasks](../../guides/background-tasks/), [transactions](../../databases/transactions/), and [sending email](../../sending-emails/).
