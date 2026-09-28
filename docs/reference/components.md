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
