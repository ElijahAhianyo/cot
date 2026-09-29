---
title: Production configuration
status: preview
---

Production configuration turns an application build into a particular running service. Our shop's release binary may be identical in staging and production, while the database, secret key, mail transport, and public URL differ.

A useful production review explains what each value changes. Copying a development file and renaming it does not establish that the resulting behavior is appropriate for real users.

## Configuration selection

The application CLI selects a configuration, and the Project implementation decides how to load it. Inspect that implementation before relying on an environment variable or file path. A project that constructs `ProjectConfig` in code may not read the file you edited.

Use the application's help to confirm supported options:

```bash
cargo run -- --help
```

For deployment, run the built executable rather than requiring Cargo on the serving machine. The exact executable name comes from the application package.

## Debug behavior

Disable detailed diagnostic output for public requests. Production errors should provide a safe message and a way to correlate the failure with internal logs, not source snippets or configuration values.

The build profile and runtime configuration are separate controls. A release build optimizes code; review the actual `debug` setting too, especially when a Project overrides defaults.

## Secret keys

Use an independently generated, private key for the deployment. Keep it stable across restarts and compatible across replicas when existing sessions must remain valid. A key created afresh in every process can produce intermittent authentication failures.

Rotation requires a transition plan. Cot's configuration includes fallback secret keys, but retaining an old key indefinitely defeats the purpose of eventually retiring it. Define the overlap period around session lifetime and the reason for rotation.

## Cookies and proxies

Review Secure, HttpOnly, SameSite, path, and domain behavior for the actual public deployment. Test through HTTPS and the real proxy rather than only calling the application port directly.

When a cookie is missing, distinguish “the server did not set it” from “the browser rejected it” and “the browser did not send it to this path.” Those observations point to different settings.

## Service configuration

| Resource | Production question |
| --- | --- |
| Database | Which backend, credentials, connection limit, and migration state? |
| Sessions | Can every replica recover the same session state? |
| Cache | What is shared, how does it expire, and what happens on failure? |
| Email | Which sender and transport are authorized for this environment? |
| Media | Where are bytes stored durably, and who can retrieve them? |
| Logs | Where are diagnostics collected, and which fields are excluded? |

Enabling a backend in configuration requires its compiled Cargo feature too. A URL cannot add a driver that isn't in the binary.

## Startup validation

Fail startup for missing required configuration with an actionable message. Do not log the secret itself to prove that it was loaded. Optional services can have a documented degraded mode, but a required database should not silently switch to an empty local database.

Run an integration check using the intended configuration before directing user traffic to the instance. Confirm that it reaches the correct resources; successful authentication to the wrong database is still a deployment failure.

## Changing settings safely

A cache backend change, key rotation, or email-provider change can alter behavior even without code changes. Record the change, verify it with a bounded test, and know what reverting means for already-created data or messages.

The [configuration reference](../../reference/configuration/) links exact fields. [Deployment](../../guides/deployment/) explains the runtime layout, and [recovery](../../guides/recovery/) covers reversibility.
