use std::time::SystemTime;
use diesel::prelude::*;
use crate::schema::*;

#[derive(serde::Serialize)]
pub struct CustomResponse<T> {
    pub ok: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

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

#[derive(serde::Deserialize, Insertable, Clone)]
#[diesel(table_name = users)]
pub struct UserRegister {
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize, Queryable)]
#[diesel(table_name = users)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct Pagination<T> {
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub results: Vec<T>,
}

#[derive(serde::Serialize, Selectable, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author_id: i32,
    pub created_at: SystemTime,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub content: String,
}


#[derive(serde::Serialize, Selectable, Queryable)]
pub struct Comment {
    pub id: i32,
    pub content: String,
    pub author_id: i32,
    pub post_id: i32,
    pub parent_comment_id: Option<i32>,
    pub created_at: SystemTime,
}
