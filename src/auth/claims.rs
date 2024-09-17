use crate::http::error::AuthError;
use crate::http::ApiContext;
use crate::models::user::UserRole;

use std::time::Duration;

use jsonwebtoken::{encode, Header};
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;

/// Claims are used for returning user info and validating his acces to certian parts of app
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64, // this represents user_id
    pub exp: i64, // Expiration time of the token
    pub role: UserRole,
}

impl Claims {
    pub fn new(sub: i64, role: UserRole, duration: &Duration) -> Claims {
        Claims {
            sub,
            role,
            exp: (OffsetDateTime::now_utc() + *duration).unix_timestamp(),
        }
    }

    pub fn to_jwt(&self, ctx: &ApiContext) -> Result<String, AuthError> {
        let token = encode(&Header::default(), &self, &ctx.config.encoding_key)
            .map_err(|_| AuthError::TokenCreation)?;
        Ok(token)
    }
}
