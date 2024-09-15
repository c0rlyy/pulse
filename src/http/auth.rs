use std::{path::Display, time::Duration};

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, RequestPartsExt, Router,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;

use crate::{
    models::user::{TestUs, UserInfo, UserRole},
    utils::password,
};

use super::{
    error::{ApiError, AuthError},
    ApiContext,
};

pub fn router() -> Router {
    Router::new()
        .route("/api/login", post(authorize))
        .route("/api/users/me", get(get_current_user))
}

async fn get_current_user(
    ctx: Extension<ApiContext>,
    claims: Claims,
) -> Result<Json<UserInfo>, ApiError> {
    let query = r#"
        SELECT id,role,is_private,is_active,username,role
        FROM users 
        WHERE id = $1;
    "#;
    let user = sqlx::query_as::<_, UserInfo>(query)
        .bind(claims.sub)
        .fetch_one(&ctx.db)
        .await?;

    Ok(Json(user))
}

pub async fn authorize(
    ctx: Extension<ApiContext>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthBody>, ApiError> {
    // Check if the user sent the credentials
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        Err(AuthError::MissingCredentials)?;
    }

    let query = r#"
        SELECT id,role,password
        FROM users 
        WHERE email = $1;
    "#;

    let user = sqlx::query_as::<_, TestUs>(query)
        .bind(payload.client_id)
        .fetch_optional(&ctx.db)
        .await?
        .ok_or(ApiError::unprocessable_entity([(
            "email",
            "does not exists",
        )]))?;

    if !password::verify(payload.client_secret, user.password).await? {
        Err(ApiError::Forbidden)?
    };

    let token = Claims::new(user.id, user.role, &ctx.config.default_session_lenght).to_jwt(&ctx)?;
    Ok(Json(AuthBody::new(token)))
}

impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // this works

        let Extension(ctx) = parts
            .extract::<Extension<ApiContext>>()
            .await
            .expect("state should be provided correclty");

        let token_data = decode::<Claims>(
            bearer.token(),
            &ctx.config.decoding_key,
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

/// Claims are used for returning user info and validating his acces to certian parts of app
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: i64, // this represents user_id
    exp: i64,
    role: UserRole, // Expiration time of the token
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

/// Auth body will be used to store encoded token and its token type (example Bearer)
#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

/// this will usually contain email,and password, since email is unique it can be used as id
#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    client_id: String,
    client_secret: String,
}
