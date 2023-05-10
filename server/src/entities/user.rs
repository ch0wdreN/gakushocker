use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Clone, SimpleObject, Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    pub id: i32,
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub point: i32,
    pub is_admin: bool,
}

#[derive(InputObject, Deserialize)]
pub struct UserInput {
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub point: i32,
    pub is_admin: bool,
}
