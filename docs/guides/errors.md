---
title: Error handling
status: preview
---

Errors are part of an application's behavior. A customer may ask for an order that no longer exists, submit an invalid quantity, or reach the shop while its database is unavailable. Those situations need different responses even though none produces the requested order page.

Cot handlers can return a `cot::Result<T>`. That lets a handler propagate an internal failure while the application's error handling decides how to present it. It doesn't mean every failure should become the same public response.

## Expected outcomes and failures

A query returning no matching row is different from a query that could not run. For a product lookup, the first can become the application's not-found response; the second needs an operational error path.

```rust
use cot::db::{Auto, Database, Model, model, query};

#[model]
struct Product {
    #[model(primary_key)]
    id: Auto<i64>,
    name: String,
}

async fn find_product(db: &Database, id: i64) -> cot::Result<Option<Product>> {
    Ok(query!(Product, $id == id).get(db).await?)
}
```

The `Option` represents absence. The `Result` represents whether the lookup succeeded. Collapsing both into “not found” would hide a database outage and make the application look as though every product had disappeared.

## Choosing the public response

A useful response tells the caller what they can do next. Invalid quantity input can identify the field and its rule. A temporary backend failure can give a general retry message and a request identifier. Internal connection details belong in operator diagnostics.

For APIs, define a stable error shape and machine-readable codes. For browser pages, provide readable text and a route back to a useful page. Neither format should depend on the exact wording of a low-level library error.

## Failures before the handler

Routing, argument extraction, and middleware can fail before application logic runs. A custom error page must work for those cases too. Don't assume that a matched route, authenticated user, or database connection is always available while rendering an error.

The error handler should also avoid depending on the service that just failed. A database-backed navigation menu is a poor requirement for displaying a database-outage page.

## Debug output and production output

Detailed error pages help during development, but they can reveal source locations, request data, or internal configuration. Production configuration should control that exposure explicitly. An optimized executable and a safe error policy are related deployment choices, not interchangeable switches.

Cot's [error-page guide](../../error-pages/) shows the current customization interface. Use the [`cot::error`](mod@cot::error) reference for exact error types.

## Recovering from partial work

Suppose an order is committed and the mail provider then times out. Reporting that the entire operation failed can encourage the browser to repeat the purchase. Before retrying, the application needs to know what was completed and whether repeating the operation is safe.

A timeout does not prove that a remote service did nothing. Keep stable operation identifiers where retries are possible, and record enough state to reconcile uncertain outcomes. A blanket retry around a whole handler can duplicate side effects.

## Testing errors

Test at least one case at each important boundary: invalid input, missing data, denied access, and an unavailable dependency. Assert that the response is useful and that private diagnostics stay out of it.

Also test error rendering itself. If the error template fails, the application should still have a minimal fallback response. [Observability](../../guides/observability/) explains how to connect a safe public response to detailed internal evidence.
