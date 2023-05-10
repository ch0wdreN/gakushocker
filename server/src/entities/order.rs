use async_graphql::{InputObject, SimpleObject};
use chrono::NaiveDateTime;
use sqlx::postgres::{PgHasArrayType, PgTypeInfo};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(FromRow, Clone, SimpleObject, Debug, PartialEq)]
pub struct Order {
    pub id: Uuid,
    pub user_id: i32,
    pub items: Vec<OrderItem>,
    pub total: i32,
    pub created_at: NaiveDateTime,
}

#[derive(FromRow, Clone, SimpleObject, Type, Debug, PartialEq)]
pub struct OrderItem {
    pub name: String,
    pub price: i32,
    pub quantity: i32,
}

impl PgHasArrayType for OrderItem {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_items")
    }

    fn array_compatible(_ty: &PgTypeInfo) -> bool {
        true
    }
}

#[derive(InputObject, Debug, Clone)]
pub struct OrderInput {
    pub id: Uuid,
    pub user_id: i32,
    pub total: i32,
    pub items: Vec<OrderItemInput>,
}

#[derive(FromRow, InputObject, Debug, Clone)]
pub struct OrderItemInput {
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(FromRow)]
pub struct OrderWithoutItems {
    pub id: Uuid,
    pub user_id: i32,
    pub total: i32,
    pub created_at: NaiveDateTime,
}
