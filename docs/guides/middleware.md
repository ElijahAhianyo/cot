---
title: Middleware
status: preview
---

Middleware handles concerns that surround a request’s application logic. Logging, sessions, and response headers can apply across many handlers without being repeated in each one.

## The request and response paths

A middleware layer can act before calling the next service and after that service returns. It can also return a response without calling the handler. A diagram of a middleware chain should therefore show both the inward request path and outward response path.

## Ordering and dependencies

Ordering matters when one layer relies on information supplied by another. Authentication may need session state; request logging may need to observe failures as well as successful responses. Test the assembled chain, not only each layer in isolation.

## Cot and Tower

Cot’s `RootHandlerBuilder` assembles the project’s middleware. Tower layers and services are relevant extension points. The exact bounds belong in the Rust API; application guides should show the resulting behavior and ordering.

## Testing the boundary

Calling a handler or `Router::handle` directly does not exercise the project’s full middleware chain. A test client against the complete application is the appropriate boundary for checking sessions, redirects, or headers supplied by middleware.

## Related reading

- [Application lifecycle](../../guides/lifecycle/).
- [Sessions](../../guides/sessions/).
- [Testing](../../testing/).
