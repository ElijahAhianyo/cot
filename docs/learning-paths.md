---
title: Choose a learning path
---

Different questions need different kinds of documentation. You can move between these paths whenever your goal changes.

## New to Cot

Start with [installation](../installation/), then [your first application](../tutorials/first-app/). The tutorial introduces one decision at a time. It assumes basic Rust syntax; the [Rust Book](https://doc.rust-lang.org/book/) covers ownership, traits, and async prerequisites.

## Coming from another framework

Use the [transition guides](../coming-from/overview/) to connect Django, Laravel, Rails, Spring Boot, Axum, or Actix Web concepts to Cot. Readers new to Rust can start with the shared language introduction; Rust framework users can go directly to their comparison.

## Building an application

The [guides](../guides/overview/) explain each subsystem. They use examples to explore behavior rather than ask you to reproduce an entire project. Read a page independently or follow its related links.

## Solving a specific problem

The [how-to collection](../how-to/overview/) starts from tasks such as testing a redirect or preparing a production build. It assumes an existing project and makes the relevant prerequisites explicit.

## Looking up an API

Use [Rustdoc](https://docs.rs/cot/latest/cot/) for types, traits, methods, and macros. The [reference index](../reference/overview/) also points to configuration, CLI commands, and Cargo features.

## Maintain a running service

Start with [production configuration](../guides/production/), [observability](../guides/observability/), and [recovery](../guides/recovery/). These pages connect the executable to the database, external services, rollout process, and operator decisions. Use the [release index](../releases/overview/) when planning an upgrade.

## Choose the depth you need

| Your question | Useful material |
| --- | --- |
| Can I see this working? | A tutorial and its companion application |
| Why does it behave this way? | An explanation guide with examples and edge cases |
| How do I finish this task? | A how-to with prerequisites and verification |
| What does this method accept? | The matching version's Rustdoc |

For example, a missing issue can lead to the routing explanation, a route-mismatch procedure, or the `Path` extractor reference. Pick the document that answers the question you have now. You don't need to read the surrounding sidebar from top to bottom.

## Experiment with proposed capabilities

Pages labeled Proposed describe target designs for features the framework does not currently supply. They can help evaluate architecture and future documentation, but an application using them today needs a concrete external integration or application implementation. Check the availability label before looking for a dispatch or seeding API.
