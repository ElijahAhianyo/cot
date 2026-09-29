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

## Callbacks are part of the migration inventory

An order's visible behavior may be spread across a controller, model callbacks, concerns, and jobs. Write down the business operation before translating individual classes: calculate the price, reserve stock, save the order, and request a confirmation.

In Cot, an ordinary application function can make that sequence explicit. The HTTP handler supplies validated input and trusted identity; the function carries out the operation; the handler chooses the response. This also gives commands and tests a place to invoke the same rule.

A callback that sends a message deserves particular attention. Committing the database and delivering a message are separate events. Preserve the intended failure and retry behavior, not merely the order in which methods appear.

## Typed model fields

An optional value is different from a failed lookup. We might represent a delivery note with `Option<String>`, while a database lookup returns a result that can contain no row. Handling those cases explicitly helps avoid reporting a database outage as an ordinary missing order.

Changing a struct's field changes its Rust value. Persisting that change requires a Cot database operation. Read [queries](../../databases/queries/) for insertion and update semantics rather than importing assumptions from dirty tracking or callbacks.

## Routes and view data

Treat named routes as part of the application's interface. Preserve inbound URLs and redirects when moving a feature, and use Cot's URL generation for new internal links. Check literal routes before parameter routes, including values such as `new` that might otherwise reach an ID extractor.

Templates receive a typed context. Load the related values the page needs before rendering, and keep expensive work visible in the handler or application layer. Template compilation can catch missing fields; it cannot tell us whether the selected customer belongs to the viewer's account.

## Session and job continuity

Existing cookies, signed values, password hashes, and pending jobs have formats and secrets outside the model definitions. Decide which remain served by Rails during the transition and which require explicit conversion or expiration.

Do not let two workers consume the same business operation without a shared duplicate policy. A queue migration needs a cutoff and draining plan as well as a new worker implementation.

## A useful first boundary

A read-only catalog page lets us compare routing, queries, templates, and caching without changing account sessions. Capture missing-record behavior and cache variation as well as successful rendering. Expand to writes only after schema ownership and transaction behavior are clear.

## Continue reading

If Rust is new to you, start with [web development in Rust](../rust-for-web-developers/). The [transition overview](../overview/) explains the scope and availability labels. For a complete first project, use the [tutorials](../../tutorials/overview/).

The comparison uses the official [Rails documentation](https://guides.rubyonrails.org/getting_started.html). Consult the source framework documentation for its exact version-specific behavior.
