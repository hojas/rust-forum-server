use axum::{http::StatusCode};
use deadpool_diesel::postgres::{Pool, Connection};

use crate::utils::response_error;

pub async fn get_conn(pool: Pool) -> Result<Connection, (StatusCode, String)> {
    let conn = pool.get().await.map_err(response_error::internal_error)?;
    Ok(conn)
}
