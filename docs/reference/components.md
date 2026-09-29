---
title: Built-in components
---

The Rust API is the reference for Cot's components. This page provides a task-oriented index without duplicating their signatures.

## HTTP components

| Component family | Reference | Explanation |
| --- | --- | --- |
| Routes and URL generation | [Router module](https://docs.rs/cot/latest/cot/router/) | [Routing](../../routing/) |
| Request extraction | [Request module](https://docs.rs/cot/latest/cot/request/) | [Requests and extractors](../../guides/requests/) |
| Response conversion | [Response module](https://docs.rs/cot/latest/cot/response/) | [Handlers and responses](../../guides/responses/) |
| Middleware | [Middleware module](https://docs.rs/cot/latest/cot/middleware/) | [Middleware](../../guides/middleware/) |

## Application components

[Authentication](https://docs.rs/cot/latest/cot/auth/), [forms](https://docs.rs/cot/latest/cot/form/), and [configuration](https://docs.rs/cot/latest/cot/config/) each have their own Rust module documentation. These links target the published release; use locally generated Rustdoc when inspecting unreleased APIs.

## Data and services

| Responsibility | Rust API | Explanation |
| --- | --- | --- |
| Models, fields, and queries | [`cot::db`](https://docs.rs/cot/latest/cot/db/) | [Database queries](../../databases/queries/) |
| Schema history | [`cot::db::migrations`](https://docs.rs/cot/latest/cot/db/migrations/) | [Migrations](../../databases/migrations/) |
| Authentication identity | [`cot::auth`](https://docs.rs/cot/latest/cot/auth/) | [Authentication](../../guides/authentication/) |
| Session state | [`cot::session`](https://docs.rs/cot/latest/cot/session/) | [Sessions and cookies](../../guides/sessions/) |
| Form parsing and validation | [`cot::form`](https://docs.rs/cot/latest/cot/form/) | [Validation](../../guides/validation/) |
| Cache access | [`cot::cache`](https://docs.rs/cot/latest/cot/cache/) | [Caching](../../caching/) |
| Email delivery | [`cot::email`](https://docs.rs/cot/latest/cot/email/) | [Sending email](../../sending-emails/) |
| Test requests and application clients | [`cot::test`](https://docs.rs/cot/latest/cot/test/) | [HTTP tests](../../guides/http-tests/) |

Feature gates apply to several modules. Use the module's availability information and the [feature index](../features/) when a path is missing from your build.

## Composition interfaces

`Project` owns application-wide composition. `App` contributes a feature's routes and other registered resources. `AppBuilder` connects app instances to the project; a Rust module declaration alone does not perform that registration.

The [`cot::project`](https://docs.rs/cot/latest/cot/project/) module documents lifecycle contexts and root-handler construction. The [`cot::cli`](https://docs.rs/cot/latest/cot/cli/) module documents application tasks. Read [projects and apps](../../guides/projects/) before choosing an integration point for a reusable package.

## Choosing the right level

Use the public high-level interface when it expresses the operation you need. For example, a handler returning HTML can use `Html`; it does not need to manually reconstruct every response header. Reach for a lower-level response when the status, headers, or body behavior actually requires it.

Conversely, a familiar guide topic is not evidence that a built-in component exists. Authorization rules can be ordinary application code, and pages labeled Proposed describe target capabilities. Only verified public APIs belong in this component index.

## Reading an API entry

Check the item's feature gate, trait bounds, return type, errors, and examples together. A method returning `Result<Option<T>>` distinguishes an operation failure from an absent value; a method taking ownership affects whether the caller can reuse the value afterward. These details are part of the contract, not incidental Rust syntax.
