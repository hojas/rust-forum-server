use axum::{http::StatusCode, response::Json};
use axum_sessions::extractors::ReadableSession;
use crate::modules::response::models::{MessageResponse, ResponseResult};

pub fn get_user_id(session: ReadableSession) -> ResponseResult<i32> {
    let user_id = session.get::<i32>("user_id").unwrap();
    if user_id == 0 {
        let message = MessageResponse { message: "unauthorized".to_string() };
        return Err((StatusCode::UNAUTHORIZED, Json(message)));
    }
    Ok(Json(user_id))
}

pub fn get_user_email(session: ReadableSession) -> ResponseResult<String> {
    let user_email = session.get::<String>("user_email").unwrap();
    if user_email.is_empty() {
        let message = MessageResponse { message: "unauthorized".to_string() };
        return Err((StatusCode::UNAUTHORIZED, Json(message)));
    }
    Ok(Json(user_email))
}

pub fn is_admin(session: ReadableSession) -> ResponseResult<bool> {
    let role = session.get::<String>("user_role").unwrap_or("".to_string());

    return if role.eq("admin") {
        Ok(Json(true))
    } else {
        let message = MessageResponse { message: "not found".to_string() };
        Err((StatusCode::NOT_FOUND, Json(message)))
    }
}
