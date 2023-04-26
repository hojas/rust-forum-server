use axum::{
    http::StatusCode,
    response::Json,
};

pub type ResponseResult<T> = Result<Json<T>, (StatusCode, Json<MessageResponse>)>;

#[derive(serde::Serialize, Debug)]
pub struct MessageResponse {
    pub message: String,
}
