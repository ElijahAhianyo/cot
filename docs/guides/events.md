---
title: Events and notifications
status: preview
---

An event records that something happened in the application. A notification communicates information to a person or another system. The same event may lead to no notification, one notification, or several.

## Event boundaries

An issue-created event should describe an accepted domain change. Publishing it before the issue transaction commits can leave listeners responding to a change that never became durable.

## Listeners and coupling

An in-process listener runs within the application’s lifetime. A queued listener has different timing and failure behavior. Choosing one affects both latency and which failures can change the original request’s outcome.

## Notification delivery

Email and other notification channels need delivery tracking and recipient preferences. A future Cot integration should distinguish generating a message from delivering it successfully.

## Related reading

- [Sending email](../../sending-emails/).
- [Background tasks](../../guides/background-tasks/).
