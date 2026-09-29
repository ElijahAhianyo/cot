---
title: Reusable apps and integrations
status: preview
---

A reusable app packages a coherent piece of framework behavior so more than one project can use it. A catalog, help center, or internal reporting area may contribute routes, templates, static assets, and database migrations.

Reusability does not require publishing a crate immediately. We can begin with a module, give it a clear boundary, and move it into a crate when another project needs it.

## The app boundary

Cot's App trait is the integration point. A project chooses which apps to register and where to mount their routes. The reusable component should not assume that its routes begin at the site root.

For a help app, a local route `/topics/` might be mounted at `/help/topics/` in one project and `/support/topics/` in another. Generate links through named routes where appropriate instead of embedding one project's prefix in every template.

## Public configuration

Expose the decisions the host project genuinely needs to make. For example, a reporting app may need a title and an authorization adapter. It should not require the host to reproduce the app's internal file layout or set unrelated global variables.

Validate required configuration when constructing or registering the component. A missing dependency should have an actionable failure, not a panic deep inside the first user's request.

## Framework-independent logic

Keep calculations and domain types separate from HTTP adaptation when that makes them reusable. A report-total function that accepts rows and returns a value can be tested without starting Cot.

Not every helper needs an App implementation. Rust crates and modules remain the right tools for code that contributes no routes or framework resources. [Projects and apps](../../guides/projects/) explains that distinction.

## Resource names and collisions

Routes, assets, migrations, and configuration names can collide when apps are assembled. Give public names a clear ownership convention and test the app alongside another component using similar local names.

A reusable app should also work at a non-root prefix. That test catches hard-coded links, asset assumptions, and redirects that appeared correct only in its standalone example project.

## Migrations and compatibility

A host that installs a reusable app is accepting its schema and upgrade path. Treat migrations as part of the public contract. Renaming or removing historical migrations can affect existing installations even when a fresh database builds successfully.

Document supported Cot versions and required Cargo features. Avoid forcing an unrelated database backend or optional integration into every host without a reason.

## Testing integration

Use a small host project as an integration fixture. Register the app, mount it under a prefix, supply its declared dependencies, and exercise a representative request. Include missing configuration and denied-access cases.

Separately test the component's ordinary functions without the host. This keeps failures focused: a calculation failure shouldn't require reading a route-registration trace.

## Distribution

A local path dependency is sufficient while projects are developed together. A Git revision can identify a shared snapshot. Publishing is useful when the API and release process are ready; it isn't required merely to try the component in a second project.

The [reusable-app tutorial](../../tutorials/reusable-app/) provides a small route-only exercise. The [component reference](../../reference/components/) points to the exact extension traits.
