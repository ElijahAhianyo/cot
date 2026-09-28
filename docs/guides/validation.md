---
title: Validation
status: preview
---

Parsing asks whether input can become a Rust value. Validation asks whether that value is acceptable for an operation. An integer can be a valid number and still be an invalid month or a price the application does not allow.

## Field and cross-field rules

A field rule can reject a blank title. A cross-field rule can reject an end date before a start date. Keeping the distinction visible makes error messages more useful and avoids embedding every rule in a database query.

## Application and database rules

An application can check that a username appears unused, but another request may insert it before the first request completes. A database constraint remains necessary when correctness depends on uniqueness.

## Representing failures

Forms need errors associated with fields and values that can be redisplayed. JSON clients need a stable representation that they can interpret. A validation failure should preserve useful context without echoing secrets.

## Related reading

- [Forms](../../forms/).
- [Transactions](../../databases/transactions/).
