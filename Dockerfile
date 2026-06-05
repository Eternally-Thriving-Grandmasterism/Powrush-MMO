# Multi-stage Dockerfile for Powrush-MMO Server
# Sovereign, minimal footprint, production-ready for public deployment
# Matches Ra-Thor monorepo v14.11+ production standards
# Enables one-command docker compose up for humans to play + engage Ra-Thor lattice

FROM rust:1-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies
# - pkg-config, build-essential: for native crates (snappy, etc.)
# - libssl-dev: if any TLS features enabled later
# - libsnappy-dev: for snappy = "0.3" compression in server
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    build-essential \
    libssl-dev \
    libsnappy-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace manifests first (better layer caching)
COPY Cargo.toml ./
COPY server/Cargo.toml server/Cargo.toml
COPY shared/Cargo.toml shared/Cargo.toml
COPY powrush-divine-module/Cargo.toml powrush-divine-module/Cargo.toml
COPY client/Cargo.toml client/Cargo.toml 2>/dev/null || true

# Dummy sources to cache dependencies
RUN mkdir -p server/src shared/src powrush-divine-module/src && \
    echo 'fn main(){}' > server/src/main.rs && \
    echo 'pub fn _dummy(){}' > shared/src/lib.rs && \
    echo 'pub fn _dummy(){}' > powrush-divine-module/src/lib.rs

# Pre-build to cache deps (ignore errors if no lockfile)
RUN cargo build --release -p powrush-server 2>/dev/null || true

# Copy full source (now that deps cached)
COPY . .

# Final release build (optimized profile from workspace Cargo.toml)
RUN cargo build --release -p powrush-server

# ============================================
# Runtime stage - minimal Debian for sovereignty + small attack surface
FROM debian:bookworm-slim

WORKDIR /app

# Runtime dependencies only
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    libsnappy1v5 \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary
COPY --from=builder /app/target/release/powrush-server /usr/local/bin/powrush-server

# Expose the ports defined in docker-compose and DEPLOYMENT-SOVEREIGN.md
EXPOSE 7777/tcp 7778/tcp 8080/tcp

# Optional inline healthcheck (compose also defines one)
HEALTHCHECK --interval=30s --timeout=10s --start-period=15s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run as non-root for security (optional: add USER powrush)
# USER 1000

ENTRYPOINT ["powrush-server"]
CMD []
