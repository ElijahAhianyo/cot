---
title: Browser tests
status: preview
---

A browser test checks behavior that only appears when the page, JavaScript, cookies, and navigation work together. A login handler can return the right response while the browser refuses its cookie; an HTML test can pass while a menu is impossible to use on a small screen.

We'll use a customer adding a delivery note to an order. The browser test should follow the controls a person uses and verify the visible result. Cot can provide the running test application; a separate browser tool drives the browser.

## Choosing what belongs in a browser

Keep calculations and most input variations in faster unit or HTTP tests. Use the browser for a small set of important journeys: sign in, submit a form, follow a redirect, upload a file, or navigate a responsive menu.

A browser test should not become the only place authorization is checked. Direct HTTP tests can try denied requests much more thoroughly, including calls that bypass the interface entirely.

## Starting from known state

Use isolated accounts and order data. Start the application with test configuration and wait for a real readiness condition before opening the page. `TestServer` provides a server boundary for Rust-driven integration tests; an external browser runner can also start a separately configured executable.

Do not reuse your personal development account or depend on yesterday's database. If a test assumes an unshipped order, create that state explicitly.

## A useful journey

The intended scenario can be described independently of a particular browser library:

```text
Sign in as the owner of order 42.
Open the order-detail page.
Enter "Leave with reception" in the delivery-note field.
Submit the form.
Wait for the saved-note confirmation.
Reload the page and confirm that the note is still present.
```

Reloading matters: it distinguishes a persisted change from a message inserted only into the current document. A separate test should confirm that a different customer cannot open or change the order.

## Selectors and waiting

Prefer selectors based on roles, labels, and stable application semantics. A selector tied to the fifth nested div can fail after an unrelated layout change. Accessible form labels make both the interface and the tests clearer.

Wait for a meaningful condition, such as the updated text or expected URL. Fixed sleeps make tests either slow or intermittent. Waiting for complete network idleness can also be inappropriate when the page has live reload, polling, or a long-lived connection.

## Viewports and keyboard use

A desktop test doesn't exercise a collapsed mobile menu. Test at least the layouts whose interaction changes, and check that essential controls remain reachable without horizontal scrolling.

Keyboard navigation is worth checking on important flows: focus should reach the form, errors should be discoverable, and a modal shouldn't trap focus after closing. A screenshot alone cannot establish those behaviors.

## Failures and artifacts

On failure, retain a screenshot, console errors, the current URL, and relevant application logs. Avoid storing passwords, tokens, or private uploaded documents in test artifacts.

A failed screenshot comparison can indicate a rendering change; a failed business assertion indicates a different kind of problem. Keep those tests separate enough that reviewers can tell what needs investigation.

## External services

Use controlled adapters for email, payments, and shipping during ordinary browser runs. Verify one deliberate end-to-end integration separately when needed. A browser test should not send real recovery messages or charge a card merely because it exercises the checkout page.

See [HTTP tests](../../guides/http-tests/) for the request boundary and [database tests](../../guides/database-tests/) for reliable persistent setup.
