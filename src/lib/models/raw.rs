use time::PrimitiveDateTime;
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
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct UserAuthProvider {
    pub id: i32,
    pub user_id: i32,
    pub provider: String,
    pub provider_user_id: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Node {
    pub id: i32,
    pub family_tree_id: i32,
    pub name: String,
    pub photo_url: Option<String>,
    pub description: Option<String>,
    pub alias: Option<String>,
    pub user_id: Option<i32>,
    pub parent_node_id: Option<i32>,
    pub parent_relation: Option<String>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct FamilyTree {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub creator_id: i32,
    pub root_node_id: i32,
    pub visibility_level_id: i32,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct ForkedFamilyTree {
    pub id: i32,
    pub original_tree_id: i32,
    pub forked_by: i32,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct TreeVisibilityLevel {
    pub id: i32,
    pub level_name: String,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Admin {
    pub id: i32,
    pub user_id: i32,
    pub family_tree_id: i32,
    pub permissioned_subtree_id: i32,
    pub created_at: PrimitiveDateTime,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct MergeRequestStatus {
    pub id: i32,
    pub status_name: String,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct MergeRequest {
    pub id: i32,
    pub fork_id: i32,
    pub status_id: i32,
    pub created_at: PrimitiveDateTime,
}
