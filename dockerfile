# ── Planner ──────────────────────────────────────────────────────
FROM lukemathwalker/cargo-chef:latest-rust-slim-bookworm AS planner
WORKDIR /app

# Copy root manifests
COPY Cargo.toml Cargo.lock ./

# Copy workspace member manifests only (not full source)
COPY crates/core/Cargo.toml crates/core/Cargo.toml
COPY crates/core/src/lib.rs crates/core/src/lib.rs

COPY crates/chains/evm/Cargo.toml crates/chains/evm/Cargo.toml
COPY crates/chains/evm/src/lib.rs crates/chains/evm/src/lib.rs

COPY crates/operator/Cargo.toml crates/operator/Cargo.toml
COPY crates/operator/src/main.rs crates/operator/src/main.rs

COPY crates/api/Cargo.toml crates/api/Cargo.toml
COPY crates/api/src/lib.rs crates/api/src/lib.rs

RUN cargo chef prepare --recipe-path recipe.json

# ── Builder ──────────────────────────────────────────────────────
FROM lukemathwalker/cargo-chef:latest-rust-slim-bookworm AS builder
WORKDIR /app
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/* && apt-get clean

COPY --from=planner /app/recipe.json ./
RUN cargo chef cook --release --recipe-path recipe.json

# Copy full source tree only now
COPY . .

# Build your operator binary
RUN cargo build --release --bin paradapp-operator
RUN strip target/release/paradapp-operator

# ── Runtime ──────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* && apt-get clean

COPY --from=builder /app/target/release/paradapp-operator /usr/local/bin/app

ENV RUST_LOG=info
ENV REDIS_URL=redis://redis:6379

EXPOSE 8080

ENTRYPOINT ["app"]