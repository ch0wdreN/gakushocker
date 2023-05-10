use crate::constants;
use crate::repository_impl::order::OrdersImpl;
use crate::repository_impl::product::ProductsImpl;
use crate::repository_impl::user::UsersImpl;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub type ConnectionPool = Pool<Postgres>;

#[derive(Clone)]
pub struct RepositoryProvider(pub ConnectionPool);

impl RepositoryProvider {
    pub fn orders(&self) -> OrdersImpl {
        OrdersImpl { pool: &self.0 }
    }
    pub fn products(&self) -> ProductsImpl {
        ProductsImpl { pool: &self.0 }
    }
    pub fn users(&self) -> UsersImpl {
        UsersImpl { pool: &self.0 }
    }
}

pub async fn db_pool() -> ConnectionPool {
    let db_url = constants::db_url();
    PgPoolOptions::new()
        .max_connections(200)
        .connect(&db_url)
        .await
        .expect(format!("missing db {}", db_url).as_str())
}
