---
title: Database and service tests
status: preview
---

Tests need predictable starting state. A database, cache, email transport, and external service each introduce state that can survive longer than one test unless the test boundary controls it.

## Isolation

A rollback can isolate some database work but does not roll back an email or a file upload. Tests that exercise several services need a cleanup or replacement strategy for each one.

## Fixtures and factories

Fixtures provide known records. Factories can produce variations. Both should make the tested condition clear rather than hide essential values behind unrelated defaults.

## Deterministic failures

A useful service replacement can simulate a timeout or rejected message, not only success. Time-dependent tests should avoid relying on real waiting when the application can expose a controllable time boundary.

## Related reading

- [Testing](../../testing/).
- [Seeding and test data](../../guides/seeding/).
