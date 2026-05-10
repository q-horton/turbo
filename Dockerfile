FROM lukemathwalker/cargo-chef:latest-rust-1.95 AS chef
WORKDIR /turbo

# Checks the dependencies
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /turbo/recipe.json recipe.json

# Build dependencies - caching
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build --release --bin turbo

# We do not need the Rust toolchain to run the binary!
FROM debian:trixie-slim AS runtime
WORKDIR /turbo
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /turbo/target/release/turbo /usr/local/bin
ENTRYPOINT ["/usr/local/bin/turbo"]
