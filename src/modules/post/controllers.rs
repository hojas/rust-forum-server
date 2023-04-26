use std::collections::HashMap;
use axum::{http::StatusCode, response::Json, extract::{State, Path, Query}};
use axum_sessions::extractors::ReadableSession;
use diesel::prelude::*;
use deadpool_diesel::postgres::Pool;

use crate::schema::posts;
use crate::utils;

use crate::modules::{
    response::utils as response_utils,
    pagination::{models::Pagination, utils as pagination_utils},
    post::models::{Post, NewPost},
};

pub async fn create_post(
    State(pool): State<Pool>,
    session: ReadableSession,
    Json(new_post): Json<NewPost>,
) -> Result<Json<Post>, (StatusCode, String)> {
    let user_email = session.get::<String>("user_email").unwrap();
    if user_email.is_empty() {
        return Err((StatusCode::NOT_FOUND, "Not Found".to_string()));
    }

    if new_post.title.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "title is empty".to_string()));
    }
    if new_post.content.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "content is empty".to_string()));
    }

    let conn = utils::pool::get_conn(pool).await?;

    let res = conn
        .interact(|conn| {
            diesel::insert_into(posts::table)
                .values(new_post)
                .returning(Post::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(response_utils::internal_error)?
        .map_err(response_utils::internal_error)?;

    Ok(Json(res))
}

pub async fn get_post(
    State(pool): State<Pool>,
    Path(id): Path<i32>,
) -> Result<Json<Post>, (StatusCode, String)> {
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
) -> Result<Json<Pagination<Post>>, (StatusCode, String)> {
    let conn = utils::pool::get_conn(pool).await?;

    let total = conn.interact(|conn|
        posts::table
            .count()
            .get_result(conn)
            .unwrap()
    ).await
        .map_err(response_utils::internal_error)
        .unwrap();

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
