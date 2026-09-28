---
title: From Django
status: preview
---

Django's project-and-app vocabulary gives us a useful starting point. Cot also has projects and apps, but their behavior is defined through Rust traits and explicit registration. A similar name doesn't make a Django app portable to Cot.

## Familiar concepts

These are starting points for comparison, not interchangeable APIs.

| In Django | Explore in Cot |
| --- | --- |
| URLconf and views | [Routes and handlers](../../routing/) |
| Projects and installed apps | [Project and App traits](../../guides/projects/) |
| Models and QuerySets | [Models and queries](../../databases/queries/) |
| Templates and context | [Templates and typed context](../../templates/) |
| Admin site | [Admin panel](../../admin-panel/) |

## A product page, from URL to database

Consider a product-detail view. The URL identifies the product, the handler loads it, and the template presents it. That division remains useful in Cot. What changes is the contract at each boundary: Rust types describe the inputs and results, and database operations use Cot's API.

A missing product is also different from a failed database connection. Preserve that distinction when deciding which HTTP response to return. The [query guide](../../databases/queries/) shows the actual operations; Django's QuerySet chaining and relationship behavior aren't a syntax recipe for those operations.

## Templates and relationships

Don't assume that a template expression will fetch related database rows. Decide which data the page needs and load it deliberately. That makes both missing data and query cost visible before rendering.

## Moving existing accounts

A user table alone isn't a working login migration. Password encoding, session storage, cookie settings, and account rules need separate compatibility checks. Similar authentication terminology does not establish compatibility between the two frameworks.

## Continue reading

If Rust is new to you, start with [web development in Rust](../rust-for-web-developers/). The [transition overview](../overview/) explains the scope and availability labels. For a complete first project, use the [tutorials](../../tutorials/overview/).

The comparison uses the official [Django documentation](https://docs.djangoproject.com/en/6.0/intro/overview/). Consult the source framework documentation for its exact version-specific behavior.
