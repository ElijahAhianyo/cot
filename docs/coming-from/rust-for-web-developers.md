---
title: Web development in Rust
status: preview
---

The HTTP request is still a request, and the database still needs a transaction. Rust changes how we represent the data and which mistakes we can catch before the server starts.

For a concrete example, imagine an order form with a product ID, a quantity, and an optional delivery note. We can use those three fields to understand several important differences without rebuilding the whole shop.

## Missing data and invalid data

An absent delivery note and an invalid quantity mean different things. Rust's `Option<T>` represents whether a value is present. `Result<T, E>` represents whether an operation succeeded. Neither replaces validation: a quantity can parse as an integer and still be too large for the available stock.

This distinction helps us keep input errors separate from database failures. Read [forms](../../forms/) for Cot's form handling and [error handling](../../guides/errors/) for the response boundary.

## Ownership and request lifetimes

Rust tracks which value owns data and when a reference to that data is valid. A borrowed delivery note cannot outlive the value that stores it. When work must keep data after a request ends, that work needs data it owns rather than a reference into the request.

Cloning can be appropriate, but cloning a whole request is rarely the right design question. We usually need a small owned value, such as an order ID, rather than all the request's headers and body.

## Async does not mean background work

An async database call can yield while it waits for I/O. The handler still needs to await its result before it can use the saved order. Blocking work can occupy an executor thread; adding `async` to a function doesn't make that work nonblocking.

A durable background job is a separate concern. Spawning a task doesn't give it persistence, retries, or recovery after a process crash. Cot's [background task page](../../guides/background-tasks/) describes a proposed capability, not an available queue API.

## The compiler and the application

Types can prevent us from accidentally passing a customer ID where a distinct order-ID type is required. They don't prove that the current user owns the order. Authorization, race conditions, and business rules still need deliberate design and tests.

For language foundations, read the Rust Book chapters on [ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html), [error handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html), and [async programming](https://doc.rust-lang.org/book/ch17-00-async-await.html). Then use [Cot's learning paths](../../learning-paths/) to choose a project or a particular subsystem.
