use crate::database::ConnectionPool;
use crate::entities::order::Order;
use crate::repositories::order::OrdersRepository;
use async_graphql::InputObject;
use sqlx::{Error, FromRow, Postgres, QueryBuilder};
use uuid::Uuid;

#[derive(InputObject)]
pub struct OrderInput {
    id: Uuid,
    user_id: i32,
    total: i32,
    items: Vec<OrderItemInput>,
    status: String,
}

#[derive(FromRow, InputObject)]
struct OrderItemInput {
    product_id: i32,
    quantity: i32,
}

#[derive(FromRow)]
struct OrderWithoutItems {
    id: Uuid,
    user_id: i32,
    total: i32,
    status: String,
    created_at: chrono::NaiveDateTime,
}

pub struct OrdersImpl<'a> {
    pub pool: &'a ConnectionPool,
}

#[axum::async_trait]
impl<'a> OrdersRepository for OrdersImpl<'a> {
    async fn save(&self, input: OrderInput) -> Result<Order, Error> {
        let pool = self.pool;
        let sql = r#"
            INSERT INTO
                orders (id, user_id, total, status)
            VALUES
                ($1, $2, $3, $4)
            ON CONFLICT
                (id)
            DO UPDATE SET
                user_id=EXCLUDED.user_id,
                status=EXCLUDED.status
        "#;
        let tx = pool.begin().await?;
        let insert_query = sqlx::query(sql)
            .bind(&input.id)
            .bind(&input.user_id)
            .bind(&input.total)
            .bind(&input.status);
        insert_query.execute(pool).await?;
        tx.commit().await?;
        let sql = "INSERT INTO order_items (order_id, product_id, quantity) ";
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(sql);
        let items_iter = input.items.iter();
        query_builder.push_values(items_iter.take(input.items.len()), |mut b, item| {
            b.push_bind(&input.id)
                .push_bind(item.product_id)
                .push_bind(item.quantity);
        });
        let query = query_builder.build();
        let tx = pool.begin().await?;
        query.execute(pool).await?;
        tx.commit().await?;
        let sql = "SELECT * FROM order_view WHERE id=$1";
        let orders = sqlx::query_as(&sql).bind(input.id).fetch_one(pool).await?;
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
            o.id, o.user_id, o.total, o.created_at, o.status
        "#;
        let tx = pool.begin().await.unwrap();
        let deleted_order: OrderWithoutItems =
            sqlx::query_as(&sql).bind(id).fetch_one(pool).await?;
        tx.commit().await.unwrap();
        Ok(Order {
            id: deleted_order.id,
            user_id: deleted_order.user_id,
            items: vec![],
            total: deleted_order.total,
            created_at: deleted_order.created_at,
            status: deleted_order.status,
        })
    }

    async fn list(&self) -> Result<Vec<Order>, Error> {
        let pool = self.pool;
        let sql = "SELECT * FROM order_view";
        let orders = sqlx::query_as(&sql).fetch_all(pool).await?;
        Ok(orders)
    }
}
