use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use sqlx::postgres::{PgHasArrayType, PgTypeInfo};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(FromRow, Clone, SimpleObject)]
pub struct Order {
    pub id: Uuid,
    pub user_id: i32,
    pub items: Vec<OrderItem>,
    pub total: i32,
    pub created_at: NaiveDateTime,
    pub status: String,
}

#[derive(FromRow, Clone, SimpleObject, Type)]
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
