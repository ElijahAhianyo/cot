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

## Input types replace implicit assumptions

Consider an order request containing a product ID and quantity. A Rust input type describes the values we can decode; a validation rule establishes that the quantity is within policy. The server still retrieves the price and determines the customer from trusted state.

Keep these operations visible in the handler or a called application function. A field omitted from the input type cannot accidentally become an editable order attribute through a broad assignment operation. An explicitly accepted field still needs an authorization rule when changing it affects another resource.

## Queries and saved state

When a Cot model changes in memory, use the documented persistence operation to write it. Inspect the result and transaction boundary. Recreating an Eloquent model's columns does not recreate its observers, scopes, casts, or application-specific events.

For example, a shop might ensure every order query is restricted to the current organization. Make that restriction explicit in the Cot query path and test list, detail, export, and update endpoints. Missing it in one endpoint is enough to expose another organization's data.

## Templates and links

Cot templates use typed context and their own template syntax. Decide which values the handler supplies before rendering, and generate application links from route names where possible. A template should not need to discover dependencies or perform a hidden database lookup to display the customer name.

HTML escaping and permission checks address different problems. Escaping a customer's name prevents it from becoming markup; it doesn't establish that the viewer may see that customer.

## Replacing a queued workflow

Inventory queued jobs, scheduled commands, notifications, and retry policies before selecting a migration boundary. A route can return the same response while its background behavior changes substantially.

For an order-confirmation flow, record the durable event identity, retry limits, failure visibility, and duplicate-email policy. Cot's proposed task and scheduling guides explain the target design, but an existing Laravel queue requires a concrete integration or a retained worker service today.

## Verify a vertical slice

Move one operation with its validation, permission rule, database write, response, and side effect. Compare rejected requests as carefully as successful ones. Keep the existing service authoritative for the remaining routes until the new slice's behavior is established.

## Continue reading

If Rust is new to you, start with [web development in Rust](../rust-for-web-developers/). The [transition overview](../overview/) explains the scope and availability labels. For a complete first project, use the [tutorials](../../tutorials/overview/).

The comparison uses the official [Laravel documentation](https://laravel.com/framework/docs/13.x/lifecycle). Consult the source framework documentation for its exact version-specific behavior.
