---
title: CLI commands
status: preview
---

Cot has a project generator and an application CLI. They run in different contexts: `cot new` creates a project, while a generated application's executable manages that application.

## Project generator

| Command | Purpose |
| --- | --- |
| `cot new <name>` | Generate an application project |
| `cot --help` | Inspect the installed generator's commands |

## Application CLI

From an application directory, `cargo run -- --help` displays the commands available to that application. Custom tasks and enabled features can change the list.

| Option | Default in this checkout | Purpose |
| --- | --- | --- |
| `-c`, `--config` | `dev` | Select the configuration name or file |
| `-l`, `--listen` | `127.0.0.1:8000` | Select the server address or port |

## Management commands

The current implementation registers checks and static-file collection. Database migration subcommands are feature-dependent. The application's own `--help` is the most direct command reference; the [`cot::cli` Rust API](https://docs.rs/cot/latest/cot/cli/) describes extension interfaces.
