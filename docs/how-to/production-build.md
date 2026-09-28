---
title: Prepare a production build
status: preview
---

Use this checklist when an existing Cot application is ready for its first release build. Deployment-specific proxy and service configuration remains a separate task.

## Build the application

From the application directory:

```bash
cargo build --release --locked
```

The executable is written under `target/release/`. Its filename follows the application's binary target, so do not assume every project produces a binary named `cot`.

## Review runtime configuration

Select the intended configuration, provision its secret key and service credentials, and confirm debug behavior. Check the application's CLI with `--help` before writing a process-manager command. [Configuration](../../guides/configuration/) explains why a release build alone does not establish these settings.

## Verify before accepting traffic

Confirm that the application starts with the intended configuration and can reach its persistent services. Check a successful request, a missing page, and a deliberate application error. Public responses should not expose private diagnostics.

## Continue with deployment

[Deployment architecture](../../guides/deployment/) covers the proxy, persistent services, health checks, and shutdown. This preview is a build-and-verification outline, not a complete container or hosting recipe.
