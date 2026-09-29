---
title: Display issues
status: preview
---

We'll connect the saved issues to two browser pages: a collection at `/issues/` and an individual issue at `/issues/{id}`. Continue with the running [companion application](../first-app/) and at least one saved issue.

## Read the route table

Find `IssuesApp::router` in `src/main.rs`. Its routes distinguish the list, the new-issue form, and the detail page:

| Path | Method | Handler |
| --- | --- | --- |
| `/` | GET | `home` |
| `/issues/` | GET | `list_issues` |
| `/issues/new` | GET and POST | `new_issue` and `create_issue` |
| `/issues/{id}` | GET | `show_issue` |

The literal `new` route appears before the ID route. Otherwise, a broad parameter route could select `new` and then fail to parse it as a number. The [routing guide](../../routing/) explains why extraction doesn't make Cot try the next route.

## Inspect the list query

`list_issues` receives Cot's configured `Database`, retrieves the issues, and passes them to `IssueList`. The query orders by the primary key so the display has a defined order.

The template loops over those values and escapes their titles when rendering HTML. Try reporting an issue whose title contains `<strong>Unexpected markup</strong>`. The title should appear as text, not become a bold HTML element supplied by the user.

This small list retrieves all issues. That is appropriate for exploring a few records; a real collection needs a bounded query and [pagination](../../guides/pagination/) before it grows large.

## Follow a detail lookup

The lookup inside `show_issue` uses the ID extracted from the URL. This isolated function shows the same database boundary:

```rust
use cot::db::{Auto, Database, model, query};
use cot::error::NotFound;

#[model]
struct Issue {
    #[model(primary_key)]
    id: Auto<i64>,
    title: String,
    description: String,
}

async fn find_issue(db: &Database, id: i64) -> cot::Result<Issue> {
    query!(Issue, $id == id)
        .get(db)
        .await?
        .ok_or_else(|| NotFound::new().into())
}
```

The companion already defines `Issue` in `models.rs`; don't add a second model when following the application. The complete block above is a standalone illustration of the lookup.

A successful query can return no row. We convert that absence to a not-found error. A database failure takes the `?` path instead. These are different outcomes even though neither can render the requested issue.

## Give the template its data

`IssueDetail` holds the retrieved `Issue`. Its template displays the title and description and links back to the collection. The handler finishes loading data before rendering; the template does not perform an implicit database query.

Change the template to add a heading above the description, restart, and view an existing issue. That edit changes presentation without changing the stored row.

The companion uses inline templates to keep the example compact. The [template guide](../../templates/) shows file-based templates, layouts, and route-aware links for larger applications. Its fixed root-relative links assume this example is mounted at the root; a reusable app should generate links from route names.

## Check the unsuccessful requests

Use a numeric ID that doesn't exist: `/issues/999999`. Expect a not-found response. Then open `/issues/not-a-number`. That request fails numeric path extraction before the detail query runs.

Finally, revisit `/issues/new`. It should still show the form, proving the literal route isn't being treated as a numeric ID. We will preserve these distinctions in tests rather than relying on manual checks alone.

Next, [submit and validate changes](../forms-auth/).
