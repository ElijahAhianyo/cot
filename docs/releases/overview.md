---
title: Releases and upgrades
status: preview
---

A release can change Rust interfaces, configuration, generated code, or runtime behavior. An upgrade guide should make those consequences visible before an application changes versions.

## Upgrade an application

The existing [upgrade guide](../../upgrade-guide/) remains the canonical application migration document.

## Intended release-note structure

Each release should identify new capabilities, breaking changes, bug fixes, compatibility requirements, and known limitations. A feature announcement should link to its guide and Rust API rather than duplicate both.

## Deprecations and support

This prototype reserves a place for a documented deprecation and support policy. It does not invent support windows or release commitments that the project has not adopted.
