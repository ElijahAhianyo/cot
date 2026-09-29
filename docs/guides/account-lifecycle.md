---
title: Account lifecycle
status: preview
---

An account lasts longer than a login session. Customers register, verify an address, change credentials, lose access, and sometimes leave the service. Each transition changes what the application should trust.

We'll follow a customer registering for our shop. Cot supplies authentication and user-model building blocks; the complete registration, verification, and recovery flows described here are application design, not a claim that every screen or token API is built in.

## Registration and account state

A submitted email address is not yet evidence that the person controls that mailbox. We can represent registration and verification as separate states rather than treating every newly created row as fully trusted.

An illustrative state model is:

| State | Example capability | Transition |
| --- | --- | --- |
| Pending verification | Request a new verification email | Valid verification is consumed |
| Active | Place orders under the account policy | Suspension or deletion is requested |
| Suspended | Use an approved recovery/support path | Authorized reinstatement |
| Closed | No new account activity | Retention policy determines stored records |

These names belong to the example application. Choose states around actual permissions rather than adding flags without defining what they change.

## Verification links

A verification token should identify one purpose and expire. A token issued to verify an address should not also reset a password. If the customer changes the pending address, an older link must not verify the new address accidentally.

The response after consuming a link should make its outcome clear: verified, already used, expired, or invalid according to the application's disclosure policy. Avoid placing secrets or full tokens in logs and analytics URLs.

## Password recovery

Recovery is a way to regain control, so it deserves the same care as login. The request endpoint should avoid unnecessarily revealing which addresses have accounts. The token must be time-limited and usable only for the intended operation.

After the password changes, decide what happens to existing sessions. Keeping all sessions may surprise a person recovering from compromise; invalidating all sessions may require the current browser to sign in again. State and test the chosen policy.

A successful email-provider call is not proof that the message reached the inbox. Provide a bounded resend path without allowing unlimited messages to arbitrary recipients.

## Changing an email address

Treat the current verified address and proposed replacement as different values. Updating the primary address before verifying the replacement can remove the user's working recovery route.

Sensitive changes may require recent authentication rather than merely an old valid session. Record the change in an audit trail using identifiers and relevant outcomes, without copying passwords or verification tokens.

## Suspension and deletion

Suspension changes access; it doesn't necessarily delete stored orders. Deletion may involve anonymizing personal information while retaining records required by the application's legitimate retention policy. Decide how attachments, exports, notifications, and backups participate.

A page that hides the account isn't enough if API credentials or existing sessions still work. Every entry point needs to enforce the account's current state.

## Failure cases worth testing

Test expired and repeated links, a changed address, two recovery requests in succession, and a suspended account with an existing session. Check that concurrent attempts cannot consume a one-time token twice.

For implemented authentication operations, use [authentication](../../guides/authentication/). For intended abuse controls and asynchronous delivery, see [rate limiting](../../guides/rate-limiting/) and [background tasks](../../guides/background-tasks/), both clearly marked as proposed capabilities.
