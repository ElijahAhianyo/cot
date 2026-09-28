---
title: Scheduled tasks
status: preview
---

Scheduling determines when work becomes due. A queue determines how accepted work is executed. Keeping those responsibilities separate helps explain missed schedules, overlapping runs, and retries.

## Time and calendars

A daily report at 09:00 needs a time zone. Daylight-saving changes can make a local time occur twice or not at all. A scheduler’s behavior at those boundaries belongs in its contract.

## Overlap and coordination

A task that takes longer than its interval can overlap its next run. Multiple scheduler instances can also enqueue duplicates. A mature design needs explicit overlap and coordination policies.

## Missed runs

After an outage, the system may catch up, run once, or skip missed intervals. Each choice has different effects on reports, billing, and cleanup jobs. This page is a design preview, not a built-in scheduler configuration guide.

## Related reading

- [Background tasks](../../guides/background-tasks/).
