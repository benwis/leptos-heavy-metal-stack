FROM rustlang/rust:nightly AS chef
# We only pay the installation cost once, 
# it will be cached from the second build onwards
RUN cargo install cargo-chef 
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo +nightly chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Install OpenSSL - it is dynamically linked by some of our dependencies, and the protobuf compiler
RUN apt-get update -y \
    && apt-get install -y openssl ca-certificates libssl-dev  \ 
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# Build our project dependencies, not our application! 
RUN cargo +nightly chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true
RUN cargo +nightly install cargo-leptos --branch beta --git https://github.com/akesson/cargo-leptos cargo-leptos
RUN cargo +nightly leptos build --release

ENV LEPTOS_ENV=PROD

# RUN echo $(ls -1a .)

ENTRYPOINT ["/app/target/release/leptos_heavy_metal_stack"]
# May need to run or include cargo sqlx prepare --merged before running this to update the sqlx
