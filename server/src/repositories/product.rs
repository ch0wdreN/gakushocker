use crate::entities::product::Product;
use crate::repository_impl::product::ProductInput;
use sqlx::Error;

#[axum::async_trait]
pub trait ProductsRepository {
    async fn save(&self, input: ProductInput) -> Result<Product, Error>;
    async fn delete(&self, id: i32) -> Result<Product, Error>;
    async fn list(&self) -> Result<Vec<Product>, Error>;
}
