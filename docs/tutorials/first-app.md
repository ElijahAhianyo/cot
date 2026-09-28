---
title: Your first Cot application
status: preview
---

In this tutorial, we'll build a small issue tracker. An issue has a title, a description, and an owner. We can list issues, create one through a form, and restrict changes to the person who owns it.

This is a preview of the tutorial's intended chapters. The existing [introduction](../../introduction/) remains available for complete setup and view examples.

## Before you begin

You need a Rust toolchain, the Cot CLI, and a generated project. [Installation](../../installation/) covers that setup. The tutorial uses one local database and a single application process to keep the early choices limited.

## Create the project

```bash
cot new issue_tracker
cd issue_tracker
cargo run
```

Open `http://localhost:8000`. The generated welcome page is our first checkpoint: the project builds and handles a request.

## What we'll build

| Chapter | Working checkpoint |
| --- | --- |
| [Model an issue](../model/) | An issue can be stored and retrieved. |
| [Display issues](../views/) | The browser shows a list and an individual issue. |
| [Accept changes](../forms-auth/) | Invalid input is explained and edit access is enforced. |
| [Test and prepare a release](../test-deploy/) | Core behavior has tests and the release build is understood. |

## Explore without following the tutorial

If you're looking up a feature, the [guides](../../guides/overview/) explain it independently. You don't need to complete this application to understand routing, forms, or queries.
