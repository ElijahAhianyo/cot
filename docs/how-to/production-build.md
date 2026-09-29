---
title: Prepare a production build
status: preview
---

Prepare an existing Cot application for a deployment by building a reproducible artifact and checking its runtime requirements. You need a committed lockfile, a known target platform, and access to the deployment's configuration and persistent services.

The commands below run from your application's directory. They build the application in that directory, not the Cot framework workspace.

## Build the release artifact

```bash
cargo build --release --locked
```

The executable appears under `target/release/`, unless your Cargo configuration changes the target directory or you select a cross-compilation target. Its name comes from your binary target. For a binary named `issue_tracker`, the default path is `target/release/issue_tracker`.

`--locked` requires Cargo to honor the lockfile. It does not make a macOS binary runnable on Linux. Build for the deployment platform, and record the source revision, compiler, target, and enabled Cargo features with the artifact.

## Check the runtime package

Include the files your application reads at runtime. Depending on your project, these can include configuration files, TLS certificates, and external templates or assets. Resources embedded by a macro are part of the compiled artifact; runtime file reads still need files.

Run the binary in the intended runtime image or host environment. This catches missing shared libraries and certificate roots that a successful build on the developer's machine cannot detect.

## Select production configuration

Inspect the executable's command interface:

```bash
./target/release/issue_tracker --help
```

For this example binary, a production invocation can select the `prod` configuration and listen on the service interface:

```bash
./target/release/issue_tracker --config prod --listen 0.0.0.0:8000
```

Supply the configuration through your project's supported loading mechanism. Provision a real secret key, database credentials, session settings, and service endpoints. Check debug behavior explicitly. A release build is an optimization profile, not a substitute for reviewing production settings.

Binding to all interfaces is appropriate inside many hosting environments; the surrounding network and proxy still determine who can reach the port. Use the address required by your deployment.

## Plan schema changes before startup

Cot applies registered migrations during project startup. Review pending changes before starting the new executable against a production database. Establish who runs the rollout, how competing instances are coordinated, and whether old and new application versions can use the resulting schema together.

A backup is useful only if it can be restored. For a destructive change, rehearse the recovery procedure before the rollout. See [backups and recovery](../../guides/recovery/).

## Verify the running artifact

Check a normal page, a missing page, an authenticated request, and an application failure through the public endpoint. Confirm that the intended configuration is active and public responses do not contain private diagnostics.

Exercise a database write and read, a session across requests, and any external service the release depends on. Restart the process and confirm that persistent data remains available. Test graceful termination under a request so the process manager's shutdown timeout is meaningful.

Keep these checks with the release procedure. [Deployment architecture](../../guides/deployment/) covers the proxy, storage, and process responsibilities beyond the build.
