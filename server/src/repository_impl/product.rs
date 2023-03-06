use crate::database::ConnectionPool;
use crate::entities::product::Product;
use crate::repositories::product::ProductsRepository;
use async_graphql::InputObject;
use sqlx::Error;

#[derive(InputObject)]
pub struct ProductInput {
    pub name: String,
    pub price: i32,
    pub stock: i32,
}

pub struct ProductsImpl<'a> {
    pub pool: &'a ConnectionPool,
}

#[axum::async_trait]
impl<'a> ProductsRepository for ProductsImpl<'a> {
    async fn save(&self, input: ProductInput) -> Result<Product, Error> {
        let pool = self.pool;
        let sql = r#"
            INSERT INTO
                products (name, price, stock)
            VALUES
                ($1, $2, $3)
            ON CONFLICT
                (name)
            DO UPDATE SET
                name=EXCLUDED.name,
                price=EXCLUDED.price,
                stock=EXCLUDED.stock
            RETURNING
                id, name, price, stock;
        "#;
        let tx = pool.begin().await.unwrap();
        let saved_product = sqlx::query_as(&sql)
            .bind(input.name)
            .bind(input.price)
            .bind(input.stock)
            .fetch_one(pool)
            .await;
        tx.commit().await.unwrap();
        saved_product
    }

    async fn delete(&self, id: i32) -> Result<Product, Error> {
        let pool = self.pool;
        let sql = r#"
            DELETE FROM
                products
            WHERE
                id=$1
            RETURNING
                id, name, price, stock;
        "#;
        let tx = pool.begin().await.unwrap();
        let deleted_product = sqlx::query_as(&sql).bind(id).fetch_one(pool).await;
        tx.commit().await.unwrap();
        deleted_product
    }

    async fn list(&self) -> Result<Vec<Product>, Error> {
        let pool = self.pool;
        let sql = "SELECT * FROM products";
        let products = sqlx::query_as(&sql).fetch_all(pool).await;
        products
    }
}
