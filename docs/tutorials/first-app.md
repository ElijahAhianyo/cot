---
title: Your first Cot application
status: preview
---

We'll work with a small issue tracker: a list of issues, a detail page, and a form for reporting a problem. An issue has a title and a description. By the end, we'll have followed a request through routing, validation, database storage, and rendering, then tested the same behavior without a browser.

The tutorial includes a [complete companion application](https://github.com/ElijahAhianyo/cot/tree/docs-preview/docs/site/checks/issue-tracker). We use that source throughout the chapters, so you can run the result before exploring each part. It is a local learning application, not an account-management system or a production starter.

## Before you begin

You need Git, Cargo, and a Rust toolchain supported by this Cot checkout. [Installation](../../installation/) explains the distinction between published and development versions. Basic knowledge of structs, functions, and `Result` is useful; [web development in Rust](../../coming-from/rust-for-web-developers/) connects those ideas to requests.

The companion uses the Cot crate in the same repository. You don't need to publish a crate or configure the documentation site's separate cot-site dependency to run this application.

## Run the application

If you already have the preview checkout, use it. Otherwise, clone the preview into a new directory:

```bash
git clone --branch docs-preview https://github.com/ElijahAhianyo/cot.git cot-docs-preview
cd cot-docs-preview/docs/site/checks/issue-tracker
cargo run --locked
```

Open `http://127.0.0.1:8000/`. The application redirects to the issue list. Select **Report an issue**, enter “The sign-in button is hard to find,” and describe what happens on a phone. Save the form. You should arrive at a page showing the new issue.

Leave the server running while using the browser. To stop it, press Ctrl+C in its terminal. If port 8000 is occupied, restart with `cargo run --locked -- --listen 127.0.0.1:8001` and use port 8001 in the browser.

## Find the moving parts

Open the companion directory in your editor:

| File | Responsibility |
| --- | --- |
| `src/main.rs` | Project setup, app registration, routes, handlers, templates, and form |
| `src/models.rs` | The `Issue` model |
| `src/migrations.rs` | Registered migration list |
| `src/migrations/m_0001_initial.rs` | Initial issue table |
| `Cargo.toml` | Cot dependency and enabled features |

The small application keeps its HTTP code in one file so we can see the connections. Larger applications can separate these pieces into modules without changing the responsibilities.

## Follow your first request

The project's `register_apps` method mounts `IssuesApp`. That app supplies the router. The root handler redirects to `/issues/`, where `list_issues` loads records and renders `IssueList`.

Change the list template's heading from `Issues` to `Reported issues`, save the file, and restart the application. Refresh the list. The new heading shows that we're running our changed source. The issue you created should still be present because normal runs use a SQLite file in the working directory.

Restore the heading if you want your output to match the later screenshots or examples. Editing a template doesn't require a database migration.

## What comes next

| Chapter | Checkpoint |
| --- | --- |
| [Model an issue](../model/) | Understand the stored fields and verify data survives a restart |
| [Display issues](../views/) | Trace list, detail, invalid-ID, and missing-record requests |
| [Accept changes](../forms-auth/) | Submit valid and invalid forms and examine where access rules belong |
| [Test and prepare a release](../test-deploy/) | Run behavior tests and identify the work needed before public deployment |

The form is deliberately local and unauthenticated. It does not implement login, owner-only editing, or CSRF protection. The later chapter explains those boundaries so a working learning example isn't mistaken for a complete public issue tracker.
