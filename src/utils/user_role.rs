use axum_sessions::extractors::ReadableSession;

pub fn is_admin(session: ReadableSession) -> bool {
    let role = session.get::<String>("user_role").unwrap_or("".to_string());
    role == "admin"
}
