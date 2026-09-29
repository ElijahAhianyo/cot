---
title: Rate limiting
status: proposed
---

Rate limiting bounds how often an actor can perform an operation. It can protect a password-recovery endpoint from repeated email requests or keep one customer from using all the capacity of an export service.

This is the proposed Cot rate-limiting design. The examples describe policies and observable behavior; they are not calls to an existing limiter API.

## What is being limited?

A useful policy names the action, the identity, the allowance, and the time window. “Limit requests” leaves too much unspecified. A product lookup and a password-recovery email have different costs and abuse patterns.

For illustration, a shop might define:

| Action | Identity | Example allowance |
| --- | --- | --- |
| Request account recovery | Normalized account identifier plus a separate network safeguard | 3 requests in 15 minutes |
| Start an export | Authenticated customer | 2 starts in 10 minutes |
| Browse the public catalog | A deployment-defined client identity | A larger short-term allowance |

These numbers are examples, not recommended defaults. Choose them from the cost of the operation, expected usage, and the consequences of incorrectly rejecting a legitimate customer.

## Choosing an identity

An authenticated account is often a better identity than an IP address for account-specific work. Several people can share an address, and one actor can use many addresses. Before trusting a forwarded client-IP header, confirm which proxy is allowed to supply it.

Include the action and relevant tenant in the key. Otherwise requesting an export could consume the allowance intended for login, or one organization could affect another's quota.

## Windows and bursts

A fixed window can allow a burst near its boundary: several requests just before reset and several just after. A rolling window or token-bucket policy smooths traffic differently. The algorithm should match the intended behavior, especially for expensive operations.

A concurrency limit answers another question: how many exports may run at once? An hourly quota alone doesn't stop all allowed exports from starting simultaneously. Some operations need both controls.

## Shared state and atomic decisions

If the application runs on multiple instances, they need a coordinated view of the allowance. Separate in-memory counters would multiply the effective limit as instances are added.

The decision to check and consume capacity must be atomic in the chosen backend. A read followed by an unrelated write can allow concurrent requests to pass the same remaining slot. The cache API's existence alone doesn't establish that it provides the atomic operation a limiter needs.

## Rejected requests

A rejected HTTP request should use a documented response, commonly 429, and provide useful retry information when it can be calculated accurately. Avoid encouraging every client to retry at precisely the same instant; clients may need bounded backoff and jitter.

Account-recovery responses must also preserve the application's account-disclosure policy. Different limit messages should not inadvertently reveal whether a username exists.

## Backend failure

Decide whether a limiter outage rejects the operation or permits it. Failing open preserves availability but loses protection. Failing closed protects the resource but can block legitimate use. A public product page and a costly email operation may reasonably choose different policies.

Record allowed, rejected, and backend-failure outcomes separately. Test window boundaries, concurrent requests, key separation, and clock assumptions. Until native support exists, any gateway or application-owned limiter must be configured and tested as an explicit integration.
