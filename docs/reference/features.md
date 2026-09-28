---
title: Cargo features and compatibility
status: preview
---

Cargo features select optional framework capabilities. They are build-time choices; application configuration controls how an enabled capability is used at runtime.

## Development checkout

This checkout declares Rust **1.94** as its minimum version. The table below is a snapshot of its feature declarations, not a compatibility promise for every published release.

| Feature | Purpose |
| --- | --- |
| `sqlite`, `postgres`, `mysql` | Database backend support |
| `json` | JSON support |
| `cache` | Caching interfaces |
| `redis` | Redis integration, including its required cache support |
| `email` | Email support |
| `openapi` | API schema integration |
| `swagger-ui` | Swagger UI integration |
| `test` | Test utilities |
| `live-reload` | Development reload support |

## Default and aggregate features

The current default set is `sqlite`, `postgres`, `mysql`, and `json`. The `full` aggregate is defined in the crate manifest; its name should not be interpreted as a guarantee that every optional feature, including OpenAPI, is enabled.

## Release-specific details

Consult the selected release's Cargo manifest and [Rust API](https://docs.rs/cot/latest/cot/) when choosing a version. The [upgrade guide](../../upgrade-guide/) covers application changes between releases.
