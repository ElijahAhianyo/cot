---
title: Coming from another framework
status: preview
---

You already know what a web application needs: routes, data, validation, and a way to run it in production. The unfamiliar part is where those responsibilities live in Cot, and how Rust changes the way we express them.

These explanation guides connect that experience to Cot. They aren't automated conversion instructions or a promise of feature parity. Each page highlights familiar concepts, differences that matter, and a useful place to continue reading.

## Choose your starting point

| Your background | Start here | Main change to explore |
| --- | --- | --- |
| Python and Django | [From Django](../django/) | Explicit Rust types, apps, and database operations |
| PHP and Laravel | [From Laravel](../laravel/) | Application composition and explicit dependencies |
| Ruby on Rails | [From Rails](../rails/) | Explicit registration and typed data |
| Java or Kotlin and Spring Boot | [From Spring Boot](../spring-boot/) | Traits and composition without component scanning |
| Rust and Axum | [From Axum](../axum/) | Framework-managed apps and integrated services |
| Rust and Actix Web | [From Actix Web](../actix-web/) | Different app, handler, and state contracts |

## If Rust is new to you

[Web development in Rust](../rust-for-web-developers/) introduces ownership, optional values, errors, and async work through web application examples. It links to the Rust Book for the language details. You don't need to relearn HTTP to learn Rust.

## Comparing an existing application

A matching feature name isn't enough to establish compatibility. For an order endpoint, we need to compare accepted inputs, authorization, transaction boundaries, error responses, and side effects. A request that returns the same JSON can still send two confirmation emails after a retry.

The framework pages use small cases like this to explain the boundaries. For an actual migration, capture the old application's observable behavior in tests before changing which service handles the request. Database schemas, password hashes, cookies, and queued work each need their own compatibility decision.

## Read the availability labels

This documentation preview includes proposed capabilities such as background tasks, rate limiting, and seeding. Their presence in the sidebar does not mean Cot provides them. Follow the current feature documentation and Rust API when deciding whether an application can move today.

## Plan around behavior, not file counts

For a candidate feature, write down its inputs, observable outputs, owned data, and external effects. Include malformed input, permission denial, missing records, retries, and simultaneous requests. Those cases make a migration boundary concrete.

A read-only catalog may be independently replaceable. Checkout often depends on account identity, inventory, payment state, and email delivery. Choose the boundary with those dependencies visible, then decide which service owns each write during the transition.

## Keep one source of authority

If both applications use the same database, assign schema changes to one migration process. If both can receive the same operation, define how they recognize duplicates. If users move between them, decide whether identity is shared through a supported protocol or established separately.

The comparison pages help locate Cot's equivalents and differences. They do not establish wire compatibility for cookies, password encodings, serialized jobs, or internal model representations.
