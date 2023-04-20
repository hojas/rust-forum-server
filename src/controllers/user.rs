use axum::response::Json;

pub async fn get_user() -> Json<&'static str> {
    Json("user")
}
