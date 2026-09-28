---
title: Configuration
status: preview
---

Configuration describes the environment in which a project runs. The same application code may use a local database during development and a managed database in production; that difference should not require a different handler.

## Selecting a configuration

Cot’s default `Project::config` implementation calls `read_config` with the selected configuration name. The application CLI defaults to `dev`. A project can override this method, so its configuration sources are not necessarily the same as another project’s.

## Typed settings

`ProjectConfig` groups framework settings such as database, authentication, middleware, caching, and email configuration. The exact fields and feature requirements belong in Rustdoc; the guide explains how those groups affect the application.

## Secrets and environments

A production configuration needs a private secret key and environment-appropriate credentials. Configuration examples should identify where secrets come from without embedding working credentials in source control. Avoid treating a debug build and a production configuration as interchangeable concepts.

## Related reading

- [Configuration reference](../../reference/configuration/).
- [Production configuration](../../guides/production/).
