use std::time::SystemTime;
use axum_sessions::extractors::ReadableSession;
use super::models::{User, UserInfo};

pub fn get_none_user() -> User {
    let user = User {
        id: 0,
        email: "none".to_string(),
        password: "none".to_string(),
        username: "none".to_string(),
        avatar_url: "".to_string(),
        signature: "".to_string(),
        role: "".to_string(),
        last_login_at: SystemTime::now(),
        created_at: SystemTime::now(),
    };
    user
}

pub fn get_user_info(user: &User) -> UserInfo {
    let user_info = UserInfo {
        id: user.id,
        email: user.email.clone(),
        username: user.username.clone(),
        avatar_url: user.avatar_url.clone(),
        signature: user.signature.clone(),
        role: user.role.clone(),
        last_login_at: user.last_login_at,
        created_at: user.created_at,
    };
    user_info
}

pub fn is_admin(session: ReadableSession) -> bool {
    let role = session.get::<String>("user_role").unwrap_or("".to_string());
    role == "admin"
}
