# Build from the Cot repository root, not docs/.
FROM rust:1.97-trixie AS builder

# Pin the renderer so changes on its branch cannot silently change this preview.
ARG COT_SITE_REV=3823c4be8bfaad5a9f3e42763d9dd7ec095b1078
WORKDIR /build/cot-site
RUN git init . \
    && git remote add origin https://github.com/ElijahAhianyo/cot-site.git \
    && git fetch --depth 1 origin "$COT_SITE_REV" \
    && git checkout --detach FETCH_HEAD \
    && git submodule update --init --depth 1 bootstrap \
        syntax-highlighting/Packages syntax-highlighting/Packages-new \
        syntax-highlighting/sublime-jinja2 syntax-highlighting/sublime_toml_highlighting

WORKDIR /build/cot
COPY . .
WORKDIR /build/cot/docs/site
RUN cargo build --release --locked --jobs 2

FROM debian:trixie-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3t64 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --create-home --uid 10001 preview
COPY --from=builder /build/cot/docs/site/target/release/docs-site /usr/local/bin/docs-site
WORKDIR /home/preview
USER preview
ENV PORT=10000
EXPOSE 10000
CMD ["/bin/sh", "-c", "exec /usr/local/bin/docs-site --listen 0.0.0.0:${PORT:-10000}"]
