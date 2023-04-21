#[derive(serde::Serialize)]
pub struct CustomResponse<T> {
    pub ok: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

#[derive(serde::Serialize)]
pub struct FailedResponse {
    pub ok: bool,
    pub message: String,
}
