---
name: docker-fullstack
description: Docker and Docker Compose patterns for this project. Use when modifying Dockerfiles, docker-compose.yml, or diagnosing build/startup issues.
trigger: /docker-fullstack
---

# /docker-fullstack

Docker patterns for this Rust + Vue.js monorepo.

## Rust multi-stage Dockerfile — dependency layer caching

The key trick: compile a dummy `main.rs` first so all dependencies are cached in one layer. When only `src/` changes, Docker reuses that layer and only recompiles application code (~10-30s vs ~3-5 min).

```dockerfile
FROM rust:latest AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/project_dashboard_backend*
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/project-dashboard-backend ./server
COPY --from=builder /app/migrations ./migrations
```

**Use `debian:bookworm-slim`, not Alpine** — SQLx with rustls requires glibc.

## Vue multi-stage Dockerfile — build-time env vars

`VITE_*` vars are baked into the JS bundle at build time, not runtime. Pass them as Docker `ARG`:

```dockerfile
FROM node:22-alpine AS builder
ARG VITE_API_BASE_URL=/api/v1
ENV VITE_API_BASE_URL=$VITE_API_BASE_URL
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM nginx:1.27-alpine AS runtime
COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
```

Override at build time:
```bash
docker build --build-arg VITE_API_BASE_URL=https://api.prod.example.com ./frontend
```

## Docker Compose — service ordering with health checks

The backend must wait for Postgres to accept connections before starting, otherwise `sqlx::migrate!()` fails on cold start.

```yaml
postgres:
  healthcheck:
    test: ["CMD-SHELL", "pg_isready -U ${POSTGRES_USER} -d ${POSTGRES_DB}"]
    interval: 5s
    retries: 10

backend:
  depends_on:
    postgres:
      condition: service_healthy   # waits for healthy, not just started
```

`condition: service_healthy` is the critical difference from plain `depends_on`.

## nginx SPA config — two required rules

```nginx
# 1. SPA fallback: hard refresh on any route must serve index.html
location / {
    try_files $uri $uri/ /index.html;
}

# 2. API proxy: frontend container forwards /api/ to backend service
# Uses Docker internal DNS — "backend" resolves to the backend container
location /api/ {
    proxy_pass http://backend:8080;
}
```

Without rule 1, refreshing `/projects/abc` returns a 404. Without rule 2, API calls from the browser go to port 80 (nginx) and need forwarding.

## Common commands

```bash
make dev                          # docker compose up --build
docker compose up postgres -d     # start only postgres for local cargo dev
docker compose logs -f backend    # tail backend logs
docker compose down -v            # stop and delete volumes (wipes DB)
```

## Environment variables

`docker-compose.yml` has built-in defaults — `make dev` works with no `.env` file:

| Variable | Default |
|---|---|
| `POSTGRES_USER` | `dashboard` |
| `POSTGRES_PASSWORD` | `secret` |
| `POSTGRES_DB` | `project_dashboard` |
| `JWT_SECRET` | `dev-secret-change-in-production` |
| `VITE_API_BASE_URL` | `/api/v1` |

Override any variable by creating `.env` from `.env.example`. `VITE_API_BASE_URL` is baked into the frontend JS bundle at Docker build time — changing it requires a rebuild.
