use std::time::SystemTime;
use diesel::prelude::*;
use crate::schema::*;

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
