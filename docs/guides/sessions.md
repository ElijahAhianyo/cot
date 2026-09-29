---
title: Cookies and sessions
status: preview
---

A session keeps a small amount of state across requests from the same browser. It can remember that a customer is logged in or which catalog view they prefer. It should not become the authoritative record of an order or payment.

A cookie and a session are related but different. The cookie travels between browser and server. The configured session store holds the server-side state associated with that conversation.

## Reading and writing session values

Cot exposes a `Session` to handlers when session middleware is installed. A preference is a useful example because its absence has a harmless default:

```rust
use cot::session::Session;

async fn remember_catalog_view(session: Session) -> cot::Result<&'static str> {
    session.insert("catalog_view", "grid").await?;
    Ok("Catalog preference saved")
}
```

The value has to be serializable for storage. Choose stable keys and small values; storing every product in the session increases serialization and storage cost and leaves the browser working with stale catalog data.

For the exact read, remove, and expiry methods, use the [`Session`](struct@cot::session::Session) reference. Authentication should use the Auth interface rather than writing framework authentication keys directly.

## Choosing a store

The session middleware supports configured stores, and its in-memory option is useful for local development. Memory owned by one process is not shared with another process and is lost when that process exits.

Consider a shop running two replicas. If login writes session data only in replica A, a later request handled by replica B cannot retrieve that state. A shared, supported store addresses the location problem, but its own availability and expiry behavior still need consideration.

The [configuration reference](../../reference/configuration/) links to the supported store types and their feature requirements. Don't choose a store based only on its name; confirm how it behaves during restarts and outages.

## Cookie settings

Cookie settings determine where the browser sends the session identifier and which browser APIs can access it. Review the cookie name, path, Secure flag, HttpOnly flag, and SameSite policy for your deployment.

For an HTTPS production site, cookie configuration and proxy behavior must agree about the public scheme. A development site accessed over plain HTTP can behave differently. When login appears to succeed but the next request is anonymous, inspect whether the browser actually stored and sent the cookie.

SameSite is one defense related to cross-site requests. It is not a substitute for a complete policy for state-changing requests. See [web security](../../guides/web-security/).

## Expiry and authentication

Session expiry answers how long the conversation remains available. Account validity answers whether the person may still sign in or act. A stored session does not mean a disabled account should retain access forever.

Logging out and changing credentials also raise different questions: are we ending this browser's session, revoking other sessions, or changing the evidence used to validate them? Use the current authentication contract for implemented behavior and define any additional revocation policy explicitly.

## Concurrent requests

Two browser tabs can update the same session at nearly the same time. A sequence that reads a value, changes it, and writes it back is not automatically an atomic operation across requests.

For example, incrementing an order count in session state can lose an update. Keep durable counters and order state in a database with the appropriate concurrency controls. A session can point to a shopping cart without being the only place the cart exists.

## Testing session behavior

Test more than one request: set a preference, reuse the cookie, and confirm that a second request can read it. Then test a request without that cookie, an expired session, and the actual store used in deployment.

The [authentication guide](../../guides/authentication/) builds on this state, while [HTTP tests](../../guides/http-tests/) explains how to choose the right test boundary.
