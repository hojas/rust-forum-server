use axum::{http::StatusCode, response::Json};
use crate::models::CustomResponse;

pub fn get_success_response<T>(
    data: T,
) -> Result<Json<CustomResponse<T>>, (StatusCode, String)> {
    let res = CustomResponse {
        ok: true,
        data: Some(data),
        message: None,
    };
    Ok(Json(res))
}

pub fn get_failed_response<T>(
    message: &str
) -> Result<Json<CustomResponse<T>>, (StatusCode, String)> {
    let res = CustomResponse {
        ok: false,
        data: None,
        message: Some(message.to_string()),
    };
    Ok(Json(res))
}
