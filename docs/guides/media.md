---
title: Uploads and media storage
status: preview
---

Uploaded media belongs to users and can outlive an application deployment. Static assets belong to the application release. Treating both as files in the same directory hides important differences in access, persistence, and cleanup.

## The upload lifecycle

A typical upload passes through request limits, parsing, validation, storage, and a database record that refers to it. Client-provided filenames and content types are claims, not proof of what a file contains.

## Public and private files

A public image and a private invoice need different access rules. An unguessable filename is not a replacement for authorization. A mature storage integration should make private access and download expiry explicit.

## Storage and cleanup

Local disks and object storage have different deployment and failure behavior. Saving a file and committing a database transaction are separate operations; a failed request can leave an orphan unless the application accounts for it.

## Target coverage

This page previews the intended storage guide. Backend configuration, signed downloads, and cleanup APIs must be documented against implemented Cot integrations before these concepts become executable instructions.

## Related reading

- [Static assets](../../static-files/).
- [Authorization](../../guides/authorization/).
