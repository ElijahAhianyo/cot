---
title: Authentication
status: preview
---

Authentication establishes who is making a request. It is separate from deciding what that person may do. A signed-in user can still lack permission to edit an issue or view a private project.

## Users and credentials

Cot exposes a `User` abstraction and an `AuthBackend` interface. A backend verifies identity; application code should not confuse receiving a user ID with proving ownership of that identity.

## Sessions and login state

Browser authentication commonly persists across requests through a session. Login, logout, and session invalidation are related operations, not merely changes to a page’s appearance.

## Passwords and recovery

Passwords need dedicated password hashing. Recovery and verification flows also need expiry and single-use semantics. This preview reserves account-lifecycle coverage without claiming all recovery endpoints are built in.

## Authentication boundaries

An anonymous request, an invalid credential, and a valid user without access are different conditions. The response should fit the client: a browser may need a login page, while an API client needs a machine-readable failure.

## Related reading

- [Sessions](../../guides/sessions/).
- [Authorization](../../guides/authorization/).
- [Admin interface](../../admin-panel/).
