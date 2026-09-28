---
title: Releases and recovery
status: preview
---

A release changes a running system, not only a binary on disk. Application code, database schema, configuration, and stored files may each change at a different time.

## Compatible transitions

A mixed-version rollout needs old and new instances to tolerate the transitional schema. An additive change followed by a later cleanup can be safer than a single incompatible replacement.

## Rollback boundaries

Returning to an old binary does not automatically reverse a data migration or an external side effect. A release plan should identify which changes can actually be undone.

## Restore verification

A backup is useful only if it can be restored into a working application. Recovery checks should include relationships, media references, and the time needed to restore service.

## Related reading

- [Migrations](../../databases/migrations/).
- [Deployment architecture](../../guides/deployment/).
