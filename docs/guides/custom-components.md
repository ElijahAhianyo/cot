---
title: Custom components
status: preview
---

Extractors, middleware, and service adapters extend different parts of an application. Choosing the right boundary makes an extension easier to understand and test.

## Extractors

An extractor turns request data into a typed argument. It fits concerns such as a validated request identifier, while business operations generally belong in application code.

## Middleware

A middleware layer surrounds request handling and can observe the response. It fits behavior shared across routes, subject to the ordering and state requirements of the chain.

## Adapters and contracts

A service adapter connects an application-facing contract to a backend. Its failure behavior and lifecycle matter as much as its success methods. Future storage or task adapters should document those contracts explicitly.

## Related reading

- [Requests and extractors](../../guides/requests/).
- [Middleware](../../guides/middleware/).
