use std::collections::HashMap;
use axum::{http::StatusCode, response::Json, extract::{State, Path, Query}};
use axum_sessions::extractors::ReadableSession;
use diesel::prelude::*;
use deadpool_diesel::postgres::Pool;

use crate::schema::comments;
use crate::utils;

use crate::modules::{
    response::{
        models::{MessageResponse, ResponseResult},
        utils as response_utils,
    },
    pagination::{
        models::Pagination,
        utils as pagination_utils,
    },
};
use super::models::{Comment, NewComment};

pub async fn create_comment(
    State(pool): State<Pool>,
    session: ReadableSession,
    Json(new_comment): Json<NewComment>,
) -> ResponseResult<Comment> {
    let user_email = session.get::<String>("user_email").unwrap();
    if user_email.is_empty() {
        let message = MessageResponse { message: "not found".to_string() };
        return Err((StatusCode::NOT_FOUND, Json(message)));
    }

    if new_comment.content.is_empty() {
        let message = MessageResponse { message: "content is empty".to_string() };
        return Err((StatusCode::BAD_REQUEST, Json(message)));
    }

    let conn = utils::pool::get_conn(pool).await?;
    let comment = conn.interact(|conn| {
        diesel::insert_into(comments::table)
            .values(new_comment)
            .returning(Comment::as_returning())
            .get_result(conn)
    }).await
        .map_err(|e| response_utils::internal_error(e, None))?
        .map_err(|e| response_utils::internal_error(e, None))?;

    Ok(Json(comment))
}

pub async fn get_comment_list(
    State(pool): State<Pool>,
    Path(post_id): Path<i32>,
    Query(query): Query<HashMap<String, String>>,
) -> ResponseResult<Pagination<Comment>> {
    let conn = utils::pool::get_conn(pool).await?;

    let total = conn.interact(move |conn|
        comments::table
            .filter(comments::post_id.eq(post_id))
            .count()
            .get_result(conn)
    ).await
        .map_err(|e| response_utils::internal_error(e, None))?
        .map_err(|e| response_utils::internal_error(e, None))?;

    let page_info = pagination_utils::get_page_info(query);

    let res = conn.interact(move |conn|
        comments::table
            .filter(comments::post_id.eq(post_id))
            .order(comments::created_at.desc())
            .offset((page_info.page - 1) * page_info.page_size)
            .limit(page_info.page_size)
            .load(conn)
    ).await
        .unwrap()
        .unwrap();

    let paged_res = Pagination {
        total,
        page: page_info.page,
        page_size: page_info.page_size,
        results: res,
    };

    Ok(Json(paged_res))
}
