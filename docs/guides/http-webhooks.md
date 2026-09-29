---
title: Outbound HTTP and webhooks
status: preview
---

An outbound HTTP request crosses a boundary we don't control. The destination can be slow, unavailable, or uncertain about whether it completed an operation. A webhook crosses the same boundary in the other direction: another service tells our application that something happened.

We'll use a shipping provider. Our shop asks it to create a shipment, and the provider later sends delivery updates. The patterns here are integration design; choose and configure a Rust HTTP client explicitly rather than assuming a built-in Cot client facade.

## Timeouts and connection reuse

Reuse a client where its contract supports it, and configure connection and overall request timeouts. An absent timeout can leave a request waiting far longer than the application's useful response window.

Timeouts should fit the operation. A price lookup used while rendering a page may need a short budget. A large export upload belongs in a different workflow. Bound response sizes as well as request sizes so a remote service cannot cause unlimited buffering.

## Transport success and business success

Receiving an HTTP response means the transport returned something. It doesn't mean the operation succeeded. Inspect the status and validate the response representation before using its values.

A provider's error body can help diagnosis, but don't blindly relay it to the customer. It may contain internal identifiers or echo sensitive input. Translate it into the application's own error contract and retain safe diagnostic context.

## Retrying a write

Suppose the shipment request times out. The provider may have created the shipment before the response was lost. Sending the same request again without a stable operation identity can create a second shipment.

Use the provider's documented idempotency facility when available, and keep the identity stable across retries of the same logical operation. Retrying a different order with the same key is also a mistake. Limit attempts and use an appropriate delay; a retry loop should not amplify an outage.

## Receiving webhooks

Authenticate a webhook according to the provider's protocol. Signature verification may depend on the exact raw body bytes, so verify before parsing or normalizing the body when the protocol requires that.

Check freshness and replay behavior as well as the signature. A valid old message can still be repeated. Store the provider's event identity and make processing repeat-safe.

An illustrative delivery record can track provider, event ID, receipt time, processing state, and a safe error summary. It should not become an unrestricted archive of every sensitive payload field.

## Ordering and acknowledgment

A delivered event can arrive before a delayed in-transit event. Decide how the application handles older updates instead of letting arrival order overwrite newer state.

Only acknowledge durable acceptance once the application can recover the work after a crash. If processing happens later, the acknowledgment means accepted, not fully processed. If the provider retries on a failed response, that retry must not duplicate the business effect.

## Destination restrictions

If users can configure webhook destinations, a syntactically valid URL isn't enough. Apply a destination policy that accounts for internal addresses, DNS resolution, and redirects. Keep outbound credentials scoped to the intended provider.

## Testing failures

Use a controlled test server to return a timeout, a malformed response, a retryable failure, and a duplicate event. Assert the outgoing method, headers, body, and repeat identity. Keep ordinary tests from contacting real shipping or payment services.

See [background tasks](../../guides/background-tasks/) for deferred processing and [observability](../../guides/observability/) for tracing one operation across service boundaries.
