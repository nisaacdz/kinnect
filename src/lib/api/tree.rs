use axum::response::{Html, IntoResponse};

pub fn get_tree() -> impl IntoResponse {
    Html("get_tree".to_string())
}

pub fn update_tree() -> impl IntoResponse {
    Html("update_tree".to_string())
}

pub fn delete_tree() -> impl IntoResponse {
    Html("delete_tree".to_string())
}
