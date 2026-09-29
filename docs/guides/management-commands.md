---
title: CLI and management commands
status: preview
---

Management commands run application work from a terminal or an automation system. Rebuilding a catalog index, inspecting configuration, and repairing a bounded set of records don't need public HTTP endpoints.

Cot's CLI task interface lets a Project register commands. These tasks run when invoked; they are not the durable background jobs described in the proposed queue guide.

## Generator and application CLI

The `cot` executable creates projects and provides development tooling. The generated application's own executable has a separate CLI. From its project directory, inspect that CLI with:

```bash
cargo run -- --help
```

The `--` passes subsequent arguments to the application. Use command-specific help to confirm available options rather than assuming a command from Django's management interface or Laravel's Artisan exists in Cot.

## Registering a task

A custom task implements `CliTask` and supplies its command definition and execution logic. The Project registers it through `register_tasks`. The repository's `examples/custom-task` demonstrates that complete integration using Clap.

Keep argument parsing at the command boundary and application logic in ordinary functions. A catalog-repair function can then be tested directly without constructing terminal arguments for every case.

## A repair command's contract

Suppose an operator needs to rebuild search entries for products whose descriptions changed. A useful command design includes:

| Input | Meaning |
| --- | --- |
| Product selection | A bounded set or an explicit all-products choice |
| Environment configuration | Which database and index are targeted |
| Dry-run option | Report intended changes without writing |
| Batch size | Bound memory and request cost |
| Resume position | Continue after an interrupted batch |

This is an illustrative application command, not a list of built-in Cot flags. Names and defaults should make the scope clear before the operator runs it.

## Repeatable execution

Automation can retry a command after losing its output or receiving a nonzero exit. A repeat-safe repair should recognize completed work or replace derived state deterministically.

If the command sends email or changes an external system, repeated execution needs the same idempotency design as an HTTP endpoint. Calling it from a terminal doesn't make duplicate side effects harmless.

## Output and exit status

Write useful summaries: scanned, changed, skipped, and failed counts. Keep progress diagnostics distinct from machine-readable output when scripts consume the command. Avoid printing secrets or full customer records as routine progress.

A nonzero exit should mean something an operator or scheduler can act on. Partial success deserves a documented policy and enough information to resume safely; a generic success message after silently skipping failures is misleading.

## Configuration and initialization

The command should load the intended project configuration and only the resources needed for its work through supported bootstrap interfaces. Verify the target environment before a destructive operation, and make broad selections explicit.

A dry run must use the same selection and validation rules as execution. A separate simplified query can give a reassuring preview that does not match the later write.

## Scheduling commands

An external scheduler can invoke a command, but overlapping invocations and missed runs remain application concerns. Use a shared coordination mechanism when a process-local guard isn't sufficient.

See [scheduled tasks](../../guides/scheduling/) for the intended scheduling semantics and the [CLI reference](../../reference/cli/) for current command discovery.
