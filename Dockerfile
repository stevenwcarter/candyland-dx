FROM rust:1-alpine AS builder
RUN apk add --no-cache musl-dev curl
RUN cargo install cargo-chef
WORKDIR /app

FROM builder AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM builder AS cook
RUN apk add --no-cache musl-dev curl openssl-libs-static pkgconf openssl-dev bash
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --root /.cargo -y --force --locked
ENV PATH="/.cargo/bin:$PATH"
RUN dx bundle --platform web -r

FROM nginx:1.29.2 AS runtime
WORKDIR /var/www
EXPOSE 80
COPY nginx.conf /etc/nginx/nginx.conf
COPY --from=cook /app/target/dx/candyland-dx/release/web/public/ ./

CMD ["nginx"]
