use axum::http::StatusCode;

pub fn bad_request_error<E>(_err: E) -> (StatusCode, String)
    where E: std::error::Error {
    (StatusCode::BAD_REQUEST, "HTTP 400 Bad Request".to_string())
}

pub fn not_found_error<E>(_err: E) -> (StatusCode, String)
    where E: std::error::Error {
    (StatusCode::NOT_FOUND, "HTTP 404 Not Found".to_string())
}

pub fn internal_error<E>(_err: E) -> (StatusCode, String)
    where E: std::error::Error {
    (StatusCode::INTERNAL_SERVER_ERROR, "HTTP 500 Internal Server Error".to_string())
}
