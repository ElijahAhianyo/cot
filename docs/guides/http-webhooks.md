---
title: Outbound HTTP and webhooks
status: preview
---

An outbound request crosses a boundary the application does not control. A webhook crosses that boundary in the other direction. Both need policies for timeouts, authenticity, duplicate work, and failure.

## Timeouts and retries

A timeout does not prove that the remote operation failed. A retry of a read differs from a retry that creates a payment or shipment. The operation’s idempotency contract determines whether a retry is safe.

## Authenticity and replay

A webhook signature can verify a sender and payload, while a timestamp or delivery identifier helps detect replay. Signature validation needs the exact representation required by the sender.

## Acknowledgment and processing

A webhook endpoint may acknowledge durable acceptance before processing finishes. It needs to retain enough information to handle retries and duplicate deliveries. This prototype does not introduce an HTTP client wrapper or webhook API.

## Related reading

- [Background tasks](../../guides/background-tasks/).
- [Error handling](../../guides/errors/).
