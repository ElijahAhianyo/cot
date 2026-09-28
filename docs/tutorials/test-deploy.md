---
title: Test and prepare a release
status: preview
---

We'll verify the tracker’s important behavior before considering it ready to run outside development. This final chapter preview connects application tests with the deployment choices they cannot prove on their own.

## Exercise the HTTP boundary

Test the issue list, detail page, invalid input, and denied edit. Tests should assert the response and the stored result. The [testing guide](../../testing/) describes Cot's request builders and application test client.

## Prepare a release build

A release build and production configuration solve different problems. Building optimized code does not configure a database, provision secrets, or decide where uploads live. [Prepare a production build](../../how-to/production-build/) separates those checks.

## Verify the deployment

A useful final checkpoint covers the application process, persistent data, and public error behavior. The [deployment guide](../../guides/deployment/) explains how these fit together.

## Continue independently

You can now choose a task from [how-to guides](../../how-to/overview/) or study a subsystem in [guides](../../guides/overview/). The next feature does not need to follow a prescribed chapter order.
