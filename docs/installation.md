---
title: Installation
status: preview
---

Cot applications are Rust projects managed by Cargo. Install the toolchain and Cot CLI, create a project, and start it locally. The development checkout described by this documentation requires Rust 1.94 or later; match the requirements of the release you actually use.

## Check your Rust tools

You need `rustc` and `cargo` available in your terminal. [Rust's installation instructions](https://www.rust-lang.org/tools/install) describe the supported setup for your operating system.

```bash
rustc --version
cargo --version
```

If compilation later reports an unsupported compiler version, compare the installed toolchain with the project's manifest and toolchain configuration. Installing a newer toolchain does not always change a directory-specific override.

## Install the published CLI

```bash
cargo install --locked cot-cli
```

This installs the published generator. Confirm that the executable is available:

```bash
cot --version
cot --help
```

If `cot` isn't found after installation, check that Cargo's binary directory is on your shell's PATH. If a different version runs, inspect which executable your shell resolves before reinstalling it.

## Create an application

Run these commands from a directory where a new `issue_tracker` directory can be created:

```bash
cot new issue_tracker
cd issue_tracker
cargo run
```

Open `http://127.0.0.1:8000/`. The generated welcome page confirms that the project compiled, started, and answered a request. Keep the terminal running while using the application; stop it with Ctrl+C.

If that port is occupied, choose another:

```bash
cargo run -- --listen 127.0.0.1:8001
```

Use the same port in the browser. The default local listening address is appropriate for this first check.

## Work with the development checkout

Published tooling and unreleased framework source can differ. From the root of a Cot source checkout, install its CLI:

```bash
cargo install --path cot-cli --locked
```

Then point `cot new --cot-path` at that checkout's `cot` crate directory when generating a project. The path must contain the crate's `Cargo.toml`, not the workspace manifest one directory above it. Use `cot new --help` for the exact options supported by that CLI.

The [tutorial companion](../tutorials/first-app/) already has a relative dependency on the matching crate in its checkout. You don't need to regenerate that application or publish a version to run it.

## Understand the generated files

The application manifest selects Cot and its features. `src/main.rs` defines project composition, an app, and an initial handler. The configuration directory supplies development settings, while templates and static files provide the welcome page's presentation.

Database-backed applications also need their registered migrations. The generated project connects these components; deleting an app registration or changing configuration can alter more than the initial page.

## Check a failed first run

A compiler error belongs to the build stage. A connection error after the binary starts belongs to runtime setup. A browser that cannot connect may be using the wrong address or a process that exited. Read the terminal's first relevant error before changing unrelated settings.

For a small working application with tested examples, continue to [your first Cot application](../tutorials/first-app/). To understand the generated structure independently, read [projects and apps](../guides/projects/).
