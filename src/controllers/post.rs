use std::collections::HashMap;
use axum::{http::StatusCode, response::Json, extract::{State, Query}};
use diesel::prelude::*;
use deadpool_diesel::postgres::Pool;

use crate::models::{Pagination, Post, NewPost};
use crate::schema::posts;
use crate::utils;

pub async fn create_post(
    State(pool): State<Pool>,
    Json(new_post): Json<NewPost>,
) -> Result<Json<Post>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(utils::internal_error)?;

    let res = conn
        .interact(|conn| {
            diesel::insert_into(posts::table)
                .values(new_post)
                .returning(Post::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(utils::internal_error)?
        .map_err(utils::internal_error)?;

    Ok(Json(res))
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

pub async fn get_post(
    State(pool): State<Pool>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<Json<Post>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(utils::internal_error)?;

    let id = (query.get("id").unwrap()).parse::<i32>().unwrap();

    let post = conn.interact(move |conn|
        posts::table
            .select(Post::as_select())
            .filter(posts::id.eq(id))
            .first(conn)
    ).await
        .map_err(utils::internal_error)?
        .map_err(utils::internal_error)?;

    Ok(Json(post))
}

pub async fn get_post_list(
    State(pool): State<Pool>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<Json<Pagination<Post>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(utils::internal_error)?;

    let total = conn.interact(|conn|
        posts::table
            .count()
            .get_result(conn)
            .unwrap()
    ).await
        .map_err(utils::internal_error)
        .unwrap();

    let default_page = String::from("1");
    let default_page_size = String::from("20");
    let page = (query.get("page").unwrap_or(&default_page)).parse::<i64>().unwrap();
    let page_size = (query.get("page_size").unwrap_or(&default_page_size)).parse::<i64>().unwrap();

    let res: Vec<Post> = conn.interact(move |conn|
        posts::table
            .select(Post::as_select())
            .order(posts::created_at.desc())
            .offset((page - 1) * page_size)
            .limit(page_size)
            .load(conn)
    ).await
        .map_err(utils::internal_error)?
        .map_err(utils::internal_error)?;

    let paged_res = Pagination {
        total,
        page,
        page_size,
        results: res,
    };

    Ok(Json(paged_res))
}
