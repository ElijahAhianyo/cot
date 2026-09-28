---
title: Model relationships
status: preview
---

Relationships describe how records refer to each other. An issue can belong to a project, and a comment can belong to an issue. Those references have consequences for loading data, preserving integrity, and deleting records.

## References and integrity

A foreign key identifies a related record. Database constraints and the Rust representation work together: a typed field does not by itself describe every rule the database will enforce.

## Loading related records

A page that lists fifty issues may also display each author. Loading authors one at a time can multiply queries. The guide should distinguish supported loading behavior from assumptions imported from another ORM.

## Deletion and ownership

Deleting a parent can remove children, prevent deletion, or leave another representation, depending on the schema. The application’s ownership model and the database’s deletion policy should agree.

## Related reading

- [Models and fields](../../databases/overview/).
- [Queries](../../databases/queries/).
