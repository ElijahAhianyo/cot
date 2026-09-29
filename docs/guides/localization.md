---
title: Localization and time zones
status: preview
---

Language, number formatting, and time zones are separate choices. A customer can read the shop in English, pay in euros, and receive a delivery in a time zone different from the server's.

We'll use a delivery appointment to make those distinctions concrete. “Tomorrow at 9” isn't a complete stored appointment: we need to know whose tomorrow and which local clock the customer meant.

## Language and locale

Language controls translated words. Locale also affects conventions such as decimal separators, date order, and plural forms. A locale should come from a deliberate application policy, such as an account preference with an explicit fallback.

A browser's language header is a useful hint, not a guarantee that the user wants every page in that language. Provide a way to choose, and keep that choice consistent across links, forms, and emails.

A complete translation-catalog service is part of the intended application design here. This page does not introduce a Cot translation macro or claim compatibility with another framework's language-file format.

## Messages with values

Keep a sentence together when translating it. Building a message by joining “You have ”, a number, and “ orders” assumes that other languages use the same word order and plural rules.

An illustrative message contract can have a stable key and named values:

```text
Message: orders.awaiting_collection
Values: count = 3, collection_point = "Central shop"
English rendering: "3 orders are ready at Central shop."
```

The key identifies the message; the rendering is language-specific. Treat inserted values as data and apply the escaping rules of the output format. A translated HTML string doesn't make an untrusted collection-point name safe.

## Dates, times, and instants

A calendar date, a local time, and an instant are different types of information. A birthday is a date. A recurring shop opening time is a local clock rule. A payment receipt timestamp identifies an instant.

For an appointment entered as a local datetime, retain the intended time-zone context when converting it. A numeric UTC offset alone doesn't capture future daylight-saving transitions for a named region.

Cot's forms include date and time field support; consult [forms](../../forms/) for the current field types and options. Don't assume that a plain local datetime is automatically interpreted in the customer's time zone.

## Ambiguous and nonexistent local times

When clocks move forward, some local times don't occur. When they move backward, a local time can occur twice. Silently choosing an interpretation can move an appointment by an hour.

A booking interface needs a policy: reject the ambiguous value with an explanation, ask the customer to choose, or use a documented rule that fits the business. Test both transitions for the time zones the application supports.

## Formatting and parsing

Display formatting is not a storage format. Store values in types and representations that preserve their meaning, then format them at the output boundary. An API should document its datetime format independently of the human-readable browser display.

Likewise, accepting a localized price requires explicit parsing. A comma may be a decimal separator or a grouping separator depending on the locale. Don't guess differently in the form, database import, and background job.

## Caches and scheduled work

If translated output is cached, the cache key needs the relevant language or locale. Otherwise the first visitor can determine the language shown to later visitors.

Scheduled work needs an explicit time-zone rule too. A daily delivery reminder at local 9 a.m. is not the same schedule as every 24 hours. See [scheduled tasks](../../guides/scheduling/) for handling that distinction and missed runs.
