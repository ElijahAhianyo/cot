---
title: Custom components
status: preview
---

A custom component adapts an application-specific need to a framework boundary. We might need authentication backed by an existing identity service, an error page with the shop's layout, or a request extractor used by several handlers.

The useful starting point is the contract being replaced or extended. A component should do the work that contract promises and leave unrelated business rules visible elsewhere.

## Choose the narrowest boundary

| Need | Boundary to inspect |
| --- | --- |
| Supply identity from another system | Authentication backend traits |
| Decode repeated request metadata | Request extractor traits |
| Apply behavior around requests | Middleware and handler builder |
| Change error presentation | Project error-handler hook |
| Add an operator command | CLI task interface |
| Contribute routes and resources | App trait |

Use the [component reference](../../reference/components/) for the authoritative types. These are different contracts; a custom extractor should not become a hidden general-purpose service container.

## Define inputs and failures

For an extractor that reads a request ID, decide whether the header is optional, how long it may be, and which characters are accepted. A missing value and an invalid value may need different behavior.

For a remote identity backend, distinguish rejected credentials from an unavailable provider. Returning “anonymous” for every backend error can hide an outage and change the application's authorization behavior.

## Keep dependencies explicit

A component that needs an HTTP client or configuration should receive it through a deliberate construction path. Opening a new connection pool on every extraction can make otherwise small handlers expensive.

State shared across requests needs the same ownership and concurrency review as the rest of the application. Configuration can be shared; the current customer's identity should remain scoped to the request.

## Avoid recursive failure paths

A custom error handler should not require the database when it is handling a database outage. A logging adapter should not recursively invoke the operation that it is trying to diagnose.

Give the component a minimal failure path. For an error page, that may be a fixed response if the branded template cannot render. For an external provider, it may be a bounded timeout and an explicit unavailable outcome.

## Contracts and versions

Rust trait bounds help establish type compatibility, but a successful compile doesn't establish behavioral compatibility. An adapter can implement the expected method and still retry unsafely, ignore cancellation, or misclassify an error.

Review dependency versions and feature flags when integrating ecosystem components. A middleware designed for a different body or error type may require an adapter even if both projects use Tower.

## Testing an extension

Test the component's direct contract with controlled inputs, then test it once through a real Cot project. The direct test exposes parsing and error behavior; the project test catches registration, ordering, and missing-context problems.

For the request-ID example, include absence, a valid value, an excessive value, and untrusted punctuation. For an authentication adapter, include success, rejection, timeout, and a user that disappears between requests.

## Document the host's obligations

A reusable component should state required middleware, configuration, resources, and failure behavior. A host application should not have to discover an initialization order by reading a panic message in production.

See [middleware](../../guides/middleware/), [authentication](../../guides/authentication/), and [management commands](../../guides/management-commands/) for concrete boundaries.
