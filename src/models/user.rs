use std::time::SystemTime;
use diesel::prelude::*;
use crate::schema::*;

#[derive(serde::Serialize, Selectable, Queryable, Clone)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub username: String,
    pub avatar_url: String,
    pub signature: String,
    pub role: String,
    pub last_login_at: SystemTime,
    pub created_at: SystemTime,
}

#[derive(serde::Serialize)]
pub struct UserInfo {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub avatar_url: String,
    pub signature: String,
    pub role: String,
    pub last_login_at: SystemTime,
    pub created_at: SystemTime,
}
