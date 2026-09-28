---
title: Installation
status: preview
---

Cot applications use Cargo, Rust's package manager. The development checkout currently requires Rust 1.94 or later; published versions may have different requirements.

## Install the CLI

```bash
cargo install --locked cot-cli
```

This installs the published CLI. To experiment with an unreleased checkout, use the matching CLI source instead of assuming the published generator includes unreleased features.

## Create and run a project

```bash
cot new issue_tracker
cd issue_tracker
cargo run
```

The generated application listens on `http://localhost:8000` by default. A welcome page confirms that the application is running.

## Choose the next step

The [first application tutorial](../tutorials/first-app/) explains the proposed learning sequence. The existing [introduction](../introduction/) includes working examples of project structure and views. For versions and optional capabilities, see [Cargo features](../reference/features/).
