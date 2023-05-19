use std::collections::HashMap;
use axum::{
    http::StatusCode,
    response::Json,
    extract::{State, Query, rejection::JsonRejection},
};
use axum_sessions::extractors::{ReadableSession, WritableSession};
use diesel::prelude::*;
use deadpool_diesel::postgres::Pool;

use crate::schema::users;
use crate::utils;
use crate::modules::{
    request::utils as request_utils,
    response::{
        models::{MessageResponse, ResponseResult},
        utils as response_utils,
    },
    session::utils as session_utils,
    user::{models::{User, UserInfo}, utils as user_utils},
};
use super::{models::{UserRegister, UserLogin}, utils as auth_utils};

async fn register_user(
    State(pool): State<Pool>,
    email: &str,
    password: &str,
) -> ResponseResult<UserInfo> {
    let conn = utils::pool::get_conn(pool).await?;
    let user = UserRegister {
        email: email.to_string(),
        password: auth_utils::hash_password(&password.to_string()),
    };

    let user: User = conn.interact(move |conn| {
        diesel::insert_into(users::table)
            .values(&user)
            .returning(User::as_returning())
            .get_result(conn)
    }).await
        .map_err(|e| response_utils::internal_error(e, None))?
        .map_err(|e| response_utils::internal_error(e, None))?;

    let domain = std::env::var("DOMAIN").unwrap();
    let token = auth_utils::hash_password(&user.email);
    let verify_email_url = format!("https://{}/auth/verify_email?token={}", domain, token);
    let content = format!("请打开链接，完成邮箱验证：{}", verify_email_url);
    auth_utils::send_email(user.email.as_str(), "请验证邮箱", &content);

    let user_info = user_utils::get_user_info(&user);
    Ok(Json(user_info))
}

pub async fn register(
    State(pool): State<Pool>,
    payload: Result<Json<UserLogin>, JsonRejection>,
) -> ResponseResult<UserInfo> {
    let user_register = request_utils::parse_body::<UserLogin>(payload)?;

    let email_exists = auth_utils::check_email(&user_register.email).await;
    if !email_exists {
        let message = MessageResponse { message: "email is invalid".to_string() };
        return Err((StatusCode::BAD_REQUEST, Json(message)));
    }

    if user_register.password.len() < 6 {
        let message = MessageResponse { message: "password is too short".to_string() };
        return Err((StatusCode::BAD_REQUEST, Json(message)));
    }

    let conn = utils::pool::get_conn(pool.clone()).await?;

    let email = user_register.email.clone();
    let user_result = conn.interact(|conn|
        users::table
            .select(User::as_select())
            .filter(users::email.eq(email))
            .first(conn)
            .ok()
    ).await.unwrap();

    match user_result {
        Some(_user) => {
            let message = MessageResponse { message: "email have been registered".to_string() };
            Err((StatusCode::BAD_REQUEST, Json(message)))
        }
        None => {
            let user = register_user(
                State(pool),
                &user_register.email,
                &user_register.password,
            ).await?;
            Ok(user)
        }
    }
}

pub async fn verify_email(
    Query(query): Query<HashMap<String, String>>,
) -> ResponseResult<MessageResponse> {
    let token = query.get("token").unwrap_or(&String::from("")).to_string();
    let email = auth_utils::hash_password(&token);
    let email_exists = auth_utils::check_email(&email).await;

    if email_exists {
        Ok(Json(MessageResponse { message: "email verified".to_string() }))
    } else {
        let message = MessageResponse { message: "email is not valid".to_string() };
        Err((StatusCode::BAD_REQUEST, Json(message)))
    }
}

pub async fn login(
    State(pool): State<Pool>,
    mut session: WritableSession,
    payload: Result<Json<UserLogin>, JsonRejection>,
) -> ResponseResult<UserInfo> {
    let user_login = request_utils::parse_body(payload)?;
    let message_str = "email or password not valid";
    let conn = utils::pool::get_conn(pool).await?;

    let email = user_login.email.clone();
    let user = conn.interact(|conn|
        users::table
            .select(User::as_select())
            .filter(users::email.eq(email))
            .first(conn)
    ).await
        .map_err(|e| response_utils::bad_request_error(
            e, Some(message_str.to_string()),
        ))?
        .map_err(|e| response_utils::bad_request_error(
            e, Some(message_str.to_string()),
        ))?;

    return if auth_utils::verify_password(&user_login.password, &user.password) {
        session.insert("user_id", &user.id)
            .expect("failed to login");
        session.insert("user_email", &user.email)
            .expect("failed to login");
        session.insert("user_role", &user.role)
            .expect("failed to login");

        let user_info = user_utils::get_user_info(&user);
        Ok(Json(user_info))
    } else {
        let message = MessageResponse { message: message_str.to_string() };
        Err((StatusCode::BAD_REQUEST, Json(message)))
    };
}

pub async fn get_user(
    State(pool): State<Pool>,
    session: ReadableSession,
) -> ResponseResult<UserInfo> {
    let email = session_utils::get_user_email(session).unwrap().0;

    if email.len() > 0 {
        let conn = utils::pool::get_conn(pool).await?;
        let user = conn.interact(|conn|
            users::table
                .select(User::as_select())
                .filter(users::email.eq(email))
                .first(conn)
        ).await
            .map_err(|e| response_utils::internal_error(e, None))?
            .map_err(|e| response_utils::internal_error(e, None))?;

        let user_info = user_utils::get_user_info(&user);
        Ok(Json(user_info))
    } else {
        let message = MessageResponse { message: "not logged in".to_string() };
        Err((StatusCode::BAD_REQUEST, Json(message)))
    }
}

pub async fn logout(mut session: WritableSession) -> ResponseResult<MessageResponse> {
    session.destroy();
    let message = MessageResponse { message: "logout success".to_string() };
    Ok(Json(message))
}
