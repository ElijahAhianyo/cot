---
title: Authorization
status: preview
---

Authorization decides whether a particular user may perform an operation on a particular resource. Authentication provides identity; it does not imply access.

## Roles and ownership

An administrator role can permit a broad action. Ownership can permit editing one issue but not another. A useful policy describes both the operation and the resource rather than checking only whether a user is signed in.

## Enforcement boundaries

Hiding an edit button improves the interface but does not protect the endpoint. The server must check access for every path that performs the operation, including APIs, admin actions, and background work.

## Denial and information disclosure

Returning a denial can reveal that a private record exists. The application needs a consistent policy for whether to return a forbidden response or conceal the resource.

## Target policy interface

This is a conceptual preview of authorization coverage. Named policies, role storage, and object-level helpers should only receive executable examples after their Cot interfaces are established.

## Related reading

- [Authentication](../../guides/authentication/).
- [Error handling](../../guides/errors/).
