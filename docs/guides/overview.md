---
title: Explore the guides
---

The guides explain how Cot’s parts work and how they relate to one another. Examples illustrate behavior, alternatives, and edge cases; you can read each page independently.

For a guided project, use [tutorials](../../tutorials/overview/). For a particular task, use [how-to guides](../../how-to/overview/). The [Rust API](https://docs.rs/cot/latest/cot/) remains the detailed reference.

## Coming from another framework

The [transition guides](../../coming-from/overview/) explain familiar concepts and important differences for Django, Laravel, Rails, Spring Boot, Axum, and Actix Web readers. They include a shared introduction to Rust for web developers.

## Finding the right boundary

A request can fail during routing, extraction, validation, authorization, persistence, or response rendering. The topic groups below follow those responsibilities. Start with the one closest to your question, then follow the contextual links when the explanation crosses into another subsystem.

The examples begin with small cases and continue into limits such as concurrent requests, retries, and unavailable dependencies. Proposed labels identify designs for capabilities that are not currently built in; they are separate from a page's place in this directory.

## Fundamentals

- [Projects and apps](../../guides/projects/)
- [Application lifecycle](../../guides/lifecycle/)
- [Configuration](../../guides/configuration/)
- [Async execution and shared state](../../guides/async-state/)

## Requests and responses

- [Routing](../../routing/)
- [Requests and extractors](../../guides/requests/)
- [Handlers and responses](../../guides/responses/)
- [Middleware](../../guides/middleware/)
- [Cookies and sessions](../../guides/sessions/)
- [Error handling](../../guides/errors/)
- [Error pages](../../error-pages/)

## Pages and forms

- [Templates](../../templates/)
- [Forms](../../forms/)
- [Validation](../../guides/validation/)
- [Static files](../../static-files/)
- [Uploads and media storage](../../guides/media/)
- [Localization and time zones](../../guides/localization/)

## Databases

- [Overview](../../databases/overview/)
- [Model relationships](../../guides/relationships/)
- [Queries](../../databases/queries/)
- [Pagination](../../guides/pagination/)
- [Transactions](../../databases/transactions/)
- [Migrations](../../databases/migrations/)
- [Seeding and test data](../../guides/seeding/)
- [Query performance](../../guides/query-performance/)

## Authentication and security

- [Authentication](../../guides/authentication/)
- [Account lifecycle](../../guides/account-lifecycle/)
- [Authorization](../../guides/authorization/)
- [Web security](../../guides/web-security/)
- [Rate limiting](../../guides/rate-limiting/)

## Application services

- [Caching](../../caching/)
- [Sending Emails](../../sending-emails/)
- [Background tasks](../../guides/background-tasks/)
- [Scheduled tasks](../../guides/scheduling/)
- [Events and notifications](../../guides/events/)
- [Admin panel](../../admin-panel/)

## APIs and integrations

- [JSON APIs](../../guides/json-apis/)
- [OpenAPI](../../openapi/)
- [Outbound HTTP and webhooks](../../guides/http-webhooks/)
- [Real-time communication](../../guides/realtime/)

## Testing

- [Testing](../../testing/)
- [HTTP and application tests](../../guides/http-tests/)
- [Database and service tests](../../guides/database-tests/)
- [Browser tests](../../guides/browser-tests/)

## Deployment and operations

- [Deployment architecture](../../guides/deployment/)
- [Production configuration](../../guides/production/)
- [Logging, tracing, and health](../../guides/observability/)
- [Performance and scaling](../../guides/performance/)
- [Releases and recovery](../../guides/recovery/)

## Extending Cot

- [Reusable apps and integrations](../../guides/reusable-apps/)
- [Custom components](../../guides/custom-components/)
- [CLI and management commands](../../guides/management-commands/)
