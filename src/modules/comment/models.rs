use std::time::SystemTime;
use diesel::prelude::*;
use crate::schema::comments;

#[derive(serde::Serialize, Selectable, Queryable)]
pub struct Comment {
    pub id: i32,
    pub content: String,
    pub author_id: i32,
    pub post_id: i32,
    pub parent_comment_id: Option<i32>,
    pub created_at: SystemTime,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = comments)]
pub struct NewComment {
    pub content: String,
    pub parent_comment_id: Option<i32>,
}
