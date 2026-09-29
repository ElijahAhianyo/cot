---
title: Application lifecycle
status: preview
---

A request passes through several parts of Cot before a handler returns a response. Understanding that path helps us place work where it belongs and explain failures that happen before the handler is called.

We'll use a product page as an example. A browser requests `/products/42/`, the application looks up product 42, and the response contains its details. There are two lifetimes to keep in mind: the running application and that single request.

## Application initialization

The Project implementation supplies configuration and registers apps. Cot uses the selected configuration and registered components to build the context and request handler. Apps contribute routes and other resources through their trait methods; the project assembles them into one service.

This is where we decide which apps are installed and which middleware wraps the router. Opening shared resources here avoids repeating expensive initialization in every product lookup. Configuration errors should be discovered before the application is treated as ready to serve traffic.

The exact integration points are documented on [`Project`](trait@cot::Project) and [`App`](trait@cot::App). A command and an HTTP server can use the same project definition, but they do not necessarily need the same resources or execute the same work.

## The request path

The following is a conceptual request path. An installed middleware can return early, so not every request reaches every stage.

```text
Browser request
  → middleware
  → route matching
  → extraction of handler arguments
  → handler and application logic
  → response through the surrounding middleware
  → browser
```

For `/products/42/`, routing finds the handler associated with the path and method. Extraction turns the captured ID into the type the handler expects. The handler then decides whether the product exists and whether it can be shown.

Those are separate decisions. A path can match while its captured value fails to parse. A valid numeric ID can refer to a missing product. A product can exist but be private. Keeping the stages separate makes both the code and its error messages clearer.

## Work around the handler

Middleware can attach session state, record timing, or modify a response. A timing layer that surrounds the whole application includes time spent in inner middleware; a timer inside the handler doesn't. Both measurements can be useful, but they answer different questions.

A redirect may also happen before the handler runs. If a request to a product page appears to do nothing, inspect its status and Location header before adding logging inside the product query. [Diagnosing routing](../../how-to/diagnose-routing/) follows that investigation.

## Producing the response

A handler's return type determines how its result becomes an HTTP response. Plain text, HTML, JSON, and an explicit Response serve different purposes. Returning an error transfers responsibility to error handling; it doesn't automatically mean the visitor should see the error's internal description.

Middleware observes the response on its way out. Once a streamed response has sent its headers, an application cannot replace them with a different status because a later chunk failed. Operations that determine whether a download is permitted belong before streaming begins.

## Request completion and other work

A response finishing is not proof that every related external action succeeded. If an order handler returns before sending its confirmation email, the application needs a separate way to track that work. An in-process spawned task can disappear when the process exits; it is not a durable queue.

For supported code, follow the current handler and command APIs. The [background task guide](../../guides/background-tasks/) describes the intended durable-work design separately.

## Using the lifecycle to diagnose failures

| Symptom | First boundary to inspect |
| --- | --- |
| Server never begins listening | Configuration and initialization |
| Handler logging never appears | Middleware, routing, and extraction |
| Product lookup succeeds but response fails | Rendering and response conversion |
| Request succeeds but email is absent | Side-effect execution and delivery records |
| Requests become slow under load | Shared resources, blocking work, and dependency latency |

Continue with [middleware](../../guides/middleware/) for ordering and [error handling](../../guides/errors/) for failures at these boundaries.
