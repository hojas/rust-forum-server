use std::time::SystemTime;
use diesel::prelude::*;
use crate::schema::postscripts;

#[derive(serde::Serialize, Selectable, Queryable)]
pub struct Postscript {
    pub id: i32,
    pub post_id: i32,
    pub content: String,
    pub created_at: SystemTime,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = postscripts)]
pub struct NewPostscript {
    pub post_id: i32,
    pub content: String,
}
