use crate::models::{User, UserInfo};

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
