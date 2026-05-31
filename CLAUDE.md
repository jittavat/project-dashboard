# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

### Full stack (Docker)

```bash
make dev          # docker compose up --build (postgres:5432, backend:8080, frontend:3000)
make build        # cargo build --release + npm run build
make test         # cargo test + vitest run
make coverage     # cargo tarpaulin + vitest --coverage
make lint         # cargo fmt --check + clippy + eslint
make migrate      # cargo sqlx migrate run
make clean        # cargo clean + rm dist/ + docker compose down -v
```

### Backend (Rust) — requires DATABASE_URL

```bash
cd backend
cargo build                     # compile (needs DATABASE_URL env set)
cargo test                      # unit tests
cargo test <test_name>          # single test
cargo run                       # run server on :8080
cargo sqlx migrate run          # apply pending migrations
cargo sqlx prepare              # generate .sqlx/ offline query cache
cargo tarpaulin --out Html      # coverage report → tarpaulin-report.html
cargo fmt                       # format
cargo clippy -- -D warnings     # lint
```

### Frontend (Vue)

```bash
cd frontend
npm run dev       # vite dev server on :5173 (proxies /api → :8080)
npm run build     # type-check + vite build → dist/
npm run test      # vitest run (passWithNoTests)
npm run coverage  # vitest coverage → coverage/
npm run lint      # eslint --fix
npm run format    # prettier write
```

## Architecture

### Backend (`backend/src/`)

Single Actix-web binary. Key files:

- `main.rs` — server setup, DB pool, runs `sqlx::migrate!()` on startup
- `config.rs` — `Settings::from_env()` reads all env vars; passed as `web::Data<Settings>` to handlers
- `routes.rs` — all route registrations + utoipa OpenAPI spec (Swagger UI at `/swagger-ui/`)
- `middleware/auth.rs` — `CurrentUser` implements `FromRequest`; handlers that require auth declare it as a parameter
- `handlers/{auth,projects,tickets}.rs` — request handlers; use `sqlx::query_as!()` compile-time checked macros
- `errors.rs` — `AppError` enum implements `ResponseError`; all handlers return `Result<HttpResponse, AppError>`
- `migrations/` — run in order on startup; define postgres enums `ticket_status` and `ticket_priority`

**`AppError::NotFound` takes a String argument** — it is not a unit variant. Use `.ok_or_else(|| AppError::NotFound("msg".into()))?` when converting `fetch_optional` results to 404s. `.ok_or(AppError::NotFound)?` does not compile.

**SQLx `query_as!` import rule:** The target struct (e.g. `Ticket`) must appear in the `use` statement of any handler file that calls `query_as!(Ticket, ...)`, even if the struct is never referenced directly. Removing it causes `E0422: cannot find struct Ticket` at compile time. Do not treat it as an unused import.

**SQLx requirement:** The Dockerfile sets `SQLX_OFFLINE=true` so Docker builds never need a database. This requires the `.sqlx/` query cache to be committed to the repo. After any schema change (new migration or modified query), regenerate it:

```bash
docker compose up postgres -d   # needs a live DB once
make sqlx-prepare               # runs cargo sqlx prepare → writes backend/.sqlx/
# commit backend/.sqlx/
```

For local `cargo build` (outside Docker), set `DATABASE_URL` in `backend/.env` and the macro connects to the live DB directly.

### Frontend (`frontend/src/`)

Vue 3 Composition API SPA with code-split lazy-loaded routes.

- `api/` — axios client in `index.ts` (JWT interceptor, 401 redirect, **`camelizeKeys` response interceptor** that auto-converts all snake_case response keys to camelCase); `auth.ts`, `projects.ts`, `tickets.ts` call the API
- `stores/` — Pinia stores (Composition API style). Auth store holds `jwt` ref; `initialize()` must be called on app mount to restore from localStorage
- `router/index.ts` — all routes under `/` wrap `AppLayout`; `beforeEach` guard redirects unauthenticated users to `/login`
- `components/layout/` — `AppLayout` is the authenticated shell wrapping all protected views
- `views/ProjectDetailView.vue` — kanban board; tickets grouped by `status` across 5 columns
- `views/TimelineView.vue` — pure-SVG Gantt chart; route `/projects/:projectId/timeline`; three modes: By Ticket (bars labeled by assignee), By Assignee (y-axis = assignees, bars labeled by task title), By Epic. Calendar x-axis: month band row + 7-day tick row. Bars span `startDate || createdAt` → `finishedDate || today`. Hover tooltip via `<Teleport to="body">`. No external charting library.
- `vitest.config.ts` — separate from `vite.config.ts`; uses `mergeConfig` to inherit vite config (required because vitest 4.x bundles its own vite copy)
- `tsconfig.json` — includes `"ignoreDeprecations": "6.0"` required by TypeScript 6 (`baseUrl` is deprecated but still needed for `@/` aliases)

### Auth flow

1. `POST /api/v1/auth/login` → returns `{ access_token, user }`
2. Frontend stores JWT in `localStorage` under key `access_token`
3. Axios interceptor attaches `Authorization: Bearer <jwt>` on every request
4. Backend `CurrentUser` extractor validates JWT on protected routes

### Docker Compose services

| Service  | Port | Notes                                              |
| -------- | ---- | -------------------------------------------------- |
| postgres | 5432 | healthcheck required before backend starts         |
| backend  | 8080 | runs migrations on startup                         |
| frontend | 3000 | nginx serves SPA; proxies `/api/` to backend       |

### Environment variables

`docker-compose.yml` has built-in defaults so `make dev` works with no `.env` file (uses `dashboard`/`secret` for postgres, `dev-secret-change-in-production` for JWT). Override by creating a `.env` from `.env.example`. `VITE_API_BASE_URL` is baked into the frontend bundle at Docker build time.

## graphify

This project has a knowledge graph at graphify-out/ with god nodes, community structure, and cross-file relationships.

Rules:
- For codebase questions, first run `graphify query "<question>"` when graphify-out/graph.json exists. Use `graphify path "<A>" "<B>"` for relationships and `graphify explain "<concept>"` for focused concepts. These return a scoped subgraph, usually much smaller than GRAPH_REPORT.md or raw grep output.
- If graphify-out/wiki/index.md exists, use it for broad navigation instead of raw source browsing.
- Read graphify-out/GRAPH_REPORT.md only for broad architecture review or when query/path/explain do not surface enough context.
- After modifying code, run `graphify update .` to keep the graph current (AST-only, no API cost).

## Skills

Project-specific skills in `.claude/skills/`. Invoke with `/project:update-project --skills` to regenerate after code changes.

| Trigger | Description |
| ------- | ----------- |
| `/sqlx-workflow` | SQLx compile-time queries, migrations, offline cache, and error diagnosis |
| `/rust-actix-scaffold` | Scaffold a new handler, route, or model in the Rust backend |
| `/vue3-vite-scaffold` | Add a new view, Pinia store, or API module to the Vue frontend |
| `/docker-fullstack` | Run the full stack with Docker Compose, env-var overrides, postgres healthcheck |
| `/vitest-vite-split` | Why `vitest.config.ts` is separate from `vite.config.ts` and how to write tests |

## Project commands

| Command | What it does |
| ------- | ------------ |
| `/project:update-project` | Re-read the codebase and update `CLAUDE.md` |
| `/project:update-project --skills` | Update `CLAUDE.md` and all `.claude/skills/` files |
| `/project:update-project --skills-only` | Update skill files only |
| `/project:update-project --skill <name>` | Rewrite a single skill (e.g. `--skill sqlx-workflow`) |
