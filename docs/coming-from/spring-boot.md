---
title: From Spring Boot
status: preview
---

Spring Boot readers already think about application composition, typed inputs, and service boundaries. The main adjustment is how those components are connected: Cot uses Rust traits and explicit construction rather than Spring's component discovery model.

## Familiar concepts

These are starting points for comparison, not interchangeable APIs.

| In Spring Boot | Explore in Cot |
| --- | --- |
| Application bootstrap | [Project lifecycle](../../guides/lifecycle/) |
| HTTP controllers | [Routes and handlers](../../routing/) |
| Configured dependencies | [Shared state and component ownership](../../guides/async-state/) |
| Persistence layer | [Models and queries](../../databases/queries/) |
| External configuration | [Configuration reference](../../reference/configuration/) |

## Traits aren't container registrations

A Rust trait describes behavior that a type implements. Implementing an application-specific trait doesn't automatically register that type as an injectable service. Construction and ownership remain part of the application design.

Cot's Project and App traits provide framework integration points. Read those contracts before deciding where startup work, route registration, and shared resources belong.

## Typed input still needs validation

For an order endpoint, successfully decoding a quantity as an integer tells us about its representation. It doesn't tell us whether stock is available or whether the caller may place the order. Keep those rules separate from request decoding.

## Persistence and transactions

Don't assume that a loaded model participates in an ORM persistence context, or that changing a field will eventually flush it to the database. Use Cot's documented [query operations](../../databases/queries/) and [transaction boundaries](../../databases/transactions/) to make writes explicit.

## Async and blocking dependencies

A Rust async handler doesn't make a blocking client asynchronous. Inventory database drivers, SDKs, and CPU-heavy work when moving an endpoint. The [async guide](../../guides/async-state/) explains why ownership and execution both matter for shared services.

## Construction and ownership

Suppose an order service needs a pricing client and a database handle. In Rust, construction determines who owns those resources and what can be shared. A shared pointer permits shared ownership; it doesn't make an unsafe operation thread-safe or supply a missing lifecycle policy.

Keep request-specific identity outside globally shared mutable state. Otherwise, one customer's request can influence another's operation. Pass the actor or scoped application context to the function that needs it.

This is a useful place to separate HTTP adaptation from business rules. A pricing function operating on ordinary Rust types can be tested without starting a server, while integration tests exercise the configured client and database.

## Transactions are visible boundaries

For an order and its lines, decide which writes must commit together. Use Cot's transaction interfaces around that operation and handle the returned result. Do not assume an annotation or a call through a framework proxy supplies the boundary.

An external payment or mail request cannot be rolled back with the database. Represent its state explicitly and plan retries around a stable operation identity. [Transactions](../../databases/transactions/) and [HTTP clients and webhooks](../../guides/http-webhooks/) discuss those separate failure domains.

## Error results and public responses

Rust's `Result` makes a fallible operation visible to callers. The `?` operator propagates an error; it does not choose your complete public API error contract.

An invalid quantity, an unavailable pricing service, and an unauthorized customer should remain distinguishable. Define the status and response body at the HTTP boundary, while keeping internal diagnostics available to operators. Avoid exposing a dependency's raw error message as a client contract.

## Production integration

Inventory health endpoints, metrics, tracing, configuration sources, shutdown behavior, and scheduled operations supplied by the existing application and its dependencies. Cot does not gain those behaviors merely because the main endpoint compiles.

For example, readiness might depend on reaching a database while liveness only establishes that the process can respond. Preserve the load balancer's intended decision when replacing those endpoints.

## Testing the replacement

Start with a service operation whose inputs and outputs are well understood. Keep unit tests for domain rules, then add Cot HTTP tests for extraction and response mapping. Finally, run integration tests against the chosen database and external clients. Each layer proves a different part of the replacement.

## Continue reading

If Rust is new to you, start with [web development in Rust](../rust-for-web-developers/). The [transition overview](../overview/) explains the scope and availability labels. For a complete first project, use the [tutorials](../../tutorials/overview/).

The comparison uses the official [Spring Boot documentation](https://docs.spring.io/spring-boot/reference/using/structuring-your-code.html). Consult the source framework documentation for its exact version-specific behavior.
