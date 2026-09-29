---
title: Learn Cot by building
---

Tutorials provide a learning sequence with a concrete result. Each series has prerequisites and checkpoints; use the guides when you want to explore a topic without following a project.

<div class="doc-card-grid">
<a class="doc-card" href="../../tutorials/first-app/"><strong>Your first application</strong><span>A local issue tracker: stored data, pages, validated forms, and tests.</span></a>
<a class="doc-card" href="../../tutorials/json-api/"><strong>Build a JSON API</strong><span>A price-quote endpoint with typed JSON, validation, and contract tests.</span></a>
<a class="doc-card" href="../../tutorials/reusable-app/"><strong>Build a reusable app</strong><span>A component that another Cot project can register and configure.</span></a>
</div>

## Choose a tutorial

Start with the issue tracker if you're new to Cot. Its five chapters share one application and build understanding through observable requests. The JSON and reusable-app tutorials are independent; they assume basic familiarity with handlers and project registration.

All three applications live in the [tutorial companion](https://github.com/ElijahAhianyo/cot/tree/docs-preview/docs/site/checks/issue-tracker). Each has tests and an explicit scope. The local issue form does not implement authentication or production security controls, and the quote API does not place orders.

## Use the checkpoints

Run the application before changing it. Try the successful request, then the failure case described in the chapter. After editing, run the relevant tests and compare the result. A concrete response is more useful than assuming that compilation establishes the entire behavior.

The pages retain Preview status while this documentation proposal is reviewed. For a topic you want to understand independently, follow its explanation guide rather than working through a tutorial you don't need.
