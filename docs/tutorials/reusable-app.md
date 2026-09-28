---
title: Build a reusable Cot app
status: preview
---

This tutorial preview turns an application feature into a component another project can register. The example is a small announcements app with its own routes and configuration.

## Establish the boundary

Decide which values the host project supplies and which behaviors belong to the app. Keep project-specific paths and credentials out of the app's implementation.

## Register it in another project

A second project provides a useful test of the boundary. Mounting the app under a different URL prefix should reveal hard-coded links and assumptions about its name.

## Test and document the public interface

The complete tutorial will include two small host projects, integration tests, and documentation of the app's public configuration and feature requirements. [Reusable apps](../../guides/reusable-apps/) explains the design considerations.
