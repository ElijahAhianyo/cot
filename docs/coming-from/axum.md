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

## Continue reading

The [transition overview](../overview/) explains the scope and availability labels. For a complete first project, use the [tutorials](../../tutorials/overview/).

The comparison uses the official [Axum documentation](https://docs.rs/axum/latest/axum/). Consult the source framework documentation for its exact version-specific behavior.
