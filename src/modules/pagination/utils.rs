use std::collections::HashMap;
use super::models::PageInfo;

pub fn get_page_info(query: HashMap<String, String>) -> PageInfo {
    let default_page = String::from("1");
    let default_page_size = String::from("20");
    let page = (query.get("page").unwrap_or(&default_page)).parse::<i64>().unwrap();
    let page_size = (query.get("page_size").unwrap_or(&default_page_size)).parse::<i64>().unwrap();

    let page_info = PageInfo {
        page,
        page_size,
    };

    page_info
}
