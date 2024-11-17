use std::{env, sync::Arc};

use super::models::*;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::sync::OnceCell;
const DATABASE_POOL: OnceCell<Arc<PgPool>> = OnceCell::const_new();

pub async fn get_pool() -> Arc<PgPool> {
    dotenv().ok();
    DATABASE_POOL
        .get_or_init(|| async {
            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
                .await
                .expect("Failed to create pool");
            Arc::new(pool)
        })
        .await.clone()
}

pub async fn get_family_tree_by_id(family_tree_id: i32) -> Result<FamilyTree, sqlx::Error> {
    let family_tree = sqlx::query_as!(
        FamilyTree,
        r#"
        SELECT * FROM FamilyTrees
        WHERE id = $1
        "#,
        family_tree_id
    )
    .fetch_one(&*get_pool().await)
    .await?;
    Ok(family_tree)
}

pub async fn get_all_nodes_in_tree(family_tree_id: i32) -> Result<Vec<Node>, sqlx::Error> {
    let nodes = sqlx::query_as!(
        Node,
        r#"
        SELECT * FROM Nodes
        WHERE family_tree_id = $1
        "#,
        family_tree_id
    )
    .fetch_all(&*get_pool().await)
    .await?;
    Ok(nodes)
}

pub async fn get_user_by_email(email: &str) -> Result<User, sqlx::Error> {
    let mut user = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM Users
        WHERE email = $1
        "#,
        email
    )
    .fetch_one(&*get_pool().await)
    .await?;
    user.password_hash.take();
    Ok(user)
}

pub async fn get_user_by_id(user_id: i32) -> Result<User, sqlx::Error> {
    let mut user = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM Users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_one(&*get_pool().await)
    .await?;
    user.password_hash.take();
    Ok(user)
}

pub async fn create_user(
    email: &str,
    password_hash: &str,
    name: &str,
    description: Option<&str>,
    profile_photo_url: Option<&str>,
) -> Result<User, sqlx::Error> {
    let mut user = sqlx::query_as!(
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
    .fetch_one(&*get_pool().await)
    .await?;
    user.password_hash.take();
    Ok(user)
}
