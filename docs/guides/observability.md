---
title: Logging, tracing, and health
status: preview
---

A response tells a client what happened. Logs, traces, and metrics help an operator understand why it happened and whether the system is behaving as expected.

## Connecting events

A request identifier and tracing spans can connect work across handlers and services. Structured fields make it possible to filter related events without parsing prose messages.

## Choosing useful signals

Latency, error rates, queue age, and database pool pressure answer different operational questions. Labels with unbounded values, such as raw user input, can make a metric costly and difficult to use.

## Health checks

Liveness asks whether a process should be restarted. Readiness asks whether it should receive traffic. A transient dependency problem should not automatically create an endless restart cycle.

## Private information

Logs and traces can outlive the request and reach other systems. Credentials, session tokens, and sensitive body data need explicit handling rather than indiscriminate recording.

## Related reading

- [Error handling](../../guides/errors/).
- [Deployment architecture](../../guides/deployment/).
