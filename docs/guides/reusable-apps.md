---
title: Reusable apps and integrations
status: preview
---

A reusable app defines a boundary another project can depend on. Its routes, configuration, and initialization behavior form a contract just as its public Rust types do.

## Public interfaces

Consumers need to know which types and settings are intended for use and which are implementation details. Exposing fewer stable extension points is often easier to maintain than exposing all internal structure.

## Registration and dependencies

An app should make its required services and feature flags clear. Project-specific paths, secrets, or assumptions about middleware ordering should not be hidden inside a reusable component.

## Compatibility

Changes to routes, database schemas, and configuration can break consumers without changing a Rust signature. Versioning and migration documentation need to account for those interfaces too.

## Related reading

- [Projects and apps](../../guides/projects/).
- [Cargo features](../../reference/features/).
