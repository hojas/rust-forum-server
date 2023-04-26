use std::time::SystemTime;
use diesel::prelude::*;
use crate::schema::collected_posts;

#[derive(serde::Serialize, Selectable, Queryable)]
pub struct CollectedPost {
    pub id: i32,
    pub user_id: i32,
    pub post_id: i32,
    pub created_at: SystemTime,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = collected_posts)]
pub struct NewCollectedPost {
    pub user_id: i32,
    pub post_id: i32,
}
