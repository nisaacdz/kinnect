use axum::response::{Html, IntoResponse};

pub fn get_node() -> impl IntoResponse {
    Html("get_node".to_string())
}

pub fn update_node() -> impl IntoResponse {
    Html("update_node".to_string())
}

pub fn delete_node() -> impl IntoResponse {
    Html("delete_node".to_string())
}
