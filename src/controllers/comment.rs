use std::collections::HashMap;
use axum::{http::StatusCode, response::Json, extract::{State, Path, Query}};
use axum_sessions::extractors::ReadableSession;
use diesel::prelude::*;
use deadpool_diesel::postgres::Pool;

use crate::models::{pagination::Pagination, comment::{Comment, NewComment}};
use crate::schema::comments;
use crate::utils::{self, response_error, pool};

pub async fn create_comment(
    State(pool): State<Pool>,
    session: ReadableSession,
    Json(new_comment): Json<NewComment>,
) -> Result<Json<Comment>, (StatusCode, String)> {
    let user_email = session.get::<String>("user_email").unwrap();
    if user_email.is_empty() {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
    }

    if new_comment.content.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "content is empty".to_string()));
    }

    let conn = pool::get_conn(pool).await?;

    let res = conn
        .interact(|conn| {
            diesel::insert_into(comments::table)
                .values(new_comment)
                .returning(Comment::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(response_error::internal_error)?
        .map_err(response_error::internal_error)?;

    Ok(Json(res))
}

pub async fn get_comment_list(
    State(pool): State<Pool>,
    Path(post_id): Path<i32>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<Json<Pagination<Comment>>, (StatusCode, String)> {
    let conn = pool::get_conn(pool).await?;

    let total = conn.interact(move |conn|
        comments::table
            .filter(comments::post_id.eq(post_id))
            .count()
            .get_result(conn)
            .unwrap()
    ).await
        .map_err(response_error::internal_error)
        .unwrap();

    let page_info = utils::get_page_info(query);

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
