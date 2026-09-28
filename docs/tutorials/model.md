---
title: Model an issue
status: preview
---

We'll give the tracker something to store: an issue with a title and a description. This chapter preview shows the learning sequence and checkpoints; the linked model and migration guides contain the current API examples.

## Define the first record

Start with the smallest useful issue: an identifier, a title, and a description. Leave comments and attachments out of this chapter so we can see the relationship between one model and one stored record.

The [model guide](../../databases/overview/) shows Cot's field declarations and primary-key conventions. Use those declarations for the issue rather than introducing a tutorial-specific data layer.

## Create the schema

Generate the migration for the model and inspect the resulting schema change before applying it. The [migration guide](../../databases/migrations/) describes the current command and migration workflow.

## Store and retrieve an issue

Create an issue titled “The sign-in button is hard to find,” then retrieve it by its identifier. [Queries](../../databases/queries/) provides the insertion and lookup examples this chapter builds on.

## Checkpoint

The record should still exist after the application restarts. An in-memory list alone does not meet this checkpoint. The complete chapter will include the exact model, generated migration, and executable assertions.
