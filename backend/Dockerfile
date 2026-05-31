# ── Stage 1: Builder ─────────────────────────────────────────────────────────
FROM rust:latest AS builder

WORKDIR /app

# sqlx compile-time query checking requires either a live DATABASE_URL or the
# offline query cache (.sqlx/). Set SQLX_OFFLINE=true so Docker builds never
# need a database. Generate the cache once with: make sqlx-prepare
ENV SQLX_OFFLINE=true

# Cache dependency compilation separately from application code
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/project_dashboard_backend*

# Copy source and .sqlx/ query cache, then build
COPY . .
RUN cargo build --release

# ── Stage 2: Runtime ─────────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/project-dashboard-backend ./server
COPY --from=builder /app/migrations ./migrations

RUN useradd -ms /bin/bash appuser && chown -R appuser:appuser /app
USER appuser

EXPOSE 8080

CMD ["./server"]
