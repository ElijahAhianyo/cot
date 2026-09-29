---
title: HTTP and application tests
status: preview
---

An HTTP test checks the contract a client sees: the request it sends, the response it receives, and any resulting state change. A handler can compile and still return the wrong status, omit a redirect location, or let one customer read another's order.

Cot provides request builders and a project test client. Choose the smallest boundary that includes the behavior you need to verify.

## Testing a router

A router test is useful for path matching, extraction, and handler responses:

```rust
use cot::http::StatusCode;
use cot::request::extractors::Path;
use cot::router::{Route, Router};
use cot::test::TestRequestBuilder;

async fn product(Path(id): Path<i64>) -> String {
    format!("Product {id}")
}

let router = Router::with_urls([
    Route::with_handler("/products/{id}/", product),
]);
let response = router.handle(
    TestRequestBuilder::get("/products/42/").build()
).await?;
assert_eq!(response.status(), StatusCode::OK);
assert_eq!(response.into_body().into_bytes().await?, "Product 42");
```

This checks that the router passes the captured ID to the handler. It doesn't exercise project middleware, a real socket, or browser behavior. State that boundary clearly when interpreting a passing test.

## Testing the assembled project

Use `cot::test::Client` when the behavior depends on app registration, middleware, configuration, or the error handler. That catches failures a direct handler call can miss, such as an app never being mounted or authentication running without session state.

Use `TestServer` when an external HTTP client needs a listening server. A real browser test belongs at that boundary, while a pure calculation can remain an ordinary Rust unit test.

## Initialization boundaries

In this checkout, `Client::new` loads the test configuration and builds the project handler, but it does not execute the server's migration phase or app `init` hooks. Prepare required data and schema explicitly, or use a real test server when that startup behavior is part of the test. An in-process client is not evidence that the listening server's full initialization path ran.

## Assertions that explain a contract

A status assertion is a start, not the entire test. For a redirect, assert the exact destination and intended status. For a JSON response, inspect the fields the client relies on. For an order creation, verify the stored order and ensure a retry doesn't create a second one when the endpoint promises repeat-safe behavior.

Avoid asserting an entire HTML document when only one semantic result matters. Whitespace and unrelated navigation changes should not obscure the reason a checkout test failed.

## Authentication and cookies

A login test needs more than a successful response to the login form. Make a subsequent request using the resulting session state and verify the current user. Then check a request without that state and a request made as a different user.

Do not assume a test helper behaves exactly like a browser cookie jar. Inspect its contract and pass cookie or session state explicitly when needed. Browser tests are valuable for the complete cookie and redirect flow.

## Errors and side effects

Exercise malformed input, valid-but-rejected input, missing resources, and dependency failures. Check that the intended error response appears and that a rejected request hasn't written data or sent a message.

Keep mail and external HTTP services isolated. A fake can verify the requested effect; a separate integration test can check the real adapter. One cannot replace the other completely.

## Independent tests

Each test should establish its own relevant data. Two tests sharing an order ID and mutating it can pass individually but fail in parallel. Use isolated resources and deterministic fixtures, and clean them up even when a test fails.

Continue with [database tests](../../guides/database-tests/), [browser tests](../../guides/browser-tests/), and the focused [redirect how-to](../../how-to/test-redirect/).
