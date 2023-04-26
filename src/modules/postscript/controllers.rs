use axum::{http::StatusCode, response::Json, extract::{State, Path}};
use axum_sessions::extractors::ReadableSession;
use diesel::prelude::*;
use deadpool_diesel::postgres::Pool;

use crate::schema::{posts, postscripts};
use crate::utils;

use crate::modules::{
    response::{
        models::{MessageResponse, ResponseResult},
        utils as response_utils,
    },
    post::models::Post,
};
use super::models::{Postscript, NewPostscript};

pub async fn create_postscript(
    State(pool): State<Pool>,
    session: ReadableSession,
    Json(new_postscript): Json<NewPostscript>,
) -> ResponseResult<Postscript> {
    let user_email = session.get::<String>("user_email").unwrap();
    if user_email.is_empty() {
        let message = MessageResponse { message: "Unauthorized".to_string() };
        return Err((StatusCode::UNAUTHORIZED, Json(message)));
    }

    if new_postscript.content.is_empty() {
        let message = MessageResponse { message: "content is empty".to_string() };
        return Err((StatusCode::BAD_REQUEST, Json(message)));
    }

    let conn = utils::pool::get_conn(pool).await?;

    // check if post exists
    conn.interact(move |conn| {
        posts::table
            .select(Post::as_select())
            .filter(posts::id.eq(new_postscript.post_id))
            .first(conn)
    })
        .await
        .map_err(|e|
            response_utils::bad_request_error(
                e, Some("post not found".to_string()),
            )
        )?
        .map_err(|e|
            response_utils::bad_request_error(
                e, Some("post not found".to_string()),
            )
        )?;

    // create postscript
    let postscript = conn
        .interact(|conn| {
            diesel::insert_into(postscripts::table)
                .values(new_postscript)
                .returning(Postscript::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(|e| response_utils::internal_error(e, None))?
        .map_err(|e| response_utils::internal_error(e, None))?;

    Ok(Json(postscript))
}

pub async fn get_postscript_list(
    State(pool): State<Pool>,
    Path(post_id): Path<i32>,
) -> ResponseResult<Vec<Postscript>> {
    let conn = utils::pool::get_conn(pool).await?;

    let postscript_list = conn.interact(move |conn|
        postscripts::table
            .filter(postscripts::post_id.eq(post_id))
            .order(postscripts::created_at.desc())
            .load(conn)
    )
        .await
        .unwrap()
        .unwrap();

    Ok(Json(postscript_list))
}
