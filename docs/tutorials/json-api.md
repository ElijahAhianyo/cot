---
title: Build a JSON API
status: preview
---

This tutorial preview follows an issue API from a small read endpoint to a tested contract. It assumes familiarity with handlers and database queries, but not with Cot's JSON or OpenAPI integrations.

## Read and create issues

Begin with a collection endpoint and a detail endpoint. Then accept a creation request with explicit fields. Keep public response values separate from fields that should remain internal to the database.

## Handle invalid requests

Exercise malformed data, validation errors, missing records, and denied operations. Each failure should have a predictable status and response shape.

## Add pagination and a schema

A larger collection needs stable ordering and a continuation contract. An OpenAPI description should agree with the implemented response and error shapes. Pagination here is target tutorial coverage, not an assertion of a built-in paginator.

## Verify the contract

The completed tutorial will finish with HTTP tests for successful and unsuccessful requests and a generated schema checked against those responses.

See [JSON APIs](../../guides/json-apis/), [OpenAPI](../../openapi/), and [HTTP tests](../../guides/http-tests/) for the related explanations.
