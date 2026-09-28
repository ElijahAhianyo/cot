---
title: From Actix Web
status: preview
---

Actix Web readers can keep their understanding of async handlers, routing, and shared data. Pay particular attention to the word “app”: Actix Web's App builder and Cot's App trait describe different integration boundaries.

## Familiar concepts

These are starting points for comparison, not interchangeable APIs.

| In Actix Web | Explore in Cot |
| --- | --- |
| App and scopes | [Projects, apps, and route composition](../../guides/projects/) |
| Resources and route registration | [Cot routes](../../routing/) |
| Extractors | [Cot request extractors](../../guides/requests/) |
| web::Data | [Shared state and its ownership](../../guides/async-state/) |
| Responder | [Cot response conversion](../../guides/responses/) |

## An app is a different boundary

Actix Web's application builder connects routes, middleware, and application data. In Cot, an App implementation contributes to a Project. Preserve the responsibilities of a feature, but inspect Cot's registration lifecycle before mapping one structure to the other.

## Shared state needs a fresh check

Suppose a service caches a product catalog in memory. We need to know which handlers share that cache and when it is constructed. Moving a state wrapper without checking those lifetimes can create separate caches where the application expected one shared value.

Treat Actix Web's application factory and Cot's initialization as separate contracts. Neither a similar type name nor a successful compile proves that initialization happens at the same point.

## Reusing the useful parts

Domain calculations and data structures that don't depend on Actix Web can often remain separate from the HTTP adaptation. Extractors, response conversion, middleware, and framework test helpers require closer review.

Compare failure responses as well as success: an unsupported method, a malformed body, and missing authentication should retain the intended public behavior. [HTTP tests](../../guides/http-tests/) provide the right place to express those expectations.

## Continue reading

The [transition overview](../overview/) explains the scope and availability labels. For a complete first project, use the [tutorials](../../tutorials/overview/).

The comparison uses the official [Actix Web documentation](https://actix.rs/docs/application/). Consult the source framework documentation for its exact version-specific behavior.
