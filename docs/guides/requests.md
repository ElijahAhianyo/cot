---
title: Requests and extractors
status: preview
---

A request carries a method, a URL, headers, and sometimes a body. Extractors turn selected parts of that request into handler arguments. This keeps parsing close to the handler’s signature and makes the expected inputs visible.

## Request heads and bodies

Path parameters, query parameters, and headers are available without reading the body. The body is a stream of bytes. Reading it has different costs and ownership constraints from looking up a header.

## Extraction and validation

For `/articles/42`, a `Path<u64>` argument asks for a numeric identifier. `/articles/herbs` can match the route but fail extraction. Parsing establishes a type; it does not establish that article 42 exists or that the current user can read it.

## Failure before the handler

Malformed input can fail before application logic runs. A useful error response identifies the invalid input without returning secrets or internal diagnostics. Request limits also matter before allocating memory for large bodies.

## Custom extractors

Cot separates `FromRequestHead` and `FromRequest`. That distinction expresses whether an extractor needs the body. Custom authentication or request-context extractors should keep this boundary clear rather than read the body unnecessarily.

## Related reading

- [Routing](../../routing/).
- [Forms](../../forms/).
- [Validation](../../guides/validation/).
