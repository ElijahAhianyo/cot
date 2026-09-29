---
title: Model an issue
status: preview
---

Our issue needs a stable identifier, a short title, and enough detail to reproduce the problem. We'll inspect that model, connect it to the migration that creates its table, and check that the record outlives the server process.

Continue in the [companion application's directory](../first-app/). Keep using the same working directory when starting the server, because the development database path is relative to it.

## Read the model

Open `src/models.rs`. It contains:

```rust
use cot::db::{Auto, model};

#[derive(Debug)]
#[model]
pub struct Issue {
    #[model(primary_key)]
    pub id: Auto<i64>,
    pub title: String,
    pub description: String,
}
```

`#[model]` connects the struct to Cot's ORM. The primary key identifies one row, and `Auto<i64>` lets the database assign it during insertion. We don't ask a person reporting an issue to choose that ID.

The title and description are stored as strings. The form will impose input lengths in a later chapter. A model's storage type and a form's validation policy answer different questions; a string field alone doesn't establish a useful title length.

## Inspect the migration

Open `src/migrations/m_0001_initial.rs`. Find the operation creating `issue_tracker__issue`, then locate the `id`, `title`, and `description` fields. The ID has the primary-key and automatic-value settings corresponding to the model.

Now open `src/migrations.rs`. It includes that migration in `MIGRATIONS`. Finally, find `IssuesApp::migrations` in `src/main.rs`; it returns the registered migration list to Cot. All three connections matter: defining a struct alone doesn't create the table.

The companion already includes the generated migration. When changing a model in your own project, generate a new migration with a matching Cot CLI:

```bash
cot migration make
```

Run that command from the application directory. Review the generated operation before starting against data you care about. Do not rewrite an already-applied initial migration to pretend the database always had the new shape.

## Follow an insertion

In `create_issue`, the valid form becomes a new `Issue` with `Auto::auto()` as its initial ID. `insert(&db).await?` writes the row and updates the model with its generated key.

The handler uses that key in the redirect. This is why the browser reaches the particular issue it just created rather than an arbitrary position in the list. A failed insert propagates an error; it shouldn't produce a successful redirect that points at a nonexistent record.

For related insertion and update examples, see [queries](../../databases/queries/). We use insertion here because reporting an issue creates a new resource.

## Check persistence

Run the application and create two issues with different titles. Open each detail page and note its URL. Stop the server, then start it again from the same directory.

Both issues should still appear, and their detail URLs should identify the same records. The companion configures `sqlite://issue-tracker.sqlite3?mode=rwc` for ordinary runs. Its tests use an isolated temporary SQLite file instead, so they don't change your browser's learning data.

If the list appears empty after a restart, check the working directory and selected database before adding another migration. Starting against a different file can look like data loss even though the original file still exists.

## Keep the first model small

Comments, attachments, and owners would introduce relationships and permission rules. We can add those after the basic storage and request boundaries are clear. The [relationship guide](../../guides/relationships/) explains how those associations change queries and lifecycle decisions.

Next, [display the saved issues](../views/) and distinguish an invalid identifier from an identifier that has no record.
