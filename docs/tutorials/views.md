---
title: Display issues
status: preview
---

We'll make the saved issue visible in a browser. One page lists issues, and another displays an issue selected by its identifier. This chapter preview keeps the persistence work from the previous chapter separate from the HTTP behavior.

## Give each page a route

The list belongs at `/issues/`; an individual issue belongs at `/issues/{id}`. The [routing guide](../../routing/) explains how a path parameter becomes a handler argument and what happens when it cannot be parsed.

## Render the result

A handler retrieves the relevant data and passes it to a template. The list template links to each issue through its route name. The [template guide](../../templates/) covers context, rendering, and named links.

## Handle a missing issue

An identifier can be valid even when the record doesn't exist. The completed chapter will show both a successful lookup and a missing-record response, so a broken link doesn't become an unexplained server error.

## Checkpoint

Opening an issue from the list should display the same title stored in the database. Changing the ID to a nonexistent value should produce an appropriate not-found response. A nonnumeric ID exercises a different failure: path extraction.
