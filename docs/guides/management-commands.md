---
title: CLI and management commands
status: preview
---

Management commands run application work outside an HTTP request. Data repair, imports, and operational checks still need the correct configuration and access to application services.

## Command context

Cot exposes CLI tasks through its project interfaces. A command’s setup should make clear whether it needs configuration, a database, or the complete application context.

## Automation contracts

Exit status and machine-readable output matter when a command runs in automation. A successful message should mean the operation actually completed; diagnostics should not obscure structured output.

## Repeatability

An interrupted import can leave partially completed work. Commands that may be retried need progress tracking or another way to avoid duplicated effects.

## Related reading

- [CLI reference](../../reference/cli/).
- [Configuration](../../guides/configuration/).
