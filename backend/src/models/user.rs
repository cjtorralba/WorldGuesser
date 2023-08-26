use crate::error::AppError;
use axum::async_trait;
use axum::extract::FromRequestParts;
use cookie::Cookie;
use derive_more::Display;
use http::request::Parts;
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use once_cell::sync::Lazy;
use serde_derive::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct User {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct UserSignup {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct LoggedInUser {
    pub token: Claims,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Display)]
#[display(fmt = "id: {}, email: {}, exp: {}", id, email, exp)]
pub struct Claims {
    pub id: i32,
    pub email: String,
    pub exp: u64,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync, // Making a trait bound, so that S can ONLY be an async object, when S: Send + Sync
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let jwt_token = parts
            .headers
            .get("cookie")
            .and_then(|value| Cookie::parse(value.to_str().unwrap_or_default()).ok())
            .and_then(|cookie| {
                if cookie.name() == "jwt" {
                    Some(cookie.value().to_string())
                } else {
                    None
                }
            })
            .ok_or(AppError::InvalidToken)?;

        let token_data = decode::<Claims>(&jwt_token, &KEYS.decoding, &Validation::default())
            .map_err(|_| AppError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

pub struct OptionalClaims(pub Option<Claims>);

#[async_trait]
impl<S> FromRequestParts<S> for OptionalClaims
where
    S: Send + Sync, // Making a trait bound, so that S can ONLY be an async object, when S: Send + Sync
{
    type Rejection = Infallible; // Uses Infallible since we are not rejecting the request

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let jwt_token = parts
            .headers
            .get("cookie")
            .and_then(|value| Cookie::parse(value.to_str().unwrap_or_default()).ok())
            .and_then(|cookie| {
                if cookie.name() == "jwt" {
                    Some(cookie.value().to_string())
                } else {
                    None
                }
            });

        if let Some(jwt) = jwt_token {
            if let Ok(token_data) = decode::<Claims>(&jwt, &KEYS.decoding, &Validation::default()) {
                return Ok(OptionalClaims(Some(token_data.claims)));
            }
        }

        Ok(OptionalClaims(None))
    }
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

/// Lazy struct means that is will only ever be initialized on demand,
/// and onnce it is initialized, there will only ever be one instance of them
pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("MISSING JWT SECRET!");
    Keys::new(secret.as_bytes())
});
