---
title: Test a redirect
status: preview
---

Use this pattern when a handler should redirect to another route. It checks the status and destination without requiring a browser.

## Prerequisites

The example belongs in an application test module with Cot's `test` feature enabled. It uses a local router, so it does not include project middleware.

## Make the request and inspect the response

```rust
use cot::http::{StatusCode, header};
use cot::response::Redirect;
use cot::router::{Route, Router};
use cot::test::TestRequestBuilder;

async fn old_address() -> Redirect {
    Redirect::new("/articles/")
}

let router = Router::with_urls([
    Route::with_handler("/old-articles/", old_address),
]);
let response = router.handle(TestRequestBuilder::get("/old-articles/").build()).await?;
assert_eq!(response.status(), StatusCode::SEE_OTHER);
assert_eq!(response.headers()[header::LOCATION], "/articles/");
```

## Verify the right boundary

This checks a redirect returned by the handler. To check a redirect supplied by middleware, such as trailing-slash handling, send the request through a complete application test client instead. See [testing](../../testing/) and [middleware](../../guides/middleware/).

## Test the destination independently

A correct `Location` header doesn't prove that the target route works. Add a second request to `/articles/` and assert its content or handler-specific result. Keep the two assertions separate so a failure tells us whether URL selection or destination handling broke.

Avoid automatically following the redirect in the first test. Following it immediately can turn a wrong redirect status into an apparently successful 200 response.

## Cover submitted forms

For a form submission, send POST with the form data and verify the resulting redirect. Also check the database change that the submission promises. A 303 response by itself doesn't prove that an article was saved.

Refreshes are another reason to check the status explicitly: a redirect used after POST communicates how the client should make the next request. Use the status provided by the chosen redirect type instead of asserting that every 3xx response is equivalent.

For rejected input, assert that the form is shown with useful errors and that no row was created. That request should not silently take the successful redirect path.

## Keep redirect targets within the intended boundary

When a login flow accepts a return destination, test external URLs and malformed values as well as a normal local path. The application must apply its destination policy before constructing the redirect. A test that uses only a fixed internal string does not exercise that policy.

Check query parameters and escaping when the destination carries state. Prefer named URL generation for application routes so changing a route pattern does not leave redirect strings scattered through handlers and tests.
