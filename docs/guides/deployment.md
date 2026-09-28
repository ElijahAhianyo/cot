---
title: Deployment architecture
status: preview
---

A production Cot application is more than its executable. Requests may pass through a reverse proxy, data may live in a separate service, and uploaded files must survive application releases.

## The application process

The application binary handles requests. A process manager or container platform starts it, observes failure, and controls restarts. Listening addresses and configuration must fit the surrounding infrastructure.

## Proxies and persistent services

A proxy can terminate TLS and forward requests. The database, session store, and media storage have their own persistence and availability requirements. Several application instances need a deliberate approach to shared state.

## Releases and migrations

A database migration can affect both old and new application instances during a rollout. Backward-compatible schema changes reduce the risk of a mixed-version deployment.

## Health and shutdown

A process can be running without being ready to serve traffic. Readiness and liveness answer different questions. Shutdown should give active requests an appropriate opportunity to finish.

## Related reading

- [Production configuration](../../guides/production/).
- [Prepare a production build](../../how-to/production-build/).
- [Releases and recovery](../../guides/recovery/).
