---
title: Routing
---

Cot provides a [`Router`](struct@cot::router::Router) that connects URL paths to request handlers, also called views. It determines which handler receives a request and can generate URLs from route names. In this guide, we'll cover how routes are matched, how handlers receive path parameters, and how routes fit together across an application.

The examples use articles and archives from a publishing app. For project setup and a first view, see the [introduction](../introduction/). For the complete routing API, see [`cot::router`](cot::router).

## Defining routes

A [`Route`](struct@cot::router::Route) pairs a path with a handler. An app defines its routes through [`App::router`](trait@cot::project::App#method.router), which returns a router containing those pairs. For example, we can associate `/articles/` with a handler that returns a list of article titles:

```rust
use cot::App;
use cot::router::{Route, Router};
use cot::router::method::get;

async fn list_articles() -> &'static str {
    "42: Growing herbs on a windowsill\n43: When to water your seedlings"
}

struct JournalApp;

impl App for JournalApp {
    fn name(&self) -> &'static str {
        "journal"
    }

    fn router(&self) -> Router {
        Router::with_urls([
            Route::with_handler_and_name("/articles/", get(list_articles), "articles"),
        ])
    }
}
```

A GET request to `/articles/` calls `list_articles` and returns its text. The `get` wrapper restricts which HTTP methods the route accepts; [HTTP methods](#http-methods) covers this in more detail.

The last argument, `"articles"`, is the route's name. Names let us refer to a route when generating a link without repeating its path. `Route::with_handler` defines the same kind of route without a name.

Routing uses only the URL's path. `/articles/?sort=newest` matches the same route as `/articles/`, because the query string doesn't affect the match. Literal text is case-sensitive, and the whole path must match: `/Articles/` and `/articles/recent` don't match `/articles/`. If no route leads to a matching handler, Cot reports 404 Not Found.

## Path parameters

A literal path works when the URL is always the same. For individual articles, though, part of the path changes with the article we're requesting. Cot supports this through path parameters: `{id}` in `/articles/{id}` captures a non-empty value up to the next `/`.

The [`Path`](struct@cot::request::extractors::Path) extractor converts captured values into the types a handler expects. An extractor reads request data before the handler runs. In the example below, `Path<u64>` parses the article ID as an unsigned integer:

```rust
use cot::error::NotFound;
use cot::request::extractors::Path;
use cot::router::{Route, Router};
use cot::router::method::get;

async fn article(Path(id): Path<u64>) -> cot::Result<&'static str> {
    match id {
        42 => Ok("Growing herbs on a windowsill: choose a spot with plenty of light."),
        43 => Ok("When to water your seedlings: check the soil before watering."),
        _ => Err(NotFound::new().into()),
    }
}

let router = Router::with_urls([
    Route::with_handler_and_name("/articles/{id}", get(article), "article"),
]);
```

For `/articles/42`, the handler receives `42` and returns the windowsill article. The example keeps two articles in the handler; an application with a database would use the same ID to [retrieve an article](../databases/queries/).

Path matching, extraction, and the handler's own logic are separate stages. That distinction matters when a request fails:

| Request | Result |
| --- | --- |
| `/articles/42` | The path matches, `42` parses as a `u64`, and the handler returns the article. |
| `/articles/99` | The path and type are valid, but the handler returns 404 Not Found because the article doesn't exist. |
| `/articles/herbs` | The path matches, but `"herbs"` can't be parsed as a `u64`. Extraction returns 400 Bad Request before the handler runs. |
| `/articles/42/comments` | The full path doesn't match this route. If no other route matches, Cot reports 404 Not Found. |

The type in `Path<T>` controls conversion, not route selection. A `Path<String>` would accept `"herbs"` as a string, but changing `T` doesn't change which paths match `/articles/{id}`.

### Overlapping routes

Sometimes more than one route can match a path. `/articles/new`, for example, matches both a literal `/articles/new` route and `/articles/{id}`. Cot checks routes in their registered order and uses the first matching handler; it doesn't rank literal paths ahead of parameters.

In the following example, the literal route comes first:

```rust
use cot::request::extractors::Path;
use cot::router::{Route, Router};
use cot::router::method::get;
# use cot::error::NotFound;
# async fn article(Path(id): Path<u64>) -> cot::Result<&'static str> {
#     match id {
#         42 => Ok("Growing herbs on a windowsill: choose a spot with plenty of light."),
#         43 => Ok("When to water your seedlings: check the soil before watering."),
#         _ => Err(NotFound::new().into()),
#     }
# }

async fn new_article() -> &'static str {
    "Start a new article: give it a title, then tell us what you've been growing."
}

let router = Router::with_urls([
    Route::with_handler("/articles/new", get(new_article)),
    Route::with_handler_and_name("/articles/{id}", get(article), "article"),
]);
```

Here, `/articles/new` reaches `new_article`, while `/articles/42` reaches `article`. Reversing the order sends `/articles/new` to the parameterized route, where parsing `"new"` as a `u64` fails. Cot doesn't try the next route after an extraction error.

This is why specific routes generally belong before broader ones, with catch-all routes last. With `Path<String>` on the parameterized route instead, putting it first would pass `"new"` to the article handler rather than report a parsing error.

### Multiple parameters

A route can capture more than one value. For an archive organized by year and month, we can extract both values as a tuple:

```rust
use cot::request::extractors::Path;
use cot::router::{Route, Router};
use cot::router::method::get;

async fn archive(Path((year, month)): Path<(u16, u8)>) -> String {
    format!("Articles published in {year}-{month:02}")
}

let router = Router::with_urls([
    Route::with_handler("/archive/{year}/{month}", get(archive)),
]);
```

Tuple values follow their order in the path. For `/archive/2026/09`, `year` is `2026` and `month` is `9`, producing `Articles published in 2026-09`.

Type conversion doesn't validate every application rule. A month of `13` fits in a `u8`, so it passes extraction even though it isn't a calendar month. A value of `256` doesn't fit and fails extraction. The handler is responsible for checking the calendar range.

### Parameters as structs

With several parameters, named fields can be easier to follow than tuple positions. `Path` also supports structs that implement Serde's `Deserialize`, matching field names to parameter names. This example requires Serde with its `derive` feature:

```rust
use cot::request::extractors::Path;
use serde::Deserialize;

#[derive(Deserialize)]
struct ArchiveParams {
    year: u16,
    month: u8,
}

async fn archive(Path(params): Path<ArchiveParams>) -> String {
    format!("Articles published in {}-{:02}", params.year, params.month)
}
```

This handler accepts the same `/archive/{year}/{month}` route as the tuple version. The difference is how we refer to the values: `params.year` and `params.month` identify them by name.

When working with a `Request` directly, [`RequestExt::path_params`](trait@cot::request::RequestExt#tymethod.path_params) provides access to the captured values without a `Path` argument.

### Wildcard parameters

A regular parameter stops at the next `/`. Sometimes we need to capture a path with several segments, such as `herbs/indoor/basil`. A wildcard parameter, written as `{*path}`, captures the remaining path as one value:

```rust
use cot::request::extractors::Path;
use cot::router::{Route, Router};
use cot::router::method::get;

async fn guide_path(Path(path): Path<String>) -> String {
    format!("Requested gardening guide: {path}")
}

let router = Router::with_urls([
    Route::with_handler("/guides/{*path}", get(guide_path)),
]);
```

For `/guides/herbs/indoor/basil`, `path` contains `"herbs/indoor/basil"`, including the slashes. With `/guides/{path}` instead, only a single segment such as `/guides/basil` would match.

A wildcard must capture at least one character, so `/guides/` doesn't match `/guides/{*path}`. An index at `/guides/` needs its own route. The wildcard must also be the last part of the pattern, with nothing after its closing brace.

Capturing a file-like path doesn't serve a file automatically. The handler above returns text; Cot's [static-file support](../static-files/) handles serving application assets.

### Parameter syntax and constraints

Parameter names start with a letter or underscore and contain only letters, digits, or underscores. Invalid patterns, such as an unclosed brace, panic when the route is constructed.

Route patterns don't support optional parameters or regular-expression constraints. An archive that accepts both `/archive/2026` and `/archive/2026/09` needs two routes: `/archive/{year}` and `/archive/{year}/{month}`. Constraints on captured values belong in extraction or handler logic.

## HTTP methods

The URL path identifies a route, but the HTTP method can determine what happens there. An article editor might use GET to display a form and POST to accept a submission, both at `/articles/new`.

Cot's [`MethodRouter`](struct@cot::router::method::MethodRouter) groups those handlers under one route. We can chain methods to associate each one with a different handler:

```rust
use cot::router::{Route, Router};
use cot::router::method::get;

async fn new_article() -> &'static str {
    "Start a new article: give it a title, then tell us what you've been growing."
}

async fn submit_article() -> &'static str {
    "Article submission received"
}

let router = Router::with_urls([
    Route::with_handler(
        "/articles/new",
        get(new_article).post(submit_article),
    ),
]);
```

A GET request reaches `new_article`; a POST request reaches `submit_article`. These handlers return text to show which method was selected. Reading and validating a submission is covered in the [forms guide](../forms/).

Path selection happens before method selection. If we instead register a GET-only `/articles/new` route followed by a POST-only route at the same path, POST stops at the first route with 405 Method Not Allowed. It doesn't fall through to the second route. Grouping the methods in one method router lets both handlers share the path.

### Method defaults and fallbacks

The helpers in [`cot::router::method`](cot::router::method) support `get`, `head`, `post`, `put`, `patch`, `delete`, `options`, `trace`, and `connect`. Their corresponding methods can be chained in the same way as `.post()` above.

If we pass a handler directly to `Route::with_handler` or `Route::with_handler_and_name`, without a method wrapper, it accepts every HTTP method. With a method router, the defaults are more restrictive:

- HEAD uses the GET handler if we haven't registered a HEAD handler.
- Other unregistered methods return 405 Method Not Allowed. OPTIONS needs its own handler too.
- `.fallback(handler)` replaces the default response to unhandled methods on that path. It doesn't handle unmatched URL paths.

## Nested routers

A router can contain other routers. This lets a group of related routes share a prefix while keeping their individual patterns together. [`Route::with_router`](struct@cot::router::Route#method.with_router) associates that prefix with a nested router:

```rust
use cot::router::{Route, Router};
use cot::router::method::get;
# use cot::request::extractors::Path;
# use cot::error::NotFound;
# async fn list_articles() -> &'static str {
#     "42: Growing herbs on a windowsill\n43: When to water your seedlings"
# }
# async fn article(Path(id): Path<u64>) -> cot::Result<&'static str> {
#     match id {
#         42 => Ok("Growing herbs on a windowsill: choose a spot with plenty of light."),
#         43 => Ok("When to water your seedlings: check the soil before watering."),
#         _ => Err(NotFound::new().into()),
#     }
# }

let articles = Router::with_urls([
    Route::with_handler_and_name("/", get(list_articles), "articles"),
    Route::with_handler_and_name("/{id}", get(article), "article"),
]);

let router = Router::with_urls([
    Route::with_router("/articles", articles),
]);
```

Using the article handlers shown above, this router matches `/articles/` and `/articles/{id}`. For `/articles/42`, the parent matches `/articles` and passes `/42` to the nested router. A prefix match alone isn't enough: the nested router still needs to find a handler for the remaining path.

The prefix has no trailing slash, while each child path starts with one. Cot joins these parts as written when generating URLs, so a slash on both sides would produce a double slash.

### Parameters in prefixes

A prefix can contain parameters too. For article routes nested under `/journals/{journal_id}/articles`, a request to `/journals/7/articles/42` captures both `journal_id` and `id`.

The article handler must then extract both values. A tuple follows the full path order, so `Path<(u64, u64)>` receives `(7, 42)`. A struct can use fields named `journal_id` and `id`. Give parent and child parameters distinct names so a child capture doesn't replace a parent capture.

### App prefixes

App registration also gives routes a shared prefix. In a project's `register_apps` method, `register_with_views` includes the app's router under that prefix:

```rust
use cot::{App, AppBuilder};
use cot::project::RegisterAppsContext;
# struct JournalApp;
# impl App for JournalApp {
#     fn name(&self) -> &'static str { "journal" }
# }
# struct MyProject;
# impl cot::Project for MyProject {
fn register_apps(&self, apps: &mut AppBuilder, _context: &RegisterAppsContext) {
    apps.register_with_views(JournalApp, "/garden");
}
# }
```

With this registration, an app route `/articles/` is available at `/garden/articles/`. An empty prefix places the app's routes at the site root. Registration also associates the router with the app's name, which is used when generating URLs.

## Named routes and URL generation

A route name lets us refer to a destination without repeating its URL pattern. This matters when a path changes: links built from the name follow the new pattern, while hard-coded links still point to the old one.

Generating a URL from a route name is called *reversing*. For a route named `"article"` with the pattern `/articles/{id}`, [`reverse!`](macro@cot::reverse) fills in the ID:

```rust
use cot::router::Urls;

async fn featured_article_url(urls: Urls) -> cot::Result<String> {
    let url = cot::reverse!(urls, "article", id = 42)?;
    Ok(url)
}
```

The [`Urls`](struct@cot::router::Urls) extractor provides the project's router and the current app name. In a handler belonging to the app that owns this route, the example returns `/articles/42`. With an app prefix of `/garden`, it returns `/garden/articles/42`. Nested router prefixes are included too.

We can also pass a `Request` to `reverse!` when the handler already has one. For links in rendered HTML, the same idea applies to [reversing URLs in templates](../templates/#urls).

### App namespaces

Different apps can use the same route name. Cot distinguishes them through the app name, so `"article"` is looked up in the current app when an app name is available.

A qualified name identifies the app explicitly. For example, `cot::reverse!(urls, "journal:article", id = 42)?` refers to the `article` route in the app named `journal`, even from a handler in another app. The `journal` part comes from `JournalApp::name`, not from the URL prefix.

Distinct route names within an app keep URL generation unambiguous. Registering an app associates its routes with that app's name; grouping routes with `Route::with_router` doesn't create another app namespace.

### Parameters and encoding

Every parameter in the full path needs a value, including parameters in a parent prefix. Reversing `/journals/{journal_id}/articles/{id}` requires both `journal_id = 7` and `id = 42` to produce `/journals/7/articles/42`. A missing parameter or an unknown route name produces an error, which the example propagates with `?`.

Values are converted to strings and inserted as supplied. Cot doesn't percent-encode them, so encode values that need it before passing them in. Extra parameters don't become a query string: adding `preview = true` won't append `?preview=true`. Build query strings separately.

### Redirects

A redirect can use a route name too. [`reverse_redirect!`](macro@cot::reverse_redirect) combines URL generation with a 303 See Other response. For example, a featured-article handler can redirect to the current selection:

```rust
use cot::response::Response;
use cot::router::Urls;

async fn featured_article(urls: Urls) -> cot::Result<Response> {
    cot::reverse_redirect!(urls, "article", id = 42)
}
```

The redirect's destination follows the named route, including its app and router prefixes. Missing routes or parameters produce the same errors as `reverse!`.

## Trailing slashes

Cot treats `/articles` and `/articles/` as different paths. A route for one doesn't automatically match the other, and the router doesn't add or remove slashes during matching.

[`TrailingSlashMiddleware`](struct@cot::middleware::TrailingSlashMiddleware) provides optional redirects to paths with a trailing slash. It belongs to the project's middleware chain, outside the router:

```rust
use cot::middleware::TrailingSlashMiddleware;
use cot::project::{MiddlewareContext, RootHandler, RootHandlerBuilder};
# struct MyProject;
# impl cot::Project for MyProject {
fn middlewares(&self, handler: RootHandlerBuilder, context: &MiddlewareContext) -> RootHandler {
    handler
        .middleware(TrailingSlashMiddleware::from_context(context))
        .build()
}
# }
```

With this middleware, a GET request to `/articles` redirects to `/articles/` only if the original path has no matching handler and the appended slash produces a match. The redirect uses 308 Permanent Redirect and preserves the query string: `/articles?sort=newest` becomes `/articles/?sort=newest`.

If both paths already have handlers, neither needs a redirect. The middleware only redirects GET and HEAD; it doesn't redirect POST requests or remove trailing slashes. A form posting to `/articles` therefore still needs a handler at that exact path.

Middleware and routing are separate layers. Calling `Router::handle` directly in a test checks routing without running this middleware; a complete test application includes the middleware chain.

The [testing guide](../testing/) covers requests through routers and complete applications. Custom responses to routing and handler errors are covered in [error pages](../error-pages/). For routes that also describe an API, see the API-specific handler constructors in the [OpenAPI guide](../openapi/).

## Route matching and resource access

A matched route establishes which handler can process the request. It doesn't establish that the requested record exists or that the current user may access it.

For `/orders/42`, there are three separate questions: does the path select the order handler, can `42` be extracted as the handler's ID type, and may the current actor read that stored order? Keeping those boundaries separate helps us choose the right response and the right test.

The [request guide](../guides/requests/) explains extraction, [authorization](../guides/authorization/) explains resource decisions, and [diagnose a route mismatch](../how-to/diagnose-routing/) provides a focused troubleshooting procedure.
