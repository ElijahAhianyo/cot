---
title: Logging, tracing, and health
status: preview
---

Observability helps us explain what the application did. A customer reporting “my order failed” needs more investigation than a count of 500 responses. We need to connect the request to the database operation, external calls, and any work that continued afterward.

We'll use a request to place an order. The examples below describe useful diagnostic records; they do not imply that Cot automatically configures a complete metrics or tracing backend.

## Logs describe events

A structured log separates fields from the message. Instead of hiding everything inside a sentence, record a stable event name and relevant identifiers:

```text
Event: order.create.failed
Request ID: req-73
Order attempt ID: attempt-42
Dependency: database
Outcome: timeout
Elapsed milliseconds: 1200
```

These values are illustrative. Avoid recording passwords, session cookies, bearer tokens, or full payment payloads. An identifier is useful only if its disclosure is acceptable for the log's audience and retention policy.

## Correlating a request

Assign or validate a request identifier at the entry boundary and include it in relevant diagnostics. If the client supplies an identifier, bound and validate it rather than allowing arbitrary content into logs.

A public error response can include a safe reference that support staff use to locate internal details. The reference should not encode the secret, query, or stack trace that it is meant to help find.

## Traces describe relationships

A trace connects spans of work: request handling, database queries, provider calls, and rendering. It helps distinguish a slow query from time spent waiting for a connection or retrying a provider.

Async work can move between executor threads, so thread identity alone is not enough to correlate it. Propagate context through the chosen instrumentation interfaces. For background work, connect the task to its initiating operation while preserving its own execution identity.

## Metrics describe patterns

Measure traffic, latency, errors, and saturation. For latency, distributions and percentiles reveal slow requests that an average can hide. For saturation, connection-pool waits and queued work may explain why CPU usage is low while responses are slow.

Keep metric labels bounded. Using every raw URL, customer ID, or request ID as a metric label can create an enormous number of series. A route pattern is often more useful than a path containing a unique order ID.

## Health checks

Liveness asks whether the process is operating. Readiness asks whether it should receive traffic. A deep diagnostic endpoint may check more dependencies, but that doesn't mean every dependency belongs in a readiness probe.

For example, an email-provider outage may delay confirmations while catalog browsing still works. Removing every web instance from service could turn a partial outage into a complete one. Define the intended degraded behavior first.

## Alerts and investigation

An alert should identify a condition that needs action. A single rejected login is usually expected behavior; a sustained rise in database failures is different. Connect alerts to a runbook with the relevant dashboards, safe checks, and recovery choices.

When investigating, compare the time of the symptom with releases, configuration changes, and dependency events. Avoid treating correlation as proof; use request-level evidence to test the explanation.

## Testing diagnostics

Force a known failure in a controlled environment and confirm that its response reference leads to the expected logs. Verify that private fields are absent. An observability setup that has never been exercised may fail precisely when it is needed.

See [error handling](../../guides/errors/), [performance](../../guides/performance/), and [recovery](../../guides/recovery/) for how to use the evidence.
