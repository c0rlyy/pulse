// i will never touch those imports coz i dont know why but i ll never want to have the same error
// that when i define a new function in module WIHTOUT EVEN CALLING IT
// i get errors in every single one of my functions even tho i have not touch a single line of code
// LOVE RUST
use crate::auth::auth::{UpdateUserEmail, UpdateUserPassword, UserDeletePayload};
use crate::auth::claims::Claims;
use crate::models::user::{BasicUserInfo, NewUser, User, UserInfo, UserRole};
use crate::utils::password;
use argon2::Params;
use axum::extract::Query;
use axum::http::Extensions;
use axum::routing::delete;
use axum::routing::patch;

use axum::{
    async_trait,
    extract::{Extension, FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

use super::error::{ApiError, ResultExt};
use super::ApiContext;

pub fn router() -> Router {
    // By having each module responsible for setting up its own routing,
    // it makes the root module a lot cleaner.
    Router::new()
        .route(
            "/api/users",
            get(get_all_users).post(create_user).delete(delete_user),
        )
        .route("/api/users/password", patch(update_password))
        .route("/api/users/email", patch(update_email))
        .route("/api/users/search", get(search_users))
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
        return Err(ApiError::unprocessable_entity([(
            "username",
            "username is too long, max 32",
        )]));
    }

    if new_user.email.len() > 64 {
        return Err(ApiError::unprocessable_entity([(
            "email",
            "email is too long, max 64",
        )]));
    }

    new_user.password = password::hash(new_user.password).await?;

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

pub async fn delete_user(
    ctx: Extension<ApiContext>,
    claims: Claims,
    Json(payload): Json<UserDeletePayload>,
) -> Result<StatusCode, ApiError> {
    let query = r#"select password from users where id =$1"#;
    let pass = sqlx::query_as::<_, UserDeletePayload>(query)
        .bind(claims.sub)
        .fetch_optional(&ctx.db)
        .await?
        .ok_or(ApiError::NotFound)?;

    if !password::verify(payload.password, pass.password).await? {
        return Err(ApiError::Unauthorized);
    }

    let query = r#"delete from users where id =$1"#;

    sqlx::query(query).bind(claims.sub).execute(&ctx.db).await?;
    // returning just that operation was sucesfull
    Ok(StatusCode::OK)
}

pub async fn update_password(
    ctx: Extension<ApiContext>,
    claims: Claims,
    Json(mut payload): Json<UpdateUserPassword>,
) -> Result<StatusCode, ApiError> {
    let query =
        r#"select password,id,username,role,is_private,is_active,email from users where id =$1"#;
    let pass = sqlx::query_as::<_, User>(query)
        .bind(claims.sub)
        .fetch_optional(&ctx.db)
        .await?
        .ok_or(ApiError::NotFound)?;

    //TODO fix function signature and the stuff inside of it
    if !password::verify(payload.password.clone(), pass.password).await? {
        return Err(ApiError::Unauthorized);
    }

    let query = r#"UPDATE users SET password = $1 WHERE id = $2"#;

    payload.new_password = password::hash(payload.new_password).await?;
    sqlx::query(query)
        .bind(payload.new_password)
        .bind(claims.sub)
        .execute(&ctx.db)
        .await?;
    // returning just that operation was sucesfull
    Ok(StatusCode::OK)
}

pub async fn update_email(
    ctx: Extension<ApiContext>,
    claims: Claims,
    Json(payload): Json<UpdateUserEmail>,
) -> Result<StatusCode, ApiError> {
    tracing::info!("do i even execute???");
    let query =
        r#"select password,id,username,role,is_private,is_active,email from users where id =$1"#;
    let pass = sqlx::query_as::<_, User>(query)
        .bind(claims.sub)
        .fetch_optional(&ctx.db)
        .await?
        .ok_or(ApiError::NotFound)?;

    if !password::verify(payload.password, pass.password).await? {
        return Err(ApiError::Unauthorized);
    }

    let query = r#"update users set email = $1 where id =$2"#;

    sqlx::query(query)
        .bind(payload.new_email)
        .bind(claims.sub)
        .execute(&ctx.db)
        .await?;
    // returning just that operation was sucesfull
    Ok(StatusCode::OK)
}

#[derive(Debug, Deserialize)]
pub struct SearchUserParams {
    pub username: String,
    pub limit: i64,
    pub skip: i64,
}

pub async fn search_users(
    ctx: Extension<ApiContext>,
    Query(params): Query<SearchUserParams>,
) -> Result<Json<Vec<BasicUserInfo>>, ApiError> {
    let query = r#"
    SELECT id, username, is_private, is_active 
    FROM users 
    WHERE username LIKE $1
    AND (is_private = false AND is_active = true)
    LIMIT $2
    OFFSET $3
    "#;

    let search_pattern = format!("%{}%", params.username);

    // tracing::info!("SQL: {}", query);
    // tracing::info!(
    //     "Parameters: search_pattern={}, limit={}, skip={}",
    //     search_pattern,
    //     params.limit,
    //     params.skip
    // );

    let users = sqlx::query_as::<_, BasicUserInfo>(query)
        .bind(search_pattern) // Bind the formatted search pattern
        .bind(params.limit)
        .bind(params.skip)
        .fetch_all(&ctx.db)
        .await?;

    if users.is_empty() {
        return Err(ApiError::NotFound);
    }

    Ok(Json(users))
}
