---
title: Test and prepare a release
status: preview
---

We'll run the issue tracker's tests, inspect what they prove, and separate a working learning application from a deployable service. Continue in the [companion directory](../first-app/).

## Run the application tests

```bash
cargo test --locked
```

The tests create a separate temporary SQLite file for each project. The `test_client` helper explicitly applies the registered migrations before constructing Cot's application client. The client does not run the server's migration phase itself. They do not reuse the `issue-tracker.sqlite3` file containing your browser experiments.

Open the `tests` module at the end of `src/main.rs`. The first test posts a valid form, checks the redirect, follows its destination, and verifies the saved description. It then submits an invalid form and confirms that useful input is retained without appearing as a saved issue.

## Read the assertions as a contract

| Assertion | What it establishes |
| --- | --- |
| Valid POST returns 303 | The successful form uses the intended redirect status |
| `Location` reaches a detail page | The response identifies a retrievable issue |
| Detail body contains the description | The saved data reaches the page |
| Invalid form retains the description | Rejection doesn't discard useful input |
| Invalid description is absent from the list | The rejected form didn't create an issue |

The test follows the response's actual destination rather than assuming the new ID is always 1. That keeps the assertion about application behavior rather than an incidental database sequence.

## Check routing failures

The second test distinguishes a missing numeric ID, an invalid nonnumeric ID, the literal form URL, and an unsupported method. These requests exercise different boundaries and should not all be treated as one generic failure.

Temporarily move `/issues/{id}` ahead of `/issues/new` in the route list, then run the tests. The form-route assertion should reveal the mistake. Restore the intended order and rerun the suite. This gives us a concrete reason to keep the route-order regression test.

## Add a test for escaping

Extend the valid-form test with a title containing `<script>alert(1)</script>`. Retrieve its detail page and assert that the raw script tag is absent and escaped text is present. Keep the assertion focused on escaping rather than the entire HTML document, so changing a heading doesn't break an unrelated security check.

For a complete public application, add tests around authentication, resource permissions, CSRF behavior, and concurrent writes. Those controls are outside this local companion's implemented scope.

## Build an optimized executable

```bash
cargo build --release --locked
```

With the default target directory, the resulting executable is `target/release/issue_tracker`. Run it from the companion directory to keep the same relative database path:

```bash
./target/release/issue_tracker --listen 127.0.0.1:8001
```

Open port 8001 and verify that your local saved issues remain available. Stop the process when finished.

The companion deliberately uses programmatic development configuration, including debug behavior. Passing a different configuration name does not turn that custom loader into a production loader. Before deployment, replace it with an environment-appropriate configuration implementation and add the missing public-facing controls.

## Prepare a real deployment

Use [prepare a production build](../../how-to/production-build/) for artifact and environment checks. Decide where the database lives, how migrations run, how traffic reaches the process, and how data is restored after failure. Then exercise the application through that deployed path.

We now have a concrete application to reason about: routes select handlers, typed input is validated, database operations preserve issues, templates display them, and tests cover both successful and unsuccessful requests. Use the [guides](../../guides/overview/) to explore a subsystem in more depth without following another tutorial sequence.
