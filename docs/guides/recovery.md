---
title: Releases and recovery
status: preview
---

A release changes a running system, not only an executable. Code, schema, configuration, assets, and queued work can all move at different times. Recovery planning makes those transitions deliberate.

We'll use a shop adding a new delivery-status field. The goal is to deploy it without making old instances fail while new instances are starting, and to know what can be reversed if the release misbehaves.

## Identify the release

Record the application revision, dependency lockfile, configuration revision or relevant changes, and migration state. A version label that points to different source each time it is built is difficult to investigate.

Build once and promote the same artifact when possible. Rebuilding independently for staging and production can introduce differences that the staging test did not cover.

## Compatible schema changes

Suppose new code reads `delivery_status`, but old code knows only `shipped`. A staged change can add a compatible field, deploy code that handles the transition, backfill existing rows, and remove the old representation only after it is no longer needed.

The exact sequence depends on constraints and application behavior. The principle is to account for both versions during overlap rather than assuming every process changes simultaneously.

A migration that succeeds on an empty database may still lock a large table or reject existing data. Test representative data and observe the target database's behavior.

## Rolling back code

A previous binary can only be restored safely if the current schema and data remain compatible with it. Rolling back code doesn't undo emails, provider requests, or data written by the newer version.

Write down the decision boundary: when can we redeploy the previous artifact, when do we need a forward fix, and when does recovery require restoring data? Those are different operations with different consequences.

## Backups and restore points

A backup is useful only if it can be restored. Test the restore process in an isolated environment and verify meaningful records, relationships, and application behavior afterward.

Database and media backups need a consistent story. Restoring an attachment row without its bytes leaves a broken download; restoring bytes without their metadata creates an orphan. Decide the acceptable recovery point and how reconciliation works.

Keep restored environments isolated from real email, payment, and webhook destinations. Testing recovery should not replay customer side effects.

## Background work during releases

A queued payload may outlive the application version that created it. Preserve compatibility or version the payload and provide the necessary handlers during the transition. Stop and restart workers according to the chosen queue's contract rather than assuming web-instance deployment replaces them too.

If a failed release processed some jobs, retrying the whole queue can repeat side effects. Use task identities and recorded outcomes to decide what remains incomplete.

## A useful release check

Before directing all traffic to a release, verify a small set of meaningful paths: public reads, authenticated reads, a bounded write, assets, and an expected error. Compare error rate and latency with the earlier baseline.

If the check fails, stop expanding the release while the evidence is still clear. Avoid changing several unrelated settings at once; that makes it harder to tell which change restored service.

## Practicing recovery

A short rehearsal can reveal missing credentials, incompatible tools, or undocumented steps. Record who can perform the operation and where its evidence is stored. Recovery instructions should be executable by the people on call, not only the person who designed the deployment.

See [deployment](../../guides/deployment/), [database tests](../../guides/database-tests/), and [observability](../../guides/observability/) for the supporting checks.
