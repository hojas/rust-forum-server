use axum::response::Json;

pub async fn get_home() -> Json<&'static str> {
    Json("rust-forum-server")
}
