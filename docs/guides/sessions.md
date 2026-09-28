---
title: Cookies and sessions
status: preview
---

HTTP requests are independent. A session lets an application associate several requests with the same browser, while a cookie carries information between the browser and server. They are related, but they are not the same storage mechanism.

## Where state lives

A session cookie can identify a record in a server-side store. The store holds application values; the browser sends the identifier on later requests. Cot’s session integration and middleware configuration define how this association is managed.

## Expiry and invalidation

Cookie expiry controls how long a browser retains a cookie. Server-side expiry controls whether stored state remains valid. Logging out needs to invalidate the relevant authentication state, not merely hide a button in the interface.

## Concurrent requests

Two requests from the same browser can run at once. A read-modify-write operation on session data should not be assumed to behave like a database transaction. Counters or business records generally need a more appropriate persistence boundary.

## Deployment and storage

Process-local state and shared storage behave differently when an application has several instances. The chosen session store must fit that deployment. Cookie attributes also depend on whether the site is served over HTTPS and how cross-site requests are handled.

## Related reading

- [Authentication](../../guides/authentication/).
- [Middleware](../../guides/middleware/).
- [Configuration reference](../../reference/configuration/).
