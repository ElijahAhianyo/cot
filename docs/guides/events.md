---
title: Events and notifications
status: proposed
---

An event records that something happened in the application. “Order placed” is a fact; “send a confirmation email” is a request to do work. Keeping those ideas separate helps us add reactions without hiding the operation that must succeed first.

The event-dispatch and notification system described here is a proposed design. Ordinary Rust calls remain appropriate when one component needs another to perform a required step.

## Facts and commands

When an order is committed, several components may care: email delivery, analytics, and fulfillment. An event can give each listener the relevant fact without making the order handler implement every reaction.

An illustrative event looks like this:

```text
Event ID: order-placed-42
Type: order.placed
Version: 1
Order ID: 42
Customer ID: 7
Occurred at: 2026-09-01T10:15:00Z
```

Include enough information to identify the event and interpret its schema. Avoid copying an entire user or payment object into the payload when listeners only need IDs. Event storage and logs may have a different retention policy from the source database.

## Synchronous listeners

A synchronous listener runs as part of the current operation. Its latency and failures affect that operation. If email delivery is synchronous, a slow provider can make placing an order slow.

Decide whether listener failure should abort the operation. That choice should be visible in the application contract, not an accidental consequence of which listener happened to run first.

For a required calculation, an explicit function call can be clearer than an event. A hidden listener that computes the order total makes it harder to see whether every code path establishes the invariant.

## Queued listeners

A queued listener runs later and needs durable delivery if losing its work is unacceptable. It can receive the same event more than once, so a listener that sends email or updates an external system needs a repeat-safe design.

An event name doesn't supply durability by itself. A process can crash between committing the order and publishing the event. The outbox pattern discussed in [background tasks](../../guides/background-tasks/) addresses that boundary.

## Ordering and stale facts

An order can be placed, amended, and canceled before a slow listener handles the first event. Decide whether the listener acts on the event's historical facts or loads the current order and checks its state.

If ordering matters, define its scope. Ordering per order is different from a global order across the entire shop. Distributed queues and multiple workers do not automatically preserve the business order you intended.

## Notifications and preferences

A notification is a message for a person through a channel such as email or an in-app inbox. The event is one possible trigger. Recipient preferences, consent, channel availability, and sensitive content still need to be evaluated.

For example, a customer may opt out of promotional email without opting out of an essential order receipt. Store and apply those categories deliberately rather than using one broad “email enabled” flag for every message.

## Testing reactions

Test that the business operation records the intended event only after the appropriate state change. Test each listener separately with repeated, outdated, and unsupported-version events. Finally, test at least one real delivery path through the chosen infrastructure.

A test that only verifies “an event was dispatched” does not show that the customer received a correct notification. See [sending email](../../sending-emails/) for the current transport interface and [HTTP integrations](../../guides/http-webhooks/) for delivery outside the application.
