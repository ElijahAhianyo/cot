---
title: Requests and extractors
status: preview
---

A request contains the information a client sends to our application: a method, a URL, headers, and sometimes a body. Cot's extractors turn the parts a handler needs into typed arguments.

We'll use a product catalog throughout this guide. A product-detail page needs an ID from the path. A search page needs query parameters. Creating a product needs a body. Those inputs have different meanings even when they all arrive in one HTTP request.

## Path parameters

A path parameter identifies part of the resource being requested. The `Path` extractor decodes the values captured by the router:

```rust
use cot::request::extractors::Path;
use cot::router::{Route, Router};

async fn product_detail(Path(product_id): Path<i64>) -> String {
    format!("Product {product_id}")
}

let router = Router::with_urls([
    Route::with_handler("/products/{product_id}/", product_detail),
]);
```

For `/products/42/`, the handler receives `42`. The example returns plain text; a real handler would query the product and choose a response for a missing row.

A route match doesn't guarantee successful extraction. `/products/blue/` has the right path shape, but `blue` cannot become an `i64`. Extraction can fail before the handler runs. Neither successful parsing nor a matching database row establishes permission to view that product.

## Query parameters

Query parameters modify a request without changing the route's identity. For example, `/products/?search=notebook` and `/products/?search=pen` reach the same route. Cot provides `UrlQuery` for typed query data.

Choose defaults deliberately. An absent search term can mean “show the catalog,” while an empty term might reasonably mean the same thing. An absent page number can mean the first page; a malformed number should not silently select a surprising page. [Pagination](../../guides/pagination/) covers bounded page sizes and stable ordering.

Do not use query parameters as evidence of identity. A request containing `customer_id=7` is a claim made by the client, not proof that the caller is customer 7.

## Request heads and request bodies

Cot separates extractors into `FromRequestHead` and `FromRequest`. Head extractors inspect information available without consuming the body. Body extractors consume the request, which is why a handler can have only one extractor implementing `FromRequest` and it belongs at the body-consuming boundary of the signature.

This matters when combining input types. A handler can read a path ID and then decode a form body. It cannot independently consume the same stream once as JSON and again as a form. If an endpoint accepts multiple formats, the application must make that choice before consuming the body.

## Form data and JSON

`RequestForm<F>` is used for a form type implementing `Form`. Its result distinguishes a valid form from validation errors. JSON has a different representation and content type; the [JSON API guide](../../guides/json-apis/) explains the corresponding interface.

The Content-Type header describes the representation, not whether its values are acceptable. A JSON number may deserialize correctly but violate a business rule. A form field can contain a valid product ID but refer to an unavailable product.

## Limits and untrusted metadata

A body can be large even when its headers are small. Decide where request-size limits are enforced and what response an oversized request receives. Avoid reading an unbounded body into memory simply to inspect one field.

Headers describing the original host, scheme, or client address require a trusted proxy configuration. A header supplied directly by an arbitrary client should not determine an authorization decision or a rate-limit identity.

## Custom extractors

A custom extractor is useful when several handlers share a well-defined input contract. For example, an application could extract a validated request identifier from a header. The extractor should explain whether missing data is allowed, which failure it returns, and whether it consumes the body.

Keep business decisions near the resource they concern. A request identifier extractor doesn't need to load an order or decide who can cancel it. See [validation](../../guides/validation/) and [authorization](../../guides/authorization/) for those boundaries.
