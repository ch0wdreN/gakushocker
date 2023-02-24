use async_graphql::SimpleObject;
use sqlx::FromRow;

#[derive(FromRow, Clone, SimpleObject)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: i32,
    pub stock: i32,
}
