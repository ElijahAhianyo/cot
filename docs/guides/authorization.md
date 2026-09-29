---
title: Authorization
status: preview
---

Authorization decides whether an actor may perform an action on a resource. Being signed in is only one input to that decision. A customer can be authenticated and still have no right to cancel another customer's order.

We'll use order cancellation to develop the rule. The code below is ordinary application-owned Rust, not a proposed Cot policy macro. That keeps the decision visible and makes it usable from handlers, commands, and tests.

## A resource rule

Suppose a customer can cancel their own order before it ships:

```rust
struct OrderAccess {
    customer_id: i64,
    shipped: bool,
}

fn can_cancel(customer_id: i64, order: &OrderAccess) -> bool {
    customer_id == order.customer_id && !order.shipped
}

let order = OrderAccess { customer_id: 7, shipped: false };
assert!(can_cancel(7, &order));
assert!(!can_cancel(8, &order));
```

The caller must obtain `customer_id` from trusted authentication state and the order facts from authoritative storage. Passing values copied from the submitted form would let the client choose the answer.

## Checking every entry point

A hidden Cancel button improves the interface but doesn't enforce the rule. The mutation endpoint must check it too. So must a JSON endpoint or management operation that exposes the same action, unless that entry point deliberately has a different policy.

A shared policy function avoids duplicating subtly different conditions. It shouldn't depend on a template being rendered first or on the browser following a particular sequence of pages.

## Roles and ownership

Roles can express broad responsibilities, such as support staff or shop administrators. Resource ownership expresses a relationship to a particular row. Many rules need both, plus the current state of the resource.

Avoid treating “staff” as universal permission by accident. A support agent might view an order but not refund it. Name the action in the policy so tests can describe the intended permission precisely.

## Tenant boundaries

In an application serving multiple organizations, include the organization in resource selection and permission checks. A globally valid order ID doesn't imply that it belongs to the current organization.

This also applies to list endpoints, exports, caches, and background work. Protecting the detail page while leaving an unrestricted export endpoint still exposes the same records.

## Changes between checking and writing

The example checks an in-memory snapshot. In a real system, an order can ship between the permission check and cancellation. If “cannot cancel after shipping” is an invariant, enforce the relevant state transition atomically with the database operation.

Authorization and concurrency meet here: the user may be allowed in principle while the resource is no longer in the state required for the action. Return an appropriate conflict or refreshed state instead of pretending the earlier snapshot remains current.

## Denied responses

Choose a consistent policy for anonymous requests, authenticated-but-denied requests, and resources whose existence should not be disclosed. Browser flows may redirect to login; API flows need a documented status and error shape.

A denial should not include another customer's private order details. Operators can receive a useful audit record without exposing that data to the caller.

## Testing the boundary

For cancellation, test the owner before shipping, a different customer, an anonymous request, an already shipped order, and any explicitly privileged staff role. Add a concurrency test where the shipped state changes before the write.

Cot's authentication interfaces supply identity; application rules supply these resource decisions. See [authentication](../../guides/authentication/), [transactions](../../databases/transactions/), and [HTTP tests](../../guides/http-tests/) for the surrounding implementation.
