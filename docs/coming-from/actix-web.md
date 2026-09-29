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

## Construction is observable behavior

If a catalog cache is created during application construction, verify which requests and server instances share it. A counter that appears global in one design may become local to a worker or process in another. The type of the state wrapper alone does not answer that question.

Write a test that makes requests through the same configured project and observes the intended shared state. For multi-process behavior, use an external store when the requirement is actually global. No in-memory wrapper creates cross-process coordination.

## Adapt handlers at the boundary

Keep domain functions independent of either framework where practical. A function that calculates a basket total can still accept product prices and quantities. The Cot handler extracts those inputs, obtains dependencies, invokes the function, and converts its result into a response.

Replace Actix-specific extractors and response traits with their Cot contracts deliberately. Read the rejection behavior for body decoding and path conversion. Similar handler syntax does not mean the same error is returned for malformed input.

## Registration and route order

A Cot app contributes routes to a project through explicit registration. Grouping code in a module does not expose it, and naming a type `App` does not establish its mount prefix.

For a route group under `/catalog`, test the final mounted paths, unsupported methods, and any trailing-slash redirects. Include a literal path such as `/catalog/new` alongside an ID path. Broad patterns and extraction failures can reveal differences that an ordinary `/catalog/42` test misses.

## Middleware and errors

Map the purpose of each existing middleware layer: request identity, session loading, authentication, response headers, or tracing. Then reconstruct the required wrapping order using Cot's builder.

Verify denied requests and framework-generated error responses as well as successful handler responses. A response header or log record required by operations should not disappear because the handler returned an error before its normal response path.

## Tests and operational behavior

Port HTTP boundary tests to Cot's request builders or full-project client. Preserve existing tests for framework-independent Rust logic. Add a real server test when behavior depends on sockets, streaming, proxy headers, or browser cookies.

Before switching traffic, verify startup configuration and termination under load. Reusing a Rust compiler and async runtime does not establish identical application initialization or shutdown behavior.

## Continue reading

The [transition overview](../overview/) explains the scope and availability labels. For a complete first project, use the [tutorials](../../tutorials/overview/).

The comparison uses the official [Actix Web documentation](https://actix.rs/docs/application/). Consult the source framework documentation for its exact version-specific behavior.
