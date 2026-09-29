---
title: Projects and apps
status: preview
---

A Cot project brings together the configuration and apps that make up a running service. An app contributes a particular part of that service, such as a product catalog or the admin interface. We can keep both in one Rust crate to begin with and separate reusable code later.

For the examples on this page, our shop has a catalog and an orders area. Customers browse products in the catalog and place orders through a separate set of handlers. These are useful boundaries because their routes and responsibilities are different, not because every database table needs its own app.

## Projects and apps

The [`Project`](trait@cot::Project) trait describes the service as a whole. Its integration points include configuration, app registration, middleware, and error handling. The [`App`](trait@cot::App) trait describes what a component contributes to that project.

Here is a small catalog app. The handler returns plain text so we can focus on registration:

```rust
use cot::App;
use cot::router::{Route, Router};

async fn catalog_home() -> &'static str {
    "Browse our catalog"
}

struct CatalogApp;

impl App for CatalogApp {
    fn name(&self) -> &str {
        "catalog"
    }

    fn router(&self) -> Router {
        Router::with_urls([
            Route::with_handler_and_name("/", catalog_home, "home"),
        ])
    }
}
```

The app defines a route relative to where the project mounts it. Mounting the catalog at `/catalog` makes that root handler available at `/catalog/`. The app doesn't need to know its public prefix. See [routing](../../routing/) for prefixes, names, and reverse lookups.

## Registration and Rust modules

A module and an app solve different problems. A Rust module groups code and controls visibility. Registering an app connects its behavior to Cot. Creating a `catalog.rs` file alone doesn't add its routes to the running server.

We can organize a small project like this:

```text
src/
  main.rs          Project configuration and app registration
  catalog.rs       Catalog app and its handlers
  orders.rs        Order app and its handlers
  pricing.rs       Calculations shared by both apps
```

This is an example layout, not a required naming convention. As `catalog.rs` grows, it can become a `catalog/` module with separate model and handler files. Its public app type can stay the same.

## Choosing a boundary

A pricing calculation usually doesn't need an App implementation. An ordinary function that accepts a price and a quantity is easier to use from a handler, a test, or a command. Use an app when a component needs to contribute framework behavior; use Rust modules for the rest.

A separate crate becomes useful when the component needs an independent dependency set or will be shared between projects. It adds a public API to maintain, so splitting every feature into a crate at the start can make changes harder rather than clearer.

## Shared resources and startup

A database connection pool belongs to the running application, not to an individual product handler. Request extractors expose resources from the project context. The handler can ask for a database without opening a new pool for every request.

Data specific to one customer belongs to the request or an appropriate persistent store. Putting the current customer in mutable global state would let concurrent requests overwrite one another. [Async execution and shared state](../../guides/async-state/) explains this distinction in more detail.

## Reusing an app

A reusable catalog app should not assume that it is mounted at the site root, that another app uses a particular route name, or that a specific configuration file exists on disk. State these dependencies at its public boundary. That lets a second project mount the same app without copying its internals.

For a runnable starting point, use [your first application](../../tutorials/first-app/). For the order of initialization, continue with the [application lifecycle](../../guides/lifecycle/).
