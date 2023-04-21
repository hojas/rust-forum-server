use std::time::SystemTime;
use crate::models::user::User;

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
