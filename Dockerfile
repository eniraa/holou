FROM lukemathwalker/cargo-chef:latest-rust-slim AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:stable-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/holou /usr/local/bin
ENTRYPOINT ["/usr/local/bin/holou"]