<div align="center">
<h1><a href="https://cot.rs">Cot Website</a></h1>

[![Docker Build Status](https://github.com/cot-rs/cot/workflows/Docker%20Images/badge.svg)](https://github.com/cot-rs/cot/actions/workflows/docker.yml)
</div>

This contains the sources needed to build the website for the Cot web framework.

## Development

Make sure you have `cargo` installed. You can get it through [rustup](https://rustup.rs/).

Then, the easiest way to run the development server is to run:

```bash
cargo run
```

The website doesn't need any external resources (such as the database), so nothing more is needed.

### Modifying the guide or other Markdown files

Because of the internals of Markdown processing macros work, you will need to use the nightly toolchain if you want to see the changes made to the Markdown files in the without using `cargo clean`.

### Live reloading

To make the development more convenient, you can use [bacon](https://dystroy.org/bacon/) to get live reloading capabilities. After installing it, you can execute:

```bash
bacon serve
```

All the changes you do in Rust source files or the templates should be automatically reflected in the web browser.

## Docker image

You can also build and run the website using Docker. To build the image, run the following command inside the `docs/` directory:

```bash
docker build -t cot-site . && docker run -p '8000:8000' --rm -it cot-site
```

## Documentation architecture preview

The development documentation at `/guide/master/` is an interactive prototype of
Cot's proposed documentation structure. It contains existing guides and 74 expanded new pages, including routing.
The tutorial companion provides three runnable applications. Proposed framework
features remain design documentation, not implemented integrations.

- `src/navigation.rs` registers documentation areas, topic groups, and pages.
  Sidebar grouping is independent of page URLs.
- The sibling `cot-site` library renders collapsible areas, topic groups,
  breadcrumbs, page types, status labels, and the on-page contents list.
- Rustdoc remains the API reference. The reference pages here index configuration,
  CLI commands, Cargo features, and component modules.
- New Markdown frontmatter may use `status: preview` for draft documentation or
  `status: proposed` for an explicitly unsupported feature. Omitting `status`
  preserves existing page rendering. These states appear in both the sidebar and
  the page notice; they are separate from the selected Cot version.
- Previous/next links stay within a tutorial series. Explanation pages have no
  implied reading sequence. Historical documentation retains its original links.
- Existing topic URLs remain registered. The development landing page is now
  the documentation directory; the previous introduction is available at
  `/guide/master/introduction/`.
- Rustdoc links point to the published release and are labeled accordingly in the
  reference index. They must not be interpreted as development-checkout API docs.

Both `cot-site` and its Cot dependency must resolve to local checkouts for this
preview. On stable Rust, touching `src/navigation.rs` forces Markdown macros to
rebuild after a content change. Run snippet tests from the repository root with
`cargo nextest run -p cot-test`; do not compile the site simultaneously because
that runner traverses the documentation directory, including `site/target`.

### Checking the preview

With the site running, check registered pages, internal links, and heading anchors
from the repository root:

```bash
python3 docs/site/checks/check_navigation.py http://127.0.0.1:18080/guide/master/
```

The browser check covers desktop and mobile navigation, tutorial sequences,
breadcrumbs, search labels, Rustdoc links, and horizontal overflow:

```bash
node docs/site/checks/browser.cjs http://127.0.0.1:18080/guide/master/
```

It requires Playwright and Chromium. Set `PLAYWRIGHT_MODULE` to use an existing
Playwright installation, `CHROME_PATH` to use a local Chrome executable, and
`DOCS_ARTIFACTS` to choose a screenshot directory. The check blocks external font
requests so font availability does not determine whether navigation works.

### Editorial research

The existing `docs/databases/queries.md` is the voice model for explanatory guides.
The bundled Microsoft style PDF was consulted for **Top 10 tips for Microsoft
style and voice** (page 18) and **Scannable content** (page 1098). The prototype
uses concise concept introductions, example cases, and descriptive subheadings.

The navigation proposal uses [Diátaxis](https://diataxis.fr/start-here/) and
[Tom Johnson's navigation guidance](https://idratherbewriting.com/files/doc-navigation-wtd/design-principles-for-doc-navigation/).
The [Django 6.0 contents](https://docs.djangoproject.com/en/6.0/contents/) and
[Laravel 13 documentation](https://laravel.com/framework/docs/13.x) informed topic
coverage. These inventories are not evidence that Cot implements those features.

Representative page research:

| Prototype coverage | Django 6.0 | Laravel 13 | Pattern used |
| --- | --- | --- | --- |
| Tutorials | [First app](https://docs.djangoproject.com/en/6.0/intro/tutorial01/) | [Installation](https://laravel.com/framework/docs/13.x) | A small initial result and explicit prerequisites |
| Lifecycle and requests | [HTTP topics](https://docs.djangoproject.com/en/6.0/topics/http/) | [Lifecycle](https://laravel.com/framework/docs/13.x/lifecycle) | Explain component boundaries before details |
| Middleware | [Middleware](https://docs.djangoproject.com/en/6.0/topics/http/middleware/) | [Middleware](https://laravel.com/framework/docs/13.x/middleware) | Request/response ordering and dependencies |
| Sessions | [Sessions](https://docs.djangoproject.com/en/6.0/topics/http/sessions/) | [Sessions](https://laravel.com/framework/docs/13.x/session) | Storage, expiry, and concurrent-request implications |
| Background tasks | [Tasks](https://docs.djangoproject.com/en/6.0/topics/tasks/) | [Queues](https://laravel.com/framework/docs/13.x/queues) | Distinguish acceptance, execution, retry, and completion |
| Configuration and deployment | [Deployment](https://docs.djangoproject.com/en/6.0/howto/deployment/) | [Configuration](https://laravel.com/framework/docs/13.x/configuration), [deployment](https://laravel.com/framework/docs/13.x/deployment) | Separate framework configuration from operational setup |

The expanded feature-by-feature research and source mapping is in
[the editorial record](editorial/README.md). It includes scope, source versions,
lessons adopted, unavailable counterparts, and code-validation boundaries.

### Framework transitions and blog

`coming-from/` contains explanation pages, grouped under Guides, with an entry
from the home page and learning paths. The Rust introduction serves readers from
Python, PHP, Ruby, and Java/Kotlin; Axum and Actix Web readers can skip it.
Comparisons identify conceptual counterparts, not drop-in compatibility.

Blog is a navbar destination, excluded from both documentation sidebar variants.
For this prototype its two Markdown pages use the existing versioned page renderer
and master URLs. A production blog should have unversioned URLs, publication dates,
authors, and an archive; the sample has no invented author, release, or customer story.

Research for the transition overview, six framework pages, Rust introduction, and
learning-path links reused Django 6.0's [architectural overview](https://docs.djangoproject.com/en/6.0/intro/overview/)
and Laravel 13's [request lifecycle](https://laravel.com/framework/docs/13.x/lifecycle).
Their useful pattern is to connect familiar responsibilities before explaining
framework-specific composition. Neither has a direct counterpart for moving to Cot.
The shared Rust page explains language concepts rather than translating their APIs.
The other comparisons use official [Rails getting started](https://guides.rubyonrails.org/getting_started.html),
[Spring Boot code structure](https://docs.spring.io/spring-boot/reference/using/structuring-your-code.html),
[Axum crate documentation](https://docs.rs/axum/latest/axum/), and
[Actix Web application documentation](https://actix.rs/docs/application/).
These unpinned sources were consulted September 28, 2026; the pages do not claim
exhaustive migration coverage or source-version compatibility.

The blog index and sample editorial follow the separation of news from maintained
documentation visible in the [Django weblog](https://www.djangoproject.com/weblog/)
and [Laravel blog](https://laravel.com/blog/). The sample discusses this prototype's
information architecture rather than claiming a published Cot design decision.
Microsoft PDF pages 18 and 1098 informed the short introductions, descriptive
headings, and comparison tables. Cot's Project/App contracts and existing queries
and template guides were checked for the conceptual mappings.

### Checking out the shared preview

Clone the `docs-preview` branches of `ElijahAhianyo/cot` and
`ElijahAhianyo/cot-site` into sibling directories named `cot` and `cot-site`.
Initialize the cot-site submodules before building. No crates.io publication is
required; both manifests use relative paths to these checkouts.

```bash
git clone --branch docs-preview https://github.com/ElijahAhianyo/cot.git
git clone --branch docs-preview --recurse-submodules https://github.com/ElijahAhianyo/cot-site.git
cd cot/docs/site
cargo run --locked -- --listen 127.0.0.1:18080
```

Open `http://127.0.0.1:18080/guide/master/`. These commands assume a new parent
directory, with neither checkout already present. The [preview deployment recipe](deploy/README.md) includes both repositories
without publishing crates. Content changes alone do not confirm that a hosted
service has deployed the new revision.
