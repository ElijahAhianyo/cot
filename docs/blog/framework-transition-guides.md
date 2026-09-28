---
title: Why framework transition guides belong in the docs
status: preview
---

*Sample article · Documentation design · Not a release announcement*

A developer who has shipped a Django application doesn't need another explanation of what a URL is. They may need to know what a Cot app owns, how database errors reach a handler, and why the compiler rejects a reference that outlives its request.

An Axum developer starts elsewhere. They already know Rust's ownership rules, but may want to understand why Cot has a Project and an App when their existing service begins with a Router.

Those are different entry points into the same framework.

## Start with what the reader knows

A comparison table helps a reader find the right subject. It also has a limit: putting two concepts in the same row can suggest that their behavior is identical.

The transition pages make the difference explicit. A familiar term points to a Cot guide; the surrounding explanation names the assumptions that need checking. For an order endpoint, that might be a transaction boundary, a model callback, or the lifetime of shared state.

## One home for each feature

The routing guide should explain Cot's routing behavior. A Django transition page should explain how to approach that guide with Django experience. Duplicating the full routing explanation would leave two pages to update every time behavior changes.

This also helps readers move beyond their starting framework. Once the comparison has done its job, the main guide becomes the place to return to.

## Keep time-sensitive stories separate

A blog can describe a design decision at a particular moment. Documentation needs to stay aligned with the version the reader is using. A release story can link to a new feature, but it shouldn't become the only place that feature is explained.

That's the division explored in this preview: a blog for discussions and stories, and [transition guides](../../coming-from/overview/) for questions readers will keep bringing to Cot.
