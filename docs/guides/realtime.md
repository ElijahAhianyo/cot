---
title: Real-time communication
status: preview
---

A long-lived connection changes the relationship between a request and a response. Instead of returning one completed result, the server may send a stream of updates while the client stays connected.

## Choosing a transport

Server-sent events fit server-to-client updates. WebSockets allow bidirectional messages. A periodically refreshed HTTP response can still be sufficient when updates are infrequent.

## Connection lifecycle

Clients disconnect, reconnect, and miss messages. A useful protocol defines whether missed updates can be resumed and how authorization is rechecked during a long connection.

## Capacity and backpressure

Slow clients can accumulate pending data. A production design needs limits and a policy for dropping, buffering, or resynchronizing updates. This is target documentation coverage; no Cot transport API is implied.

## Related reading

- [Responses](../../guides/responses/).
- [Performance and scaling](../../guides/performance/).
