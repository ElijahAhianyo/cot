---
title: Configuration reference
status: preview
---

Cot's configuration types define the accepted fields and their defaults. This index helps locate the right type and connects TOML groups to the behavior they control. The complete field-level reference is the [`cot::config` Rust module](https://docs.rs/cot/latest/cot/config/).

Rustdoc links on this page describe the published release. Use locally generated Rustdoc for unreleased fields in a development checkout.

## Selection and loading

The application CLI's `--config` option selects the configuration passed to `Project::config`; its default is `dev`. The default project implementation loads configuration from the project's configuration files. A custom implementation can change that loading behavior.

`ProjectConfig::from_toml` parses a TOML string. Programmatic builders construct the same configuration types. Neither approach establishes a universal environment-variable mapping: a deployment must supply values through the loader it actually uses.

## Top-level settings

| Setting | Meaning | Where to check details |
| --- | --- | --- |
| `debug` | Controls debug behavior | `ProjectConfig` field documentation |
| `register_panic_hook` | Controls framework panic-hook registration | `ProjectConfig` field documentation |
| `secret_key` | Main secret used by framework cryptographic operations | `SecretKey` and `ProjectConfig` |
| `fallback_secret_keys` | Additional keys accepted for supported key-rotation paths | `ProjectConfig` field documentation |
| `extra` | Application-specific TOML values | `ProjectConfig::extra` and application validation |

Set production debug behavior deliberately. Do not infer the complete runtime policy from whether the executable was optimized.

## Service and middleware groups

| TOML group | Rust API | Responsibility |
| --- | --- | --- |
| `[database]` | [`DatabaseConfig`](https://docs.rs/cot/latest/cot/config/struct.DatabaseConfig.html) | Connection URL for the selected database backend |
| `[auth_backend]` | [`AuthBackendConfig`](https://docs.rs/cot/latest/cot/config/struct.AuthBackendConfig.html) | Authentication backend selection |
| `[middlewares.session]` | [`SessionMiddlewareConfig`](https://docs.rs/cot/latest/cot/config/struct.SessionMiddlewareConfig.html) | Cookie settings, expiry, and session store |
| `[static_files]` | [`StaticFilesConfig`](https://docs.rs/cot/latest/cot/config/struct.StaticFilesConfig.html) | Asset URLs, rewriting, and cache policy |
| `[cache]` | [`CacheConfig`](https://docs.rs/cot/latest/cot/config/struct.CacheConfig.html) | Cache store and operation settings |
| `[email]` | [`EmailConfig`](https://docs.rs/cot/latest/cot/config/struct.EmailConfig.html) | Email transport configuration |

Some groups require Cargo features. A valid value for a database URL does not enable its backend in a binary compiled without that backend. See [Cargo features](../features/) for build-time selection.

## A development database

This fragment selects a local SQLite file and permits creating it:

```toml
[database]
url = "sqlite://db.sqlite3?mode=rwc"
```

The relative path depends on the process's working directory. It is different from an in-memory database, and the file needs persistent storage if records must survive replacement of the host or container.

This is one configuration group, not a complete production configuration. The project still needs its other required values and registered database migrations.

## Session settings

Session configuration includes `secure`, `http_only`, `same_site`, `domain`, `path`, `name`, `always_save`, `expiry`, and `store`. Cookie scope and expiry are distinct from the storage backend's retention behavior.

For example, a secure cookie is intended for HTTPS. A local HTTP development setup may therefore require different settings from a public HTTPS deployment. Keep that difference in environment-specific configuration instead of weakening the production policy.

## Validation and deployment

Run the application's `check` task against the selected configuration in an environment that can reach its services. A successful parse proves the configuration has a valid shape; it does not prove credentials work or the service is reachable.

The [configuration guide](../../guides/configuration/) explains ownership and selection. [Production configuration](../../guides/production/) covers secrets, environment boundaries, and rollout decisions.
