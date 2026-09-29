---
title: Solve a specific problem
---

How-to guides address a particular task in an existing application. They keep the prerequisites, required changes, and verification together. Conceptual background belongs in the linked guides.

<div class="doc-card-grid">
<a class="doc-card" href="../../how-to/test-redirect/"><strong>Test a redirect</strong><span>Check both the response status and its destination.</span></a>
<a class="doc-card" href="../../how-to/diagnose-routing/"><strong>Diagnose a route mismatch</strong><span>Distinguish path, method, and extraction failures.</span></a>
<a class="doc-card" href="../../how-to/production-build/"><strong>Prepare a production build</strong><span>Separate compilation from runtime configuration.</span></a>
<a class="doc-card" href="../../how-to/private-uploads/"><strong>Serve private uploads</strong><span>Consider access checks, delivery, and denied requests.</span></a>
<a class="doc-card" href="../../how-to/queued-email/"><strong>Send email after a commit</strong><span>A proposed workflow connecting transactions and durable tasks.</span></a>
</div>

## Find the explanation

The [guide directory](../../guides/overview/) covers the concepts behind these tasks. The procedures include prerequisites and verification. Recipes labeled Proposed depend on capabilities Cot does not yet provide.

## Before applying a recipe

Identify the application version and the boundary you're changing. A router-only test doesn't include middleware, and a successful build doesn't establish that production configuration is correct. Each procedure states the setup needed for its outcome.

For a storage or queue integration, follow the provider's actual interface where the procedure calls for an integration-specific operation. The recipe explains the required behavior without inventing a Cot API for a capability that isn't built in.

## Verify the result

Complete the successful check and the relevant failure check. For a private file, denied access is part of success. For a redirect, both the status and destination matter. Keep a regression test when the task fixes a behavior that could break again.
