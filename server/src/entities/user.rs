use sqlx::FromRow;
use async_graphql::SimpleObject;

#[derive(FromRow, Clone, SimpleObject)]
pub struct User {
    pub id: i32,
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub point: i32,
}
