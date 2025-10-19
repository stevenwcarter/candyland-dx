FROM rust:1-alpine as builder
RUN apk add --no-cache musl-dev curl
RUN cargo install cargo-chef
WORKDIR /app

FROM builder as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM builder as cook
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN curl -fsSL https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | sh
RUN cargo binstall dioxus-cli --root /.cargo -y --force
ENV PATH="/.cargo/bin:$PATH"
RUN dx bundle --platform web

# ---- Pure scratch runtime ----
FROM scratch as runtime
WORKDIR /usr/local/app
COPY --from=cook /app/target/x86_64-unknown-linux-musl/release/server ./
COPY --from=cook /app/target/dx/candyland-dx/release/web/ ./

ENV PORT=8080
EXPOSE 8080
ENTRYPOINT ["/usr/local/app/server"]
