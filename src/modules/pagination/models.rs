#[derive(serde::Serialize)]
pub struct PageInfo {
    pub page: i64,
    pub page_size: i64,
}

#[derive(serde::Serialize)]
pub struct Pagination<T> {
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub results: Vec<T>,
}
