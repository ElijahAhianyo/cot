---
title: Handlers and responses
status: preview
---

A handler produces the result of an application operation. A response is the HTTP representation of that result: its status, headers, and body. Keeping these ideas separate helps when the same application serves both HTML pages and JSON clients.

## Return values and conversion

Cot’s `IntoResponse` trait converts supported return values into HTTP responses. Plain text, rendered HTML, redirects, and structured data have different representation needs even when they describe the same record.

## Status and representation

A successful lookup, a validation failure, and a missing record should not all look like an indistinguishable success response. The body explains the result to the caller; the status and content type let HTTP clients interpret it.

## Redirects

A redirect asks the client to make another request. Named-route redirects keep the destination tied to the route definition rather than a repeated URL string. Cot’s `reverse_redirect!` produces a 303 response.

## Streaming and downloads

Large downloads should be considered separately from small in-memory responses. Content type, disposition, cancellation, and errors after headers have been sent all affect the design. This preview does not imply a new Cot streaming API.

## Related reading

- [Routing](../../routing/).
- [Error handling](../../guides/errors/).
- [Templates](../../templates/).
