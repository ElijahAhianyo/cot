---
title: Community and contributing
---

Cot is developed in the open. A useful report or contribution makes the problem reproducible and gives maintainers enough context to evaluate it.

## Built with Cot

Discover [projects built by the community](../showcase/), explore their source, and find ideas for your own application.

<div class="doc-card-grid">
<a class="doc-card" href="../showcase/"><strong>Explore the showcase</strong><span>Websites, tools, and experiments built with Cot.</span></a>
<a class="doc-card" href="../showcase/#share-your-project"><strong>Share your project</strong><span>Tell the community what you’ve made and how Cot fits.</span></a>
</div>

## Get help

Use [GitHub discussions](https://github.com/cot-rs/cot/discussions) for questions and design conversations. Include the Cot version, relevant configuration, and a small example when behavior is unclear.

## Report a bug

[Open an issue](https://github.com/cot-rs/cot/issues) with expected behavior, actual behavior, and reproduction steps. Remove secrets and private data from logs and sample projects.

## Contribute

Read the repository's [contribution guide](https://github.com/cot-rs/cot/blob/master/CONTRIBUTING.md) and [AI policy](https://github.com/cot-rs/cot/blob/master/AI_POLICY.md). Security reports should follow the project's [security policy](https://github.com/cot-rs/cot/blob/master/SECURITY.md).

## Make a question reproducible

Start with the result you expected and the result you observed. Include the Cot and Rust versions, enabled features, database backend where relevant, and the smallest code path that shows the behavior. A route question usually needs the mount prefix and HTTP method as well as the handler.

For a database issue, include the relevant model and migration shape with fictional data. For a build issue, include the first relevant compiler diagnostic and the dependency declaration. Avoid screenshots of text when searchable text will explain the problem more clearly.

## Reduce a bug report

Try to reproduce the issue in a small generated project or focused test. Remove unrelated handlers and services until the remaining example still fails. State the exact command or request that reproduces it and whether the result is consistent.

Do not include production credentials, session cookies, access tokens, customer records, or private storage URLs. Replace those values before posting. A reproduction should establish the failure without requiring access to someone else's account or infrastructure.

## Propose a documentation improvement

Identify the page and the point where the explanation stopped helping. For example, “I couldn't tell whether the client runs migrations” gives an editor a concrete gap to address. Include the correct behavior and a source or test when you know it.

Keep an explanation focused on understanding, a how-to focused on a task, and a tutorial focused on a learning outcome. Rust API contracts belong in Rustdoc. Cross-links can connect those needs without copying the same reference into several pages.

## Prepare a contribution

Follow the checkout's contribution instructions and run checks relevant to the change. A behavior change needs evidence for the intended outcome; a documentation example should compile and, when it describes a workflow, run as documented.

Explain the problem and the resulting behavior in the change description. For a proposed feature, distinguish design questions from an implemented contract. Maintainers need to assess both the application's benefit and the cost of maintaining the public API.

Report a suspected vulnerability through the security policy's designated route. Public troubleshooting threads are appropriate for ordinary bugs, not for publishing sensitive exploit details before coordinated handling.
