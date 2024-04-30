FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR /go_calculator
LABEL authors="jakob753951"

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /go_calculator/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin go_calculator

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm AS runtime
WORKDIR /go_calculator
COPY --from=builder /go_calculator/target/release/go_calculator /usr/local/bin
ENTRYPOINT ["/usr/local/bin/go_calculator"]