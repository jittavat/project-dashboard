# Project Dashboard

A Jira-style project management tool built with Rust (Actix-web) and Vue 3. Manage projects, tickets, and epics with a kanban board and a pure-SVG Gantt chart.

## Features

- **Kanban board** — drag tickets across Backlog → To Do → In Progress → In Review → Done
- **Epics** — group tickets under epics with start/finish date boundaries; editing a ticket date outside its epic boundary prompts to extend the epic automatically
- **Gantt / Timeline** — pure-SVG chart with three views: By Ticket, By Assignee, and By Epic
- **JWT auth** — register and login; all project/ticket routes are protected
- **Tags, priorities, assignees, and dates** on every ticket and epic

## Stack

| Layer    | Technology                                |
| -------- | ----------------------------------------- |
| Backend  | Rust · Actix-web · SQLx · PostgreSQL 17  |
| Frontend | Vue 3 · Pinia · Vue Router · Tailwind CSS |
| Infra    | Docker Compose · nginx                    |

## Quick start

Requires Docker and Docker Compose.

```bash
git clone https://github.com/jittavat/project-dashboard.git
cd project-dashboard
make dev          # builds images, starts postgres → backend → frontend
```

Open [http://localhost:3000](http://localhost:3000). Register a new account to get started.

No `.env` file is needed — `docker-compose.yml` ships with safe development defaults.

## Development

### Full stack

```bash
make dev          # docker compose up --build
make test         # cargo test + vitest
make lint         # cargo fmt/clippy + eslint
make build        # release builds for both services
make clean        # tear down containers + volumes + build artefacts
```

### Backend only (requires `DATABASE_URL`)

```bash
cd backend
cp .env.example .env          # set DATABASE_URL + JWT_SECRET
cargo run                     # API on :8080, Swagger UI at /swagger-ui/
cargo test
cargo sqlx migrate run        # apply pending migrations
```

### Frontend only

```bash
cd frontend
npm install
npm run dev       # Vite dev server on :5173, proxies /api → :8080
npm run test
npm run lint
```

## Environment variables

`docker-compose.yml` defaults work out of the box. Override by creating `.env` from `.env.example`:

| Variable           | Default                                             | Notes                          |
| ------------------ | --------------------------------------------------- | ------------------------------ |
| `DATABASE_URL`     | `postgres://dashboard:secret@postgres:5432/project_dashboard` | full connection string |
| `JWT_SECRET`       | `dev-secret-change-in-production`                   | change in production           |
| `JWT_EXPIRY_HOURS` | `24`                                                |                                |
| `RUST_LOG`         | `info`                                              |                                |
| `VITE_API_BASE_URL`| `/api/v1`                                           | baked into frontend at build time |

## Project structure

```
backend/
  src/
    handlers/       # actix-web route handlers (auth, projects, tickets, epics)
    models/         # SQLx structs and request/response types
    middleware/     # JWT CurrentUser extractor
    migrations/     # SQL migrations (applied automatically on startup)
  .sqlx/            # offline query cache (committed; required for Docker builds)

frontend/
  src/
    api/            # axios client + typed wrappers for each resource
    stores/         # Pinia stores (auth, projects, tickets, epics)
    views/          # page-level components (Board, Timeline, Ticket detail, …)
    components/     # shared UI components (BaseModal, ConfirmModal, …)
    composables/    # useConfirm, useEpicDateGuard
```

## After schema changes

SQLx validates queries at compile time using the `.sqlx/` cache. After any migration or SQL query change, regenerate it:

```bash
docker compose up postgres -d
make sqlx-prepare   # runs cargo sqlx prepare → writes backend/.sqlx/
# commit backend/.sqlx/
```
