---
title: Localization and time zones
status: preview
---

Language, formatting, and time zones are separate choices. A translated label does not determine a visitor’s preferred date format, and a stored timestamp does not determine the time zone in which it should be displayed.

## Locale selection

Applications need an explicit rule for choosing a locale: a user preference, a request value, or a default. Fallback behavior matters when a translation is missing.

## Messages and plural forms

Translations work best when they preserve complete messages and allow a language’s plural rules. Building a sentence by joining translated fragments can produce incorrect grammar.

## Time and schedules

An instant and a local calendar time answer different questions. A daily report scheduled for a local morning needs a policy for daylight-saving transitions. This prototype reserves space for Cot’s eventual localization integration.

## Related reading

- [Scheduled tasks](../../guides/scheduling/).
