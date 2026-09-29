---
title: Scheduled tasks
status: proposed
---

Scheduling decides when work becomes due. Execution decides how that work runs. A daily order summary can be triggered by an external scheduler and executed by an application command, or handed to a durable queue when one is available.

The integrated scheduler described here is a proposed design. Cot's existing CLI task interface can host application-owned commands, but this page does not claim a built-in cron parser, distributed scheduler, or scheduler command.

## A schedule describes intent

For our shop, a daily summary might have this contract:

```text
Name: daily-order-summary
Due: 09:00 in the shop's configured time zone
Scope: one summary per shop and business date
Work: calculate the previous business day's totals
Repeat identity: shop ID + business date + summary version
```

The repeat identity matters more than the spelling of the schedule expression. If the scheduler triggers twice, the application should still recognize one intended summary for that date.

## Calendar time and elapsed time

“Every 24 hours” measures elapsed duration. “Every day at 9 a.m.” follows a calendar in a time zone. They can diverge around daylight-saving changes.

Use an explicit named time zone for calendar rules. Define what happens when a scheduled local time is skipped or repeated. A business report may run once for the business date rather than once for every clock occurrence.

The [localization guide](../../guides/localization/) explains why a local datetime and an instant are different values.

## One scheduler and multiple instances

If every web replica runs the same schedule independently, adding replicas can duplicate work. A deployment needs either a single scheduling authority or a shared coordination mechanism with well-defined failure behavior.

A distributed lock needs an expiry and ownership policy. A lock that expires while the job is still running can admit another executor. A lock that never expires can stop the schedule forever after a crash.

## Overlapping runs

A task scheduled every minute may take three minutes during an outage. Decide whether a new run is skipped, delayed, merged with the active run, or executed concurrently. The correct choice depends on the operation.

Refreshing a cache may tolerate skipping an intermediate run. Billing or settlement often needs a durable record of every business period. A single “last run succeeded” flag doesn't capture both use cases.

## Missed runs and catch-up

After an hour of downtime, a scheduler can replay every missed interval, run only the latest interval, or require manual recovery. Document the policy. Automatically replaying a large backlog can overload the database precisely when it is recovering.

For the order summary, an application can enumerate missing business dates and process them in bounded batches. The task identity prevents a catch-up run from duplicating a summary already completed.

## Testing schedules

Separate the calendar calculation from the work itself. Test due-time calculations with a controlled clock, and test the summary function with a specific business date. Avoid tests that wait until the next real minute or assume the developer's time zone.

Monitor lateness as well as failures. A task that never starts has no failed execution to count. Useful observations include its expected due time, actual start, completion, and whether it was skipped or replayed.

See [management commands](../../guides/management-commands/) for today's command integration and [background tasks](../../guides/background-tasks/) for the intended durable execution boundary.
