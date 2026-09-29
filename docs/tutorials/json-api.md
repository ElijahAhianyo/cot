---
title: Build a JSON API
status: preview
---

We'll build and exercise a price-quote endpoint. A client sends a quantity, and the API returns a total in cents. We'll add a useful validation error and test the response contract.

Use the [tutorial companion checkout](../first-app/). Its `src/bin/quote_api.rs` is a complete, separate application, so this tutorial doesn't require the issue-tracker database or earlier chapters. The companion manifest enables Cot's `json` and `test` features and includes Serde with derive support.

## Run the API

From `docs/site/checks/issue-tracker`:

```bash
cargo run --locked --bin quote_api -- --listen 127.0.0.1:8002
```

In a second terminal, request a quote:

```bash
curl -i -H 'Content-Type: application/json' --data '{"quantity":3}' http://127.0.0.1:8002/quotes
```

Expect status 200, a JSON content type, and a body containing `"total_cents":1350`. The example price is 450 cents per item. Using integer cents keeps this small calculation independent of binary floating-point rounding.

## Describe accepted input

Open `src/bin/quote_api.rs`. `QuoteRequest` contains `quantity: u16` and derives `Deserialize`. The handler's `Json<QuoteRequest>` argument asks Cot to decode that body before calling the handler.

The response is a different type, `Quote`, deriving `Serialize`. Separating input and output makes the contract visible: the client supplies a quantity, while the server supplies the total. A client cannot set the authoritative price by adding a convenient field to the request.

The fixed price keeps this exercise small. A real catalog would load the price from authoritative storage and define currency, tax, and rounding rules separately.

## Validate the quantity

Send a quantity of zero:

```bash
curl -i -H 'Content-Type: application/json' --data '{"quantity":0}' http://127.0.0.1:8002/quotes
```

The handler returns 422 with the code `invalid_quantity` and the message “Choose between 1 and 20 items.” Zero fits the Rust integer type but fails the purchasing policy. That is why decoding and validation are separate.

Try 20 and 21. The first should succeed; the second should take the same validation path as zero. Keeping boundary values in tests prevents a future edit from accidentally excluding the maximum allowed quantity.

## Compare decoding failures

Now send `{"quantity":"three"}` or an incomplete JSON document. Cot rejects those before the quote calculation runs. They are different from a decoded integer outside the application's range.

The example customizes its domain-validation response, not every framework rejection. If your public API promises one error envelope for all failures, implement and test the relevant extraction and error-handling paths too. Do not infer a universal JSON error contract from one branch in the handler.

## Check method handling

Send GET to the same path:

```bash
curl -i http://127.0.0.1:8002/quotes
```

The route registers POST, so GET is not a second way to request a quote. An unsupported method should remain distinct from an unknown URL or invalid quantity.

## Run the contract test

Stop the server when finished, then run:

```bash
cargo test --locked --bin quote_api
```

The test checks the success status, content type, calculated body, validation status and code, and unsupported method. Extend it with the 20 and 21 boundary cases you tried manually.

The endpoint doesn't store orders or charge customers. If you extend it into checkout, add authentication, authoritative inventory checks, transaction boundaries, and retry behavior. For collections, see [pagination](../../guides/pagination/). For a machine-readable contract, continue with [OpenAPI](../../openapi/), keeping the schema aligned with both successful and unsuccessful responses.
