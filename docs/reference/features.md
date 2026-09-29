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

## Feature dependencies

Enabling `sqlite`, `postgres`, or `mysql` also enables Cot's shared `db` support. `redis` enables `cache` and `json`; `cache` itself depends on `json`. `openapi` enables JSON and schema integration, while `swagger-ui` also enables `openapi`.

The current `full` set includes the default backends and JSON, plus `fake`, `live-reload`, `test`, `cache`, `redis`, and `email`. It does not include `openapi` or `swagger-ui`. Check the manifest when a capability appears unavailable despite using `full`.

`fake` enables support for generated sample values. It is not a durable seeding command or a guarantee that generated data respects your application's business constraints. `test` exposes testing helpers; enabling it does not execute the application's tests.

## Inspect the selected graph

From your application directory, inspect the features Cargo actually resolved:

```bash
cargo tree -e features -i cot
```

Cargo features are additive across uses of the same package in the dependency graph. Disabling default features in one dependency declaration does not remove a feature enabled by another dependency. Inspect the graph when a supposedly disabled backend still appears in the build.

A minimal application should choose the capabilities it uses, then verify that combination in CI. An example that compiles with `full` does not prove it compiles with a narrower selection.

## Compatibility has several boundaries

Keep the Cot crate, generator, and documentation versions aligned. Review the Rust minimum version, target platform, native libraries, and dependent service versions when preparing a release. A compatible Cargo version requirement alone does not establish database schema or session compatibility between deployed application versions.

For a local checkout, generate the API documentation with the features used by your application. An item hidden behind a disabled feature may be absent from local Rustdoc even though it appears in the all-features published documentation.
