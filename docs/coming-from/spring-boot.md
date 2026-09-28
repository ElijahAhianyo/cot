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

## Continue reading

If Rust is new to you, start with [web development in Rust](../rust-for-web-developers/). The [transition overview](../overview/) explains the scope and availability labels. For a complete first project, use the [tutorials](../../tutorials/overview/).

The comparison uses the official [Spring Boot documentation](https://docs.spring.io/spring-boot/reference/using/structuring-your-code.html). Consult the source framework documentation for its exact version-specific behavior.
