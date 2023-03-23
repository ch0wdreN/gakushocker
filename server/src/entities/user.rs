use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Clone, SimpleObject, Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    pub id: i32,
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub point: i32,
}
