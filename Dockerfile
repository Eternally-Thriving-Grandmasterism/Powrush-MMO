# Multi-stage Dockerfile for Powrush-MMO Server (v18.10+)
# Sovereign, minimal footprint, production-ready for public + free-tier testing deployment
# Matches Ra-Thor monorepo + PATSAGi Council standards
# Enables one-command `docker compose up` for testing the MMOARPG on free hosting (Oracle Cloud Always Free recommended)

FROM rust:1-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies (optimized for Bevy/Tokio server + compression)
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    build-essential \
    libssl-dev \
    libsnappy-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace manifests first for excellent layer caching
COPY Cargo.toml ./
COPY server/Cargo.toml server/Cargo.toml
COPY shared/Cargo.toml shared/Cargo.toml
COPY powrush-divine-module/Cargo.toml powrush-divine-module/Cargo.toml
COPY client/Cargo.toml client/Cargo.toml 2>/dev/null || true

# Dummy sources to cache dependencies (prevents full rebuilds)
RUN mkdir -p server/src shared/src powrush-divine-module/src && \
    echo 'fn main(){}' > server/src/main.rs && \
    echo 'pub fn _dummy(){}' > shared/src/lib.rs && \
    echo 'pub fn _dummy(){}' > powrush-divine-module/src/lib.rs

# Pre-build to cache deps (use correct package name)
RUN cargo build --release -p powrush-mmo-server 2>/dev/null || true

# Copy full source
COPY . .

# Final optimized release build
RUN cargo build --release -p powrush-mmo-server

# ============================================
# Runtime stage - minimal Debian for sovereignty + tiny attack surface
FROM debian:bookworm-slim

WORKDIR /app

# Runtime deps only
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    libsnappy1v5 \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary (name matches package)
COPY --from=builder /app/target/release/powrush-mmo-server /usr/local/bin/powrush-mmo-server

# Expose ports (TCP game, WS, HTTP/health/metrics)
EXPOSE 7777/tcp 7778/tcp 8080/tcp

# Healthcheck for orchestrators and free hosting monitors
HEALTHCHECK --interval=30s --timeout=10s --start-period=20s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Non-root user (uncomment for extra security on shared free hosts)
# RUN useradd -m -u 1000 powrush && chown -R powrush:powrush /app
# USER powrush

ENTRYPOINT ["powrush-mmo-server"]
CMD []

# Sovereign labels
LABEL org.opencontainers.image.title="Powrush-MMO Server"
LABEL org.opencontainers.image.description="Sovereign Resource-Based Economy MMOARPG Server - Mercy-gated, Ra-Thor governed"
LABEL org.opencontainers.image.version="18.10"
LABEL org.opencontainers.image.vendor="Autonomicity Games"
LABEL org.opencontainers.image.source="https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO"
