---
title: Uploads and media storage
status: preview
---

Uploaded media is data supplied while the application is running. Product photos, support attachments, and invoice files need a different lifecycle from the CSS and JavaScript shipped with a release.

We'll use a private attachment on an order. The customer can upload a receipt and authorized staff can download it. The example storage contract on this page is an application design; Cot's existing upload example is not a complete managed-media service.

## Files and metadata

The file's bytes and its database metadata serve different purposes. A storage key locates the bytes. A database record associates those bytes with an order, an owner, and a lifecycle state.

An illustrative record might contain:

| Field | Example | Purpose |
| --- | --- | --- |
| Attachment ID | 73 | Public application identifier |
| Order ID | 42 | Resource to which access is checked |
| Storage key | `receipts/8f2d…` | Internal location chosen by the server |
| Original name | `payment.pdf` | Display information supplied by the client |
| State | `pending_scan` | Whether the application permits download |

The original filename is useful for display, but it should not decide the filesystem path. Two customers can upload files with the same name, and a supplied name can contain path separators or other unexpected characters.

## Receiving an upload

A multipart request can contain several fields and files. Enforce a total request limit as well as any individual file limit. Reading a large file into a single allocation can exhaust memory before later validation has a chance to reject it.

The repository's file-upload example shows the current request integration. A mature media service additionally needs a storage backend, metadata rules, authorization, retention, and operational monitoring. Those are separate from recognizing multipart syntax.

## Validating content

A filename extension and a client-supplied Content-Type are claims about a file. They aren't proof of its contents. Use appropriate inspection for accepted formats, and consider the resource cost of processing them: a small compressed file can expand into a very large representation.

For our receipt, the application can keep the upload unavailable while a scanner processes it. An unavailable scanner should not silently turn an unreviewed attachment into a public file. The user interface can show a pending state and explain when retrying is useful.

## Saving bytes and saving a row

A database transaction usually cannot roll back an object-store upload. If the bytes are stored but the metadata write fails, the application needs a way to remove the orphan. If the row is committed before the bytes exist, readers need to recognize that incomplete state.

A useful design makes the stages explicit: accept, store temporarily, validate, and mark ready. Each transition should be retryable without creating duplicate attachments. These are proposed application lifecycle states, not built-in Cot methods.

## Serving private files

A download endpoint should load the attachment's resource, check access, and then serve the bytes or issue a short-lived storage URL. A private file must not also be reachable through an unrestricted static-file path.

A signed download URL is a bearer capability until it expires. Keep its lifetime appropriate to the use case, and avoid treating the existence of a signature as a fresh account-permission check.

## Replacing and deleting files

When a customer replaces a receipt, define whether existing links continue to identify the old bytes or the new version. Immutable storage keys make that distinction easier to manage. Deleting metadata and deleting bytes also need reconciliation when one succeeds and the other fails.

The [private-upload how-to](../../how-to/private-uploads/) turns these boundaries into an implementation checklist. [Recovery](../../guides/recovery/) covers the relationship between database backups and stored files.
