---
title: Production configuration
status: preview
---

Production settings determine what information the application exposes, which services it trusts, and how much work it will accept. They should reflect the deployment rather than inherit development convenience by accident.

## Debugging and secrets

Public error pages should not expose internal diagnostics. Secret keys and service credentials need an appropriate deployment source and a rotation plan. Cargo’s build profile and application configuration are separate controls.

## Resource limits

Connection pools, request bodies, timeouts, and concurrency limits interact. Raising one limit can move pressure to another service rather than increase useful throughput.

## Proxy trust

Forwarded scheme and client-address information should only be accepted from trusted infrastructure. The decision affects redirects, secure cookies, and any limit keyed by client address.

## Related reading

- [Configuration](../../guides/configuration/).
- [Deployment architecture](../../guides/deployment/).
