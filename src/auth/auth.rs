use serde::{Deserialize, Serialize};

/// Auth body will be used to store encoded token and its token type (example Bearer)
#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

/// this will usually contain email,and password, since email is unique it can be used as id
#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct UserDeletePayload {
    pub password: String,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct UpdateUserPassword {
    pub password: String,
    pub new_password: String,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct UpdateUserEmail {
    pub password: String,
    pub new_email: String,
}

impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}
