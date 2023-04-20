use axum::{http::StatusCode, response::Json, extract::{State}};
use axum_sessions::{extractors::{ReadableSession, WritableSession}};
use diesel::prelude::*;
use deadpool_diesel::postgres::Pool;

use crate::schema::users;
use crate::models::{CustomResponse, UserLogin, UserRegister, User, UserInfo};
use crate::utils;

async fn register_user(
    State(pool): State<Pool>,
    email: &str,
    password: &str,
) -> Result<Json<CustomResponse<UserInfo>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(utils::internal_error)?;

    let user = UserRegister {
        email: email.to_string(),
        password: utils::hash_password(&password.to_string()),
    };

    let user: User = conn
        .interact(move |conn| {
            diesel::insert_into(users::table)
                .values(user)
                .returning(User::as_returning())
                .get_result(conn)
        })
        .await
        // TODO: catch error
        // return { ok: false... }
        .map_err(utils::internal_error)?
        .map_err(utils::internal_error)?;

    let user_info = utils::get_user_info(&user);
    utils::get_success_response(user_info)
}

pub async fn register(
    State(pool): State<Pool>,
    mut session: WritableSession,
    Json(user_register): Json<UserRegister>,
) -> Result<Json<CustomResponse<UserInfo>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(utils::internal_error)?;

    let email = user_register.email;
    let password = user_register.password;

    let user_none = utils::get_none_user();
    let user_none2 = utils::get_none_user();

    let email_clone = email.clone();
    let user_result = conn.interact(move |conn|
        users::table
            .select(User::as_select())
            .filter(users::email.eq(email_clone))
            .first(conn)
            .unwrap_or(user_none)
    ).await
        .unwrap_or(user_none2);

    let found_user: Option<User> = if user_result.id > 0 { Some(user_result) } else { None };
    match found_user {
        Some(_user) => {
            utils::get_failed_response("Email have been registered.")
        }
        None => {
            let res = register_user(State(pool), &email, &password).await?;
            if res.ok {
                session.insert("user_email", &email)
                    .map_err(utils::internal_error)?;
            }

            Ok(res)
        }
    }
}

pub async fn login(
    State(pool): State<Pool>,
    mut session: WritableSession,
    Json(user_login): Json<UserLogin>,
) -> Result<Json<CustomResponse<UserInfo>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(utils::internal_error)?;

    let user = conn.interact(|conn|
        users::table
            .select(User::as_select())
            .filter(users::email.eq(user_login.email))
            .first(conn)
    ).await
        .map_err(utils::internal_error)?
        .map_err(utils::internal_error)?;

    return if utils::verify_password(&user_login.password, &user.password) {
        session
            .insert("user_email", &user.email)
            .expect("Failed to login.");

        let user_info = utils::get_user_info(&user);
        utils::get_success_response(user_info)
    } else {
        utils::get_failed_response("Invalid credentials.")
    };
}

pub async fn get_user(
    State(pool): State<Pool>,
    session: ReadableSession,
) -> Result<Json<CustomResponse<UserInfo>>, (StatusCode, String)> {
    let null_string = String::from("");
    let email = session.get::<String>("user_email").unwrap_or(null_string);

    return if email.len() > 0 {
        let conn = pool.get().await.map_err(utils::internal_error)?;

        let user = conn.interact(|conn|
            users::table
                .select(User::as_select())
                .filter(users::email.eq(email))
                .first(conn)
        ).await
            .map_err(utils::internal_error)?
            .map_err(utils::internal_error)?;

        let user_info = utils::get_user_info(&user);
        utils::get_success_response(user_info)
    } else {
        utils::get_failed_response("Not logged in.")
    };
}

pub async fn logout(
    mut session: WritableSession
) -> Result<Json<CustomResponse<String>>, (StatusCode, String)> {
    session.destroy();
    utils::get_success_response("".to_string())
}
