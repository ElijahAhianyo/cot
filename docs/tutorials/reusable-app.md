---
title: Build a reusable Cot app
status: preview
---

We'll separate a small announcements feature from the project that hosts it. The app owns a route; the host chooses its message and mount prefix. Two host configurations will verify that those choices really belong to the host.

Use the [tutorial companion checkout](../first-app/). This example has no models or migrations and runs independently of the issue tracker.

## Run the first host

From `docs/site/checks/issue-tracker`:

```bash
cargo run --locked --bin announcements -- --listen 127.0.0.1:8003
```

Open `http://127.0.0.1:8003/support/`. The page shows a short support announcement. The trailing slash is part of this example's route; it doesn't install trailing-slash middleware to normalize alternatives.

## Inspect the app boundary

Open `src/announcements.rs`. The complete app is:

```rust
use cot::router::method::get;
use cot::router::{Route, Router};
use cot::App;

pub struct AnnouncementsApp {
    pub message: &'static str,
}

impl App for AnnouncementsApp {
    fn name(&self) -> &'static str {
        "announcements"
    }

    fn router(&self) -> Router {
        let message = self.message;
        Router::with_urls([
            Route::with_handler("/", get(move || async move { message })),
        ])
    }
}
```

The message is a static string in this bounded example. The router captures it for the handler, which returns plain text. The app doesn't read a host-specific configuration filename or assume that its public URL begins with `/support`.

An owned message loaded at runtime would need a suitable ownership and sharing design. The [shared-state guide](../../guides/async-state/) explains that next step; a borrowed configuration value cannot outlive its owner merely because a handler needs it.

## Find the host's responsibilities

Open `src/bin/announcements.rs`. Its `HostProject` holds the prefix and message. During `register_apps`, it constructs `AnnouncementsApp` and passes the prefix to `register_with_views`.

Change the prefix from `/support` to `/news`, then restart. The announcement should appear at `/news/`; the old `/support/` path should no longer select it. Change the message and confirm the response changes without editing the reusable app module.

Restore the original values before continuing if you want to keep the checked-in example unchanged.

## Test a second host

Stop the server and run:

```bash
cargo test --locked --bin announcements
```

The test constructs two independent projects: one mounts the app under `/support`, the other under `/news`. It checks each response's content and confirms that neither project accidentally exposes the app at `/`.

This catches two common coupling mistakes: hard-coding the host's prefix inside the app and ignoring configuration supplied by the host. It doesn't prove that every possible integration works; the app's public inputs define the supported variation.

## Move the boundary into a crate

The example keeps the app in a separate module so its boundary is visible without packaging work. To distribute it, that module can become a library crate's public interface. The host then depends on the library and imports its app type.

Keep the executable, listening address, and deployment configuration in the host. Document the library's Cot version requirement and enabled features. A reusable package should not silently bring a development database or host-specific secret into another project.

If the app grows templates with links, generate those links from route names and the supplied routing context. The fixed root-relative links used in a single-purpose project would defeat this mount-prefix test.

## Add capabilities deliberately

Models introduce migrations and schema ownership. Static assets introduce URL and collection behavior. Authentication introduces a contract with the host's identity system. Each addition deserves an integration test in a host with different settings, not only a unit test inside the library.

Continue with [reusable apps and integrations](../../guides/reusable-apps/) for those design questions and [custom components](../../guides/custom-components/) for narrower extension points.
