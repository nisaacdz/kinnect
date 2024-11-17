use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub profile_photo_url: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
