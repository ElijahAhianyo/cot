---
title: Releases and upgrades
status: preview
---

An application upgrade can affect Rust interfaces, generated code, configuration, and stored data. Review those boundaries before changing the dependency version, then test the behavior your application relies on.

The [upgrade guide](../../upgrade-guide/) is the canonical application migration document. The repository's [release history](https://github.com/cot-rs/cot/releases) records published releases. This documentation preview does not assign release dates or support promises to proposed features.

## Match the documents to the version

A development guide can describe code newer than the published crate. Conversely, the latest Rustdoc link may describe a release older than the checkout you are building. Record the installed Cot version, generator version, and source revision before diagnosing a mismatch.

For a released dependency, read the corresponding release notes and API documentation. For a local checkout, inspect its manifest and generate local Rustdoc. [Cargo features and compatibility](../../reference/features/) explains the build-time part of that comparison.

## Review an upgrade

| Area | Questions to resolve |
| --- | --- |
| Rust APIs | Which imports, signatures, trait implementations, or feature flags changed? |
| Configuration | Did accepted fields, defaults, or loading behavior change? |
| Generated source | Does the generator produce code requiring a matching framework version? |
| Database | Do migrations preserve data and compatibility during rollout? |
| Sessions and authentication | Can users remain signed in, and are key or storage changes required? |
| External contracts | Do clients observe different statuses, bodies, headers, or retry behavior? |

A successful compile resolves only part of this table. Keep representative HTTP and persistence tests around behavior that matters to your application.

## Test the transition

First test the new version with a fresh environment. Then rehearse upgrading representative existing data. A schema that can be created from scratch may still fail on duplicate, null, or unusually long values already present in an installation.

For deployments where old and new processes overlap, establish which schema and message formats both versions understand. A source-level rollback may not reverse a data transformation. See [releases and recovery](../../guides/recovery/) for operational sequencing.

## Understand change categories

A new capability should link to its maintained guide and API reference. A breaking change needs the affected behavior and the application action required. A bug fix can also change an observable outcome if an application depended on the earlier behavior.

Deprecation notices should identify the replacement and any adopted removal plan. Use the project's actual published policy for support windows; a page in this prototype is not a maintenance commitment.

## Report an upgrade problem

Include the old and new versions, relevant features, a small reproduction, and the observed difference. Distinguish a compiler failure from a changed response or a migration failure. Remove secrets and private data from diagnostic material.

The [community page](../../community/overview/) points to support and contribution channels. A reproducible compatibility report is more useful than a large application dump with an unexplained failure.
