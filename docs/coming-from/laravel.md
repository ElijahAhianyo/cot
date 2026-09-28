---
title: From Laravel
status: preview
---

If you're used to Laravel, start with how the application is assembled. Cot gives routes, data, forms, and other services a shared framework structure. Rust makes the types and dependencies in that structure explicit.

## Familiar concepts

These are starting points for comparison, not interchangeable APIs.

| In Laravel | Explore in Cot |
| --- | --- |
| Routes and controller actions | [Routes and handlers](../../routing/) |
| Service providers and bootstrapping | [Project and app lifecycle](../../guides/lifecycle/) |
| Eloquent models | [Cot models and queries](../../databases/queries/) |
| Blade views | [Templates](../../templates/) |
| Artisan commands | [Cot CLI reference](../../reference/cli/) |

## Where dependencies come from

Laravel's lifecycle describes bootstrapping through service providers and its container. In Cot, begin with the Project and App contracts. Don't translate a container binding into a similarly named Rust method; decide which component owns the dependency and how the handler accesses it.

For an order-confirmation endpoint, the order data, database connection, and mail configuration have different lifetimes. Request-specific values should not become mutable state shared by all requests.

## Saving an order and sending mail

The order must exist before an email can refer to it. Sending mail inside a database transaction also raises a failure question: an email can't be rolled back with the transaction. Explain and test that boundary even if the initial implementation sends mail synchronously.

Laravel queue terminology is useful background, but Cot's [background tasks](../../guides/background-tasks/) are proposed. There is no implied equivalent of dispatching a Laravel job in this preview.

## Keep application rules explicit

A valid form does not establish permission to modify the order. Keep validation, authorization, persistence, and side effects distinguishable. When replacing an existing endpoint, compare its error status and response body as well as the successful response.

## Continue reading

If Rust is new to you, start with [web development in Rust](../rust-for-web-developers/). The [transition overview](../overview/) explains the scope and availability labels. For a complete first project, use the [tutorials](../../tutorials/overview/).

The comparison uses the official [Laravel documentation](https://laravel.com/framework/docs/13.x/lifecycle). Consult the source framework documentation for its exact version-specific behavior.
