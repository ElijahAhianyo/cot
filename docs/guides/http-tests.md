---
title: HTTP and application tests
status: preview
---

The boundary of a test determines what it proves. Calling a handler directly checks its behavior with supplied inputs. Sending a request through a complete application also checks routing, extraction, middleware, and shared setup.

## Handler, router, and application

A router test can detect an overlapping path or unsupported method. It will not prove that a session layer is installed. A full application test can verify that those components work together.

## State across requests

Login flows require more than one isolated request. Tests should preserve the appropriate cookies and assert both the successful login and the access it grants or denies.

## Useful assertions

Status alone can miss a wrong redirect destination or an incorrect content type. Assertions should reflect the contract: headers, body, persisted state, and relevant side effects.

## Related reading

- [Testing](../../testing/).
- [Middleware](../../guides/middleware/).
- [Test a redirect](../../how-to/test-redirect/).
