---
title: From Axum
status: preview
---

With Axum experience, async handlers and request extractors will already feel familiar. The larger change is the surrounding application: Cot provides project and app structure alongside integrated features such as models, forms, and an admin panel.

## Familiar concepts

These are starting points for comparison, not interchangeable APIs.

| In Axum | Explore in Cot |
| --- | --- |
| Router and handlers | [Cot routing and handlers](../../routing/) |
| Request extractors | [Cot request extractors](../../guides/requests/) |
| Application state | [Project context and shared state](../../guides/async-state/) |
| Middleware composition | [Cot middleware integration](../../guides/middleware/) |
| Chosen persistence library | [Cot models, when appropriate](../../databases/overview/) |

## Familiar shapes, different contracts

Two extractors can have similar names without implementing the same trait. Check Cot's imports, extraction order, and rejection behavior before adapting a handler. A mechanical import replacement doesn't establish compatibility.

For a product lookup, compare malformed IDs, missing rows, and backend failures separately. Those cases often reveal differences that a successful request hides.

## What belongs in an app

A Cot app contributes framework behavior through the App trait, including its router. That boundary can group a feature such as a catalog with its related framework integration. It isn't simply another name for an Axum Router.

## Reusing existing Rust code

Code that accepts ordinary Rust values and returns a domain result is often easier to reuse than code tied to framework request types. For example, a pricing calculation can remain independent while the handler translates request data into its inputs.

Middleware and state wrappers need closer inspection. Shared ecosystem dependencies don't guarantee that their trait bounds, body types, or error conversions match. Check the [Rust API](https://docs.rs/cot/latest/cot/) and the local dependency versions before adopting an adapter.

## Method routing and rejection behavior

A handler's successful return type is only part of its interface. Compare an unsupported HTTP method, a malformed path parameter, invalid JSON, and an absent resource. The framework may reject the request before your handler runs.

Cot selects a matching route and then performs method handling and extraction. A selected route that cannot parse its parameter does not fall through to another route. Keep literal paths such as `/products/new` ahead of a broad `/products/{id}` pattern where required.

Use Cot's router and full-project tests for the adapted endpoint. Existing tests around a domain function can remain useful, but tests built on an Axum router do not exercise the new framework boundary.

## State and service handles

For a catalog client, decide whether the project owns one shared handle or constructs a value for each request. Cheaply cloning a handle can be correct when the underlying client shares its pool; cloning a container of mutable data has different semantics.

Cot's built-in extractors obtain configured framework resources. A custom resource needs an explicit integration with the request or project context and the appropriate extraction trait. Do not assume Axum's state machinery or extractor traits are interchangeable with Cot's.

## Persistence is a separate choice

Moving HTTP handling does not require rewriting every domain calculation. If adopting Cot's ORM, however, make the schema and transaction decision explicit. An existing SQL client and a Cot database handle may use different connection pools and independent transactions.

Two writes to the same database are not atomic merely because both happen inside one handler. Keep an operation within a deliberately chosen transaction boundary. Read [database queries](../../databases/queries/) before replacing persistence code mechanically.

## Middleware adapters

Shared Tower concepts can make integration possible, but compare service request and response types, error conversion, trait bounds, and wrapping order. Also check whether the middleware is attached to the main handler, error handling, or both.

A tracing layer that works on successful responses may behave differently when the framework renders an error. Test the actual failure path and shutdown behavior, not only compilation of the adapter.

## A bounded migration

Move one route group into a Cot app and mount it at the existing prefix. Keep its public contract stable and make framework-specific types stop at the HTTP boundary. That gives us a useful comparison of Cot's project integration without forcing unrelated Rust code through an unnecessary rewrite.

## Continue reading

The [transition overview](../overview/) explains the scope and availability labels. For a complete first project, use the [tutorials](../../tutorials/overview/).

The comparison uses the official [Axum documentation](https://docs.rs/axum/latest/axum/). Consult the source framework documentation for its exact version-specific behavior.
