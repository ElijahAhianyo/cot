---
title: Diagnose a route mismatch
status: preview
---

Use this procedure when a request reaches the wrong handler or returns an unexpected 404, 405, or 400. You need the application's route declarations and the exact request path and method. If a proxy is involved, keep both the public URL and the path it forwards.

## Reproduce one request

Send the request without automatically following redirects. For a locally running application:

```bash
curl -i http://127.0.0.1:8000/articles/new
```

Record the status, any `Location` header, and the method. A redirected request may reach a different path, so diagnosing only the final response can hide the original mismatch. Redact cookies and authorization headers before sharing the request.

## Reconstruct the complete path

Read the app's mount prefix, any nested router prefixes, and the route pattern together. An app mounted at `/shop` with a `/products/{id}` route receives `/shop/products/42`, not `/products/42`.

Check case, separators, and trailing slashes. Match the path without the query string: changing `?page=2` does not select a different path pattern. If a reverse proxy removes `/shop`, compare the resulting upstream path with the registered path.

## Check route order

Look for an earlier pattern that can match the same request. Put `/articles/new` before `/articles/{id}` when `new` is a literal page name. Otherwise, the parameter route can select `new` and then fail to parse it as a number.

Cot does not recover from that extraction failure by trying another route. Moving the literal route is the fix; broadening the numeric handler's error handling changes the wrong boundary. Catch-all patterns need the same review because they can consume several segments.

## Distinguish the failure

| Observation | Check next |
| --- | --- |
| 404 before the handler runs | Complete path, mount prefix, and selected route |
| 405 on a recognized path | Methods registered on that route |
| 400 for a parameter value | The selected handler's extractor types |
| Redirect before the handler | Trailing-slash or authentication middleware |
| Handler runs but returns 404 | Resource lookup and application access policy |

A path match and a method match are separate decisions. Registering a GET handler does not make POST valid. Likewise, a numeric ID can parse correctly and still identify no stored row.

## Isolate the router

Reproduce the route selection with `TestRequestBuilder` and `Router::handle`. Use both the successful path and the failing path. This removes the proxy and project middleware from the experiment.

If the router behaves correctly, repeat through Cot's full application client. A difference between those results points to project setup or middleware. If both pass, inspect the request delivered by the running server and proxy rather than changing the route speculatively.

## Keep a regression test

After correcting the declaration, test the literal path, a valid parameter, an invalid parameter, and an unsupported method. Assert the intended handler's response, not only that the status is successful. Two handlers can both return 200 while showing different content.

The [routing guide](../../routing/) explains matching and extraction in detail. [HTTP tests](../../guides/http-tests/) helps choose the test boundary for the behavior you found.
