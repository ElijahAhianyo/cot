---
title: Middleware
status: preview
---

Middleware handles behavior around a request handler. Sessions, authentication, request logging, and response headers often belong here because several routes need the same behavior.

For our shop, product pages may be public while order pages need the current customer. Both still need consistent error handling and request diagnostics. Middleware lets us share those concerns without repeating them in every handler.

## Wrapping the handler

Cot's `Project::middlewares` receives a `RootHandlerBuilder`. Each middleware wraps the handler built so far. This means the last wrapper added sees an incoming request before the earlier wrappers do; the response travels back through the wrappers in the opposite direction.

Here is the authentication/session ordering used by a project that needs both:

```rust
use cot::Project;
use cot::middleware::{AuthMiddleware, SessionMiddleware};
use cot::project::{MiddlewareContext, RootHandler, RootHandlerBuilder};

struct ShopProject;

impl Project for ShopProject {
    fn middlewares(
        &self,
        handler: RootHandlerBuilder,
        context: &MiddlewareContext,
    ) -> RootHandler {
        handler
            .middleware(AuthMiddleware::new())
            .middleware(SessionMiddleware::from_context(context))
            .build()
    }
}
```

The session wrapper runs before authentication on the incoming path, so authentication can use the session. This example only shows middleware composition; app registration and configuration belong to the rest of the project.

## Order is a dependency

Rather than memorize a list, ask what each layer requires. Authentication needs session state when using session-based login. A request logger intended to observe redirects must surround the component that produces those redirects. A response-header layer must see the responses on which the header is required.

A conceptual trace for the example is:

```text
request → session → authentication → router and handler
response ← session ← authentication ← router and handler
```

If a wrapper returns early, the inner handler isn't called. That is expected for a redirect or rejected request; it explains why logging placed inside the handler may never appear.

## State belongs to the right lifetime

A middleware instance can be shared across many requests. Store configuration and reusable clients there, but keep request-specific values on the request or in local variables. A field named `current_customer` on shared mutable middleware would mix concurrent users.

Similarly, a timer should be created for each request. Reusing one start time across the process would measure time since startup, not request duration.

## Error responses and early returns

A layer that adds a header after a successful inner call may need separate handling for an error returned by that call. Test both paths. Seeing the header on a 200 response doesn't prove that a 404 or authentication failure includes it.

A useful test matrix includes a normal response, an unknown route, a rejected input, and an early redirect. The goal is to verify which boundary the middleware surrounds, not merely that its constructor compiles.

## Using ecosystem middleware

Cot provides integration with Tower-compatible layers through its handler builder. Compatibility still depends on the request, response, body, and error types expected by a particular layer. Consult the Rust API and the layer's own contract when adapting it.

Do not infer a security guarantee from installing a component with a familiar name. For example, sessions identify a conversation with a browser; they don't by themselves establish resource authorization or CSRF protection.

## Keeping middleware focused

A product-stock rule is usually better expressed next to the operation that changes stock. Middleware rarely has enough context to know which product is changing or which transaction must protect it. Use middleware for cross-cutting request behavior and ordinary application functions for domain rules.

Continue with [sessions](../../guides/sessions/), [authentication](../../guides/authentication/), and [HTTP tests](../../guides/http-tests/).
