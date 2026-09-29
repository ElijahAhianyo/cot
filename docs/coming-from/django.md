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

## Queries are explicit work

In our product page, the query result has two boundaries: the database operation can fail, and a successful lookup can find no product. Cot represents those separately. Match the absent value to the application's not-found policy and let a database failure remain an operational error.

Don't infer when Cot executes a query from experience with Django's QuerySet evaluation. Follow the actual async operation and its return type. Likewise, fetching an associated customer is an explicit operation; a foreign-key value isn't proof that the customer object is already loaded.

For a list of 50 orders, identify whether customer names require 50 additional queries. Start with a query budget for that page and inspect actual queries before adding caches. See [relationships](../../guides/relationships/) and [query performance](../../guides/query-performance/).

## Forms and model constraints

A title that passes form validation may still conflict with a database constraint. Two requests can both validate an available slug before either writes it. Keep the unique constraint in the schema and handle the write outcome as well as the form result.

Cot's form type describes accepted input. It doesn't automatically inherit every business rule from a similarly shaped model. Keep an editable form's fields separate from server-owned values such as the current user, account ID, and moderation state.

## Middleware and settings

Translate the purpose of each middleware layer before translating its position. Authentication needs session state, and a response layer can observe failures or redirects produced by inner layers. Cot's middleware builder has its own wrapping order; preserve the intended request and response behavior rather than copying a list unchanged.

A settings name in Django does not establish a Cot environment variable. Identify the selected Cot configuration loader and how production secrets reach it. Then test the deployed settings through the actual process entry point.

## Replacing one feature

A product catalog is often a clearer first boundary than login. Capture its URL patterns, trailing-slash behavior, pagination, unavailable-product policy, and rendered content in tests. Route only that boundary to Cot while keeping one owner for schema changes.

Treat administration, password migration, and session continuity as separate decisions. A successful catalog migration doesn't establish that existing Django sessions can be accepted by Cot.

## Continue reading

If Rust is new to you, start with [web development in Rust](../rust-for-web-developers/). The [transition overview](../overview/) explains the scope and availability labels. For a complete first project, use the [tutorials](../../tutorials/overview/).

The comparison uses the official [Django documentation](https://docs.djangoproject.com/en/6.0/intro/overview/). Consult the source framework documentation for its exact version-specific behavior.
