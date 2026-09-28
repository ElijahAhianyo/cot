---
title: Accept changes
status: preview
---

We'll let a signed-in user create an issue and change their own issues. This chapter preview introduces input handling and permissions together because both determine whether a change should be accepted.

## Display and submit a form

GET displays the form; POST processes its values. A rejected form should preserve useful input and show errors near the relevant fields. The [forms guide](../../forms/) contains the current form API examples.

## Associate an owner

The server takes the owner from the authenticated request. It should not trust a submitted owner ID to establish who created the issue. [Authentication](../../guides/authentication/) explains the identity boundary.

## Enforce access

A second user may be able to read the issue without being allowed to edit it. Check permission on the server before performing the update, including when a request is sent directly rather than through the page. [Authorization](../../guides/authorization/) explains this distinction.

## Checkpoint

A blank title produces a useful error. A valid submission creates one issue. A different user cannot change it by posting to the edit URL directly. The final tutorial implementation must demonstrate all three outcomes.
