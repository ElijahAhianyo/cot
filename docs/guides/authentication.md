---
title: Authentication
status: preview
---

Authentication establishes who is making a request. In our shop, a customer signs in so later requests can identify their account. Whether that customer may cancel a particular order is a separate authorization decision.

Cot separates the authentication backend, the user interface, and the request's authentication state. That lets a handler use the current user without implementing password verification itself.

## Checking credentials and logging in

`Auth::authenticate` checks credentials through the configured backend. A successful check returns a user; it doesn't log that user into the current session. `Auth::login` performs that second step.

For the database backend, the core operation looks like this:

```rust
use cot::auth::Auth;
use cot::auth::db::DatabaseUserCredentials;
use cot::common_types::Password;

async fn sign_in(
    auth: Auth,
    username: String,
    password: Password,
) -> cot::Result<bool> {
    let credentials = DatabaseUserCredentials::new(username, password);
    match auth.authenticate(&credentials).await? {
        Some(user) => {
            auth.login(user).await?;
            Ok(true)
        }
        None => Ok(false),
    }
}
```

This is application logic called after the login form has been processed, not a complete public login endpoint. Its caller still needs input handling, CSRF protection appropriate to the browser flow, response rendering, and abuse controls.

## Three outcomes

The example keeps three outcomes distinct. `Some(user)` means the backend accepted the credentials. `None` means it did not authenticate them. An error means the check could not complete, for example because the backing service failed.

Do not turn a backend outage into a misleading “wrong password” message. Conversely, avoid revealing account existence through unnecessarily different public messages for an unknown username and an incorrect password.

## The current user

`Auth::user` returns the current user interface. When the request is not authenticated, Cot supplies an anonymous user. Check the authentication state before treating an optional user ID as the owner of a resource.

A user ID received in a path, query, or form is not the authenticated identity. For `/customers/7/orders/`, the application still needs to decide whether the current actor may view customer 7's orders.

## Sessions and middleware

Session-based authentication needs session state available before authentication runs. The [middleware guide](../../guides/middleware/) shows that ordering. The session store and cookie policy determine whether subsequent requests can recover the login state.

Cot's login implementation cycles the session identifier. That behavior is useful to understand when writing tests: don't assume a pre-login session cookie remains the identifier after login.

The database backend also needs its model and migrations available. Read the backend and `DatabaseUserApp` Rust documentation when configuring a project rather than assuming that defining a login route creates the required table.

## Logging out and changing credentials

`Auth::logout` ends the current authentication state. Account recovery, password changes, and revoking other devices require an explicit lifecycle policy. They are not interchangeable with removing a login button from a page.

Use supported password types and hashing interfaces; don't compare plaintext passwords with database strings. Never record submitted passwords in request logs.

## Testing authentication

Test successful login across requests, rejected credentials, backend failure, logout, and an anonymous request to a protected resource. Then test the resource's own authorization rule with two distinct users.

Continue with [account lifecycle](../../guides/account-lifecycle/) for recovery and verification design, and [authorization](../../guides/authorization/) for deciding what an authenticated user may do.
