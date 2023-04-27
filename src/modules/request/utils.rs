use axum::{
    http::StatusCode,
    response::Json,
    extract::rejection::JsonRejection,
};
use crate::modules::response::{
    models::{
        MessageResponse,
        ResponseResult,
    },
    utils as response_utils,
};

// https://docs.rs/axum/latest/axum/extract/index.html#optional-extractors
pub fn parse_body<T>(body: Result<Json<T>, JsonRejection>) -> ResponseResult<T> {
    match body {
        Ok(body) => {
            Ok(Json(body.0))
        }
        Err(_) => {
            let message = MessageResponse { message: "payload is invalid".to_string() };
            return Err((StatusCode::BAD_REQUEST, Json(message)));
        }
    }
}

pub fn parse_path_param_i32(path_param: String) -> ResponseResult<i32> {
    let param = path_param.parse::<i32>().map_err(response_utils::not_found_error)?;
    Ok(Json(param))
}
