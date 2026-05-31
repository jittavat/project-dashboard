use actix_web::{web, HttpResponse};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;

use crate::{
    config::Settings,
    db::DbPool,
    errors::AppError,
    middleware::auth::Claims,
    models::user::{AuthResponse, LoginRequest, RegisterRequest, User, UserPublic},
};

/// Register a new user
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User created", body = AuthResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Email already in use"),
    ),
    tag = "auth"
)]
pub async fn register(
    pool: web::Data<DbPool>,
    cfg: web::Data<Settings>,
    body: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| AppError::Validation(e.to_string()))?
        .to_string();

    let user = sqlx::query_as!(
        User,
        r#"INSERT INTO users (id, email, password_hash, display_name, created_at, updated_at)
           VALUES ($1, $2, $3, $4, NOW(), NOW())
           RETURNING *"#,
        Uuid::new_v4(),
        body.email.to_lowercase(),
        hash,
        body.display_name,
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db) if db.constraint() == Some("users_email_key") => {
            AppError::Validation("Email already in use".into())
        }
        _ => AppError::Database(e),
    })?;

    let token = make_jwt(&user.id, &cfg)?;
    Ok(HttpResponse::Created().json(AuthResponse {
        token,
        user: UserPublic::from(user),
    }))
}

/// Login with email and password
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials"),
    ),
    tag = "auth"
)]
pub async fn login(
    pool: web::Data<DbPool>,
    cfg: web::Data<Settings>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        body.email.to_lowercase()
    )
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| AppError::Auth("Invalid credentials".into()))?;

    let parsed = PasswordHash::new(&user.password_hash)
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?;
    Argon2::default()
        .verify_password(body.password.as_bytes(), &parsed)
        .map_err(|_| AppError::Auth("Invalid credentials".into()))?;

    let token = make_jwt(&user.id, &cfg)?;
    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        user: UserPublic::from(user),
    }))
}

fn make_jwt(user_id: &Uuid, cfg: &Settings) -> Result<String, AppError> {
    let now = Utc::now().timestamp() as usize;
    let claims = Claims {
        sub: *user_id,
        iat: now,
        exp: now + (cfg.jwt_expiry_hours as usize * 3600),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(cfg.jwt_secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))
}
