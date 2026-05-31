---
name: rust-actix-scaffold
description: Scaffold a production Rust REST API with actix-web, SQLx, JWT, argon2, utoipa. Use when adding new resource endpoints or wiring up a new handler in the backend.
trigger: /rust-actix-scaffold
---

# /rust-actix-scaffold

Scaffold a new resource or handler in the Rust backend (`backend/src/`).

## Dependency versions (already in Cargo.toml)

| Crate | Version | Note |
|---|---|---|
| actix-web | 4 | |
| sqlx | 0.8 | postgres, runtime-tokio, tls-rustls, uuid, chrono, migrate |
| argon2 | 0.5 | stable — do NOT use 0.6-rc |
| jsonwebtoken | 9 | |
| utoipa | 4 | actix_extras, chrono, uuid |
| utoipa-swagger-ui | 7 | |
| thiserror | 1 | |

## Handler pattern

Every handler follows this signature shape. Auth is opt-in via the `CurrentUser` parameter — omit it for public routes.

```rust
#![allow(dependency_on_unit_never_type_fallback)]  // suppress utoipa lint

#[utoipa::path(
    post,
    path = "/api/v1/resource",
    request_body = CreateResourceRequest,
    responses(
        (status = 201, description = "Created", body = Resource),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = [])),
    tag = "resource"
)]
pub async fn create_resource(
    pool: web::Data<DbPool>,
    user: CurrentUser,                    // remove for public endpoints
    body: web::Json<CreateResourceRequest>,
) -> Result<HttpResponse, AppError> {
    // validate
    if body.name.trim().is_empty() {
        return Err(AppError::Validation("name cannot be empty".into()));
    }
    // query
    let row = sqlx::query_as!(Resource,
        "INSERT INTO resources (id, name, owner_id, created_at, updated_at)
         VALUES ($1, $2, $3, NOW(), NOW()) RETURNING *",
        Uuid::new_v4(), body.name.trim(), user.0.sub,
    ).fetch_one(pool.get_ref()).await?;

    Ok(HttpResponse::Created().json(row))
}
```

## Auth extractor — `CurrentUser` (already in `middleware/auth.rs`)

Handlers that require JWT simply declare `user: CurrentUser`. The extractor reads `Authorization: Bearer <token>`, decodes with the app's `JWT_SECRET`, and returns `Claims { sub: Uuid, exp, iat }` or a 401 error — no middleware wrapping needed.

## Error handling

Return `Err(AppError::NotFound(...))`, `Err(AppError::Validation(...))`, etc. `AppError` implements `ResponseError` and maps each variant to the correct HTTP status.

**`AppError::NotFound` takes a `String` argument** — it is not a unit variant. The idiomatic 404 pattern when a row may not exist:

```rust
let row = sqlx::query_as!(Resource, "SELECT * FROM resources WHERE id = $1", id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Resource not found".into()))?;
```

Do **not** write `.ok_or(AppError::NotFound)?` — `NotFound` is a constructor function `fn(String) -> AppError`, not a value, and this fails to compile.

## SQLx compile-time queries

`sqlx::query_as!()` validates SQL at `cargo build` time. `DATABASE_URL` must be set in the environment. If building offline (CI / Docker), use:

```bash
cargo sqlx prepare   # run once with a live DB → commits .sqlx/ cache
SQLX_OFFLINE=true cargo build
```

## Registering a new handler

1. Add handler functions to `src/handlers/<resource>.rs`
2. Add `pub mod <resource>;` to `src/handlers/mod.rs`
3. Wire routes in `src/routes.rs` inside `configure()`
4. Add schema types to the `#[openapi(components(schemas(...)))]` list in `routes.rs`

## Adding a migration

Create `backend/migrations/<timestamp>_<name>.sql`. Migrations run automatically on startup via `sqlx::migrate!("./migrations")`.
