use axum::{http::StatusCode, response::Json};
use crate::modules::response::models::MessageResponse;

pub fn not_found_error<E>(_err: E) -> (StatusCode, Json<MessageResponse>)
    where E: std::error::Error
{
    let message = MessageResponse { message: "not found".to_string() };
    (StatusCode::NOT_FOUND, Json(message))
}

pub fn bad_request_error<E>(_err: E, message: Option<String>) -> (StatusCode, Json<MessageResponse>)
    where E: std::error::Error
{
    let message = if message.is_none() {
        MessageResponse { message: "bad request".to_string() }
    } else {
        MessageResponse { message: message.unwrap() }
    };
    (StatusCode::BAD_REQUEST, Json(message))
}


pub fn internal_error<E>(_err: E, message: Option<String>) -> (StatusCode, Json<MessageResponse>)
    where E: std::error::Error
{
    let message = if message.is_none() {
        MessageResponse { message: "internal error".to_string() }
    } else {
        MessageResponse { message: message.unwrap() }
    };
    (StatusCode::INTERNAL_SERVER_ERROR, Json(message))
}
