use std::collections::HashMap;
use axum::{http::StatusCode, response::Json, extract::{State, Query}};
use axum_sessions::extractors::ReadableSession;
use diesel::prelude::*;
use deadpool_diesel::postgres::Pool;

use crate::models::{pagination::Pagination, user::{User, UserInfo}};
use crate::schema::users;
use crate::utils::{self, response_error};

pub async fn get_user() -> Json<&'static str> {
    Json("user")
}

pub async fn get_user_list(
    State(pool): State<Pool>,
    Query(query): Query<HashMap<String, String>>,
    session: ReadableSession,
) -> Result<Json<Pagination<UserInfo>>, (StatusCode, String)> {
    let is_admin = utils::user_role::is_admin(session);
    if !is_admin {
        return Err((StatusCode::NOT_FOUND, "Not Found".to_string()));
    }

    let conn = pool.get().await.map_err(response_error::internal_error)?;

    let total = conn.interact(|conn|
        users::table
            .count()
            .get_result(conn)
            .unwrap()
    ).await
        .map_err(response_error::internal_error)
        .unwrap();

    let page_info = utils::get_page_info(query);
    let user_list  = conn.interact(move |conn|
        users::table
            .select(User::as_select())
            .order(users::created_at.desc())
            .offset((page_info.page - 1) * page_info.page_size)
            .limit(page_info.page_size)
            .load(conn)
    ).await
        .unwrap()
        .unwrap();

    let user_info_list = user_list.into_iter()
        .map(|user| utils::get_user_info(&user))
        .collect();
    let paged_list = Pagination {
        total,
        page: page_info.page,
        page_size: page_info.page_size,
        results: user_info_list,
    };

    Ok(Json(paged_list))
}
