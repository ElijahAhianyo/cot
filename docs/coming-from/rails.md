---
title: From Rails
status: preview
---

Rails experience transfers well at the level of application responsibilities: a route reaches application logic, models represent stored data, and views present a result. In Cot, explicit Rust definitions take the place of many naming conventions you may rely on.

## Familiar concepts

These are starting points for comparison, not interchangeable APIs.

| In Rails | Explore in Cot |
| --- | --- |
| Routes and controller actions | [Routes and handlers](../../routing/) |
| Active Record | [Models and queries](../../databases/queries/) |
| Views | [Templates](../../templates/) |
| Schema migrations | [Database migrations](../../databases/migrations/) |
| Application configuration | [Project configuration](../../guides/configuration/) |

## Names and registration

A file named after a controller isn't enough to expose an endpoint in Cot. Routes and apps must be connected through the framework's registration APIs. Rust modules organize code; they don't, by themselves, register routes or database models.

## An order is more than its fields

Suppose a Rails order model uses a callback to notify another system after a change. Copying its columns to a Cot model would lose that behavior. Identify callbacks, validations, and transaction boundaries as application rules before deciding where they belong in the new code.

The useful comparison is “what must happen when an order changes?”, not whether every model method has the same name. Use [queries](../../databases/queries/) and [transactions](../../databases/transactions/) to understand Cot's persistence operations.

## Missing and optional values

An optional delivery note can be represented with `Option<T>` in Rust. A database operation that fails has a different result. Making those cases explicit may change how a handler is structured, but it doesn't change the HTTP contract your clients already depend on.

## Schema history is a separate concern

Don't run two migration systems against a shared schema without defining who owns each change. Matching table names doesn't establish equivalent constraints, defaults, or migration history.

## Continue reading

If Rust is new to you, start with [web development in Rust](../rust-for-web-developers/). The [transition overview](../overview/) explains the scope and availability labels. For a complete first project, use the [tutorials](../../tutorials/overview/).

The comparison uses the official [Rails documentation](https://guides.rubyonrails.org/getting_started.html). Consult the source framework documentation for its exact version-specific behavior.
