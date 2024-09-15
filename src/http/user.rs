use crate::models::user::{BasicUserInfo, NewUser, User};
use crate::utils::password;
use axum::{
    async_trait,
    extract::{Extension, FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::Json,
    routing::{get, post},
    Router,
};
use sqlx::Postgres;

use super::error::{ApiError, ResultExt};
use super::ApiContext;

type Pool = sqlx::Pool<Postgres>;

pub fn router() -> Router {
    // By having each module responsible for setting up its own routing,
    // it makes the root module a lot cleaner.
    Router::new().route("/api/users", get(get_all_users).post(create_user))
}

pub async fn get_all_users(ctx: Extension<ApiContext>) -> Result<Json<Vec<User>>, ApiError> {
    let query = r#"
    SELECT * FROM users;
    "#;

    let result = sqlx::query_as::<_, User>(query).fetch_all(&ctx.db).await?;
    Ok(Json(result))
}

// nrew fn
pub async fn create_user(
    ctx: Extension<ApiContext>,
    Json(mut new_user): Json<NewUser>,
) -> Result<Json<BasicUserInfo>, ApiError> {
    if new_user.username.len() > 32 {
        Err(ApiError::unprocessable_entity([(
            "username",
            "username is to long max 32",
        )]))?
    }

    if new_user.email.len() > 64 {
        Err(ApiError::unprocessable_entity([(
            "email",
            "email is to long max 64",
        )]))?
    }

    let hash_pass = password::hash(new_user.password).await;
    new_user.password = match hash_pass {
        Ok(pass) => pass,
        // TODO FIX
        Err(err) => Err(ApiError::Anyhow(err))?,
    };

    let query = r#"
    INSERT INTO users (username, email, password, is_private)
    VALUES ($1, $2, $3, $4)
    RETURNING id, username, is_active, is_private
"#;

    let result = sqlx::query_as::<_, BasicUserInfo>(query)
        .bind(new_user.username)
        .bind(new_user.email)
        .bind(new_user.password)
        .bind(new_user.is_private)
        .fetch_one(&ctx.db)
        .await
        .on_constraint("users_username_key", |_| {
            ApiError::unprocessable_entity([("username", "userbane eakraed taken")])
        })
        .on_constraint("users_email_key", |_| {
            ApiError::unprocessable_entity([("email", "email already taken")])
        })?;

    Ok(Json(result))
}
