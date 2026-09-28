---
title: Error handling
status: preview
---

An error is part of an application’s contract with its caller. A missing article, invalid input, and an unavailable database need different handling even though none produces the requested page.

## Expected failures and bugs

An application can expect a lookup to find no record. A panic indicates a different kind of failure and should not be a routine way to reject input. Cot handlers can propagate application errors through `cot::Result`.

## Where errors originate

Routing can fail before extraction; extraction can fail before the handler; application logic can fail after both succeed. A 404 might describe an unmatched path or an absent record. Knowing the stage narrows the diagnosis.

## HTML and JSON errors

A browser page and an API client need different representations of a failure. Both should have an appropriate status. Public messages should explain what the caller can do without exposing internal state.

## Reporting and debugging

A production error response and an internal diagnostic serve different audiences. Logs and traces can retain context that must not appear in the response. Debug behavior should be an explicit deployment decision.

## Related reading

- [Error pages](../../error-pages/).
- [Logging and health](../../guides/observability/).
