---
title: Reference
---

Cot's API reference is **Rustdoc**. Types, traits, methods, macros, feature gates, and their exact contracts belong there.

<div class="doc-card-grid">
<a class="doc-card" href="https://docs.rs/cot/latest/cot/"><strong>Open the Rust API ↗</strong><span>Browse the published Cot crate, its modules, and its API examples.</span></a>
</div>

The remaining reference pages help you find configuration, commands, and build settings. They supplement Rustdoc rather than recreate it.

<div class="doc-card-grid">
<a class="doc-card" href="../../reference/configuration/"><strong>Configuration</strong><span>An index of configuration types and their authoritative definitions.</span></a>
<a class="doc-card" href="../../reference/cli/"><strong>CLI commands</strong><span>The project generator, application commands, and options.</span></a>
<a class="doc-card" href="../../reference/features/"><strong>Cargo features</strong><span>Optional capabilities and compatibility requirements.</span></a>
<a class="doc-card" href="../../reference/components/"><strong>Built-in components</strong><span>Find a component in Rustdoc and its explanatory guide.</span></a>
</div>

## Match the version

The Rustdoc link opens the latest published crate. This prototype follows the development checkout, which may contain newer APIs. Generate Rustdoc from the matching checkout when checking unreleased interfaces.

## Generate reference material for your checkout

From the Cot workspace, build the crate's documentation:

```bash
cargo doc -p cot --all-features --no-deps
```

Open the generated `target/doc/cot/index.html` using the target directory configured for that build. `--all-features` includes optional public interfaces; a particular application may enable fewer features.

For application work, match the dependency version and selected features before copying an API example. The guides explain how components fit together, while Rustdoc specifies what a particular item accepts and returns.

## Find the fact you need

Use configuration reference for the setting's owning type, CLI reference for the executable and command context, and the component index for a public module. If the question is why two settings interact or how to complete a task, follow the related guide rather than trying to infer the workflow from a signature alone.
