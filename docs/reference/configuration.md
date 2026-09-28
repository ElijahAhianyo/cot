---
title: Configuration reference
status: preview
---

The complete field-level reference is the [`cot::config` Rust module](https://docs.rs/cot/latest/cot/config/). This index groups the settings by purpose and links to the authoritative definitions. Rustdoc links here describe the published release; a development checkout may differ.

## Configuration groups

| Group | Rust API | Purpose |
| --- | --- | --- |
| Project | [`ProjectConfig`](https://docs.rs/cot/latest/cot/config/struct.ProjectConfig.html) | Top-level application settings and configuration groups |
| Database | [`DatabaseConfig`](https://docs.rs/cot/latest/cot/config/struct.DatabaseConfig.html) | Database connection configuration |
| Sessions | [`SessionMiddlewareConfig`](https://docs.rs/cot/latest/cot/config/struct.SessionMiddlewareConfig.html) | Session middleware and cookie behavior |
| Static files | [`StaticFilesConfig`](https://docs.rs/cot/latest/cot/config/struct.StaticFilesConfig.html) | Asset URL and delivery settings |
| Cache | [`CacheConfig`](https://docs.rs/cot/latest/cot/config/struct.CacheConfig.html) | Cache stores and their configuration |
| Email | [`EmailConfig`](https://docs.rs/cot/latest/cot/config/struct.EmailConfig.html) | Email transports and defaults |

## Defaults and feature requirements

Use each Rust type's documentation for field types, defaults, and feature gates. This page deliberately does not maintain a second copy of every Rustdoc table.

## Configuration behavior

The [configuration guide](../../guides/configuration/) explains selection and environment boundaries. [Production configuration](../../guides/production/) discusses operational decisions.
