# syntax=docker/dockerfile:1

# ---- builder base: Rust + wasm target + cargo-chef + Trunk ----
FROM rust:1-slim AS chef
RUN apt-get update \
 && apt-get install -y --no-install-recommends curl ca-certificates \
 && rm -rf /var/lib/apt/lists/*
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-chef --locked
# Pin Trunk; it fetches matching wasm-bindgen and the Tailwind v4 CLI at build time.
ARG TRUNK_VERSION=0.21.14
RUN curl -sSfL "https://github.com/trunk-rs/trunk/releases/download/v${TRUNK_VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz" \
    | tar -xzf - -C /usr/local/bin
WORKDIR /app

# ---- plan dependency graph for caching ----
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ---- build: cook deps (cached), then bundle with Trunk ----
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target wasm32-unknown-unknown --recipe-path recipe.json
COPY . .
RUN trunk build --release

# ---- runtime: static-web-server on scratch (~5MB) ----
FROM ghcr.io/static-web-server/static-web-server:2 AS runtime
ENV SERVER_ROOT=/public \
    SERVER_PORT=80 \
    SERVER_COMPRESSION=true
COPY --from=builder /app/dist /public
EXPOSE 80
