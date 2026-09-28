---
title: Serve private uploads
status: preview
---

Use this task outline for documents that must only be downloaded by an authorized user, such as invoices or project attachments. It describes the intended workflow without inventing a storage API.

## Establish ownership

Store enough metadata to identify the file and the resource it belongs to. The requesting user must be checked against that resource before access is granted.

## Deliver the file

Choose a delivery mechanism that preserves the access boundary: application-controlled streaming or a short-lived storage URL. The storage integration needs to define expiry and revocation behavior.

## Verify denied access

Test access as the owner, another signed-in user, and an anonymous user. Guessing a filename must not bypass the permission check.

See [uploads and media storage](../../guides/media/) and [authorization](../../guides/authorization/). The final recipe depends on the selected Cot storage integration.
