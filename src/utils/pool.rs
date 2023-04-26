use axum::{http::StatusCode};
use deadpool_diesel::postgres::{Pool, Connection};

use crate::modules::response::utils;

pub async fn get_conn(pool: Pool) -> Result<Connection, (StatusCode, String)> {
    let conn = pool.get().await.map_err(utils::internal_error)?;
    Ok(conn)
}
