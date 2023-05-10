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
                price=EXCLUDED.price,
                stock=EXCLUDED.stock
            RETURNING
                id, name, price, stock;
        "#;
        let mut tx = pool.begin().await?;
        let saved_product: Product = match sqlx::query_as(&sql)
            .bind(input.name)
            .bind(input.price)
            .bind(input.stock)
            .fetch_one(&mut tx)
            .await
        {
            Ok(p) => p,
            Err(_) => {
                tx.rollback().await?;
                return Err(Error::RowNotFound);
            }
        };
        tx.commit().await?;
        Ok(saved_product)
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
        let mut tx = pool.begin().await.unwrap();
        let deleted_product: Product = match sqlx::query_as(&sql).bind(id).fetch_one(&mut tx).await
        {
            Ok(p) => p,
            Err(_) => {
                tx.rollback().await?;
                return Err(Error::RowNotFound);
            }
        };
        tx.commit().await.unwrap();
        Ok(deleted_product)
    }

    async fn list(&self) -> Result<Vec<Product>, Error> {
        let pool = self.pool;
        let sql = "SELECT * FROM products ORDER BY id ASC";
        let products = sqlx::query_as(&sql).fetch_all(pool).await;
        products
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::database::db_pool;

    #[tokio::test]
    async fn test_product_repository() -> Result<(), Error> {
        let pool = db_pool().await;
        let product_impl = ProductsImpl { pool: &pool };
        let product_input = ProductInput {
            name: "example定食".to_string(),
            price: 400,
            stock: 100,
        };
        let tx = pool.begin().await.unwrap();
        let saved_product = product_impl.save(product_input).await?;
        let expected_product = Product {
            id: saved_product.id,
            name: "example定食".to_string(),
            price: 400,
            stock: 100,
        };
        assert_eq!(expected_product, saved_product);
        let expected_found_product = expected_product;
        let all_product = product_impl.list().await?;
        let found_product = all_product
            .into_iter()
            .find(|product| product.id == expected_found_product.id);
        assert!(found_product.is_some());

        let expected_deleted_product = expected_found_product;
        let deleted_product = product_impl.delete(saved_product.id).await?;
        assert_eq!(expected_deleted_product, deleted_product);
        tx.rollback().await?;
        Ok(())
    }
}
