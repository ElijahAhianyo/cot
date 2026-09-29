# Documentation tutorial companion

Run commands from this directory. The Cot dependency points to the crate in this checkout; the documentation renderer and cot-site are not required.

```bash
cargo run --locked
cargo run --locked --bin quote_api -- --listen 127.0.0.1:8002
cargo run --locked --bin announcements -- --listen 127.0.0.1:8003
cargo test --locked
```

Run each server command in its own terminal. The default executable is the issue tracker at http://127.0.0.1:8000/. Its SQLite file is relative to the working directory. The quote API accepts POST at /quotes. The announcements example serves /support/.

This is local teaching code. The issue form is unauthenticated and has no CSRF protection; it is not a public deployment starter. Configuration is deliberately programmatic and uses development defaults. The API uses a fixed example price and does not create orders.

The issue-tracker tests create isolated temporary SQLite databases and run migrations explicitly before creating the in-process client. The other binaries test JSON contracts and mounting the app in two independent hosts. The tutorial chapters live under docs/tutorials/.
