use axum::{http::StatusCode, response::Json, extract::{State, Path}};
use axum_sessions::extractors::ReadableSession;
use diesel::prelude::*;
use deadpool_diesel::postgres::Pool;

use crate::schema::postscripts;
use crate::utils;

use crate::modules::{
    response::utils as response_utils,
    postscript::models::{Postscript, NewPostscript},
};

pub async fn create_postscript(
    State(pool): State<Pool>,
    session: ReadableSession,
    Json(new_postscript): Json<NewPostscript>,
) -> Result<Json<Postscript>, (StatusCode, String)> {
    let user_email = session.get::<String>("user_email").unwrap();
    if user_email.is_empty() {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
    }

    if new_postscript.content.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "content is empty".to_string()));
    }

    let conn = utils::pool::get_conn(pool).await?;
    let postscript = conn
        .interact(|conn| {
            diesel::insert_into(postscripts::table)
                .values(new_postscript)
                .returning(Postscript::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(response_utils::internal_error)?
        .map_err(response_utils::internal_error)?;

    Ok(Json(postscript))
}

pub async fn get_postscript_list(
    State(pool): State<Pool>,
    Path(post_id): Path<i32>,
) -> Result<Json<Vec<Postscript>>, (StatusCode, String)> {
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
