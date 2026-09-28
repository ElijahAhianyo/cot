---
title: Application lifecycle
status: preview
---

A request passes through several parts of an application before a response reaches the client. Understanding those boundaries helps explain where configuration belongs, when middleware runs, and why a handler may never be called.

## Startup and application state

Project configuration and app registration establish the application before requests are handled. Resources such as database pools belong to the application lifetime, while a parsed form belongs to one request. Cot exposes initialization through its project and app interfaces.

## From a request to a handler

Middleware surrounds request handling. The router selects a handler, extractors prepare its arguments, and the handler performs application work. A rejected method or failed extraction can return an error before the handler body runs.

## From a handler to a response

Handler results are converted to responses. Middleware can inspect or change a response on its way back out. An error handler determines how an application error is presented to the client.

## Shutdown and unfinished work

A production server needs a policy for requests that are still running when it stops. Durable background work has a different lifetime from the HTTP request that initiated it; losing a request must not silently imply that durable work completed.

## Related reading

- [Middleware](../../guides/middleware/).
- [Handlers and responses](../../guides/responses/).
- [Background tasks](../../guides/background-tasks/).
