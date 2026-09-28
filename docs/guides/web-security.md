---
title: Web security
status: preview
---

Browser security depends on several boundaries: where requests originate, what credentials accompany them, and how untrusted values become page content. One protection does not replace the others.

## Cross-site requests

CSRF protection addresses unwanted actions performed with a browser’s credentials. CORS controls which browser origins may read responses. These solve different problems and should not be presented as interchangeable switches.

## Untrusted content

Escaping helps keep a value from becoming executable markup in its output context. Uploaded files, URLs, and HTML fragments require separate decisions about what the application will accept and serve.

## Hosts, proxies, and cookies

A deployment behind a proxy needs an explicit trust boundary for forwarded information. Cookie attributes depend on HTTPS and the intended cross-site behavior.

## Coverage and verification

A completed guide should identify which protections Cot provides, which require middleware, and which remain application responsibilities. This preview does not assert that every protection listed here is enabled by default.

## Related reading

- [Templates](../../templates/).
- [Sessions](../../guides/sessions/).
- [Production configuration](../../guides/production/).
