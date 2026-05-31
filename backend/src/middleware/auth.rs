use actix_web::{dev::Payload, web, FromRequest, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};
use uuid::Uuid;

use crate::config::Settings;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
}

pub struct CurrentUser(pub Claims);

impl FromRequest for CurrentUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let cfg = match req.app_data::<web::Data<Settings>>() {
            Some(c) => c,
            None => return ready(Err(actix_web::error::ErrorInternalServerError("Config missing"))),
        };

        let result = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))
            .and_then(|token| {
                decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(cfg.jwt_secret.as_bytes()),
                    &Validation::default(),
                )
                .map(|d| CurrentUser(d.claims))
                .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))
            });

        ready(result)
    }
}
