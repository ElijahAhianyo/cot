---
title: Accept changes
status: preview
---

We'll submit a new issue, reject invalid input, and keep the useful input visible when validation fails. Continue with the [companion application](../first-app/), running locally.

The example accepts anonymous reports and has no editing endpoint. After testing that bounded flow, we'll examine the permission rule an owner-only edit feature would need. That extension is separate from the working form; the companion does not claim to provide a login system.

## Inspect the accepted fields

Find `IssueForm` in `src/main.rs`:

```rust
use cot::form::Form;

#[derive(Debug, Form)]
struct IssueForm {
    #[form(opts(min_length = 1, max_length = 120))]
    title: String,
    #[form(opts(min_length = 1, max_length = 2000))]
    description: String,
}
```

The title is limited to 120 characters and the description to 2,000. These are this application's choices. The database model remains a separate type, so adding a server-owned model field doesn't automatically expose it as form input.

## Display the form

On GET, `new_issue` builds the form context and passes it to `IssueFormPage`. The template includes a POST form with the action `/issues/new` and a submit button.

Open that page and inspect the rendered fields. The browser can help catch invalid input, but the server must validate too. A caller can bypass the browser's form controls entirely.

## Submit valid input

Report “The sign-in button is hard to find” with a description of the screen size where it happens. The POST handler receives `RequestForm<IssueForm>` and matches its result.

On `FormResult::Ok`, it constructs an `Issue`, inserts it, and redirects to the new detail URL. Refresh the detail page: the browser requests the detail page again rather than resubmitting the original form.

This redirect does not prevent every duplicate. If the original response is lost and the caller retries the POST, a second issue can be created. A retry-safe creation contract needs an operation identifier and corresponding server-side policy.

## Exercise server-side rejection

From another terminal, submit a blank title directly:

```bash
curl -i --data-urlencode 'title=' --data-urlencode 'description=The button disappears on a narrow screen.' http://127.0.0.1:8000/issues/new
```

The handler takes `FormResult::ValidationError` and renders the returned form context. The response contains the useful description and validation feedback instead of redirecting to a new issue. Visit the list and confirm that no blank-title issue was inserted.

The companion returns the form page with status 200 for invalid input. Other applications may choose another documented status; the important point is that tests agree with the chosen behavior and verify that no write occurred.

## Explore an owner-only rule

For an editing feature, a starting rule could look like this:

```rust
fn can_edit(actor_id: Option<i64>, owner_id: i64, closed: bool) -> bool {
    actor_id == Some(owner_id) && !closed
}

assert!(can_edit(Some(7), 7, false));
assert!(!can_edit(Some(8), 7, false));
assert!(!can_edit(None, 7, false));
assert!(!can_edit(Some(7), 7, true));
```

The actor must come from trusted authentication state, and the owner and closed state from the stored issue. A submitted `owner_id` is not evidence of identity. The mutation endpoint must enforce the rule even when its edit button is hidden.

## Before exposing the form publicly

Keep this learning application on the local interface. A public browser form needs an appropriate CSRF defense, access policy, abuse controls, and production configuration. Cot's form derive does not supply all those decisions. See [web security](../../guides/web-security/) and [authentication](../../guides/authentication/) when extending the example.

Next, [test the behavior](../test-deploy/) we have observed.
