use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "user_role")] // Maps to the PostgreSQL enum
#[sqlx(rename_all = "UPPERCASE")] // Matches the uppercase format of PostgreSQL enum
pub enum UserRole {
    USER,
    ADMIN,
}

#[derive(Debug, serde::Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
    pub is_active: bool,
    pub is_private: bool,
}

#[derive(serde::Deserialize, sqlx::FromRow)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_private: bool,
}

#[derive(Debug, serde::Deserialize, Serialize, sqlx::FromRow)]
pub struct BasicUserInfo {
    pub id: i64,
    pub username: String,
    pub is_active: bool,
    pub is_private: bool,
}

#[derive(Debug, serde::Deserialize, Serialize, sqlx::FromRow)]
pub struct UserInfo {
    pub id: i64,
    pub username: String,
    pub is_active: bool,
    pub is_private: bool,
    pub role: UserRole,
}

#[derive(Debug, serde::Deserialize, Serialize, sqlx::FromRow)]
pub struct BasicUserCreds {
    pub id: i64,
    pub password: String,
    pub role: UserRole,
}
