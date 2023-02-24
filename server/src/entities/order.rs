use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(FromRow, Clone, SimpleObject)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub total: i32,
    pub created_at: NaiveDateTime,
    pub status: String,
}
