---
title: Configuration
status: preview
---

Configuration lets the same application run in different environments. Our shop can use a local database during development and a managed database after deployment without changing its product handlers.

There are three separate choices: which capabilities are compiled into the binary, which configuration the project loads, and what resources that configuration identifies. Keeping them separate makes a missing setting easier to diagnose.

## Selecting configuration

The default `Project::config` implementation loads configuration through `read_config`. The application CLI selects `dev` unless another configuration is requested. A project may override `config`, so the implementation is the source of truth for where its values come from.

From a project directory, the application help shows its options:

```bash
cargo run -- --help
```

The `--` separates Cargo options from application options. `cargo run --release` selects an optimized build; it does not select a configuration named `prod`. That distinction matters when the same release binary is used in staging and production.

## Typed settings

`ProjectConfig` contains framework settings. A project can load those settings from TOML or construct them in Rust. This small example parses a setting and checks the result:

```rust
use cot::config::ProjectConfig;

let config = ProjectConfig::from_toml("debug = false")?;
assert!(!config.debug);
```

This only demonstrates parsing. It is not a complete production configuration. In particular, a production application needs its own secrets and appropriate database, cookie, and service settings.

The [configuration reference](../../reference/configuration/) points to each group. Use the Rust API for exact fields and defaults instead of assuming that a setting from another framework has an equivalent name.

## Build-time features and runtime settings

A database URL cannot enable a database driver that was excluded from the build. Cargo features determine which optional code is available. Configuration then selects how the available code behaves.

For example, choosing a PostgreSQL database requires the corresponding compiled capability as well as a reachable server and valid credentials. A useful diagnosis checks all three rather than repeatedly changing the connection string. [Cargo features](../../reference/features/) explains the compiled side.

## Secrets and environment values

The project secret key and service credentials are private configuration. Development examples should use isolated resources, not copied production credentials. Keep a record of which values a deployment needs, but provide their contents through the deployment's secret mechanism.

Don't assume that every operating-system environment variable automatically overrides a Cot setting. That behavior depends on the project's configuration loader. If your Project reads an environment variable, parse it once, report an actionable error if it is absent or invalid, and avoid printing its value.

A missing required secret should fail startup rather than quietly substitute a convenient development value. Conversely, an optional display setting can have a documented default. The distinction is whether the application remains correct without the value.

## Stable settings across instances

Two replicas of the shop need compatible settings. If each process generates a different secret key at startup, a session accepted by one instance may fail on another. A key also needs to survive a restart when existing sessions are expected to remain valid.

Key rotation is a planned transition, not a change to make independently on each machine. `ProjectConfig` includes fallback secret keys; review the current authentication implementation and the retention period before removing an old key.

## Reviewing a configuration change

Suppose we switch the catalog cache to a shared backend. The URL is only part of the change. We also need to consider serialization, key names, expiry, connection limits, and what happens if the backend is unavailable.

A configuration review should explain the resulting behavior: which resource changes, how it is verified, and whether old and new application instances can coexist. For deployment-specific decisions, see [production configuration](../../guides/production/).
