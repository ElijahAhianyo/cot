---
title: Cot documentation
---

Build web applications in Rust with tools that work together: routing, data, forms, authentication, and an admin interface.

Choose a learning path, understand a feature, or find the exact API you need.

<div class="doc-card-grid">
<a class="doc-card" href="tutorials/overview/"><strong>Learn Cot</strong><span>Build an issue tracker. Follow a sequence with working checkpoints.</span></a>
<a class="doc-card" href="guides/overview/"><strong>Understand Cot</strong><span>Explore how the framework works, with examples and edge cases.</span></a>
<a class="doc-card" href="how-to/overview/"><strong>Solve a problem</strong><span>Find focused instructions for a task in an existing application.</span></a>
<a class="doc-card" href="reference/overview/"><strong>Look up a detail</strong><span>Open the Rust API, configuration, CLI, and compatibility references.</span></a>
</div>

## Start with what you need

| If you're here to… | Start with… |
| --- | --- |
| Explore what others have built | [Community showcase](community/showcase/) |
| Bring experience from another framework | [Framework transition guides](coming-from/overview/) |
| Run your first project | [Installation](installation/) |
| Understand the request lifecycle | [Application lifecycle](guides/lifecycle/) |
| Work with data | [Models](databases/overview/) and [queries](databases/queries/) |
| Read or change a request | [Requests and extractors](guides/requests/) |
| Prepare for production | [Deployment architecture](guides/deployment/) |

## From the blog

[Why framework transition guides belong in the docs](blog/framework-transition-guides/) explores the distinction between a familiar starting point and a second copy of the feature documentation. Visit the [blog](blog/overview/) for design discussions and their context.

## About this preview

This is an interactive proposal for Cot's documentation as the framework grows. Existing guides sit alongside expanded explanation pages and tutorials with runnable companion applications. **Preview** marks draft documentation. **Proposed** marks a feature that Cot does not currently provide. Unmarked existing pages retain their current content.

The [Rust API](https://docs.rs/cot/latest/cot/) remains the authoritative API reference for published releases. The development checkout can differ from that release.

## Follow a request through the framework

For a useful overview, read [projects and apps](guides/projects/), then [routing](routing/), [requests](guides/requests/), and [responses](guides/responses/). These pages connect application composition to what happens when a browser or API client sends a request.

When the request changes data, continue with [queries](databases/queries/), [transactions](databases/transactions/), and [authorization](guides/authorization/). When preparing to run it for other people, use [testing](guides/http-tests/) and [deployment](guides/deployment/). You can enter at any of these points without completing a course first.
