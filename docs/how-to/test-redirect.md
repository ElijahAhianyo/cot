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
