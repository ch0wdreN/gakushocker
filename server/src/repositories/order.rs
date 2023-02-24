use crate::database::ConnectionPool;
use crate::entities::order::Order;
use anyhow::Result;
use async_graphql::InputObject;

#[axum::async_trait]
pub trait OrdersRepository {
    async fn save(&self, input: OrderInput) -> Result<Order>;
    async fn delete(&self, id: i32) -> Result<Order>;
    async fn find_by_id(&self, id: i32) -> Result<Order>;
    async fn find_all_by_id(&self, id: i32) -> Result<Vec<Order>>;
    async fn get_all(&self) -> Result<Vec<Order>>;
}

#[derive(InputObject)]
pub struct OrderInput {
    pub id: i32,
    pub user_id: i32,
    pub total: i32,
    pub status: String,
}

pub struct OrdersImpl<'a> {
    pub pool: &'a ConnectionPool,
}

#[axum::async_trait]
impl<'a> OrdersRepository for OrdersImpl<'a> {
    async fn save(&self, input: OrderInput) -> Result<Order> {
        let pool = self.pool;
        let sql = r#"
            INSERT INTO
                orders (id, user_id, total, status)
            VALUES
                ($1, $2, $3, $4)
            ON CONFLICT
                (id)
            DO UPDATE SET
                id=EXCLUDED.id,
                user_id=EXCLUDED.user_id,
                total=EXCLUDED.total,
                status=EXCLUDED.status
            RETURNING
                id, user_id, total, created_at, status;
        "#;
        let tx = pool.begin().await.unwrap();
        let saved_order = sqlx::query_as(&sql)
            .bind(input.id)
            .bind(input.user_id)
            .bind(input.total)
            .bind(input.status)
            .fetch_one(pool)
            .await
            .unwrap();
        tx.commit().await.unwrap();
        Ok(saved_order)
    }

    async fn delete(&self, id: i32) -> Result<Order> {
        let pool = self.pool;
        let sql = r#"
            DELETE FROM
                orders
            WHERE
                id=$1
            RETURNING
                id, user_id, total, created_at, status;
        "#;
        let tx = pool.begin().await.unwrap();
        let deleted_order = sqlx::query_as(&sql).bind(id).fetch_one(pool).await.unwrap();
        tx.commit().await.unwrap();
        Ok(deleted_order)
    }

    async fn find_by_id(&self, id: i32) -> Result<Order> {
        let pool = self.pool;
        let sql = r#"
            SELECT * FROM
                orders
            WHERE
                id=$1
        "#;
        let order = sqlx::query_as(&sql).bind(id).fetch_one(pool).await.unwrap();
        Ok(order)
    }

    async fn find_all_by_id(&self, id: i32) -> Result<Vec<Order>> {
        let pool = self.pool;
        let sql = r#"
            SELECT * FROM
                orders
            WHERE
                user_id=$1
        "#;
        let orders = sqlx::query_as(&sql).bind(id).fetch_all(pool).await.unwrap();
        Ok(orders)
    }

    async fn get_all(&self) -> Result<Vec<Order>> {
        let pool = self.pool;
        let sql = "SELECT * FROM orders;";
        let orders = sqlx::query_as(&sql).fetch_all(pool).await.unwrap();
        Ok(orders)
    }
}
