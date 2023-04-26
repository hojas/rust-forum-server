use std::collections::HashMap;
use axum::{http::StatusCode, response::Json, extract::{State, Query}};
use axum_sessions::extractors::{ReadableSession, WritableSession};
use diesel::prelude::*;
use deadpool_diesel::postgres::Pool;

use crate::schema::users;
use crate::modules::{
    response::{models::MessageResponse, utils as response_utils},
    user::{models::{User, UserInfo}, utils as user_utils},
    auth::{models::{UserRegister, UserLogin}, utils as auth_utils},
};
use crate::utils;

async fn register_user(
    State(pool): State<Pool>,
    email: &str,
    password: &str,
) -> Result<Json<UserInfo>, (StatusCode, String)> {
    let conn = utils::pool::get_conn(pool).await?;
    let user = UserRegister {
        email: email.to_string(),
        password: auth_utils::hash_password(&password.to_string()),
    };

    let user: User = conn
        .interact(move |conn| {
            diesel::insert_into(users::table)
                .values(&user)
                .returning(User::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(response_utils::internal_error)?
        .map_err(response_utils::internal_error)?;

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
    mut session: WritableSession,
    Json(user_register): Json<UserRegister>,
) -> Result<Json<UserInfo>, (StatusCode, String)> {
    let email_exists = auth_utils::check_email(&user_register.email).await;
    if !email_exists {
        return Err((StatusCode::BAD_REQUEST, "Email is not valid.".to_string()));
    }

    let conn = utils::pool::get_conn(pool.clone()).await?;

    let email = user_register.email;
    let password = user_register.password;

    let user_none = user_utils::get_none_user();
    let user_none2 = user_utils::get_none_user();

    let email_clone = email.clone();
    let user_result = conn.interact(move |conn|
        users::table
            .select(User::as_select())
            .filter(users::email.eq(email_clone))
            .first(conn)
            .unwrap_or(user_none)
    )
        .await
        .unwrap_or(user_none2);

    let found_user: Option<User> = if user_result.id > 0 { Some(user_result) } else { None };
    match found_user {
        Some(_user) => {
            Err((StatusCode::BAD_REQUEST, "Email have been registered.".to_string()))
        }
        None => {
            let user = register_user(State(pool), &email, &password).await?;
            session
                .insert("user_email", &email)
                .map_err(response_utils::internal_error)?;
            Ok(user)
        }
    }
}

pub async fn verify_email(
    Query(query): Query<HashMap<String, String>>,
) -> Result<Json<MessageResponse>, (StatusCode, String)> {
    let token = query.get("token").unwrap_or(&String::from("")).to_string();
    let email = auth_utils::hash_password(&token);
    let email_exists = auth_utils::check_email(&email).await;
    if email_exists {
        Ok(Json(MessageResponse { message: "Email is valid.".to_string() }))
    } else {
        Err((StatusCode::BAD_REQUEST, "Email is not valid.".to_string()))
    }
}

pub async fn login(
    State(pool): State<Pool>,
    mut session: WritableSession,
    Json(user_login): Json<UserLogin>,
) -> Result<Json<UserInfo>, (StatusCode, String)> {
    let conn = utils::pool::get_conn(pool).await?;

    let user = conn.interact(|conn|
        users::table
            .select(User::as_select())
            .filter(users::email.eq(user_login.email))
            .first(conn)
    ).await
        .map_err(response_utils::internal_error)?
        .map_err(response_utils::internal_error)?;

    return if auth_utils::verify_password(&user_login.password, &user.password) {
        session
            .insert("user_email", &user.email)
            .expect("Failed to login.");
        session
            .insert("user_role", &user.role)
            .expect("Failed to login.");

        let user_info = user_utils::get_user_info(&user);
        Ok(Json(user_info))
    } else {
        Err((StatusCode::BAD_REQUEST, "Invalid credentials.".to_string()))
    };
}

pub async fn get_user(
    State(pool): State<Pool>,
    session: ReadableSession,
) -> Result<Json<UserInfo>, (StatusCode, String)> {
    let null_string = String::from("");
    let email = session.get::<String>("user_email").unwrap_or(null_string);

    if email.len() > 0 {
        let conn = utils::pool::get_conn(pool).await?;

        let user = conn.interact(|conn|
            users::table
                .select(User::as_select())
                .filter(users::email.eq(email))
                .first(conn)
        ).await
            .map_err(response_utils::internal_error)?
            .map_err(response_utils::internal_error)?;

        let user_info = user_utils::get_user_info(&user);
        Ok(Json(user_info))
    } else {
        Err((StatusCode::BAD_REQUEST, "Not logged in.".to_string()))
    }
}

pub async fn logout(mut session: WritableSession) -> Result<Json<String>, (StatusCode, String)> {
    session.destroy();
    Ok(Json("".to_string()))
}
