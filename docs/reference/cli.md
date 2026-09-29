---
title: CLI commands
status: preview
---

Cot has two command interfaces. The `cot` executable creates projects and manages source-level work such as generating migrations. Your application's executable runs that application and exposes its registered management tasks.

This page describes the development checkout. Use `--help` from the version you installed when exact options differ. The [management-command guide](../../guides/management-commands/) explains how applications add tasks.

## Project and source commands

Run project-specific commands from the application's crate directory, or select the appropriate package in a workspace.

| Command | Purpose | Important inputs |
| --- | --- | --- |
| `cot new issue_tracker` | Create a project in a new directory | `--name` changes the crate name; `--cot-path` selects a local Cot crate; `--use-git` selects Git source |
| `cot migration make` | Generate schema changes from models | Optional path, `--app-name`, and `--output-dir` |
| `cot migration new backfill_titles` | Create an empty migration | Migration name, optional crate path, and `--app-name` |
| `cot migration list` | List discovered migrations | Optional project path |
| `cot cli completions bash` | Generate shell completion text | Shell name |
| `cot cli manpages` | Generate CLI manual pages | `--output-dir`; `--create` permits creating that directory |

`--cot-path` refers to the Cot crate directory containing its package manifest, not the root of the Cot workspace. Select a CLI from the same development checkout when using unreleased generator or migration behavior.

The generator's global `--package` or `-p` selects a workspace package. Its `--release` option changes where it looks for an application binary, and `--build` permits building a missing binary. These flags should not be confused with the flags Cargo consumes before `--` in a `cargo run` command.

## Application options

From an application directory:

```bash
cargo run -- --help
```

| Option | Default in this checkout | Purpose |
| --- | --- | --- |
| `-c`, `--config` | `dev` | Select configuration through the project's configuration loader |
| `-l`, `--listen` | `127.0.0.1:8000` | Select a port or listening address for the server |

For example:

```bash
cargo run -- --config dev --listen 127.0.0.1:8001
```

Cargo consumes its own options before the separator. The application receives the options after it. Once compiled, the binary accepts those application options directly without Cargo.

## Application tasks

| Task | Purpose | Operational consideration |
| --- | --- | --- |
| `check` | Verify configuration and configured service connections | Requires access to the services being checked |
| `collect-static` | Collect registered static assets | Inspect task help for output options |
| `migration rollback` | Undo migrations after a selected target | Available with database support; inspect the plan with `--dry-run` |
| Application-defined tasks | Run registered project operations | Names and arguments depend on the project |

Inspect a task before scripting it:

```bash
cargo run -- check --help
cargo run -- collect-static --help
cargo run -- migration rollback --help
```

The `cot` CLI can also forward application commands. That depends on finding the intended project binary; a stale binary can have an older task list than the source. Invoking the built executable directly makes the selected artifact explicit during deployment.

## Migrations and side effects

Generating a migration edits source files. Applying migrations changes a database. Cot applies registered migrations during project startup, while rollback is an explicit management operation. These operations are not interchangeable.

A rollback target is exclusive: the named migration remains applied and later migrations are undone. `zero` targets all migrations for the selected app. Cross-app dependencies can expand the rollback plan. Read [migrations](../../databases/migrations/) before using rollback against persistent data.

## Extension interfaces

The [`cot::cli` Rust API](https://docs.rs/cot/latest/cot/cli/) documents `Cli`, `CliTask`, and task groups. The running application's help remains the reference for the tasks that project actually registers. A library exposing a task type does not make it available until the project registers it.
