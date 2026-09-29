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

## Explain one consequential difference

Consider the phrase “save an order.” In an existing application, saving might trigger callbacks, produce an event, and schedule an email. A comparison that only shows the new insert syntax misses the behavior the developer actually needs to preserve.

A useful transition guide asks where those responsibilities move. It explains the transaction boundary, points to the query API, and identifies whether the delivery mechanism exists. The example stays small, but its consequences remain real.

That approach also helps newcomers. We can introduce a transaction through the problem it solves—avoiding half an order—then give experienced readers the details about concurrency and external effects. The beginner and expert do not need entirely separate copies of the concept.

## Give comparisons an exit

A transition page succeeds when the reader can continue in the main documentation without translating every sentence back to their old framework. Familiar concepts provide orientation; Cot's own examples establish the working vocabulary.

That is why these pages link into requests, queries, authentication, and testing instead of becoming six parallel manuals. The feature guide remains the place where edge cases and changing API behavior are maintained.

## Keep proposals visible

A mature documentation structure can show where queues, scheduling, and rate limiting would fit before every integration exists. It becomes misleading if the navigation makes a design proposal look like a shipped interface.

In this preview, Proposed labels stay visible in the page and navigation. The prose can still examine a complete use case: durable acceptance, retries, duplicate work, and operator recovery. What it cannot do is turn an illustrative contract into an API claim without implementation evidence.

## Connect the story to maintained material

A post can explain why the documentation made these choices. The maintained [transition overview](../../coming-from/overview/) helps a reader act on them, and the [guide directory](../../guides/overview/) leads to the feature explanations. Readers who arrive months later should be able to find the current behavior without reconstructing it from the publication history.
