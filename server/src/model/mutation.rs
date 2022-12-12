use async_graphql::{Context, InputObject, Object};
use sqlx::postgres::PgPool;

use crate::model::query::Order;
use crate::model::query::MenuRecord;

#[derive(InputObject)]
struct OrderInput {
    menu: String,
    price: i32,
}

#[derive(InputObject)]
struct Menu {
    menu: String,
    price: i32,
    stock: i32,
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_order(
        &self,
        ctx: &Context<'_>,
        input: OrderInput,
    ) -> Result<Order, async_graphql::Error> {
        let pool = ctx.data::<PgPool>()?;
        let mut tx = pool.begin().await?;
        let sql =
            "insert into orders (menu, price, ordered_at) values ($1, $2, CURRENT_TIMESTAMP) returning id, menu, price, ordered_at;";
        let order: Order = sqlx::query_as(sql)
            .bind(input.menu)
            .bind(input.price)
            .fetch_one(&mut tx)
            .await?;
        tx.commit().await?;
        Ok(order)
    }

    async fn create_menu(
        &self,
        ctx: &Context<'_>,
        input: Menu,
    ) -> Result<MenuRecord, async_graphql::Error> {
        let pool = ctx.data::<PgPool>()?;
        let mut tx = pool.begin().await?;
        let sql = "insert into menu (menu, price, stock) values ($1, $2, $3) returning id, menu, price, stock;";
        let menu: MenuRecord = sqlx::query_as(sql)
            .bind(input.menu)
            .bind(input.price)
            .bind(input.stock)
            .fetch_one(&mut tx)
            .await?;
        tx.commit().await?;
        Ok(menu)
    }
}
