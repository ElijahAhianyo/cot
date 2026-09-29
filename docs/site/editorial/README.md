# Expanded documentation editorial record

Reviewed September 28–29, 2026. This record is outside the published navigation.

The 74 public pages added since Cot baseline `9d3c305` have been expanded or reviewed and extended. Existing database, form, template, cache, email, and other original guide pages were not rewritten. The new pages contain approximately 43,600 words in total, including examples and navigation text; this is a scope measure, not a quality claim.

## Voice and document purpose

`docs/databases/queries.md` is the explanation voice model: concrete application cases, natural “we” where useful, behavior followed by exceptions. The bundled Microsoft Writing Style Guide was consulted at **Top 10 tips for Microsoft style and voice** (PDF page 18) and **Scannable content** (PDF page 1098). Headings name the subject, sentences use concrete actors, and tables compare genuinely parallel cases. Explanations do not prescribe a tutorial sequence. Tutorials and how-to pages intentionally use action-oriented instructions.

The existing Diátaxis/Johnson navigation structure remains in place. Rustdoc remains authoritative for API contracts. Proposed status remains on unsupported integrations; scheduling and event dispatch are now also marked Proposed. No queue, scheduler, seeding, or rate-limiter API was invented.

## Official framework comparisons

Django sources below are version 6.0; Laravel sources are version 13.x. Feature pages were retrieved and read, not inferred solely from their contents pages. These sources inform teaching structure and conceptual coverage, not Cot capability claims. The prose and application examples are original. The same source can inform multiple pages; the mappings below make that reuse explicit.

| Cot pages (under docs/) | Django 6.0 | Laravel 13.x | Approach adopted or counterpart limitation |
| --- | --- | --- | --- |
| guides/projects; guides/lifecycle; guides/async-state; guides/overview; start; learning-paths; coming-from/*; reference/overview; reference/components | [fundamentals](https://docs.djangoproject.com/en/6.0/intro/overview/) | [fundamentals](https://laravel.com/framework/docs/13.x/lifecycle) | Connect responsibilities through a request before introducing integration details. |
| guides/configuration; guides/production; reference/configuration; reference/features | [configuration](https://docs.djangoproject.com/en/6.0/topics/settings/) | [configuration](https://laravel.com/framework/docs/13.x/configuration) | Separate source selection, defaults, secrets, and build-time capability. |
| guides/middleware; guides/custom-components | [middleware](https://docs.djangoproject.com/en/6.0/topics/http/middleware/) | [middleware](https://laravel.com/framework/docs/13.x/middleware) | Explain wrapping order through dependencies and early responses. |
| guides/sessions | [sessions](https://docs.djangoproject.com/en/6.0/topics/http/sessions/) | [sessions](https://laravel.com/framework/docs/13.x/session) | Separate browser cookies, server storage, expiry, and concurrent updates. |
| guides/validation; tutorials/forms-auth | [validation](https://docs.djangoproject.com/en/6.0/topics/forms/) | [validation](https://laravel.com/framework/docs/13.x/validation) | Begin with valid input, then preserve values and explain server-side rejection. |
| guides/media; how-to/private-uploads | [media](https://docs.djangoproject.com/en/6.0/topics/files/) | [media](https://laravel.com/framework/docs/13.x/filesystem) | Separate receiving a file, storing it, and authorizing delivery. |
| guides/localization | [localization](https://docs.djangoproject.com/en/6.0/topics/i18n/timezones/) | [localization](https://laravel.com/framework/docs/13.x/localization) | Use concrete time-zone ambiguity and display/storage distinctions; no translation service is claimed. |
| guides/relationships; tutorials/model | [relationships](https://docs.djangoproject.com/en/6.0/topics/db/queries/) | [relationships](https://laravel.com/framework/docs/13.x/eloquent-relationships) | Explain model association, explicit retrieval, and query cost. |
| guides/pagination | [pagination](https://docs.djangoproject.com/en/6.0/topics/pagination/) | [pagination](https://laravel.com/framework/docs/13.x/pagination) | Explain deterministic order and offset/cursor tradeoffs without claiming a built-in paginator. |
| guides/seeding | [seeding](https://docs.djangoproject.com/en/6.0/howto/initial-data/) | [seeding](https://laravel.com/framework/docs/13.x/seeding) | Distinguish production reference data, development samples, and test fixtures. |
| guides/query-performance; guides/performance | [query-performance](https://docs.djangoproject.com/en/6.0/topics/db/optimization/) | [query-performance](https://laravel.com/framework/docs/13.x/eloquent) | Begin with observed query work; discuss repeated queries and measurement. |
| guides/authentication; guides/account-lifecycle | [authentication](https://docs.djangoproject.com/en/6.0/topics/auth/default/) | [authentication](https://laravel.com/framework/docs/13.x/authentication) | Separate credential checking, login state, recovery, and revocation. |
| guides/authorization | [authorization](https://docs.djangoproject.com/en/6.0/topics/auth/customizing/) | [authorization](https://laravel.com/framework/docs/13.x/authorization) | Express a resource rule, then tenant and concurrency boundaries. |
| guides/web-security; tutorials/forms-auth | [security](https://docs.djangoproject.com/en/6.0/topics/security/) | [security](https://laravel.com/framework/docs/13.x/csrf) | Separate escaping, CSRF, authentication, and authorization; avoid importing framework guarantees. |
| guides/rate-limiting | [rate-limiting](https://docs.djangoproject.com/en/6.0/topics/cache/) | [rate-limiting](https://laravel.com/framework/docs/13.x/rate-limiting) | Use identity, windows, atomic counters, and overload behavior. Django cache is the nearest counterpart, not a claim of an integrated limiter. |
| guides/background-tasks; how-to/queued-email | [background](https://docs.djangoproject.com/en/6.0/topics/tasks/) | [background](https://laravel.com/framework/docs/13.x/queues) | Distinguish durable acceptance, execution, retries, duplicate effects, and recovery. |
| guides/scheduling | [scheduling](https://docs.djangoproject.com/en/6.0/howto/custom-management-commands/) | [scheduling](https://laravel.com/framework/docs/13.x/scheduling) | Explain overlap, time zones, and catch-up. Django management commands are the nearest mechanism, not an equivalent scheduler. |
| guides/events | [events](https://docs.djangoproject.com/en/6.0/topics/signals/) | [events](https://laravel.com/framework/docs/13.x/events) | Distinguish facts from commands and synchronous from durable delivery. |
| guides/json-apis; tutorials/json-api; guides/responses; how-to/test-redirect | [api](https://docs.djangoproject.com/en/6.0/ref/request-response/) | [api](https://laravel.com/framework/docs/13.x/responses) | Separate representation, status, headers, and application validation contracts. |
| guides/http-webhooks | [http-clients](https://docs.djangoproject.com/en/6.0/topics/async/) | [http-clients](https://laravel.com/framework/docs/13.x/http-client) | Explain timeouts, remote effects, signature verification, and duplicate deliveries. Django async is execution context, not an outbound-client counterpart. |
| guides/realtime | [realtime](https://docs.djangoproject.com/en/6.0/topics/async/) | [realtime](https://laravel.com/framework/docs/13.x/broadcasting) | Explain connection lifetime and durable state. Django async is a nearby concept, not a built-in broadcasting counterpart. |
| guides/http-tests; tutorials/test-deploy; how-to/test-redirect | [http-tests](https://docs.djangoproject.com/en/6.0/topics/testing/tools/) | [http-tests](https://laravel.com/framework/docs/13.x/http-tests) | Use observable assertions at the selected request boundary. |
| guides/database-tests; tutorials/model; tutorials/test-deploy | [database-tests](https://docs.djangoproject.com/en/6.0/topics/testing/overview/) | [database-tests](https://laravel.com/framework/docs/13.x/database-testing) | Make schema setup and test isolation explicit; use real database behavior. |
| guides/browser-tests | [browser-tests](https://docs.djangoproject.com/en/6.0/topics/testing/tools/) | [browser-tests](https://laravel.com/framework/docs/13.x/dusk) | Separate browser behavior from in-process application tests and use stable assertions. |
| guides/deployment; guides/production; guides/recovery; how-to/production-build; releases/overview | [deployment](https://docs.djangoproject.com/en/6.0/howto/deployment/checklist/) | [deployment](https://laravel.com/framework/docs/13.x/deployment) | Separate artifact, configuration, schema changes, persistent data, and recovery. |
| guides/observability | [observability](https://docs.djangoproject.com/en/6.0/topics/logging/) | [observability](https://laravel.com/framework/docs/13.x/logging) | Connect logs and request identity to operational questions without exposing secrets. |
| guides/performance | [performance](https://docs.djangoproject.com/en/6.0/topics/performance/) | [performance](https://laravel.com/framework/docs/13.x/cache) | Measure representative work and discuss cache and capacity tradeoffs. |
| guides/reusable-apps; tutorials/reusable-app | [reusable](https://docs.djangoproject.com/en/6.0/intro/reusable-apps/) | [reusable](https://laravel.com/framework/docs/13.x/packages) | Test a feature in another host and make configuration and mount boundaries explicit. |
| guides/management-commands; reference/cli | [commands](https://docs.djangoproject.com/en/6.0/howto/custom-management-commands/) | [commands](https://laravel.com/framework/docs/13.x/artisan) | Distinguish command context, initialization, repeatability, and operational effects. |
| installation; tutorials/overview; tutorials/first-app; tutorials/model; tutorials/views; tutorials/forms-auth; tutorials/test-deploy; tutorials/json-api; tutorials/reusable-app | [tutorial](https://docs.djangoproject.com/en/6.0/intro/tutorial01/) | [tutorial](https://laravel.com/framework/docs/13.x/installation) | Use a bounded runnable example, observable changes, and failure checkpoints; scope the example honestly. |
| guides/errors | [errors](https://docs.djangoproject.com/en/6.0/ref/views/) | [errors](https://laravel.com/framework/docs/13.x/errors) | Separate expected absence, rejected input, and internal failure at the public boundary. |
| routing; guides/requests; how-to/diagnose-routing | [routing](https://docs.djangoproject.com/en/6.0/topics/http/urls/) | [routing](https://laravel.com/framework/docs/13.x/routing) | Show path selection, typed extraction, method handling, and non-fallthrough cases. |

Additional research:

- `tutorials/model`: [Django tutorial, part 2](https://docs.djangoproject.com/en/6.0/intro/tutorial02/) connects model fields, app registration, and migrations. Cot uses its own migration and server lifecycle.
- `community/overview`: [Django contributing](https://docs.djangoproject.com/en/6.0/internals/contributing/) and [Laravel contributions](https://laravel.com/framework/docs/13.x/contributions) inform reproducible reports and contribution boundaries. Cot's own contribution and security policies govern actual submissions.
- `releases/overview`: [Laravel releases](https://laravel.com/framework/docs/13.x/releases) and the Django deployment checklist inform compatibility categories. No external support window is imported as Cot policy.
- `blog/overview`, `blog/framework-transition-guides`: the [Django weblog](https://www.djangoproject.com/weblog/) and [Laravel blog](https://laravel.com/blog/) were consulted for separation of dated stories from maintained feature documentation. The article is explicitly a sample editorial; no publication history or production experience is invented.
- `coming-from/*`: Django overview and Laravel lifecycle provide the responsibility-first comparison pattern. The additional official [Rails introduction](https://guides.rubyonrails.org/getting_started.html), [Spring Boot structure](https://docs.spring.io/spring-boot/reference/using/structuring-your-code.html), [Axum API](https://docs.rs/axum/latest/axum/), and [Actix application guide](https://actix.rs/docs/application/) were consulted for their respective comparisons. Those unpinned sources are not a guarantee of cross-version compatibility.
- The directory pages `guides/overview`, `how-to/overview`, `reference/overview`, `tutorials/overview`, `coming-from/overview`, `start`, and `learning-paths` reuse the mapped sources for the sections they link to. They remain directories rather than artificially long feature chapters.

## Code evidence and verification

Cot sources checked include Project/App composition and server startup, request extraction, response conversion, routing and method selection, configuration fields and feature declarations, forms, authentication and sessions, database operations, CLI tasks, and test helpers. New code examples are checked by the existing documentation harness.

The companion in `../checks/issue-tracker/` supplies three runnable applications: the local issue tracker, quote API, and configurable announcements app. It includes a generated migration and behavior tests. A discovered distinction is documented explicitly: the in-process `Client` does not run server migrations or app init hooks. Its database fixture applies migrations before creating the client.

Plain-text diagrams and illustrative payloads use `text` fences. The snippet harness now recognizes this non-executable language without treating it as Rust; Rust, TOML, and Askama checks remain enabled.

Before sharing, run the documentation snippet suite, companion application tests, site build, link/anchor crawl, and desktop/mobile browser checks. Record actual results in the change summary. The local companion is intentionally not a production authentication or CSRF implementation.

### Validation completed September 29, 2026

- Documentation harness: 162 checks covered; the full run passed 161, and the remaining standard-Result ambiguity was fixed and its targeted rerun passed.
- Companion: four application tests passed across three binaries. Live HTTP checks also passed for creation, form rejection, missing/malformed IDs, persistence across a process restart, JSON quantities at and outside the allowed boundary, and the configured announcement mount.
- Site: built against local fork Cot and cot-site manifests, verified with resolved Cargo metadata.
- Navigation crawl: 91 registered URLs and 18,175 internal links; no duplicate IDs or missing anchors.
- Browser: desktop/mobile navigation, tutorial sequence, status labels, search, Rustdoc links, on-page contents, and overflow checks passed. Desktop explanation and mobile proposed-feature screenshots were inspected.

These checks validate the documentation and bounded examples. They do not implement the Proposed framework capabilities or establish production readiness of the tutorial companion. Existing compiler warnings remain in the selected minimal-feature framework build.

## Community showcase

Added `community/showcase` under the existing Community section, with links from
its overview and the documentation home. Existing URLs are unchanged. Real entries
come from [Cot's FAQ](https://cot.rs/faq/), checked September 29, 2026; no placeholder
projects, usage figures, or maintenance claims were invented. Cot's own website is
separated from the community project cards. Submission links open GitHub discussions;
this change does not submit a discussion or contact anyone.

[Django community](https://www.djangoproject.com/community/) and
[Laravel community](https://laravel.com/community) informed the separation of discovery,
participation, and help. They are unversioned community pages, not framework feature
references. Microsoft PDF page 18 informed short descriptions and direct invitations.

### Showcase screenshots

Captured public landing pages on September 29, 2026, in Chromium at 1280 × 800:

- `blog-20260929.jpg`: https://mackow.ski/ — Mateusz Maćkowski’s blog.
- `chombogen-20260929.jpg`: https://hand.chombo.club/ — ChomboGen’s initial generator form.
- `cot-20260929.jpg`: https://cot.rs/ — the Cot homepage.

These are unaltered browser captures, not mockups. The depicted sites and branding belong to their respective creators; inclusion does not imply endorsement. The cards link to each original site. Brewnerator was removed because its listed GitHub repository returned 404 during this check; restore it when a public project link and preview are available.

Images live in `docs/site/static/static/images/community/` and are embedded by the runner’s `ShowcaseAssets` app, so deployment needs no external image service. Use a new dated filename when replacing a screenshot because static files have a long cache lifetime. Each card supplies dimensions, descriptive alt text, and a responsive image. Future submissions request a screenshot and text alternative.

Applied Microsoft’s “Responsive content” guidance (bundled PDF, page 1096): images fit small screens and have text alternatives. The existing Django/Laravel community-page comparison still applies; this change adds visual previews without changing the page’s discovery purpose.
