use std::collections::HashMap;
use axum::{
    http::StatusCode,
    response::Json,
    extract::{State, Path, Query, rejection::JsonRejection},
};
use axum_sessions::extractors::ReadableSession;
use diesel::prelude::*;
use deadpool_diesel::postgres::Pool;

use crate::schema::{posts, collected_posts};
use crate::utils;
use crate::modules::{
    request::utils as request_utils,
    response::{
        models::{MessageResponse, ResponseResult},
        utils as response_utils,
    },
    session::utils as session_utils,
    pagination::{
        models::Pagination,
        utils as pagination_utils,
    },
};
use super::models::{Post, NewPost};

pub async fn create_post(
    State(pool): State<Pool>,
    session: ReadableSession,
    payload: Result<Json<NewPost>, JsonRejection>,
) -> ResponseResult<Post> {
    // check if user is logged in
    session_utils::get_user_email(session)?.0;

    let new_post = request_utils::parse_body(payload)?.0;
    if new_post.title.is_empty() {
        let message = MessageResponse { message: "title is empty".to_string() };
        return Err((StatusCode::BAD_REQUEST, Json(message)));
    }
    if new_post.content.is_empty() {
        let message = MessageResponse { message: "content is empty".to_string() };
        return Err((StatusCode::BAD_REQUEST, Json(message)));
    }

    let conn = utils::pool::get_conn(pool).await?;
    let res = conn.interact(|conn| {
        diesel::insert_into(posts::table)
            .values(new_post)
            .returning(Post::as_returning())
            .get_result(conn)
    }).await
        .map_err(|e| response_utils::internal_error(e, None))?
        .map_err(|e| response_utils::internal_error(e, None))?;

    Ok(Json(res))
}

pub async fn get_post(
    State(pool): State<Pool>,
    Path(id): Path<String>,
) -> ResponseResult<Post> {
    let id = request_utils::parse_path_param_i32(id).unwrap().0;
    let conn = utils::pool::get_conn(pool).await?;

    let post = conn.interact(move |conn|
        posts::table
            .select(Post::as_select())
            .filter(posts::id.eq(id))
            .first(conn)
    ).await
        .map_err(response_utils::not_found_error)?
        .map_err(response_utils::not_found_error)?;

    Ok(Json(post))
}

pub async fn get_post_list(
    State(pool): State<Pool>,
    Query(query): Query<HashMap<String, String>>,
) -> ResponseResult<Pagination<Post>> {
    let conn = utils::pool::get_conn(pool).await?;

    let total = conn.interact(|conn|
        posts::table
            .count()
            .get_result(conn)
    ).await
        .map_err(|e| response_utils::internal_error(e, None))?
        .map_err(|e| response_utils::internal_error(e, None))?;

    let page_info = pagination_utils::get_page_info(query);

    let res: Vec<Post> = conn.interact(move |conn|
        posts::table
            .select(Post::as_select())
            .order(posts::created_at.desc())
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

pub async fn get_post_list_by_author_id(
    State(pool): State<Pool>,
    Path(author_id): Path<String>,
    Query(query): Query<HashMap<String, String>>,
) -> ResponseResult<Pagination<Post>> {
    let author_id = request_utils::parse_path_param_i32(author_id).unwrap().0;
    let conn = utils::pool::get_conn(pool).await?;

    let total = conn.interact(move |conn|
        posts::table
            .filter(posts::author_id.eq(author_id))
            .count()
            .get_result(conn)
    ).await
        .map_err(|e| response_utils::internal_error(e, None))?
        .map_err(|e| response_utils::internal_error(e, None))?;

    let page_info = pagination_utils::get_page_info(query);

    let res: Vec<Post> = conn.interact(move |conn|
        posts::table
            .select(Post::as_select())
            .filter(posts::author_id.eq(author_id))
            .order(posts::created_at.desc())
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

pub async fn get_post_list_by_collected(
    State(pool): State<Pool>,
    session: ReadableSession,
    Query(query): Query<HashMap<String, String>>,
) -> ResponseResult<Pagination<Post>> {
    let conn = utils::pool::get_conn(pool).await?;

    let user_id = session_utils::get_user_id(session)?.0;

    let total = conn.interact(move |conn|
        posts::table
            .inner_join(collected_posts::table)
            .select(posts::all_columns)
            .filter(collected_posts::user_id.eq(user_id))
            .count()
            .get_result(conn)
    ).await
        .map_err(|e| response_utils::internal_error(e, None))?
        .map_err(|e| response_utils::internal_error(e, None))?;

    let page_info = pagination_utils::get_page_info(query);

    let res: Vec<Post> = conn.interact(move |conn|
        posts::table
            .inner_join(collected_posts::table)
            .select(posts::all_columns)
            .filter(collected_posts::user_id.eq(user_id))
            .order(posts::created_at.desc())
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

// pub async fn update_post(
//     State(pool): State<Pool>,
//     Json(update_post): Json<UpdatePost>,
// ) -> Result<Json<Post>, (StatusCode, String)> {
//     let conn = pool.get().await.map_err(utils::internal_error)?;
//
//     let res = conn
//         .interact(move |conn| {
//             diesel::update(posts::table)
//                 .filter(posts::id.eq(update_post.id))
//                 .set((
//                     posts::title.eq(update_post.title),
//                     posts::content.eq(update_post.content),
//                 ))
//                 .returning(Post::as_returning())
//                 .get_result(conn)
//         })
//         .await
//         .map_err(utils::internal_error)?
//         .map_err(utils::internal_error)?;
//
//     Ok(Json(res))
// }
