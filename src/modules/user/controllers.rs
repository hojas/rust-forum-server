use std::collections::HashMap;
use axum::{response::Json, extract::{State, Query}};
use axum_sessions::extractors::ReadableSession;
use diesel::prelude::*;
use deadpool_diesel::postgres::Pool;

use crate::schema::users;
use crate::utils;
use crate::modules::{
    response::{
        models::ResponseResult,
        utils as response_utils,
    },
    session::utils as session_utils,
    pagination::{
        models::Pagination,
        utils as pagination_utils,
    },
};
use super::{models::{User, UserInfo}, utils as user_utils};

pub async fn get_user_list(
    State(pool): State<Pool>,
    Query(query): Query<HashMap<String, String>>,
    session: ReadableSession,
) -> ResponseResult<Pagination<UserInfo>> {
    // check if user is admin
    session_utils::is_admin(session)?.0;

    let conn = utils::pool::get_conn(pool).await?;

    let total = conn.interact(|conn|
        users::table
            .count()
            .get_result(conn)
    ).await
        .map_err(|e| response_utils::internal_error(e, None))?
        .map_err(|e| response_utils::internal_error(e, None))?;

    let page_info = pagination_utils::get_page_info(query);
    let user_list = conn.interact(move |conn|
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
        .map(|user| user_utils::get_user_info(&user))
        .collect();

    let paged_list = Pagination {
        total,
        page: page_info.page,
        page_size: page_info.page_size,
        results: user_info_list,
    };

    Ok(Json(paged_list))
}
