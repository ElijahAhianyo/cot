---
title: Background tasks
status: proposed
---

Background tasks let a request hand work to a worker instead of keeping the caller waiting for the work to finish. An order-history export is a useful example: preparing thousands of rows can take longer than the browser should wait for an ordinary response.

This page describes the proposed durable task system. Cot's current CLI tasks and an in-process async spawn are different mechanisms; neither should be mistaken for the queue contract described here.

## Task, queue, and worker

A task describes the work. A queue stores pending work. A worker claims a task, executes it, and records the outcome. The request's responsibility is to establish that the work was accepted durably before telling the user that an export is underway.

An illustrative task payload is deliberately small:

```text
Task: export_orders
Task ID: export-73
Customer ID: 7
Cutoff: 2026-09-01T00:00:00Z
Format: CSV
Payload version: 1
```

The payload contains owned values, not a borrowed HTTP request or an open transaction. A worker loads the resources it needs when it executes the task. Decide whether it should use current account permissions or a deliberately captured authorization context; don't leave that question implicit.

## Accepted is not complete

After acceptance, the application can return an export ID and a status URL. The status may move through pending, running, completed, and failed states. A completed export also needs a defined file location, access policy, and retention period.

If the queue is unavailable, returning “export started” would be misleading. If the response is lost after acceptance, a client retry needs a way to recover the original export rather than create another one.

## Retries and idempotency

A worker can finish a side effect and crash before recording success. When the task is delivered again, repeating the side effect may create a second file, send another email, or charge a customer twice.

Idempotency means recognizing repeated execution as the same operation. For the export, use its stable task identity to find an existing result or safely replace an incomplete attempt. For external providers, use their supported idempotency mechanism and understand its retention period.

Retries do not imply exactly-once execution. Set a maximum attempt policy and distinguish temporary failures from permanent input or permission failures.

## Transactions and dispatch

Enqueuing work before a database transaction commits can let a worker observe data that does not exist yet. Enqueuing after commit leaves another gap: the commit can succeed and the process can stop before dispatch.

An outbox is one possible design. The application records the intended event in the same database transaction as the order; a separate dispatcher transfers it to the queue. The dispatcher still needs repeat-safe delivery. This is an architectural pattern, not a built-in Cot API.

## Timeouts and ownership

A worker timeout and a queue's claim timeout must agree. If the claim expires while the original worker is still active, two workers can execute the same task. Long operations may need a supported lease-renewal protocol or smaller units of work.

A process-local mutex cannot coordinate workers on different machines. Use the queue or shared backend's documented mechanism.

## Deployments and failed work

Workers can receive tasks written by an older application revision. Include a payload version when the shape may change, and plan compatibility while old and new workers overlap. Don't delete fields that queued work still needs without a transition.

Operators need attempt history, terminal failures, and a controlled retry path. Users need a visible status and a useful next action. See [scheduled tasks](../../guides/scheduling/), [events](../../guides/events/), and [queued email](../../how-to/queued-email/) for related designs.
