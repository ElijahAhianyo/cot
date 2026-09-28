---
title: Projects and apps
status: preview
---

A project assembles a running application. An app groups related behavior that the project can register: routes, models, templates, and initialization. Keeping those responsibilities separate makes it easier to reuse a feature without copying an entire server.

## Project boundaries

Cot’s `Project` trait defines the application-level setup. Configuration, app registration, and the middleware chain belong here. A project can combine a publishing app, an account app, and the admin interface.

## App boundaries

An app implements `App`. Its name identifies it within the project; its router defines the views it contributes. A useful boundary follows a feature, such as accounts, rather than splitting every model and handler into a separate app.

## Registration and prefixes

`register_with_views` mounts an app’s router under a prefix. The prefix controls public URLs; the app name identifies its routes when reversing URLs. Changing one does not inherently rename the other.

## Related reading

- [Routing](../../routing/).
- [Application lifecycle](../../guides/lifecycle/).
