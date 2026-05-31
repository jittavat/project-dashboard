.PHONY: dev build test coverage migrate lint clean sqlx-prepare \
        backend-build backend-test backend-coverage backend-lint backend-migrate backend-clean \
        frontend-install frontend-build frontend-test frontend-coverage frontend-lint frontend-clean

# ── Top-level targets ────────────────────────────────────────────────────────

dev:
	docker compose up --build

build: backend-build frontend-build

test: backend-test frontend-test

coverage: backend-coverage frontend-coverage

migrate: backend-migrate

# Run once after schema changes to regenerate .sqlx/ cache for offline Docker builds
# Migrations must be applied first so the tables exist for query validation
sqlx-prepare:
	cd backend && cargo sqlx migrate run && cargo sqlx prepare

lint: backend-lint frontend-lint

clean: backend-clean frontend-clean
	docker compose down -v --remove-orphans

# ── Backend ──────────────────────────────────────────────────────────────────

backend-build:
	cd backend && cargo build --release

backend-test:
	cd backend && cargo test

backend-coverage:
	cd backend && cargo tarpaulin --out Html --output-dir .

backend-lint:
	cd backend && cargo fmt --check && cargo clippy -- -D warnings

backend-migrate:
	cd backend && cargo sqlx migrate run

backend-clean:
	cd backend && cargo clean

# ── Frontend ─────────────────────────────────────────────────────────────────

frontend-install:
	cd frontend && npm ci

frontend-build:
	cd frontend && npm run build

frontend-test:
	cd frontend && npm run test

frontend-coverage:
	cd frontend && npm run coverage

frontend-lint:
	cd frontend && npm run lint

frontend-clean:
	cd frontend && rm -rf dist node_modules
