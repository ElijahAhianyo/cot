---
title: Rate limiting
status: proposed
---

A rate limit bounds how much work a caller can request within a period. Login attempts, expensive searches, and bulk exports need different limits because their costs and failure consequences differ.

## Choosing the identity

A key might identify a user, API credential, or IP address. Shared networks make an IP address an imperfect proxy for a person; proxy headers also need a trusted source.

## Windows and bursts

A fixed window is simple but allows a burst across the window boundary. A token bucket can allow short bursts while bounding sustained traffic. The choice should follow the operation’s cost and the intended experience.

## Multiple application instances

Independent in-memory counters can multiply an effective limit when requests reach different instances. A distributed limit needs shared coordination and a policy for store failures.

## Rejection contract

For an illustrative limit of five login attempts per minute, the sixth attempt is rejected until capacity returns. A mature interface should define 429 responses, retry information, and whether unsuccessful attempts consume capacity. No Cot limiter API is proposed here.

## Related reading

- [Authentication](../../guides/authentication/).
- [Caching](../../caching/).
