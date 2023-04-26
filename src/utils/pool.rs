use axum::{http::StatusCode, response::Json};
use deadpool_diesel::postgres::{Pool, Connection};

use crate::modules::response::{
    models::MessageResponse,
    utils,
};

pub async fn get_conn(pool: Pool) -> Result<Connection, (StatusCode, Json<MessageResponse>)> {
    let conn = pool.get()
        .await
        .map_err(|e| utils::internal_error(e, None))?;
    Ok(conn)
}
