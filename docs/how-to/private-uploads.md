---
title: Serve private uploads
status: preview
---

Use this procedure for files that belong to a restricted resource, such as an invoice or a project attachment. The example is an invoice PDF that only members of its customer account may download.

You need authenticated requests, an application authorization rule, and a storage integration that can keep objects private. Cot's upload handling supplies request-side primitives; the storage and delivery steps below belong to the integration you choose.

## Store the file privately

Keep invoice objects outside the public static-file directory. Configure the storage service so an object cannot be retrieved anonymously by guessing its key.

Generate the storage key on the server. Keep the original filename as display metadata, not as a filesystem path. Record the object's key, customer account, content type, byte count, and readiness state in the database. A record pointing to an incomplete upload should not be downloadable.

## Create a download endpoint

Give the endpoint a resource identifier, such as `/invoices/42/download`, rather than accepting an arbitrary storage path. Load invoice 42 in the current account's scope, then check whether the authenticated actor may read it.

Use the same policy as the invoice detail page. A signed-in user from another account must be denied even if they know the ID. Choose a consistent forbidden or not-found response according to whether your application reveals the invoice's existence.

Perform these checks before opening the object or generating a download URL. Checking only the page containing the download button leaves direct requests unprotected.

## Choose a delivery method

| Method | Implementation requirement | Access behavior |
| --- | --- | --- |
| Stream through the application | A storage reader and bounded response streaming | Each download begins with an application permission check |
| Redirect to a signed storage URL | A private-object signing API and a short expiry | Anyone possessing that URL may be able to use it until expiry |

For streaming, preserve bounded memory use; don't read an arbitrarily large object into one byte buffer. For signed URLs, set the expiry deliberately and avoid logging the full URL. Revoking account access may not invalidate a URL already issued by the storage provider.

Set an appropriate `Content-Type` and download disposition using the response interfaces. Sanitize the display filename before placing it in a header. Do not let submitted text create additional response headers.

## Prevent public caching

Review both application headers and proxy or CDN rules. A response authorized for one customer must not be stored as a public response for everyone requesting the same path. Signed URL redirects can also disclose access if cached incorrectly.

Test with the actual delivery topology, because a correct application response can still be mishandled by a shared cache rule.

## Verify access and failure cases

Request the file as an allowed account member, a different customer, and an anonymous visitor. Repeat by entering the download URL directly. Check that a missing object produces a controlled response and an actionable operator diagnostic without disclosing the storage key.

For signed URLs, also verify expiry and document the provider's revocation behavior. For streaming, interrupt a large download and confirm that resource use returns to normal. These checks establish the behavior of your selected storage integration; they are not guarantees supplied by a filename convention.

See [uploads and media storage](../../guides/media/) for storage design and [authorization](../../guides/authorization/) for resource policies.
