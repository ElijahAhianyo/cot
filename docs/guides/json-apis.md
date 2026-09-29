---
title: JSON APIs
status: preview
---

A JSON API exposes an application's behavior to clients that don't render its HTML pages. A browser, mobile app, or another service can submit structured input and receive a structured response.

We'll use a price quote to show the boundary. The client supplies a quantity, and the server returns a total. In a real shop, the server also looks up the authoritative product price; it should not trust a price supplied by the client.

## Typed input and output

Cot's `Json<T>` wrapper handles JSON at the request and response boundary. Request data needs deserialization; response data needs serialization.

```rust
use cot::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct QuoteRequest {
    quantity: u16,
}

#[derive(Serialize)]
struct QuoteResponse {
    total_cents: u64,
}

async fn quote(Json(input): Json<QuoteRequest>) -> Json<QuoteResponse> {
    let unit_price_cents = 450_u64;
    Json(QuoteResponse {
        total_cents: unit_price_cents * u64::from(input.quantity),
    })
}
```

This deliberately small example uses a fixed price and a bounded numeric type. It illustrates conversion, not a complete checkout endpoint. The application must still reject a zero quantity or a quantity outside its purchasing policy.

## Public representations

A database model and an API response have different responsibilities. Returning a dedicated response type lets us omit internal fields and keep the public contract stable when storage changes.

For example, a customer response may include a display name without exposing a password hash, internal flags, or a recovery token. Serialization should be an explicit choice, not an accidental consequence of reusing a convenient struct.

## Error contracts

Separate malformed JSON, valid JSON with unacceptable values, denied access, missing resources, and internal failures. Choose a documented response shape so clients don't have to inspect human-readable text to decide what happened.

An illustrative error body might contain a stable code, a message, field-level details, and a request identifier. The shape belongs to the application; it isn't a claim that Cot automatically emits those fields.

Avoid returning a successful status with an error hidden in the body. Likewise, an empty 204 response should not include a JSON document.

## Optional fields and updates

For a partial update, a missing field can mean “keep the current value,” while an explicit null can mean “clear it.” A single optional type does not always preserve both distinctions after deserialization. Design the input representation around the intended operation.

Unknown fields also need a deliberate compatibility policy. Rejecting them catches mistakes; ignoring them can make additive client changes easier. Test the policy instead of depending on an assumption about defaults.

## Collections and retries

Bound collection responses and define their ordering. Pagination metadata must describe how to continue using the same filters. [Pagination](../../guides/pagination/) covers offsets and cursors.

For writes that can be retried, distinguish an attempt from the logical operation. A lost response after a successful order creation should not force the client to guess whether to create another order. Stable operation identifiers and replay behavior belong in the endpoint contract.

## Schemas and tests

An OpenAPI schema can describe request and response shapes, but it doesn't prove that authorization or transaction behavior is correct. Keep examples, tests, and the implementation aligned. See [OpenAPI](../../openapi/) for Cot's schema integration and [HTTP tests](../../guides/http-tests/) for behavior checks.

When evolving an API, consider old clients that are still active. A renamed field, changed meaning, or new required input can be a breaking change even when the Rust code compiles.
