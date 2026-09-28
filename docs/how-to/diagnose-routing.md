---
title: Diagnose a route mismatch
status: preview
---

Use this checklist when a request reaches the wrong handler or returns an unexpected 404, 405, or 400.

## Check the full path

Include app and nested-router prefixes. Check case and trailing slashes. A query string does not change which path is matched.

## Check earlier routes

A broader route can capture a value intended for a literal route. For example, `/articles/{id}` placed before `/articles/new` can select the numeric extractor for the value `new`.

## Check the method and extraction

A matching path with an unregistered method produces 405. A value that cannot be extracted as the handler's `Path` type produces 400. Neither failure causes Cot to try the next route.

## Reproduce the result

Make a request through a router test. If the behavior depends on middleware, use a complete application test. See [routing](../../routing/) for the rules and [testing](../../testing/) for the test interfaces.
