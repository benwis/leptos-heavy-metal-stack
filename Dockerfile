FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef

WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Install OpenSSL - it is dynamically linked by some of our dependencies
RUN apt-get update -y \
    && apt-get install -y openssl ca-certificates \ 
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# Build our project dependencies, not our application! 
RUN cargo chef cook +nightly --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true

RUN cargo leptos build --release

ENV LEPTOS_ENV="PROD"

RUN echo $(ls -1a .)

# We do not need the Rust toolchain to run the binary!
FROM debian:buster-slim AS runtime
WORKDIR /app
ENTRYPOINT ["/app/leptos_heavy_metal_stack"]
