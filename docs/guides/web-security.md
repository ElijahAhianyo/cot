---
title: Web security
status: preview
---

A web application's security depends on several boundaries working together. Typed Rust values help with program correctness, but they do not prove that a request is authorized, that HTML is safely rendered, or that a browser intended to perform a state-changing action.

We'll use an order-management page to connect the main concerns. The customer can read an order, add a delivery note, and upload a receipt. Each operation introduces a different kind of untrusted input.

## Authentication and resource access

First establish the actor through the configured authentication mechanism. Then check that actor's permission for the particular order. An order ID in a hidden field or URL is still chosen by the caller.

Apply the same rule to downloads, exports, and alternate API endpoints. An authorized HTML page cannot compensate for an unrestricted attachment URL. [Authorization](../../guides/authorization/) develops the resource rule in code.

## Browser requests and CSRF

A browser can attach cookies to a request even when another site caused that request. A cookie-authenticated mutation therefore needs a deliberate cross-site request forgery defense. Making it a POST or hiding its URL isn't sufficient.

Verify the actual protection installed in the application, including token or origin validation as appropriate. This preview does not claim that Cot supplies a complete built-in CSRF system. Cookie SameSite settings contribute to the policy but should not be confused with an independently verified mutation defense.

Read-only GET routes should remain read-only. A link preview, crawler, or prefetch can visit a URL without the user intending to change account state.

## Rendering untrusted content

A delivery note is text, even if it contains `<script>` or quotation marks. Use the template engine's escaping for the output context. `Html::new` does not sanitize a string assembled from user input.

HTML text, HTML attributes, JavaScript, CSS, and URLs are different contexts. Escaping for one does not establish safety in another. If rich text is allowed, define and enforce an appropriate content policy rather than disabling escaping for all stored text.

## Database and command inputs

Use parameterized database operations for external values. The [query guide](../../databases/queries/) includes parameterized raw-SQL interfaces when the query builder is not enough.

The same boundary applies outside SQL. Don't turn a filename, search phrase, or URL parameter into a shell command. Pass structured arguments through the relevant library and validate the operation being requested.

## Uploads and outbound requests

Uploaded filenames should not choose storage paths. Download authorization must remain effective even if someone learns the storage key. See [media](../../guides/media/) for the full lifecycle.

An outbound URL supplied by a customer can point to an internal service. Restrict destinations and redirects according to the integration's purpose; a valid URL is not necessarily an allowed destination. [HTTP integrations](../../guides/http-webhooks/) discusses that boundary.

## Secrets, transport, and diagnostics

Use HTTPS for public authenticated traffic and configure proxy trust deliberately. Keep signing keys and provider credentials outside published source and avoid logging cookies, bearer tokens, or full sensitive bodies.

Production error pages should reveal enough to help the user without exposing internal diagnostics. A request identifier can connect that response to restricted logs.

## Reviewing a change

For an order mutation, ask: who can call it, which resource can change, how the browser proves intent, which values are trusted, and what happens on a retry? These questions produce focused tests rather than a vague claim that the endpoint is secure.

Security behavior evolves with dependencies and deployment. Use the project's security policy for vulnerability reports and review the actual enabled components when deploying.
