use crate::{
    auth::{
        auth::{AuthBody, AuthPayload},
        claims::Claims,
    },
    models::user::{BasicUserCreds, UserInfo},
    utils::password,
};
use axum::{
    routing::{get, post},
    Extension, Json, Router,
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

    let user = sqlx::query_as::<_, BasicUserCreds>(query)
        .bind(payload.client_id)
        .fetch_optional(&ctx.db)
        .await?
        .ok_or(ApiError::NotFound)?;

    if !password::verify(payload.client_secret, user.password).await? {
        Err(ApiError::Unauthorized)?
    };

    let token = Claims::new(user.id, user.role, &ctx.config.default_session_lenght).to_jwt(&ctx)?;
    Ok(Json(AuthBody::new(token)))
}
