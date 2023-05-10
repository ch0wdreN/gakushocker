use crate::entities::order::{Order, OrderInput};
use sqlx::Error;
use uuid::Uuid;

#[axum::async_trait]
pub trait OrdersRepository {
    async fn save(&self, input: OrderInput) -> Result<Order, Error>;
    async fn delete(&self, id: Uuid) -> Result<Order, Error>;
    async fn list(&self) -> Result<Vec<Order>, Error>;
}
