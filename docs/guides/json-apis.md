---
title: JSON APIs
status: preview
---

A JSON API is a contract between an application and its clients. Consistent representations matter as much as valid JSON: clients need to understand successes, validation failures, pagination, and changes over time.

## Request and response shapes

A database model and a public representation have different responsibilities. Exposing every stored field can leak internal state or make future schema changes break clients.

## Errors and pagination

An error should have a predictable structure and an appropriate status. A paginated response should explain how the client continues and how ordering behaves when records change.

## Compatibility

Adding a field, removing a field, and changing its meaning have different compatibility effects. Versioning should follow the public contract rather than the internal organization of the project.

## Related reading

- [Requests](../../guides/requests/).
- [Responses](../../guides/responses/).
- [OpenAPI](../../openapi/).
