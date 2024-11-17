use axum::{extract::Path, response::IntoResponse, Json};

use crate::lib::{error::ClientError, models::User};


pub fn register_user() -> Json<()> {
    unimplemented!("register_user")
}

pub fn login_user() -> impl IntoResponse {
    Json(())
}

pub fn get_user(Path(_username): Path<String>) -> Result<Json<User>, ClientError> {
    unimplemented!("get_user")
}

pub fn update_user(Path(_username): Path<String>) -> impl IntoResponse {
    Json(())
}
