---
title: Browser tests
status: preview
---

Browser tests verify behavior that a direct HTTP test cannot see: script execution, rendering, focus, and interactions across pages. They complement lower-level tests rather than replace them.

## Choosing workflows

Sign-in, a form submission, and a private download are useful end-to-end boundaries. Repeating every validation permutation in a browser creates a slow suite without necessarily increasing confidence.

## Reliable interactions

Tests should wait for an observable state rather than an arbitrary delay. Stable accessible names make interactions easier to read and less dependent on layout.

## Coverage beyond success

A failed submission should preserve useful input and expose errors. Keyboard access, mobile layouts, and empty states deserve representative checks as well as the happy path.

## Related reading

- [Forms](../../forms/).
- [Testing](../../testing/).
