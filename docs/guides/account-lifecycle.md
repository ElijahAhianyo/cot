---
title: Account lifecycle
status: preview
---

An account has a lifetime beyond the moment a password is checked. Registration, verification, recovery, and deletion affect both the user’s access and the data associated with that account.

## Registration and verification

Creating a user record and verifying control of an email address are different operations. A verification token needs a defined lifetime and must be bound to the intended account and address.

## Recovery

Password recovery changes the credentials that protect an account. A mature flow needs to account for token expiry, repeated requests, and the effect on existing sessions without revealing unnecessary account information.

## Disabling and deletion

A disabled account may retain data without allowing new sessions. Deletion can involve ownership transfers, retention rules, and media cleanup. This preview describes the intended guide; it does not claim Cot supplies every account workflow.

## Related reading

- [Authentication](../../guides/authentication/).
- [Authorization](../../guides/authorization/).
