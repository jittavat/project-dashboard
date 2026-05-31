---
name: sqlx-workflow
description: SQLx compile-time query checking, migrations, and offline mode. Use when writing database queries, adding migrations, or diagnosing SQLx build errors.
trigger: /sqlx-workflow
---

# /sqlx-workflow

SQLx patterns for this project's Rust backend.

## Why SQLx needs DATABASE_URL at compile time

`sqlx::query_as!()` connects to the database during `cargo build` to validate SQL syntax and column types against the live schema. This is the feature — typos and schema mismatches become compile errors instead of runtime panics.

**Consequence:** `cargo build` fails without `DATABASE_URL` set in the environment.

## Two workflows

### Workflow 1 — Local `cargo build` (with live DB)

```bash
# Start postgres, set DATABASE_URL in backend/.env, then:
cd backend && cargo build   # sqlx connects to DB at compile time
```

### Workflow 2 — Docker builds (offline, no DB at build time)

The `backend/Dockerfile` sets `ENV SQLX_OFFLINE=true` — Docker builds never need a running database. They read the committed `.sqlx/` query cache instead.

**One-time setup** (and after any migration or query change):

```bash
docker compose up postgres -d   # needs a live DB just for this step
make sqlx-prepare               # runs: cargo sqlx prepare → writes backend/.sqlx/
git add backend/.sqlx/
git commit -m "update sqlx query cache"
```

After this, `make dev` and `docker compose up --build` work with no database running during the build.

## Writing queries

```rust
// SELECT
let rows = sqlx::query_as!(MyStruct,
    "SELECT * FROM table WHERE owner_id = $1 ORDER BY created_at DESC",
    user_id
).fetch_all(pool.get_ref()).await?;

// INSERT returning
let row = sqlx::query_as!(MyStruct,
    "INSERT INTO table (id, name, created_at) VALUES ($1, $2, NOW()) RETURNING *",
    Uuid::new_v4(), name,
).fetch_one(pool.get_ref()).await?;

// DELETE (no return)
let affected = sqlx::query!("DELETE FROM table WHERE id = $1", id)
    .execute(pool.get_ref()).await?.rows_affected();
```

## Partial-update (PATCH/PUT) pattern

Use `COALESCE($N, column)` to allow optional field updates — `NULL` preserves the existing value, any non-NULL value overwrites it:

```sql
UPDATE resources
SET name        = COALESCE($1, name),
    description = COALESCE($2, description),
    updated_at  = NOW()
WHERE id = $3 AND owner_id = $4
RETURNING *
```

Pair with `fetch_optional` so a missing row returns a 404 rather than panicking:

```rust
let row = sqlx::query_as!(Resource, r#"UPDATE ... RETURNING *"#, ...)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Resource not found".into()))?;
```

## PostgreSQL array columns (`TEXT[]`)

`TEXT[] NOT NULL DEFAULT '{}'` columns map to `Vec<String>` in the Rust struct. Two non-obvious rules:

**In INSERT:** the default is not applied automatically when the parameter is NULL. Use `COALESCE`:
```sql
COALESCE($11, '{}'::text[])
```

**In UPDATE (partial-update pattern):** `COALESCE($11, tags)` — NULL means "keep existing", any array (including `[]`) means "overwrite".

**Parameter binding:** SQLx expects `Option<&[String]>`, not `Option<Vec<String>>`. Convert with:
```rust
body.tags.as_deref()  // Option<Vec<String>> → Option<&[String]>
```

## `query_as!` import rule

The target struct (e.g. `Ticket`) **must** remain in the `use` statement even if no handler code references it directly. `query_as!(Ticket, ...)` is a macro — the compiler sees `Ticket` only inside it. Removing the import produces `E0422: cannot find struct, variant or union type 'Ticket'`. Do not treat it as unused.

## Postgres enum types

Custom postgres enums (`ticket_status`, `ticket_priority`) require `sqlx::Type` with matching attributes:

```rust
#[derive(sqlx::Type)]
#[sqlx(type_name = "ticket_status", rename_all = "snake_case")]
pub enum TicketStatus { Backlog, Todo, InProgress, InReview, Done }
```

In queries, cast the enum explicitly: `status AS "status: _"` in SELECT, `$1 as _` in INSERT/UPDATE.

## Migrations

Files in `backend/migrations/` run in filename order on startup via `sqlx::migrate!("./migrations")`.

```bash
cd backend && cargo sqlx migrate run     # apply pending
cd backend && cargo sqlx migrate add <name>   # create new file
cd backend && cargo sqlx migrate info    # check status
```

Filename format: `<timestamp>_<description>.sql`

## Diagnosing build errors

| Error | Cause | Fix |
|---|---|---|
| `set DATABASE_URL to use query macros online` | No DATABASE_URL and no `.sqlx/` cache | Run `make sqlx-prepare` then commit `.sqlx/` |
| `type "ticket_status" does not exist` | Migration not run | `cargo sqlx migrate run` |
| `mismatched types` in query | Rust type vs column type mismatch | Check column type in migration SQL |
| `no rows returned` panic | `fetch_one` on empty result | Use `fetch_optional` instead |
| `E0422: cannot find struct 'Ticket'` | Target type removed from `use` | Re-add it; `query_as!(Ticket, ...)` requires the type to be in scope even if unused elsewhere |
| `expected &[String], found Vec<String>` | Passing `Option<Vec<String>>` to array param | Use `.as_deref()` → `Option<&[String]>` |
| `From<fn(String) -> AppError>` not implemented | Using `.ok_or(AppError::NotFound)?` — `NotFound` is a constructor, not a value | Use `.ok_or_else(\|\| AppError::NotFound("msg".into()))?` |
