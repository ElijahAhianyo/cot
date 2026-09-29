---
title: Real-time communication
status: preview
---

Real-time communication keeps a client informed while something changes on the server. An order-tracking page can update when a parcel is dispatched instead of waiting for the customer to refresh.

This guide describes an application integration design. It does not introduce a built-in Cot broadcasting service or channel API. The important contracts remain the same whether the transport is polling, server-sent events, or a WebSocket integration.

## Choosing a transport

| Transport | Useful when | Cost to consider |
| --- | --- | --- |
| Polling | Updates are infrequent and a short delay is acceptable | Repeated requests even when nothing changes |
| Server-sent events | The server sends a stream of updates to the browser | Long-lived connections and reconnection behavior |
| WebSockets | Both sides need ongoing messages | Connection lifecycle, message protocol, and backpressure |

A transport is not a durability guarantee. A connection can drop after the server sends an event but before the browser records it.

## State and notifications

For order tracking, the database holds the current order state. A message tells the browser that the state changed. Keeping an authoritative read endpoint lets a reconnecting client recover without requiring every message to arrive exactly once.

An illustrative message can contain an order ID, a revision, and a state. The revision helps a client avoid replacing a newer update with an older one. Its scope must be clear: a per-order revision is not a global sequence across all orders.

## Subscribing is an authorization decision

Knowing an order's channel name must not grant access. Authenticate the connection and authorize the specific subscription. A customer who can see order 42 should not be able to subscribe to every order by changing an identifier.

Permissions can change while a connection is open. Decide whether to recheck on each subscription, message, or relevant account event. An authenticated connection established yesterday is not proof of current access.

## Reconnecting

A reconnecting client needs a defined recovery strategy. It can fetch current state, resume from an acknowledged sequence when the service supports it, or explicitly report that some intermediate history is unavailable.

For our tracking page, fetching the current order after reconnecting is often sufficient. An audit viewer may need every historical transition and therefore a durable event log rather than a transient stream.

## Slow clients and bounded buffers

If the server produces messages faster than a client reads them, an unbounded queue grows without limit. Bound per-connection buffers and decide whether to coalesce replaceable state updates, disconnect the client, or use another recovery mechanism.

Dropping an intermediate cursor-position update may be acceptable; dropping an unacknowledged financial instruction is a different problem. Design the protocol around the information being carried.

## Deployments and multiple instances

A message held only in one process is not automatically visible to connections on another instance. A multi-instance deployment needs a distribution mechanism appropriate to the transport and consistency requirements.

During a release, clients should reconnect with a bounded backoff rather than all retrying continuously. Keep message versions compatible while old clients remain connected.

## Testing the lifecycle

Test initial connection, denied subscription, duplicate updates, out-of-order updates, reconnection, and server shutdown. Measure open connections and buffered data, not just request counts.

See [events](../../guides/events/) for event meaning, [authorization](../../guides/authorization/) for resource access, and [deployment](../../guides/deployment/) for the process boundary.
