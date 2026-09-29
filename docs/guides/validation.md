---
title: Validation
status: preview
---

Validation decides whether input is acceptable for an operation. Parsing is part of that work, but it isn't the whole decision. The string `"4"` can become an integer; whether a customer may order four notebooks depends on the shop's rules and available stock.

We'll use a product-order form to separate these decisions. The form contains a product ID, a quantity, and an optional delivery note. The authenticated customer comes from the request context, not from a hidden form field.

## Field-level rules

Cot's form types describe the fields a form accepts. Field options can express constraints close to the field definition:

```rust
use cot::form::Form;

#[derive(Form)]
struct DeliveryNoteForm {
    #[form(opts(max_length = 200))]
    note: String,
}
```

The maximum length communicates an input constraint. It doesn't establish that the note is safe to insert into raw HTML. Rendering has its own escaping rules, just as SQL queries have their own parameter-binding rules.

For the complete form definition and error-context APIs, see [forms](../../forms/). Keep the Rust type and the user-facing error message in agreement: a message asking for an integer should not accompany a field that accepts a fractional quantity.

## Parsing, validation, and business rules

We can think about the order input in three stages:

| Input | Parsing question | Business question |
| --- | --- | --- |
| Product ID | Is this a valid identifier representation? | Is the product available to this customer? |
| Quantity | Can it become the expected numeric type? | Is it positive, within the order limit, and in stock? |
| Delivery note | Is this valid text of an acceptable size? | Is a note allowed for this delivery method? |

A failed parse should not reach code that assumes a valid quantity. A successful parse should not skip the later checks. This separation also makes errors easier to explain: “Enter a whole number” and “Only three notebooks remain” ask the customer to make different corrections.

## Optional fields and empty values

An omitted field, an empty string, and a string containing spaces are distinct representations. Decide which ones your operation treats as absent. Trimming a delivery note may be reasonable; silently trimming a password changes the secret the user entered.

For partial updates, absence can mean “leave the current value unchanged,” while an explicit empty value can mean “clear it.” A type used for creating a record may not express that distinction adequately for updating one.

## Validation errors in a form

`RequestForm` provides a `FormResult`, including the invalid form context. On an ordinary input error, preserve safe entered values and show the messages next to the relevant fields. Don't panic or discard the whole form because one field failed.

Password and token fields need different treatment from a delivery note: don't echo their contents back into the page or logs. An error response should help correct the input without exposing the submitted secret.

## Rules involving the database

A uniqueness check before insertion can improve feedback, but concurrent requests can both pass it. A database constraint remains necessary for a uniqueness invariant. Translate its failure into the application's response policy instead of assuming the earlier check made failure impossible.

Stock validation has the same timing issue. Reading “three remain” and later writing an order is not enough when another customer can purchase between those operations. The rule needs an appropriate transaction or conditional write.

## Validation is not authorization

A perfectly valid order ID can belong to someone else. Enforce resource access using the authenticated actor, even when the input came from a form your own application rendered. Browser controls and hidden fields are still client-controlled data.

See [authorization](../../guides/authorization/) for resource rules and [HTTP tests](../../guides/http-tests/) for testing invalid input without losing the distinction between a client error and a server failure.
