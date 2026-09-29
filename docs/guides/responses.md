---
title: Handlers and responses
status: preview
---

A response tells the client what happened. Its status describes the outcome, its headers provide metadata, and its body carries the representation. Choosing all three together makes an endpoint predictable.

We'll use an order service as our example. Reading an order, creating an order, and redirecting a browser after a form submission are different outcomes, even if each operation succeeds.

## Returning text and HTML

A handler can return plain text directly:

```rust
async fn health() -> &'static str {
    "ready"
}
```

For an HTML document, return `Html`:

```rust
use cot::html::Html;

async fn order_help() -> Html {
    Html::new("<h1>Orders</h1><p>Choose an order to view its details.</p>")
}
```

The second example uses fixed markup. `Html::new` marks a string as HTML; it does not escape user input interpolated into that string. For customer names, product descriptions, and other external values, use the [template integration](../../templates/) with the appropriate escaping behavior.

## Explicit status and headers

When an endpoint needs more control, construct a `Response`. This example represents an empty successful response:

```rust
use cot::{Body, StatusCode};
use cot::response::Response;

let mut response = Response::new(Body::empty());
*response.status_mut() = StatusCode::NO_CONTENT;
assert_eq!(response.status(), StatusCode::NO_CONTENT);
```

An empty response should not carry a JSON document merely because the endpoint normally uses JSON. Likewise, a download's media type should describe the file, not the HTML page that linked to it.

## Choosing an outcome

The following are example application contracts rather than automatic defaults for every Cot handler:

| Operation | Example outcome | What the client learns |
| --- | --- | --- |
| Read an existing order | 200 with an order representation | The order was found |
| Create an order | 201 with its identifier or location | A new resource exists |
| Finish an action with no representation | 204 with no body | The action completed |
| Accept a durable export request | 202 with a status location | Work is accepted, not complete |
| Reject invalid input | A documented 4xx response | The client needs to change the request |
| Fail because a dependency is unavailable | An appropriate 5xx response | The server could not complete the operation |

A successful status should describe what is actually complete. If an email is still waiting in a queue, a response shouldn't claim that it was delivered.

## Redirects after form submissions

A redirect can take a browser from a successful form submission to an order-detail page. The following GET then represents the result without requiring the browser to resubmit the form when it refreshes.

Use the appropriate redirect status for the method behavior you intend. Some statuses preserve the original method and body; others commonly lead to a GET. Test the status and Location header rather than accepting any 3xx response as equivalent. [Testing a redirect](../../how-to/test-redirect/) shows a focused check.

A return URL supplied by a visitor needs validation. Without a local-destination policy, a login flow can redirect users to an unrelated site.

## Cache behavior

A public catalog page and a private order page need different cache policies. A URL alone may not capture the authenticated user's identity, language, or permissions. Caching private output under a shared key can reveal another customer's information.

Consider browser caches and intermediary caches as well as the application cache. A response can be correct when generated and still be inappropriate to reuse later.

## Errors and streaming

Return useful client-facing errors without exposing internal stack traces or credentials. Keep a request identifier available so operators can connect the response to diagnostics.

For streamed output, authorization and any checks that determine the status belong before the first bytes are sent. A failure halfway through a file cannot be repaired by sending a new status line. See [error handling](../../guides/errors/) and [media storage](../../guides/media/) for those cases.
