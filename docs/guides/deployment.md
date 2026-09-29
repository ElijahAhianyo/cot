---
title: Deployment architecture
status: preview
---

Deploying a Cot application means running a compiled program with the configuration and resources it needs. The executable is one part of the deployment; the database, uploaded files, secrets, network boundary, and release process are equally important.

We'll use a small shop that starts with one application instance and later grows to two. That change is a useful test of which assumptions belong to one process and which must hold across the deployment.

## Build and runtime

Build the release artifact from a known source revision and lockfile. The build machine needs the Rust toolchain and dependencies; the runtime needs the resulting executable and any files or shared libraries that executable requires.

```bash
cargo build --release --locked
```

Run this from the application project. A successful build establishes that the source compiles for that target. It doesn't prove the binary can connect to the production database or find its runtime configuration.

The [production-build how-to](../../how-to/production-build/) covers the practical verification. Don't copy a macOS binary to a Linux host and expect it to run; operating system and architecture are part of the target.

## The public network boundary

A hosting platform or reverse proxy often terminates HTTPS and forwards requests to the application. The application needs to listen on an address reachable from that component. Binding only to `127.0.0.1` inside a container usually prevents an external platform proxy from reaching it.

Trust forwarded headers only from the intended proxy path. Their host, scheme, and client-address values can affect redirects, cookie behavior, and request policies. A direct client must not be able to impersonate the proxy by supplying the same headers.

## Durable resources

A container filesystem is often disposable. Uploaded receipts and the production database must not depend on files that disappear when an instance is replaced. The application should know where durable data lives and which credentials permit access.

Static assets shipped with a release have a different lifecycle. Versioned or content-addressed assets can let old pages continue loading their corresponding resources during a rolling deployment.

## Adding a second instance

Two web instances need a consistent view of sessions, authentication keys, and shared state. An in-memory session store or process-local cache can behave differently on each instance. A cache may tolerate that; login state often cannot.

Connection pools also multiply. If each instance allows many database connections, adding instances can exhaust the database even while the web servers appear lightly loaded. Size the deployment as a system rather than considering each process separately.

## Readiness and shutdown

A process being alive is not the same as being ready for traffic. A readiness check should establish the minimum conditions needed for the traffic it will receive. Avoid making every optional provider outage remove the whole shop from service unless that is the intended policy.

During shutdown, stop accepting new work and allow appropriate in-flight work to finish within the platform's termination window. Long-running exports and durable jobs need their own recovery model rather than relying on the web process remaining alive indefinitely.

## Schema changes

A release that changes the database must account for old and new application instances overlapping. Adding a compatible field, deploying code that uses it, and removing an old field later is often easier to recover from than one destructive step.

Run migrations through one controlled deployment operation, not independently from every replica without understanding the migration runner's coordination.

## Verify the deployed behavior

Check a public page, a database-backed request, login across requests, private-file access, and error handling. Confirm that debug output is disabled and logs contain enough context to diagnose a failure.

See [production configuration](../../guides/production/), [observability](../../guides/observability/), and [releases and recovery](../../guides/recovery/) for those decisions.
