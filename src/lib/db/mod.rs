use sqlx::PgPool;
use super::models::*;

pub async fn get_user_by_id(pool: &PgPool, user_id: i32) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM Users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_one(pool)
    .await?;
    Ok(user)
}

pub async fn create_user(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
    name: &str,
    description: Option<&str>,
    profile_photo_url: Option<&str>,
) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO Users (email, password_hash, name, description, profile_photo_url)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
        email,
        password_hash,
        name,
        description,
        profile_photo_url
    )
    .fetch_one(pool)
    .await?;
    Ok(user)
}
