use crate::database::ConnectionPool;
use crate::entities::order::{Order, OrderInput, OrderWithoutItems};
use crate::repositories::order::OrdersRepository;
use sqlx::{Error, Postgres, QueryBuilder};
use uuid::Uuid;

pub struct OrdersImpl<'a> {
    pub pool: &'a ConnectionPool,
}

#[axum::async_trait]
impl<'a> OrdersRepository for OrdersImpl<'a> {
    async fn save(&self, input: OrderInput) -> Result<Order, Error> {
        let pool = self.pool;
        let sql = r#"
            INSERT INTO
                orders (id, user_id, total)
            VALUES
                ($1, $2, $3)
            ON CONFLICT
                (id)
            DO UPDATE SET
                total=EXCLUDED.total
        "#;
        let mut tx = pool.begin().await?;
        let insert_query = sqlx::query(sql)
            .bind(&input.id)
            .bind(&input.user_id)
            .bind(&input.total);
        if let Err(_) = insert_query.execute(&mut tx).await {
            tx.rollback().await?;
            return Err(Error::RowNotFound);
        }
        let sql = "INSERT INTO order_items (order_id, product_id, quantity) ";
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(sql);
        let items_iter = input.items.iter();
        query_builder.push_values(items_iter.take(input.items.len()), |mut s, item| {
            s.push_bind(&input.id)
                .push_bind(item.product_id)
                .push_bind(item.quantity);
        });
        let query = query_builder.build();
        query.execute(&mut tx).await?;

        let sql = "SELECT * FROM order_view WHERE id=$1";
        let orders = sqlx::query_as(&sql)
            .bind(input.id)
            .fetch_one(&mut tx)
            .await?;
        tx.commit().await?;
        Ok(orders)
    }

    async fn delete(&self, id: Uuid) -> Result<Order, Error> {
        let pool = self.pool;
        let sql = r#"
        DELETE FROM
            order_items i
        USING
            orders o
        WHERE
            o.id=i.order_id and o.id=$1
        RETURNING
            o.id, o.user_id, o.total, o.created_at
        "#;
        let mut tx = pool.begin().await?;
        let deleted_order: OrderWithoutItems =
            match sqlx::query_as(&sql).bind(id).fetch_one(&mut tx).await {
                Ok(o) => o,
                Err(_) => {
                    tx.rollback().await?;
                    return Err(Error::RowNotFound);
                }
            };
        tx.commit().await?;
        Ok(Order {
            id: deleted_order.id,
            user_id: deleted_order.user_id,
            items: vec![],
            total: deleted_order.total,
            created_at: deleted_order.created_at,
        })
    }

    async fn list(&self) -> Result<Vec<Order>, Error> {
        let pool = self.pool;
        let sql = "SELECT * FROM order_view";
        let orders = sqlx::query_as(&sql).fetch_all(pool).await?;
        Ok(orders)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::db_pool;
    use crate::entities::order::{OrderItem, OrderItemInput};

    #[tokio::test]
    async fn test_order_repository() -> Result<(), Error> {
        let pool = db_pool().await;
        let order_impl = OrdersImpl { pool: &pool };
        let order_id = Uuid::new_v4();
        let input = OrderInput {
            id: order_id,
            user_id: 1,
            total: 360,
            items: vec![OrderItemInput {
                product_id: 1,
                quantity: 1,
            }],
        };

        let tx = pool.begin().await?;

        let saved_order = order_impl.save(input.clone()).await?;
        let expected_order = Order {
            id: order_id,
            user_id: 1,
            items: vec![OrderItem {
                name: "定食A".to_string(),
                price: 420,
                quantity: 1,
            }],
            total: 360,
            created_at: saved_order.created_at,
        };
        assert_eq!(expected_order, saved_order);
        let all_order = order_impl.list().await?;
        let expect_found_order = expected_order;
        let found_order = all_order
            .into_iter()
            .find(|order| order.id == expect_found_order.id);
        assert!(found_order.is_some());
        let expected_deleted_order = Order {
            id: order_id,
            user_id: 1,
            items: vec![],
            total: 360,
            created_at: saved_order.created_at,
        };
        let deleted_order = order_impl.delete(order_id).await?;
        assert_eq!(expected_deleted_order, deleted_order);
        tx.rollback().await?;
        Ok(())
    }
}
